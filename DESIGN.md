# Архитектура и дизайн FCLC

## 1. Обзор высокоуровневой архитектуры

FCLC построен по гибридной архитектуре "Центральный Оркестратор — Локальные Узлы".
```
    [Hospital A EHR]    [Hospital B PACS]    [Clinic C LIS]
            |                    |                    |
      +-----v-----+        +-----v-----+        +-----v-----+
      |  FCLC     |        |  FCLC     |        |  FCLC     |
      |  Node A   |        |  Node B   |        |  Node C   |
      | (Adapter, |        | (Adapter, |        | (Adapter, |
      |  SecAgg   |        |  SecAgg   |        |  SecAgg   |
      |  Client)  |        |  Client)  |        |  Client)  |
      +-----+-----+        +-----+-----+        +-----+-----+
            |                    |                    |
            +---------+----------+---------+----------+
                      |                    |
               [ Secure Channel (HTTPS/mTLS) ]
                      |                    |
              +-------v-------------------v-------+
              |        FCLC Orchestrator          |
              | (Job Scheduler, Secure Aggregator,|
              |  Model Registry, Contribution Calc)|
              +-----------------------------------+
                      |                    |
              +-------v-------------------v-------+
              |       Global Model Store          |
              |    (Versioned, Encrypted at Rest) |
              +-----------------------------------+
```

## 2. Детализация компонентов

### 2.1. FCLC Node (Локальный узел)
Ответственность: Подключение к локальным данным, выполнение задач обучения/агрегации, обеспечение приватности.
```
fclc-node/
├── src/
│   ├── main.rs                          # Точка входа
│   ├── adapter/                         # Адаптеры к источникам данных
│   │   ├── mod.rs
│   │   ├── fhir_adapter.rs              # FHIR API client
│   │   ├── omop_adapter.rs              # OMOP CDM трансформатор
│   │   └── deidentify/                  # Модули деидентификации
│   │       ├── k_anonymity.rs
│   │       └── generalizer.rs
│   ├── secagg_client/                   # Клиентская часть SecAgg+
│   │   ├── mod.rs
│   │   ├── key_exchange.rs              # X25519, генерация seed_{ij}
│   │   ├── mask_generator.rs            # Генерация масок via ChaCha20
│   │   └── shamir_share.rs              # Работа с долями Шамира
│   ├── dp_noise/                        # Добавление шума DP
│   │   ├── gaussian_mechanism.rs
│   │   └── rdp_accountant.rs            # Учёт бюджента (Rényi DP)
│   ├── local_trainer/                   # Локальное обучение модели
│   │   ├── task_fetcher.rs              # Получение задачи от оркестратора
│   │   └── gradient_computer.rs
│   └── api/
│       └── node_api.rs                  # REST API для управления узлом
├── config/
│   ├── node_config.toml                 # Конфигурация узла (ключи, endpoints)
│   └── data_schema.toml                 # Схема OMOP CDM для трансформации
└── tests/
    └── integration_node_test.rs
```

**API контракт узла (REST):**
*   `POST /api/v1/task/execute` — Выполнить полученную задачу (обучение, инференс). Принимает `TaskPayload` (ID модели, гиперпараметры), возвращает `TaskResult` (градиенты/предсказания + замаскированные).
*   `GET /api/v1/status` — Возвращает статус узла (здоровье, версия, ID).
*   `POST /api/v1/secagg/setup` — Участвует в раунде обмена ключами SecAgg.

### 2.2. FCLC Orchestrator (Центральный оркестратор)
Ответственность: Координация обучения, безопасная агрегация, управление моделями, расчёт вклада.
```
fclc-orchestrator/
├── src/
│   ├── main.rs
│   ├── scheduler/                       # Планировщик заданий
│   │   ├── job_queue.rs
│   │   └── node_manager.rs              # Реестр узлов, проверка живучести
│   ├── secagg_server/                   # Серверная часть SecAgg+
│   │   ├── mod.rs
│   │   ├── aggregator.rs                # Логика агрегации замаскированных векторов
│   │   ├── dropout_handler.rs           # Восстановление при выбытии узлов
│   │   └── crypto_verifier.rs           # (Будущее) верификация NIZK proofs
│   ├── model_registry/                  # Управление версиями глобальной модели
│   │   ├── versioned_model.rs
│   │   └── storage_backend.rs           # S3 / локальная ФС
│   ├── contribution/                    # Вычисление вклада
│   │   ├── shapley_estimator.rs         # Монте‑Карло оценка значения Шепли
│   │   └── credit_ledger.rs             # Учёт "кредитов" участников
│   ├── api/
│   │   ├── admin_api.rs                 # API для администратора (запуск раундов)
│   │   └── node_api.rs                  # API для взаимодействия с узлами
│   └── privacy_accountant.rs            # Отслеживание бюджента ε_total
├── config/
│   └── orchestrator_config.toml
└── tests/
    └── integration_orchestrator_test.rs
```

