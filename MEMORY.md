# MEMORY.md — FCLC: Project Memory

## Key Decisions & Architecture Choices

### 2026-04-06 — Pilot Ready: All Endpoints Verified + Rényi DP Active
- **Rényi DP wired into production:** `RdpAccountant` from `fclc-core/src/dp/renyi.rs` connected to `fclc-server/src/state.rs` (NodeDpState)
- **Savings:** ~1.985ε/round vs linear at σ=0.89, q=0.013 (verified live 2026-04-06)
- **Effect:** ~30–40 rounds per node instead of 5 (linear limit)
- **API verified:** All 13 endpoints tested live with real DB (postgres://fclc@localhost:5432/fclc)
- **New files:**
  - `docs/DUA_template.md` — Data Use Agreement for clinic onboarding
  - `docs/IRB_protocol_template.md` — IRB/Ethics committee protocol
  - `docs/DATASETS.md` — Public datasets: Synthea → MIMIC Demo → NHANES → UK Biobank
  - `scripts/generate_demo_data.py` — 3 clinics × 500 synthetic patients
  - `data/clinic_node{1,2,3}_demo.csv` — ready-to-use demo datasets
- **Bug fixed:** TIMESTAMPTZ vs TIMESTAMP in db/mod.rs (removed AT TIME ZONE 'UTC' cast)
- **Workspace Cargo.toml:** Created at FCLC/ root (was misplaced in docs/)
- **Tests:** 38/38 pass (27 unit + 4 integration + 7 orchestrator)

### 2026-03-26 — Project Concept Finalized (Version 5.0)
- **Status at creation:** CONCEPT.md v5.0 marked final; peer review loop running via `fedclinai_loop.py` → `CONCEPT_REVIEWED.md`
- **External co-author confirmed:** Giorgi Tsomaia (external collaborator, not institutional affiliation)
- **Project alias change:** Previously referred to as `FedClinAI` → renamed to `FCLC` (Federated Clinical Learning Cooperative)
- **Repos created:** `djabbat/FCLC` (private) + `djabbat/FCLC-public` (public, excludes CLAUDE/TODO/PARAMETERS/MAP)

### Federated Approach vs. Centralized — Why Federated
- **Decision:** Federated learning (data stays at clinic) over centralized (data pooled to one server)
- **Rationale:**
  1. GDPR Article 9 — health data is "special category"; transferring raw records requires DPAs and is legally risky
  2. Georgian PDPL (Personal Data Protection Law) — mirrors GDPR, same restrictions
  3. Clinics refuse to transfer patient records (reputational + legal risk)
  4. Federated learning produces gradients/weights only — mathematically bounded leakage via DP
- **Consequence:** Every clinic runs a local `fclc-node` binary; only masked gradient updates travel to orchestrator

### Differential Privacy: Gaussian Mechanism, ε=2.0/round
- **Decision:** DP-SGD with Gaussian mechanism; ε=2.0 per round, δ=1e-5, total budget ε_total=10.0
- **Rationale:** ε=2.0 is in the practical medical FL range (strong enough for regulatory compliance, small enough utility loss); Rényi DP accounting enables subsampling to extend budget
- **Never change without justification:** These values are the privacy guarantee; weakening them invalidates compliance claims

### Byzantine Robustness: Krum at f=0.25
- **Decision:** Krum robust aggregation rejecting up to f=⌊0.25×n⌋ Byzantine nodes
- **Rationale:** Protects against poisoning attacks from malicious or compromised clinic nodes; 25% tolerance is standard in Byzantine FL literature
- **Never change without justification:** Lower f_ratio = more vulnerable; higher f_ratio = fewer valid updates used

### Shapley Value Scoring: Monte Carlo, M=150
- **Decision:** Monte Carlo Shapley with M=150 permutation samples
- **Rationale:** M=150 balances accuracy (variance < 5% for n≤10 nodes) against compute (~10 min/round at n=10)
- **Metric:** AUC on held-out validation set as the performance function v(S)

### MVP Target: Diabetes Type 2 or Oncology, 5–10 Clinics
- **Decision:** First pilot uses logistic regression on either T2DM or oncology outcomes
- **Rationale:** T2DM has well-defined ICD-10 codes, standardized labs (HbA1c, glucose), high prevalence — easy to source across clinics; oncology has high stakes motivating participation
- **Node count:** 5–10 clinics for pilot (3 minimum to proceed per round; Krum needs ≥4 for f=1)
- **Target clinics:** WLRAbastumani as potential first participant (DrJaba ecosystem connection)

### Tech Stack: Rust + Elixir/Phoenix
- **Core + Node + Server:** Rust — performance-critical (DP noise, gradient processing, Krum, Shapley)
- **Web dashboard:** Elixir/Phoenix LiveView — real-time UI without JavaScript framework complexity
- **Storage:** PostgreSQL (orchestrator only) — nodes are stateless except local data
- **No raw patient data ever leaves clinic node**

### SecAgg+: Orchestrator Sees Only Aggregated Sum
- **Decision:** Secure Aggregation (SecAgg+) protocol; masked gradients before transmission
- **Consequence:** Even if orchestrator is compromised, individual clinic updates are not recoverable
- **Status:** Implementation pending (Task #10 in TODO.md)

---

## Version History

| Version | Date | Status | Notes |
|---------|------|--------|-------|
| 0.1.0-alpha | 2026-03-28 | In progress | Rust workspace builds; CONCEPT v5.0 finalized; peer review loop running |

---

## External Collaborator

**Giorgi Tsomaia**
- Role: External co-author (scientific/methodological)
- Not affiliated with an institution in the project structure
- To be listed as co-author on any publications arising from FCLC
- Contact management: through Dr. Tkemaladze directly

---

## Grant Target

**EIC Pathfinder — deadline: 10 May 2026**
- Part B narrative (10 pages) not yet written
- Letters of support from 3 clinics required
- Ethics statement required
- Demo video (2 min) planned

---

## Lessons Learned / Early Decisions

- **OMOP CDM as normalization layer:** Using OMOP (Observational Medical Outcomes Partnership Common Data Model) enables multi-clinic data harmonization without custom mappings per clinic; ICD-10 → concept_id, LOINC for labs, ATC for meds
- **fclc-web scaffolding deferred:** Phoenix LiveView web dashboard (fclc-web) not yet scaffolded; `mix phx.new` is Task C3 — do not start fclc-web work before fclc-server REST API is stable
- **Cargo workspace strategy:** `fclc-core` as pure library (no binary) keeps dependency graph clean; `fclc-node` and `fclc-server` depend on it; avoids circular deps

---

## Pending Critical Path

1. CONCEPT peer review → ACCEPT
2. fclc-web Phoenix scaffold
3. PostgreSQL migrations + REST endpoint implementations
4. SecAgg+ masking protocol (fclc-core, Task #10)
5. 3-node local pilot (5 federated rounds)
6. EIC Pathfinder grant application (deadline 10 May 2026)
