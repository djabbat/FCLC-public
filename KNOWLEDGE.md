# KNOWLEDGE.md — FCLC: Domain Knowledge

## Federated Learning Principles

### What Federated Learning Is
Federated Learning (FL) is a machine learning paradigm where a model is trained across multiple decentralized nodes holding local data, without the data ever leaving those nodes. Only model updates (gradients or weights) are communicated to a central orchestrator.

**Core loop:**
1. Orchestrator sends global model weights to participating nodes
2. Each node trains locally on its private data for E epochs
3. Each node sends gradient update (not data) to orchestrator
4. Orchestrator aggregates updates → new global model
5. Repeat

**Key property:** Raw patient data never leaves the clinic.

### FedAvg vs. FedProx
| Algorithm | Proximal term | Best for |
|-----------|--------------|----------|
| FedAvg | None (μ=0) | IID data (same distribution across nodes) |
| FedProx | μ×‖w−w_global‖² | Non-IID data (clinics have different patient populations) |

FCLC uses **FedProx (μ=0.1)** — medical data across clinics is inherently non-IID (different demographics, specialties, data collection practices).

### Why Federated ML Outperforms Single-Institution Models
- More data → better generalization (especially for rare outcomes)
- Cross-clinic diversity reduces overfitting to one institution's biases
- Example: A model trained on 10 clinics (5,000 patients each) outperforms a model trained on 1 clinic (5,000 patients) on external validation

---

## Differential Privacy

### What Differential Privacy Guarantees
A randomized mechanism M satisfies (ε, δ)-DP if for any two datasets D and D' differing in one record, and any output set S:

```
Pr[M(D) ∈ S] ≤ e^ε × Pr[M(D') ∈ S] + δ
```

**Intuition:** An adversary who sees the output cannot determine with high confidence whether any individual record was in the training data.

### Gaussian Mechanism (DP-SGD)
The standard mechanism for differentially private gradient descent:

```
gradient_noisy = clip(gradient, max_norm) + Normal(0, σ²I)

where σ = max_norm × √(2 ln(1.25/δ)) / ε
```

**FCLC parameters:**
- ε = 2.0 per round
- δ = 1e-5
- max_norm (sensitivity) = 1.0
- σ derived automatically from above formula

### Rényi Differential Privacy (RDP) Accounting
Standard DP composition: ε_total = ε × T (linear in rounds). This is wasteful.

Rényi DP uses moments accounting with subsampling (fraction q of data per round):
- Effective ε per round ≈ q × ε (with subsampling amplification)
- Enables ~50 rounds at ε=2.0/round before budget ε_total=10.0 is exhausted (vs. only 5 rounds without subsampling)

### Privacy Budget Intuition
| ε value | Privacy level | Practical meaning |
|---------|--------------|-------------------|
| 0.1 | Very strong | Severe accuracy loss; impractical for most tasks |
| 1.0 | Strong | Significant noise; viable for large datasets |
| 2.0–8.0 | Practical | Medical FL range; good utility with meaningful protection |
| 10.0+ | Weak | Low protection; academic baseline only |

---

## Byzantine Robustness

### The Byzantine Threat Model
In federated learning, a "Byzantine" node can send arbitrary gradient updates — whether due to malicious intent (data poisoning, model poisoning) or software failure. A naive FedAvg is completely vulnerable: one poisoned update can bias the global model.

### Krum Algorithm
Krum selects the m = n − 2f gradient updates that are geometrically closest to each other (by L2 distance), rejecting the f most isolated updates.

**Guarantee:** If f < n/2, Krum converges to the correct model even with f Byzantine nodes.

**FCLC:** f = ⌊0.25 × n⌋; at n=10 clinics, f=2 Byzantine nodes are tolerated.

**Limitation:** Krum reduces effective sample size (m < n updates used); FedProx partially compensates.

---

## Shapley Value Contribution Scoring

### Why Shapley Values
Shapley values from cooperative game theory provide a **fair, axiomatic** method to attribute a model's performance improvement to each participating node.

**Axioms satisfied by Shapley values:**
- Efficiency: scores sum to total performance gain
- Symmetry: equal-contributing nodes get equal scores
- Dummy: a node contributing nothing gets 0
- Additivity: scores are additive across rounds

### Monte Carlo Shapley Estimation
Exact Shapley computation is O(2^n) — infeasible for n > 20. Monte Carlo approximation:

```
For each of M permutations π:
    For each node i:
        compute marginal contribution: v(S∪{i}) − v(S)
        where S = nodes before i in permutation π
Average marginals across permutations
```

**FCLC:** M=150 permutations; performance function v(S) = AUC on orchestrator's held-out validation set.

**Compute:** At n=10, M=150: ~1,500 AUC evaluations × ~0.5s each ≈ ~12 min/round (acceptable for 24h round intervals).

---

## Medical AI Regulatory Landscape

### GDPR and Health Data (EU)
- Health data is **Special Category** (Article 9); processing requires explicit consent or specific legal basis
- Federated learning + DP does not eliminate GDPR applicability — it reduces risk and enables legal basis claims
- **Data Processing Agreement (DPA):** Required between orchestrator operator and each clinic
- **Data Protection Impact Assessment (DPIA):** Recommended for high-risk processing

### Georgian Personal Data Protection Law (PDPL)
- Adopted 2011, amended 2023; substantially mirrors GDPR
- Special categories include health data; same restrictions apply
- Supervisory authority: Personal Data Protection Service of Georgia
- FCLC must prepare Georgian-language compliance documentation

