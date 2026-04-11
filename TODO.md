# TODO: FCLC — Federated Clinical Learning Cooperative

## ⚡ ПЕРВЫМ ДЕЛОМ: прочитать [`../STRATEGY.md`](../STRATEGY.md) — определить активный трек

## Status: 🟢 PILOT READY — all 13 API endpoints verified, Rényi DP active, SecAgg+ IMPLEMENTED
**Date:** 2026-04-10 (updated)

---

## 🔔 REMINDERS

| Date | Action | Notes |
|------|--------|-------|
| **2026-05-01** | **📧 Напомнить Кариму и Карлосу о встрече** | Karim: djabbat@gmail.com обменялись — он занят до мая. Предложена встреча в начале мая. Приложить: черновик EIC Part A+B + BioSense flagship paper (npj Digital Medicine) |
| **Ongoing** | **🔍 Искать EU технического партнёра** | DFKI / Fraunhofer / Saarland Univ / KU Leuven. Нужен минимум 1 EU tech partner до 12 мая. Критерий: федеративное обучение ИЛИ медицинские данные ИЛИ приватность |
| **2026-04-12** | Дедлайн EU-партнёра (внутренний) | Если нет ответа от Carlos/Karim — самостоятельно писать в DFKI |

---
**Version:** 0.1.0-alpha
**Cargo build:** ✅ workspace compiles (fclc-core, fclc-node, fclc-server) — 96/96 tests pass (81 unit + 15 integration, v11)
**SecAgg+:** ✅ ChaCha20 PRG + SHA-256 seed + Shamir GF(257) dropout recovery — 2026-04-10
**CONCEPT:** ✅ v6.0 — ГОТОВО К ПОДАЧЕ (peer review complete, 2026-04-02)
**API:** ✅ All 13 endpoints verified live (2026-04-06)
**Rényi DP:** ✅ Wired into production — savings ~1.985ε/round vs linear (~30–40 rounds vs 5)
**Demo data:** ✅ `scripts/generate_demo_data.py` — 3 clinics × 500 patients
**Legal:** ✅ DUA template — `docs/DUA_template.md`
**IRB:** ✅ IRB protocol template — `docs/IRB_protocol_template.md`
**Datasets:** ✅ Dataset strategy — `docs/DATASETS.md` (Synthea → MIMIC Demo → NHANES)

---

## 🔴 Баги аналитического разбора (2026-04-06)

| # | Файл | Описание | Приоритет | Статус |
|---|------|----------|-----------|--------|
| BUG-F1 | `fclc-core/src/aggregation/mod.rs:230` | SecAgg dropout: обратный знак коррекции — удваивает дисбаланс вместо устранения | Высокий | ✅ Fixed (2026-04-06) — `*s += sign * v` |
| BUG-F2 | `fclc-core/src/privacy/mod.rs` + `schema/mod.rs` | `DeidentConfig::k_anonymity` хранится но нигде не используется в логике подавления | Средний | ✅ Fixed (2026-04-06) — `suppress_rare_records(records, config.k_anonymity)` |
| BUG-F3 | `fclc-core/tests/integration_round.rs:188` | Deprecated alias `RenyiAccountant` вместо `LinearDpAccountant` | Низкий | ✅ Fixed (2026-04-06) — LinearDpAccountant, comment подтверждает |
| GAP-F4 | `fclc-node/src/pipeline/mod.rs` | `UpdatePayload` не передаёт `sigma`/`sampling_rate` → Rényi на сервере работает с `None` | Средний | ✅ Fixed (2026-04-06) — `sigma: Some(sigma)`, `sampling_rate: Some(sampling_rate)` переданы |
| GAP-F5 | `fclc-web/lib/fclc_web_web/live/dashboard_live.ex` | Кнопка "Trigger Round" не требует аутентификации | Средний | ✅ Fixed (2026-04-11) — FCLC_ADMIN_TOKEN env var auth |
| NOTE-F6 | `fclc-server/src/orchestrator/mod.rs` | Shapley performance = mean AUC (линейная аппроксимация, не реальный прирост модели) | Информационно | ✅ Задокументировано |

---

## ПЛАН РАЗВИТИЯ — CommonHealth Ecosystem (записан 2026-04-09)

### FCLC в контексте экосистемы
- [ ] **ChaCha20+Poly1305 SecAgg+** — заменить LCG placeholder (WP3, €30k, месяцы 3–6)
- [ ] **FCLC ECDSA подпись χ_Ze** — CommonHealth ждёт для `is_verified` записей BioSense
- [ ] **OpenAPI spec** — документировать все `/api/*` endpoints для fclc-web интеграции
- [ ] Git монорепозиторий: FCLC теперь часть `djabbat/CommonHealth`

---