**API контракт оркестратора:**
*   `POST /admin/v1/round/start` — (Админ) Запуск нового раунда федеративного обучения.
*   `POST /node/v1/secagg/commit` — Приём зафиксированных (committed) замаскированных обновлений от узлов.
*   `GET /node/v1/task/{task_id}` — Предоставление задачи узлу.
*   `GET /admin/v1/contributions/{round_id}` — Получение оценок вклада за раунд.

### 2.3. Общая библиотека (fclc-core)
Общие структуры данных, утилиты и криптографические примитивы.
```
fclc-core/
├── src/
│   ├── lib.rs
│   ├── models/                          # Структуры данных
│   │   ├── task.rs                      # TaskPayload, TaskResult
│   │   ├── node_info.rs
│   │   └── secagg_protocol.rs           # Сообщения SecAgg (MaskedVector, Share)
│   ├── aggregation/                     # **ЯДРО: SecAgg+**
│   │   ├── mod.rs
│   │   ├── secagg.rs                    // Главная логика SecAgg+
│   │   ├── keys.rs                      // NodeKeypair, PublicKeyPack
│   │   ├── shamir.rs                    // ShamirShare, reconstruct_secret()
│   │   └── masks.rs                     // apply_masks(), generate_pairwise_mask()
│   ├── crypto/                          # Криптографические примитивы
│   │   ├── x25519.rs                    // Обёртка над x25519_dalek
│   │   └── chacha20_rng.rs              // Детерминированный CSPRNG
│   └── utils/
│       ├── serialization.rs             // Serde для сетевой передачи
│       └── logging.rs
└── tests/                               // **44 теста для SecAgg+**
    ├── unit_tests.rs
    └── integration_test.rs              // Полный цикл агрегации с dropout
```

## 3. Последовательность выполнения (Workflow)

1.  **Инициализация:** Админ через `/admin/v1/round/start` запускает раунд. Оркестратор рассылает узлам команду на `secagg/setup`.
2.  **Обмен ключами SecAgg (Round 1):** Узлы генерируют пары ключей X25519, обмениваются публичными ключами через оркестратор, вычисляют попарные seeds, генерируют маски, создают доли Шамира.
3.  **Локальное обучение:** Оркестратор рассылает текущую глобальную модель и задачу. Узлы загружают локальные данные, вычисляют градиенты, добавляют шум DP (Gaussian mechanism), применяют свои маски SecAgg.
4.  **Фиксация и агрегация:** Узлы отправляют замаскированные градиенты и доли Шамира оркестратору (`/node/v1/secagg/commit`). Оркестратор дожидается `threshold` узлов, агрегирует векторы, при необходимости восстанавливает маски выбывших узлов через доли Шамира. Результат — чистый суммарный градиент.
5.  **Обновление модели и оценка вклада:** Оркестратор обновляет глобальную модель. Затем, используя сохранённые результаты от узлов и агрегированные результаты для случайных коалиций, вычисляет приближённые значения Шепли для каждого узла.
6.  **Учёт приватности:** Обновляется общий бюджет `ε_total`.

## 4. Требования к развёртыванию

*   **Узел:** Rust 1.75+, 2+ vCPU, 4+ GiB RAM, доступ к источнику данных (HIS/EHR), исходящий HTTPS‑доступ к оркестратору.
*   **Оркестратор:** Rust 1.75+, 4+ vCPU, 8+ GiB RAM, публичный IP/домен, SSL‑сертификат, persistent storage (для моделей и логов).
*   **Сеть:** Все коммуникации по HTTPS/mTLS. Порты по умолчанию: Orchestrator API — 8080, Admin API — 8081.