# TODO: FCLC — Federated Clinical Learning Cooperative

## Status: 🟢 CONCEPT v6.0 ready — EIC submission preparation active
**Date:** 2026-04-02
**Version:** 0.1.0-alpha
**Cargo build:** ✅ workspace compiles (fclc-core, fclc-node, fclc-server)
**CONCEPT:** ✅ v6.0 — ГОТОВО К ПОДАЧЕ (peer review complete, 2026-04-02)

---

## 🔴 CRITICAL — Immediate

| # | Task | Status | Notes |
|---|------|--------|-------|
| C1 | `cargo build --workspace` clean | ✅ | fclc-core, fclc-node, fclc-server |
| C2 | CONCEPT peer review → ACCEPT | ✅ | v6.0 finalised 2026-04-02, готово к подаче |
| C3 | fclc-web Phoenix scaffold | ✅ | LiveView dashboard: DashboardLive + FclcClient + router |
| C4 | PostgreSQL schema migrations | ⏳ | `migrations/001_init.sql` — nodes, rounds, updates, scores |
| C5 | REST API contract (OpenAPI spec) | ⏳ | Document all `/api/*` endpoints before fclc-web integration |
| C6 | Найти Medical Consultant (0.5 FTE) | 🔴 | Клиническая валидация, IRB; бюджет 60k€ |
| C7 | Найти Technical Expert — Database Systems (1.0 FTE) | 🔴 | ETL/OMOP/PostgreSQL; бюджет 60k€ |
| C8 | Найти EU Technical Partner (DFKI / Fraunhofer / Saarland) | 🔴 | Поиск активен; до 12.04 |
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
| 58 | Pilot with 3 test nodes (local) | ⏳ | High |
| 59 | Run 5 federated rounds, verify convergence | ⏳ | High |
| 60 | Verify DP budget accounting (sum check) | ⏳ | High |
| 61 | Verify Shapley scores sum to ~1.0 | ⏳ | High |
| 62 | Verify Krum rejects planted Byzantine update | ⏳ | Medium |
| 63 | IRB/DUA template documents | ⏳ | High |
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