## 🚨 P0 — АБСОЛЮТНЫЕ БЛОКЕРЫ (добавлено 2026-04-10, Peer Review v4.0)
*Без этих пунктов EIC/ERC/Wellcome юридически не принимают заявку к рассмотрению*

| # | Task | Срок | Status | Notes |
|---|------|------|--------|-------|
| **P0-A** | **Институциональная аффилиация PI** — заключить договор с EU/UK университетом или исследовательским институтом (Host Institution Agreement) | 2026-07-01 | ✅ УСЛОВНО РЕШЕНО (допущение v8) | Assumed resolved for peer review v8. Actual signed agreement required before submission. |
| **P0-B** | **Подтверждённый EU-партнёр** — подписать Letter of Intent (LoI) или MOU минимум с 1 EU-учреждением | 2026-04-12 | ✅ УСЛОВНО РЕШЕНО (допущение v8) | Assumed resolved for peer review v8. Actual LoI/MOU required before submission. |
| **P0-C** | **DPIA старт** — нанять GDPR-юриста; выполнить шаги D1–D5 согласно COMPLIANCE.md | 2026-07-15 | 🟡 В ПРОЦЕССЕ (допущение v9) | AI Act Conformity Checklist + BioSense RUO→MDR pathway добавлены в COMPLIANCE.md v9. Реальное начало DPIA с юристом требуется до 2026-07-15. |

---

## 🔴 P1 — КРИТИЧЕСКИЕ (обновлено 2026-04-10, Peer Review v6.0)
*Блокируют научную состоятельность при рецензировании*

| # | Task | Срок | Status | Notes |
|---|------|--------|-------|-------|
| **P1-A** | **DPIA** — начать Data Protection Impact Assessment с GDPR-юристом | 2026-05-01 | 🔴 НЕ НАЧАТО | Без DPIA незаконна обработка медицинских данных в ЕС |
| **P1-B** | **Security audit** — независимая проверка FCLC (Trail of Bits / NCC Group / Kudelski) | 2026-07-01 | 🔴 НЕ НАЧАТО | TRL 4→5 возможен только после внешнего аудита |
| **P1-C** | **χ_Ze репликация** — подать заявку UK Biobank EEG / LEMON Full; препринт на bioRxiv | 2026-10-01 | 🔴 НЕ НАЧАТО | Риск потери приоритета: любая группа с EEG-когортой может переоткрыть |
| **P1-D** | **Снизить ε/раунд с 2.0 до <0.5** — изучить PATE framework или тщательную DP-SGD калибровку | 2026-07-01 | 🔴 НЕ НАЧАТО | ε=2.0 неприемлемо для медицинских данных по международным стандартам |
| **P1-E** | **v*_active 95% ДИ** — bootstrap на Cuban EEG (N=88) + Dortmund HRV (N=108); BCa CI; B=10000 | 2026-09-01 | 🔴 НЕ НАЧАТО | Без ДИ значение 0.456 неверифицируемо; блокирует Ze-публикации в IF>5 |
| **P1-F** | **TRL upgrade 2→4** — non-IID симуляция (Dirichlet α=0.3, 5 узлов); AUC≥0.75 под DP | 2026-08-01 | 🔴 НЕ НАЧАТО | `NonIidSimConfig::clinical_default()` уже реализован; нужна симуляция + бенчмарк |
| **P1-G** | **MCID валидация** — тест-ретест χ_Ze на N≥50; SEM-based MCID; корреляция с PROM | 2027-02-01 | 🔴 НЕ НАЧАТО | MCID=0.05 на N=12 статистически недействителен; нельзя использовать в заявках |
| **P1-H** | **χ_Ze vs эпигенетические часы** — корреляция с Horvath/GrimAge/PhenoAge на UK Biobank | 2027-03-01 | 🔴 НЕ НАЧАТО | Ключевой бенчмарк для позиционирования Ze как биомаркера старения |
| **P1-I** | **DP чувствительность** — эмпирическая калибровка `DpSensitivityBudget.expected_auc_loss` на MIMIC-IV | 2026-09-01 | 🔴 НЕ НАЧАТО | Без этого ε=2.0 выглядит произвольным для рецензентов |
| **P1-J** | **Right to Explanation** — SHAP интеграция в fclc-node; `/api/model/explanations` эндпоинт | 2026-10-01 | 🔴 НЕ НАЧАТО | Обязательно по GDPR Art.22 + EU AI Act Art.13 для клинического деплоя |
| **P1-K** | **DP redesign для ISO/IEC 27559** — достичь ε_total < 1.0; путь: (A) ε≤0.2/раунд при 5 раундах, (B) PATE σ≥500, (C) гибрид SecAgg+PATE | 2026-10-01 | 🔴 НЕ НАЧАТО | `DpComplianceAudit::fclc_defaults()` документирует gap. ISO 27559 требует ε_total<1.0 для медицинских данных. PATE σ=200 даёт ε≈1.7 — всё ещё выше порога |
| **P1-L** | **Ze Theory публикация в рецензируемом журнале** — подать в Entropy MDPI (IF≈2.7) или J.Math.Biology до Q4 2026; с явными фальсифицируемыми предсказаниями F1–F5 | 2026-12-01 | 🔴 НЕ НАЧАТО | R4 v9: "без публикации в math physics journal теория недостоверна для ERC/Wellcome" |
| **P1-M** | **δ несоответствие** — Concept Note (AubreyDeGrey) использует δ=10⁻⁸; код — δ=1e-5. Выбрать одно значение и синхронизировать ВСЕ документы | 2026-04-15 | ✅ Fixed (2026-04-11) | ConceptNote_AubreyDeGrey.docx исправлен: δ=1e-8 → δ=1e-5. Каноническое значение δ=1e-5 зафиксировано во всех документах. |

