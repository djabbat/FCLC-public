# FCLC — Полный анализ системы
**Дата:** 2026-04-04 | **Версия:** 1.0 | **Охват:** весь codebase + документация

---

## 1. Исправленные баги (эта сессия + предыдущая)

| # | Файл | Проблема | Исправление |
|---|------|---------|-------------|
| B1 | `state.rs` | MODEL_DIM=128 ≠ FEATURE_DIM+1=9 → runtime dimension panic | 128 → 9 |
| B2 | `orchestrator/mod.rs` | FedProx μ=0.01 (CONCEPT требует 0.1) | 0.01 → 0.1 |
| B3 | `pipeline/mod.rs` | DP composition: 1×ε вместо epochs×ε | epsilon_spent = ε × local_epochs |
| B4 | `privacy/mod.rs` | suppress_rare_threshold=3 ≠ k_anonymity=5 | 3 → 5 |
| B5 | `aggregation/mod.rs` | Single-Krum делал FedProx бессмысленным (1 узел) | Multi-Krum: top-k = n − f |
| B6 | `aggregation/mod.rs` | SecAgg+ отсутствовал без документации | Добавлен stub с полным протоколом |
| B7 | `main.rs` | Нет аутентификации REST API | Bearer token middleware |
| B8 | `client/mod.rs` | RegisterRequest.name ≠ server.node_name | name → node_name |
| B9 | `client/mod.rs` | RegisterResponse.message ≠ server.status | message → status |
| B10 | `client/mod.rs` | ModelUpdatePayload: dp_epsilon_spent/train_loss/val_auc не совпадают с сервером | epsilon_spent / loss / auc |
| B11 | `client/mod.rs` | gradient: Vec<f32> ≠ server Vec<f64> | Vec<f64> + конверсия в app.rs |
| B12 | `client/mod.rs` | GlobalModelResponse: round: u32 / weights: Vec<f32> не совпадают | u64 / Vec<f64> + version: String |
| B13 | `client/mod.rs` | URL /api/model/global не существует (сервер: /api/model/current) | /api/model/current |
| B14 | `client/mod.rs` | ping() вызывал /api/model/global | /api/model/current |
| B15 | `connector/mod.rs` | Hardcoded 2024u16 для расчёта возраста | SystemTime::now() |
| B16 | `PARAMETERS.md` | input_dim=128 и local_epochs=5 не совпадали с кодом | 9 и 3 соответственно |

---

## 2. Оставшиеся технические проблемы

### 2.1 Критические (влияют на корректность)

**P1: Shapley performance function — математически некорректна**
- Файл: `orchestrator/mod.rs:258`
- Проблема: `performance_fn(coalition) = mean(AUC_i for i in coalition)` — это не производительность коалиции как модели, а среднее самооценок. Правильный Shapley требует: обучить модель на объединении данных коалиции → оценить на held-out валидационном наборе сервера.
- Следствие: Shapley scores не имеют правильной аксиоматической интерпретации. Для EIC обязательно задокументировать как "AUC-averaging approximation".
- Решение: Либо (а) добавить на сервер малый held-out набор + FedAvg-агрегацию для оценки, либо (б) использовать Leave-One-Out (LOO) аппроксимацию, либо (в) явно задокументировать ограничение.

**P2: DP accountant назван RenyiAccountant, но реализует basic composition**
- Файл: `dp/mod.rs`
- Проблема: Настоящий Rényi DP accountant использует моменты/моменты Рения для tight bounds. Текущий код: `remaining = budget - sum(epsilon_spent)` — это linear composition (не Rényi).
- Следствие: При M rounds с subsampling, истинный DP-бюджет намного меньше. Basic composition завышает потребление в ~√M раз.
- Решение: Переименовать в `LinearDpAccountant` + документировать; или реализовать настоящий Moments Accountant (через `autodp` библиотеку в Python или `opacus`-like Rust).

**P3: FedProx proximal term не применяется на стороне node**
- Файл: `pipeline/mod.rs:165-181`
- Проблема: FedProx требует, чтобы при локальном обучении к loss добавлялся proximal term `(μ/2)||w - w_global||²`. В pipeline только стандартный gradient descent без proximal regularization.
- Следствие: Агрегация делает proximal pull только на сервере (в `fedprox_aggregate`), что является нестандартным применением FedProx.
- Решение: Добавить proximal gradient step в `LocalPipeline::run_training()`:
  ```rust
  // After computing gradient:
  for (i, (g, (w, wg))) in gradient.iter_mut().zip(
      self.model.weights.iter().zip(global_weights.iter())
  ).enumerate() {
      *g += mu * (w - wg); // proximal regularization
  }
  ```

