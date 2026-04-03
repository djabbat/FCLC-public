# UPGRADE.md — FCLC (Federated Clinical Learning Consortium)

Suggestions for project development from external analysis, literature, and cross-project review.

**Format:**
```
## [YYYY-MM-DD] Title
**Source:** [what triggered this]
**Status:** [ ] proposed | [✓ approved YYYY-MM-DD] | [✓✓ implemented YYYY-MM-DD]
```

---

## [2026-03-29] Hash-Chain Contribution Audit Trail
**Source:** Cross-project analysis; federated learning literature (McMahan et al., 2017; recent FL provenance work)
**Status:** [✓ approved 2026-03-30] [✓✓ implemented 2026-03-30]

Peer review переформулировал: Hyperledger Fabric → PostgreSQL APPEND-ONLY hash-chain (нет новой инфраструктуры).
Реализовано: таблица `audit_log` (migration 002), `db::insert_audit_entry` / `get_audit_chain` / `get_latest_audit_hash`, `GENESIS_HASH = '0'×64`, `GET /api/audit` endpoint, вставка в orchestrator после каждого раунда. Алгоритм: entry_hash = SHA-256(round_id ‖ round_number ‖ gradient_hash ‖ prev_hash). 5 новых тестов хэш-функций.

Federated learning systems face trust issues when contributors dispute model attribution or data contribution weights. Appending an immutable audit log to a permissioned blockchain (e.g., Hyperledger Fabric) for each round of gradient aggregation would provide tamper-evident records of who contributed what and when. This is particularly important for FCLC's multi-institution model where contribution tracking directly affects incentive distribution.

---

## [2026-03-29] Real-Time Model Performance Monitoring (Prometheus)
**Source:** MLOps best practices; `/api/metrics` already planned in REST contract
**Status:** [✓ approved 2026-03-30] [✓✓ implemented 2026-03-30]

Peer review принял частично: Prometheus endpoint без drift detection / Grafana.
Реализовано: `GET /metrics` — text/plain Prometheus format (Content-Type: text/plain; version=0.0.4). Метрики: `fclc_rounds_total` (counter), `fclc_active_nodes` (gauge), `fclc_auc_latest` (gauge), `fclc_avg_shapley` (gauge). Существующий `GET /api/metrics` JSON сохранён для dashboard. 2 новых теста.

---

## [2026-03-29] Homomorphic Encryption Upgrade for Gradient Aggregation
**Source:** Literature review: CKKS scheme advancements (2024-2025); differential privacy limitations at small N
**Status:** [⏸ отложено — SecAgg+ достаточен; revisit при N<3 или регуляторном требовании]

The current differential privacy layer adds noise proportional to sensitivity, which degrades model utility when the number of participating nodes is small (as expected in early FCLC phases). Upgrading to partial homomorphic encryption (PHE) or the CKKS approximate arithmetic scheme for gradient aggregation would allow the central aggregator to operate on encrypted gradients without decryption, eliminating the privacy-utility tradeoff at low participation counts. Libraries: TenSEAL, OpenFHE.

---

## [2026-03-29] Cross-Border Regulatory Compliance Module
**Source:** GDPR (EU), Georgian Personal Data Protection Law (2024 revision), Kazakhstan health data regulations
**Status:** [⏸ отложено — зависит от AIM FHIR layer (⏸ после Фаз 3+5+7); нет реальных участников из разных юрисдикций]

FCLC's multi-jurisdictional design requires a compliance layer that tracks which data residency rules apply per participating node and enforces them automatically. A rule engine mapping institution location → applicable regulations → permissible data operations would prevent inadvertent cross-border transfers and generate audit-ready compliance reports. This module should integrate with FHIR consent resources (from AIM's planned FHIR layer) for patient-level opt-in/opt-out management.

---

## [2026-03-29] Real-Time Model Performance Monitoring Dashboard — Drift Detection + Grafana
**Source:** MLOps best practices; federated drift detection literature (concept drift in clinical FL)
**Status:** [⏸ отложено — drift detection требует baseline модели и реальных данных; Grafana — внешний сервис]

Federated models can silently degrade when contributing institutions experience data distribution shifts (e.g., seasonal disease patterns, new lab equipment calibration). A real-time dashboard tracking per-round validation metrics, contribution-weighted performance deltas, and anomaly detection alerts would allow consortium administrators to identify and exclude drifting nodes before they corrupt the global model. Stack recommendation: Prometheus + Grafana with a lightweight Python FL metrics exporter per node.

---

## [2026-04-02] CONCEPT v6.0 — Финальная версия, ГОТОВО К ПОДАЧЕ
**Source:** Полный цикл peer review (v1.0→v6.0 через 6 итераций + DeepSeek R1)
**Status:** ✅ [✓ approved 2026-04-02] [✓✓ finalised 2026-04-02]

- ✅ Удалены все упоминания Prof. Tamar Sanikidze и TSMU
- ✅ Роль Giorgi Tsomaia: "Lead of WP2 (Biomedical Data Systems) and WP4 (Governance & Incentive Model)"
- ✅ Добавлен Medical Consultant (0.5 FTE, 60 000€) с поддержкой IRB в юридической базе
- ✅ Добавлен Technical Expert — Database Systems (1.0 FTE, 60 000€)
- ✅ External Clinical Advisory Board (20 000€) — отдельная строка бюджета
- ✅ Бюджет итого: 2 275 000€ (15 строк + итог)
- ✅ Матрица рисков: 15 рисков (5 колонок)
- ✅ Таблица сравнения: 8 строк (FedAvg, FATE, NVFlare, OpenFL, FeTS, MELLODDY, PySyft, Flower)
- ✅ Унифицированный IRB через Medical Consultant
- ✅ Версия 6.0, дата 2 апреля 2026, статус ГОТОВО К ПОДАЧЕ