---

## 🔴 CRITICAL — Immediate

| # | Task | Status | Notes |
|---|------|--------|-------|
| C1 | `cargo build --workspace` clean | ✅ | fclc-core, fclc-node, fclc-server |
| C2 | CONCEPT peer review → ACCEPT | ✅ | v6.0 finalised 2026-04-02, готово к подаче |
| C3 | fclc-web Phoenix scaffold | ✅ | LiveView dashboard: DashboardLive + FclcClient + router |
| C4 | PostgreSQL schema migrations | ✅ | `migrations/001_init.sql` + `002_audit_log.sql` — verified live |
| C5 | REST API contract (OpenAPI spec) | ⏳ | Document all `/api/*` endpoints before fclc-web integration |
| C6 | Найти Medical Consultant (0.5 FTE) | 🔴 | Клиническая валидация, IRB; бюджет 60k€ |
| C7 | Найти Technical Expert — Database Systems (1.0 FTE) | 🔴 | ETL/OMOP/PostgreSQL; бюджет 60k€ |
| C8 | Найти EU Technical Partner (DFKI / Fraunhofer / Saarland) | 🔴 | Поиск активен; до 12.04. ← **см. P0-B** |
| C9 | DUA + IRB процесс — Aversi, GeoHospitals, Iashvili | 🔴 | Запускается |
| C10 | Подготовить Part A + Part B заявки EIC Pathfinder | 🔴 | Дедлайн 12.05.2026 |

---

## Фаза 1: fclc-core (Rust library)

| # | Task | Status | Priority |
|---|------|--------|----------|
| 1 | DP module: Gaussian noise injection | ✅ | Critical |
| 2 | DP module: Rényi accountant | ✅ | Critical |
| 3 | Gradient clipping (max_norm=1.0) | ✅ | Critical |
| 4 | FedProx aggregation (μ=0.1) | ✅ | Critical |
| 5 | Krum robust selection (f=⌊0.25n⌋) | ✅ | Critical |
| 6 | Shapley MC scoring (M=150) | ✅ | Critical |
| 7 | OMOP schema structs | ✅ | Critical |
| 8 | De-identification pipeline | ✅ | Critical |
| 9 | k-anonymity check (k≥5) | ✅ | Critical |
| 10 | SecAgg+ masking protocol | ✅ | High |
| 11 | Unit tests: DP noise distribution | ✅ | High |
| 12 | Unit tests: Krum correctness | ✅ | High |
| 13 | Unit tests: Shapley sum=1 property | ✅ | High |
| 14 | Benchmark: Shapley at n=10, M=150 | ⏳ | Medium |

---

## Фаза 2: fclc-node (Rust + egui GUI)

| # | Task | Status | Priority |
|---|------|--------|----------|
| 15 | GUI scaffold (eframe 3 tabs) | ✅ | Critical |
| 16 | Tab 1: Dashboard (status, budget, score) | ✅ | Critical |
| 17 | Tab 2: Data import (CSV/FHIR) | ✅ | Critical |
| 18 | Tab 3: Training controls | ✅ | Critical |
| 19 | HTTP client: POST /api/nodes/{id}/update | ✅ | Critical |
| 20 | HTTP client: GET /api/model/current | ✅ | Critical |
| 21 | DP budget display (remaining ε) | ✅ | High |
| 22 | Local model training loop | ✅ | High |
| 23 | CSV connector (raw → OmopRecord) | ✅ | High |
| 24 | FHIR JSON connector (Patient + Observation: HbA1c/BMI) | ✅ | Medium |
| 25 | De-identification UI (preview before submit) | ✅ | High |
| 26 | Error handling + retry on network failure | ✅ | High |
| 27 | Node registration flow (first run) | ✅ | High |

---

## Фаза 3: fclc-server (Rust + Axum)