### 2.2 Высокая важность

**P4: Нет TLS — все данные в HTTP**
- Все клиент-серверные соединения по HTTP (localhost:8080). Для клинических данных GDPR/PDPL требует TLS.
- Решение: `rustls` + `axum-server` с TLS; или nginx reverse proxy с Let's Encrypt.

**P5: Shapley compute O(n×M) — узкое место производительности**
- При n=10, M=200: 2000 вызовов performance_fn за раунд. Если каждый вызов занимает ~1ms — это 2 секунды. При реальном FedAvg с нейросетью и валидацией — часы.
- Текущая реализация (mean AUC) быстра, но см. P1. При правильной реализации нужен асинхронный Shapley worker.

**P6: Global model хранится как f64 на сервере, f32 на node — потеря точности**
- `global_model: Vec<f64>` (server) ↔ `weights: Vec<f32>` (node pipeline)
- При каждом раунде: f64→f32 (truncate) → вычисления f32 → f32→f64 (extend)
- Для 9-мерной модели пренебрежимо. Для больших моделей — проблема. Стандартизировать f32 повсюду.

**P7: Нет валидации min_nodes при force_aggregate**
- `force_aggregate` запускает агрегацию при `pending_count > 0`, т.е. можно агрегировать с 1 узлом.
- При 1 узле: Krum assert `n >= 2` запаникует! Нужна проверка `if n < 2, skip Krum`.
- Уже есть `if n >= 2` в orchestrator для Krum, но `assert!(n >= 2f)` в `krum_select` всё равно сработает если вызвать с n=1 при другом пути.

**P8: Аутентификация только через env var — не всегда активна**
- `FCLC_API_TOKEN` опциональный: если переменная не установлена, auth пропускается.
- Для production: сделать обязательным или добавить логирование предупреждения при старте без токена.

### 2.3 Средняя важность

**P9: round_id в client/app.rs — локальный счётчик**
- `state.current_round` инкрементируется локально в node, а не синхронизируется с сервером.
- Node думает, что она в round 5, сервер — в round 3.
- Решение: Получать round number из `GlobalModelResponse.round` при каждой загрузке модели.

**P10: FHIR connector только парсит Patient resources, игнорирует Observation**
- `load_fhir_json` парсит Patient (age/sex) но не Observation (HbA1c, BMI и т.д.)
- Все клинические поля (hba1c_last, bmi, has_nephropathy, etc.) остаются None/false для FHIR источников.
- Нужно парсить Observation resources и сопоставлять их с Patient по subject.reference.

**P11: CSV connector не валидирует диапазоны**
- age=200, HbA1c=999.0 — всё принимается без ошибки.
- Добавить диапазонные checks: age ∈ [0, 120], HbA1c ∈ [3.0, 20.0], BMI ∈ [10.0, 70.0].

**P12: Отсутствует endpoint GET /api/nodes (list)**
- `routes/nodes.rs` имеет `list_nodes` handler, но он не зарегистрирован в router.
- Проверить `main.rs` router — добавить `.route("/api/nodes", get(routes::nodes::list_nodes))`.