### Medical Device Regulation (EU MDR 2017/745)
- AI systems intended to diagnose or treat may qualify as medical devices under EU MDR
- FCLC's MVP outputs are **decision-support tools** (not autonomous diagnostic devices); MDR applicability depends on intended use claims
- EIC Pathfinder application should avoid language that triggers MDR classification (phrase as "research tool" and "clinical decision support" not "diagnostic device")

### IRB and Ethics
- Any research using patient data (even federated + anonymized) typically requires Institutional Review Board (IRB) approval at each participating clinic
- **Data Use Agreement (DUA):** Specifies what data is used, for what purpose, with what protections
- FCLC must prepare template IRB protocol and DUA (Tasks #63–65 in TODO.md)

---

## Key Papers & Frameworks

### Foundational FL Papers
- McMahan et al. (2017) — "Communication-Efficient Learning of Deep Networks from Decentralized Data" — original FedAvg paper
- Li et al. (2020) — "Federated Optimization in Heterogeneous Networks (FedProx)" — basis for FCLC's FedProx μ=0.1
- Blanchard et al. (2017) — "Machine Learning with Adversaries: Byzantine Tolerant Gradient Descent (Krum)" — basis for FCLC's Byzantine robustness

### Differential Privacy
- Dwork & Roth (2014) — "The Algorithmic Foundations of Differential Privacy" — foundational textbook
- Mironov (2017) — "Rényi Differential Privacy" — basis for FCLC's Rényi DP accounting
- Abadi et al. (2016) — "Deep Learning with Differential Privacy (DP-SGD)" — basis for FCLC's gradient mechanism

### Shapley in FL
- Ghorbani & Zou (2019) — "Data Shapley: Equitable Valuation of Data for Machine Learning"
- Wang et al. (2020) — "Principled Shapley-value Estimation for Federated Learning"

### Secure Aggregation
- Bonawitz et al. (2017) — "Practical Secure Aggregation for Privacy-Preserving Machine Learning (SecAgg)"
- Bell et al. (2020) — "Secure Single-Server Aggregation with Sublinear Overhead (SecAgg+)" — basis for FCLC's SecAgg+

### Open-Source Tools
| Tool | Purpose | Notes |
|------|---------|-------|
| **Flower (flwr)** | FL framework | Reference for FCLC's round protocol design |
| **PySyft** | Privacy-preserving ML | Reference for SecAgg implementation |
| **TensorFlow Federated** | FL + DP | Reference for RDP accounting |
| **OpenDP** | DP library | Rust bindings available; alternative to custom DP implementation |
| **SQLx** | Async PostgreSQL | Used in fclc-server |
| **Axum** | Rust HTTP | Used in fclc-server |
| **egui / eframe** | Rust GUI | Used in fclc-node |

### Medical Data Standards
- **OMOP CDM v5.4:** https://ohdsi.github.io/CommonDataModel/ — normalization schema
- **ICD-10-CM:** Diagnosis codes (international version used by FCLC)
- **LOINC:** Laboratory test codes
- **ATC (Anatomical Therapeutic Chemical):** Medication classification
- **HL7 FHIR R4:** Data interchange format (FCLC's secondary connector format after CSV)

---

## OMOP Common Data Model (CDM)

### Why OMOP
OMOP CDM is the de facto standard for harmonizing clinical data across institutions for observational research. Over 300 databases worldwide mapped to OMOP.

### Key Tables Used by FCLC
| Table | Content |
|-------|---------|
| `person` | Demographics (year_of_birth, gender_concept_id; no name/DOB after de-identification) |
| `condition_occurrence` | Diagnoses (ICD-10 → OMOP concept_id) |
| `measurement` | Lab results (LOINC → concept_id, value_as_number, unit) |
| `drug_exposure` | Medications (ATC → concept_id) |
| `observation` | Clinical observations not captured elsewhere |

### De-identification in FCLC's OMOP Pipeline
1. Remove: `person_source_value` (MRN), `person_name`, `birth_datetime` (retain `year_of_birth`)
2. Generalize: `year_of_birth` → 5-year bucket (`year_of_birth - (year_of_birth % 5)`)
3. Suppress: rare `condition_concept_id` values (count < 5 in clinic dataset) → mapped to OMOP concept 0 ("Other")
4. k-anonymity: each combination of quasi-identifiers must appear ≥ 5 times in the export

---

## Hash-Chain Audit Log (v0.2.0, 2026-03-30)

### Алгоритм цепочки
```
entry_hash = SHA-256(round_id_bytes ‖ round_number_le64 ‖ gradient_hash_utf8 ‖ prev_hash_utf8)
gradient_hash = SHA-256(∀w ∈ weights: w.to_le_bytes())
genesis prev_hash = '0' × 64
```

### Свойства
- APPEND-ONLY: нет UPDATE/DELETE в таблице `audit_log`
- Каждый раунд фиксирует SHA-256 агрегированных весов → тампер-очевидность
- prev_hash связывает записи в цепочку — изменение любой записи ломает все последующие
- Верификация: `GET /api/audit` → проверить что `entries[i].prev_hash == entries[i-1].entry_hash`

### Prometheus метрики (`GET /metrics`)
| Метрика | Тип | Описание |
|---------|-----|---------|
| `fclc_rounds_total` | counter | Всего завершённых раундов FL |
| `fclc_active_nodes` | gauge | Зарегистрированных узлов |
| `fclc_auc_latest` | gauge | AUC последнего раунда |
| `fclc_avg_shapley` | gauge | Средний Shapley score |