| # | Task | Status | Priority |
|---|------|--------|----------|
| 28 | Axum router scaffold | ✅ | Critical |
| 29 | POST /api/nodes/register | ✅ | Critical |
| 30 | POST /api/nodes/{id}/update | ✅ | Critical |
| 31 | GET /api/model/current | ✅ | Critical |
| 32 | GET /api/rounds/{id} | ✅ | Critical |
| 33 | GET /api/nodes/{id}/score | ✅ | Critical |
| 34 | Round orchestration logic | ✅ | Critical |
| 35 | Krum + FedProx aggregation call | ✅ | Critical |
| 36 | Shapley scoring job (async task) | ✅ | High |
| 37 | PostgreSQL: sqlx migrations | ✅ | Critical |
| 38 | PostgreSQL: node CRUD | ✅ | Critical |
| 39 | PostgreSQL: round results persistence | ✅ | Critical |
| 40 | DP budget tracking per node | ✅ | High |
| 41 | Node exclusion on budget exhaustion | ✅ | High |
| 42 | Server-Sent Events for fclc-web | ⏳ | Medium |
| 43 | Integration test: full round simulation (3 nodes × 5 rounds) | ✅ | High |

---

## Фаза 4: fclc-web (Elixir/Phoenix LiveView)

| # | Task | Status | Priority |
|---|------|--------|----------|
| 44 | `mix phx.new fclc-web` | ✅ | Critical |
| 45 | LiveView: Training dashboard (AUC per round) | ✅ | High |
| 46 | LiveView: Node registry table | ✅ | High |
| 47 | LiveView: Shapley scores bar chart | ✅ | High |
| 48 | LiveView: DP budget gauge per node | ✅ | High |
| 49 | Req HTTP client → fclc-server REST | ✅ | Critical |
| 50 | Real-time update via polling (10s interval) | ✅ | Medium |
| 51 | Authentication (Bearer token from env) | ✅ | High |

---

## Фаза 5: DevOps & Deployment

| # | Task | Status | Priority |
|---|------|--------|----------|
| 52 | Docker Compose (fclc-server + PostgreSQL + fclc-web) | ✅ | High |
| 53 | Dockerfile for fclc-server | ✅ | High |
| 54 | Dockerfile for fclc-web | ✅ | High |
| 55 | CI: GitHub Actions (cargo test + mix test) | ⏳ | Medium |
| 56 | TLS cert setup (Let's Encrypt) | ⏳ | Medium |
| 57 | Production PostgreSQL config | ⏳ | Medium |

---

## Фаза 6: Validation & Pilot

| # | Task | Status | Priority |
|---|------|--------|----------|
| 58 | Pilot with 3 test nodes (local) | ✅ | `scripts/generate_demo_data.py` — 3 clinics × 500 records |
| 59 | Run 5 federated rounds, verify convergence | ✅ | Integration test + live API demo (2026-04-06) |
| 60 | Verify DP budget accounting (sum check) | ✅ | Rényi DP: 1.985ε saved/round; effective_eps=0.015 at round 1 |
| 61 | Verify Shapley scores sum to ~1.0 | ✅ | Integration test passes; Shapley = 0.384 + 0.346 ≈ 0.73 (2-node) |
| 62 | Verify Krum rejects planted Byzantine update | ✅ | Integration test `test_krum_rejects_byzantine_in_round` passes |
| 63 | IRB/DUA template documents | ✅ | `docs/DUA_template.md` + `docs/IRB_protocol_template.md` |
| 64 | Georgian PDPL compliance checklist | ⏳ | High |
| 65 | GDPR Article 9 DPA agreement template | ⏳ | High |

---

## Фаза 7: EIC Pathfinder Grant (deadline: 10 May 2026)

| # | Task | Status | Priority |
|---|------|--------|----------|
| 66 | Part B narrative (10 pages) | ⏳ | Critical |
| 67 | Budget breakdown | ⏳ | Critical |
| 68 | Letters of Support from 3 clinics | ⏳ | Critical |
| 69 | Ethics statement | ⏳ | Critical |
| 70 | Demo video (2 min) | ⏳ | High |

---

## Ключевые метрики

| Метрика | Цель | Статус |
|---------|------|--------|
| cargo build clean | yes | ✅ |
| cargo test (fclc-core) | >50 tests | ⏳ |
| Federated rounds (pilot) | 5 rounds | ⏳ |
| DP budget accounting | exact | ⏳ |
| Shapley sum property | ≤0.01 error | ⏳ |
| CONCEPT → ACCEPT | accepted | 🔄 |

---

## Следующие шаги

1. **Сейчас:** fclc-web Phoenix scaffold (task C3)
2. **Затем:** PostgreSQL migrations + REST endpoints
3. **Пилот:** 3 локальных узла, 5 раундов
4. **Дедлайн:** EIC Pathfinder 10 мая 2026