**P13: Шаг 9 пропущен в нумерации orchestrator**
- `run_aggregation` переходит от "Step 8 (audit log)" к "Step 10 (history+counter)" — step 9 отсутствует.
- Это либо SSE нотификация (TODO #42), либо model checkpoint (TODO из PARAMETERS.md).

---

## 3. Научная валидность

| Аспект | Оценка | Комментарий |
|--------|--------|-------------|
| DP-SGD (Gaussian mechanism) | ✅ Корректен | σ = sensitivity × √(2ln(1.25/δ)) / ε — верно |
| Gradient clipping | ✅ Корректен | L2 norm clipping перед noise |
| Krum Byzantine tolerance | ✅ Корректен | n ≥ 2f+2 проверяется, f = ⌊0.25n⌋ |
| FedProx aggregation (сервер) | ✅ Корректен | (w_avg + μ*w_global) / (1+μ) |
| FedProx proximal (node) | ❌ Не реализован | См. P3 |
| Shapley computation | ⚠️ Аппроксимация | Mean AUC ≠ true coalition value, см. P1 |
| DP accounting | ⚠️ Консервативна | Basic composition, не Rényi — см. P2 |
| k-anonymity | ✅ Корректна | k=5, suppression при < k |
| Hash-chain audit log | ✅ Корректен | SHA-256 chaining, genesis="0"×64 |
| Multi-Krum | ✅ Реализован | top-k=n-f updates переданы в FedProx |

---

## 4. Соответствие CONCEPT.md

| Параметр | CONCEPT | Код | Статус |
|----------|---------|-----|--------|
| MODEL_DIM | 9 (OMOP+bias) | 9 | ✅ |
| DP ε/round | 2.0 | 2.0 | ✅ |
| DP δ | 1e-5 | 1e-5 | ✅ |
| DP total | 10.0 | 10.0 | ✅ |
| FedProx μ | 0.1 | 0.1 | ✅ |
| Byzantine fraction | 0.25 | 0.25 | ✅ |
| k-anonymity | 5 | 5 | ✅ |
| max_grad_norm | 1.0 | 1.0 | ✅ |
| Shapley MC samples | 150 | 200 (n>5) | ⚠️ Расхождение |
| local_epochs | 3 | 3 | ✅ (PARAMS исправлен) |
| min_nodes | 3 | 3 | ✅ |

---

## 5. Оставшиеся TODO (критические)

### Фаза A: Code (до пилота)
1. **SecAgg+ реализация** — заменить stub на полный протокол (Bonawitz 2017)
2. **FedProx proximal на node** — добавить `(μ/2)||w-w_global||²` в локальный training loop
3. **FHIR Observation parsing** — HbA1c/BMI из FHIR Bundle
4. **GET /api/nodes registration** — добавить в router
5. **Round sync** — node получает current round из GlobalModelResponse

### Фаза B: Infrastructure (до EIC демо)
6. **fclc-web Phoenix scaffold** — `mix phx.new fclc-web --no-ecto`
7. **Docker Compose** — fclc-server + PostgreSQL
8. **Integration test** — симуляция 3 узлов × 5 раундов

### Фаза C: Unit tests (до submission)
9. DP noise distribution test
10. Krum correctness test (Byzantine outlier rejection)
11. Shapley sum property test (sum ≈ 1.0 for normalized)
12. End-to-end round test

### Фаза D: EIC (до 12.05.2026)
13. OpenAPI spec (все /api/* endpoints)
14. Демо-видео 2 мин
15. DUA/IRB шаблоны
16. Part B narrative (10 стр.)
17. Бюджет EIC
18. Letters of Support (3 клиники)

---

## 6. Предложения по улучшению

### 6.1 Производительность
- **Gradient compression**: Top-k sparsification или quantization (8-bit) перед отправкой → 10x экономия трафика при 1000-мерных моделях
- **Async Shapley**: Вычислять Shapley в фоновом tokio task, не блокировать агрегацию
- **Connection pooling**: `reqwest::blocking::Client` на node → переключить на async + tokio

### 6.2 Научная строгость
- **Moments Accountant (Rényi DP)**: С subsampling при rate q=batch/dataset, ε_total ≈ ε√(rounds) вместо ε×rounds → в 5–7 раз больше раундов на тот же бюджет
- **Proper Shapley validation set**: Малый (~100 records) датасет на сервере для честной оценки coalition performance
- **AUC confidence intervals**: Bootstrap CI для AUC по раундам → улучшает научную презентацию для EIC

### 6.3 Безопасность
- **TLS (rustls)**: Обязательно для клинических данных
- **Rate limiting**: Tower middleware `tower_governor` — защита от flood атак
- **Audit log export**: GET /api/audit → CSV/JSON для внешней верификации
- **Mandatory auth**: Предупреждение при старте без FCLC_API_TOKEN

### 6.4 EIC Pathfinder readiness
- **Demo dataset**: Синтетические данные 3 клиник (~500 records каждая) → показать сходимость AUC за 5 раундов
- **Визуализация**: График AUC vs rounds, Shapley bar chart, DP budget depletion curve
- **Georgian PDPL + GDPR Article 9 checklist**: Документировать соответствие для DUA с Aversi/GeoHospitals

---

*Составлено: 2026-04-04. Следующий аудит: после SecAgg+ реализации.*
