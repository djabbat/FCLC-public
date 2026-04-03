# FCLC — MAP: Architecture & Component Interaction

## Part 1: System Components

| Component | Type | Location | Responsibility |
|-----------|------|----------|----------------|
| **fclc-core** | Rust lib | `fclc-core/` | DP engine, Shapley scoring, FedProx/Krum aggregation, OMOP schema, de-identification |
| **fclc-node** | Rust bin + egui GUI | `fclc-node/` | Local node at each clinic: data import, anonymization, local training, secure update submission |
| **fclc-server** | Rust bin + Axum API | `fclc-server/` | Central orchestrator: model aggregation, Shapley scoring, round management, PostgreSQL |
| **fclc-web** | Elixir/Phoenix | `fclc-web/` | Web dashboard: LiveView, training progress, node management, contribution scores |
| **PostgreSQL** | Database | orchestrator host | Persistent storage: nodes, rounds, updates, scores |

---

## Part 2: Data Flow

```
CLINIC / LOCAL NODE (fclc-node)
│
├─ HIS/EHR (CSV / FHIR JSON)
│       │
│       ▼
│  [Connector] → raw records
│       │
│       ▼
│  [De-identification]
│  • Remove: name, DOB (→ year), MRN, address
│  • Generalize: age → 5-yr interval, rare dx → suppressed
│  • k-anonymity check (k≥5)
│       │
│       ▼
│  [OMOP Normalization]
│  • ICD-10 → concept_id
│  • LOINC for labs
│  • ATC for meds
│  → OmopRecord structs
│       │
│       ▼
│  [Local Training]
│  • Logistic regression / gradient descent
│  • Gradient clipping (max_norm=1.0)
│  • DP noise injection (Gaussian, ε=2.0, δ=1e-5)
│  • Rényi DP accounting → spend from budget
│       │
│       ▼
│  [SecAgg+ Protocol] → masked gradients
│       │
│       └──────────────────────────────────────────┐
│                                                   ▼
ORCHESTRATOR (fclc-server)                    POST /api/nodes/{id}/update
│
├─ Collect updates from all participating nodes
│
├─ [Krum Robust Selection]
│  • Reject up to f = ⌊25% × n⌋ Byzantine updates
│  • Select m = n - 2f updates closest to consensus
│
├─ [FedProx Aggregation]
│  • Weighted average of selected updates
│  • Proximal term: μ=0.1 (penalty for drift from global)
│
├─ [Shapley Value Scoring]
│  • Monte Carlo, M=100–200 permutations
│  • AUC on held-out validation set as performance function
│  • O(n² × M) — ~10 min/round for n=10 nodes
│
├─ [Model Update] → new global weights
│
└─ [PostgreSQL] → persist round results, scores, model hash

                    │
                    ▼
            fclc-web (Phoenix LiveView)
            • Dashboard: AUC, round count, node status
            • Shapley scores per node (bar chart)
            • DP budget remaining per node
            • Training history (rounds × AUC plot)
            • Node registry (add/remove)
```

---

## Part 3: Component Interaction Matrix

| From ↓ / To → | fclc-core | fclc-node | fclc-server | fclc-web | PostgreSQL |
|----------------|-----------|-----------|-------------|----------|------------|
| **fclc-core** | — | exports API | exports API | — | — |
| **fclc-node** | uses (DP, schema, deident) | — | REST (submit update, get global model) | — | — |
| **fclc-server** | uses (aggregation, Shapley) | REST (send global model) | — | Server-Sent Events / REST | reads/writes |
| **fclc-web** | — | — | REST (GET metrics, rounds) | — | — |
| **PostgreSQL** | — | — | writes/reads | — | — |

---

## Part 4: Privacy Architecture

```
Privacy Stack (per outgoing contribution):

Layer 1: Direct identifier removal
         name → ∅, exact_dob → year, MRN → ∅, address → ∅

Layer 2: Quasi-identifier generalization
         age → 5-yr group, rare diagnosis → "other" if count < suppression_threshold

Layer 3: k-anonymity check
         Each equivalence class must have k ≥ 5 records
         → records in groups < k are suppressed

Layer 4: Differential Privacy (Gaussian mechanism)
         gradient_noisy = gradient_clipped + Normal(0, σ²)
         σ = sensitivity × √(2 ln(1.25/δ)) / ε
         ε = 2.0 per round, δ = 1e-5
         Budget: ε_total ≤ 10.0 (= 5 rounds at ε=2.0, or ~50 rounds at ε=0.2 with Rényi DP)

Layer 5: Secure Aggregation (SecAgg+)
         Each node's update masked before transmission
         Orchestrator sees only the sum, never individual updates
```

---

## Part 5: Key Feedback Loops

```
[Model Quality] ──→ [Shapley Score] ──→ [Access Rights] ──→ [Participation Incentive]
      ↑                                                               │
      └───────────────────────────────────────────────────────────────┘
                         (positive feedback: better data → better model → more reward)

[DP Budget Spent] ──→ [Budget Exhausted] ──→ [Node Excluded from Round]
      ↑                                              │
      └──────────────────────────────────────────────┘
                         (negative feedback: limits overcontribution)

[Byzantine Detection] ──→ [Krum Rejection] ──→ [Reputation Score Drop]
                                                       │
                                    ──→ [Automatic Exclusion]
```

---

## Part 6: Module Dependencies

```
fclc-core
├── dp/          (rand, rand_distr)
├── scoring/     (no external deps)
├── aggregation/ (no external deps)
├── schema/      (serde)
└── privacy/     (no external deps)

fclc-node
├── fclc-core
├── eframe / egui / egui_plot   (GUI)
├── reqwest (HTTP client)
├── csv (data import)
└── tokio (async runtime)

fclc-server
├── fclc-core
├── axum (REST API)
├── sqlx + tokio-postgres (DB)
├── uuid (node/round IDs)
└── tower-http (CORS, middleware)

fclc-web
├── Phoenix (Elixir)
├── Phoenix LiveView (real-time UI)
└── HTTPoison (calls fclc-server REST API)
```
