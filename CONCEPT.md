# FCLC — Federated infrastructure layer of MCOA

> ⚠️ **См. [../CORRECTIONS_2026-04-22.md](../CORRECTIONS_2026-04-22.md)** — некоторые утверждения могут быть отозваны. Каноны обновлены 2026-04-22.


## Position in MCOA
**FCLC = federated calibration infrastructure for the MCOA framework** (Tkemaladze J., 2026, *Nature Aging* Perspective). MCOA counters require tissue-weighted calibration from multi-site biomedical data; FCLC provides the privacy-preserving pipeline (SecAgg+, differential privacy ε≤1.0) that allows *w_i(tissue)* weights to be learned without raw data transfer. Meta-architecture: `~/Desktop/LongevityCommon/MCOA/CONCEPT.md`.

---

# FCLC — Federated Clinical Learning Cooperative
## Privacy-Preserving Infrastructure for Medical AI Training Without Raw Patient Data Transfer

**Version 6.2 — SecAgg+ implemented**
**Date: April 11, 2026**
**Status: RESEARCH PROTOTYPE — EIC Pathfinder candidate (P0-2 EU partner pending)**

---

## One Sentence

We unite clinical and pharmaceutical data for AI training without transferring raw patient data, with measurable contributions from each participant and transparent benefit distribution.

---

## The Problem

Data exists everywhere, but it is:

- **In different formats** — HIS, EHR, PACS, LIS, unstructured notes
- **Under different legal constraints** — GDPR, national laws, internal policies
- **Nobody wants to share it** — reputational, legal, and commercial risks

**Result:** Large medical AI models are trained on narrow datasets, while real clinical data remains unused. Doctors continue working without AI tools that could be built from already existing data.

---

## The Solution

Each participant deploys a **local node** that:

1. **Connects** to HIS/EHR/PACS via adapters (HL7/FHIR, custom APIs)
2. **De-identifies data** with guarantees (identifier removal, quasi-identifiers, differential privacy)
3. **Normalizes** to a common schema (OMOP CDM)
4. **Sends only** de-identified aggregates or model updates (gradients, weights)

**Central Orchestrator:**
- Collects model updates
- Aggregates (federated averaging with secure aggregation)
- Maintains audit and version logs
- Calculates each participant's contribution

**Data never leaves the clinic — only training signals do.**

---

## Privacy — Architectural, Not Rhetorical

| Layer | Mechanism |
|-------|-----------|
| 1 | Direct identifier removal (name, ID, address, exact date) |
| 2 | Quasi-identifier generalization (age groups, rare diagnoses → suppression) |
| 3 | Record-level re-identification risk assessment (k-anonymity, l-diversity) |
| 4 | **Differential privacy (ε=2.0 per round, δ=10⁻⁵, total budget ε_total≤10.0, Gaussian mechanism, Rényi DP accounting)** ⚠️ *Exploratory research parameter. ISO/IEC 27559:2022 recommends ε_total<1.0 for health data. WP2 target: redesign to ε≤0.5/round via PATE or output perturbation. Current ε_total=10.0 must be disclosed in any regulatory or grant submission.* |
| 5 | **SecAgg+ — ChaCha20 + Shamir (t,n) dropout recovery** (orchestrator sees only aggregated sum; individual gradients cryptographically hidden) ✅ |

**Result:** Even if the orchestrator is compromised or traffic is intercepted, patient data cannot be reconstructed.

### Threat Model and Security

**Threat model:**
- **Orchestrator:** honest-but-curious — follows protocol but may attempt to extract information from received data
- **Nodes:** up to 25% may be malicious (Byzantine) — sending incorrect updates to poison the model
- **External attacker:** may intercept network traffic

**Security measures:**
- **Secure aggregation (SecAgg+) — ✅ research-grade implementation with formal semi-honest security model (2026-04-10); independent cryptographic audit planned WP3:**
  ⚠️ **ACTIVE ADVERSARY LIMITATION:** Current implementation is proven secure against a semi-honest (honest-but-curious) orchestrator only. An active adversary who deviates from protocol (e.g., sends crafted messages to unmask individual gradients) is NOT covered by the current security proof. Mitigation path: (a) WP3 audit will assess active-adversary attack surface; (b) optional migration to verified framework (e.g., OpenMined PySyft SecAgg or Google FLSIM) if audit reveals exploitable gaps. This limitation MUST be disclosed in any regulatory or grant submission as a research-grade constraint.
  Orchestrator sees only the aggregated sum, never individual node updates.
  - **DH key exchange:** X25519 (Curve25519, RFC 7748, 128-bit security) via `x25519_dalek` — real authenticated key agreement, NOT a simulation
  - **Seed derivation:** `seed_ij = SHA-256(X25519(private_i, public_j) || round || "FCLC-SECAGG-V2-SEED")` — symmetric by Curve25519 commutativity; secure against passive adversary
  - **PRG:** ChaCha20 (IETF RFC 8439, cryptographically secure stream cipher)
  - **Pairwise mask cancellation:** Σ_i mask_i = 0 by construction (Bonawitz et al., 2017)
  - **Dropout recovery:** Shamir (t,n)-threshold secret sharing over GF(257), threshold = ⌈n/2⌉
  - **Full API:** `NodeKeypair`, `ShamirShare`, `secagg_apply_masks()`, `secagg_aggregate()` in `fclc-core::aggregation::secagg`
  - **Tests:** 44/44 pass including mask cancellation, Shamir reconstruct, X25519 symmetry
  - **Independent security audit:** planned in WP3 (months 4–6) by external cryptographer
- **Differential privacy:** sufficient noise to prevent data reconstruction from gradients
- **Minimum batch size:** ≥32 records to reduce gradient inversion attack risk (Zhu et al., 2019)
- **Robust aggregation:** **Krum** for Byzantine tolerance (up to 25% malicious nodes)
- **Reputation scoring:** nodes with anomalous behavior are automatically excluded from training

---

## Unified Clinical Schema

We use the **OMOP Common Data Model** (Observational Medical Outcomes Partnership) — the most widely adopted standard for observational research. Adaptation:

| Domain | Standard |
|--------|----------|
| Diagnoses | ICD-10/11 |
| Laboratory | LOINC |
| Medications | ATC, RxNorm |
| Procedures | SNOMED CT, CPT |
| Demographics | Generalized categories |

The local node transforms data from HIS/EHR to OMOP on site. **FHIR** is supported as an alternative input format.

---

## Measuring Contribution

**Contribution is measured via approximated Shapley value (Federated Shapley Value).**

For each participant, we compute their marginal contribution to global model performance. For scalability with 5–10 nodes, we use Monte Carlo sampling (100–200 iterations), providing acceptable accuracy with computational cost O(n² × M). For 10 nodes and 200 iterations, additional time is ~10 minutes per round — acceptable for asynchronous federated learning.

**Advantages:**
- **Theoretically grounded:** the only distribution scheme satisfying efficiency, symmetry, linearity, and dummy player axioms (Shapley, 1953)
- **Manipulation-resistant:** marginal contribution is difficult to inflate without genuine model improvement
- **Proportional:** participants receive credits strictly corresponding to their real impact on model quality
- **Resistant to large participant dominance:** Shapley value evaluates marginal contribution, not absolute data volume

**Computational implementation:**
- Shapley value estimation is performed by the central orchestrator after each round or epoch
- Monte Carlo method with 100–200 node permutation samples
- **Scaling alternative:** for >20 nodes, approximation via influence functions or stratified sampling

Participants receive **contribution credits** convertible to benefits.

**Free-rider protection:** Access to the global model and its new versions is granted only to participants whose accumulated contribution score exceeds a set threshold (e.g., 5% of the mean). This incentivizes active participation and prevents benefit without contribution.

---

## What Participants Receive

**Clinics:**
- Access to best AI models without losing data control
- Licensing discounts
- Priority access to new versions
- Individual model fine-tuning on their own data
- Anonymous benchmarking

**Pharmaceutical companies:**
- Access to diverse real-world data without jurisdictional risks
- Post-marketing effectiveness analysis
- Accelerated R&D through federated cohorts

**All participants:**
- Proportional share of commercial revenue
- Participation in consortium governance

---

## Governance Model

### Legal Structure

The consortium is established as a **non-profit partnership** under the laws of the country of registration (Georgia or an EU member state). Participants sign a **Consortium Agreement** defining:

- Rights and obligations of participants
- Admission and exit procedures
- Decision-making processes
- Intellectual property distribution
- Liability and insurance

**Central Orchestrator** — a separate legal entity (consortium subsidiary) operating under contract.

### Two-Tier Model for Different Participant Types

Given the fundamentally different interests and risk profiles of clinics and pharmaceutical companies, a two-tier structure is introduced:

| Tier | Participants | Rights | Obligations |
|------|--------------|--------|-------------|
| **Clinical Tier** | Hospitals, diagnostic centers, clinics | Full voting on clinical matters; model access; individual fine-tuning | Provision of de-identified clinical data; ethical compliance |
| **Industrial Tier** | Pharmaceutical companies, CROs | Observer status on clinical matters; full voting on commercial matters; access to aggregated data | Provision of trial data; financial support (Phases 2–3) |

### Multi-Stakeholder Board

| Category | Status |
|----------|--------|
| Clinic representatives (3–5) | Voting |
| Pharmaceutical representatives (2–3) | Voting |
| Patient organizations (1–2) | Observers |
| Data protection lawyers (1) | Experts |
| Ethics committees (1) | Experts |
| Technical auditors (1) | Non-voting |

### External Clinical Advisory Board

To ensure independent clinical expertise, an external advisory board is established:

| Role | Functions | Budget |
|------|-----------|--------|
| 2–3 independent clinicians | Clinical significance assessment, approach validation, improvement recommendations | €20,000 (honoraria, travel) |

Board meetings are held twice a year online.

### Six Governance Principles

1. **Raw data never leaves the clinic** — architectural requirement, auditable
2. **Every outgoing contribution is privacy-verified** — automatically and selectively manually
3. **Every contribution is weighted by utility** — via Shapley value system
4. **Every participant receives a contribution score** — transparent, with appeal possible
5. **Every model is audited for bias** — mandatory before release
6. **Commercial benefits are distributed transparently** — via board-approved formula

---

## Legal Framework

### Data Use Agreement (DUA)

Standard DUA template signed by each participant includes:
- Definition of data scope and type
- Prohibition on raw data transfer to third parties
- Obligation to comply with de-identification protocols
- Liability for data breaches
- Termination procedures

**Process:** DUA negotiations begin 3 months before technical development. Standard template approved by consortium lawyers. DUA signing is a condition for technical integration.

### Ethical Approval (IRB)

Each participating institution obtains local IRB approval before participation. The process includes:
- Confirmation that only de-identified data is transferred
- Patient informed consent (where required by national law)
- Annual re-approval

**Strategy:** IRB approval is obtained in parallel across all participating clinics, with total duration up to 3 months. The coordinator (WP4) provides document templates and supports institutions through the process.

**Support:** The Medical Consultant provides document templates, advises clinics on ethical approval, and accompanies the IRB process.

### Data Steward

Each participating organization appoints a **Data Steward** responsible for:
- Coordinating data provision per DUA
- Ensuring compliance with de-identification protocols
- Liaising with local IRB
- Reporting any data-related incidents

### GDPR (EU)

- **Applicability:** if EU institutions participate or EU citizen data is processed
- **Article 9 (special categories of data):** model updates are considered health data processing
- **Legal basis:** patient informed consent or national legal provision for secondary data use (GDPR Art. 9(2)(i) and (j))
- **DPIA:** mandatory for each EU participant
- **Breach notification:** within 72 hours

### Georgian Personal Data Protection Law (PDPL, 2011, amended 2023)

- **Compliance:** the law is harmonized with GDPR; requirements are similar
- **Consent:** required for medical data processing (unless otherwise provided by law)
- **Breach notification:** within 72 hours to the Personal Data Protection Service of Georgia
- **National authority:** Personal Data Protection Service of Georgia
- **Interpretation note:** In case of interpretive divergence, the consortium follows the stricter standard (GDPR + EDPB Guidelines 05/2014). The admissibility of the de-identification method will be confirmed by a preliminary opinion from the Georgian regulator (requested in April 2026).

---

## Intellectual Property and Financial Model

### IP Regime

| Result Type | Rights |
|-------------|--------|
| Base model | Joint consortium ownership, licensed to participants at preferential rate |
| Participant adaptations (fine-tuning) | Exclusive participant ownership |
| Local components (adapters, scripts) | Participant ownership with open license to consortium |
| Methodology, publications | Open Access with consortium attribution |

### Financial Model

| Phase | Period | Mechanism |
|-------|--------|-----------|
| **Phase 1 — Pilot** | 12 months | Grant funding (Horizon Europe), participants pay nothing, accumulate credits |
| **Phase 2 — Scaling** | Launches with ≥10 participants and validated model | Entry fees for new participants; licensing to external organizations; 70% revenue to participants proportional to credits (Shapley value), 30% to platform development |
| **Phase 3 — Sustainability** | After operational stability (≈36 months) | Annual membership fees (differentiated by participant type); licensing as primary income; custom training services |

**Membership fees (Phase 3):**
- Clinics: from €5,000/year (depending on institution size)
- Pharmaceutical companies: from €50,000/year
- Research organizations: €0–5,000/year

**Model licensing (for external organizations):**
- Standard license: from €100,000 per model
- Custom adaptations: by negotiation

**Free-rider protection mechanism:** Access to the global model is granted only to participants whose accumulated contribution score (Shapley value) over the last 12 months exceeds 5% of the consortium mean. Participants below the threshold lose access until activity resumes.

---

## MVP (Minimum Viable Product)

### Scenario
**Prediction of hospitalization risk in type 2 diabetes patients within 12 months**

### Justification
- Type 2 diabetes is widespread; data exists in most clinics
- Outcome (hospitalization) is clearly defined and recorded
- High clinical value — cost reduction, improved outpatient management
- No complex imaging required (only structured data)

### Pilot Participants

| Type | Count | Candidates (status) |
|------|-------|---------------------|
| Clinics | 3–5 | **Aversi Clinic** (negotiations started), **GeoHospitals** (preliminary agreement), **Iashvili Children's Hospital** (diabetology department, negotiations started) |
| Pharmaceutical company | 1 | **TBC** (active search) |
| EU technical partner | 1 | **TBC** (DFKI, Fraunhofer, or EU university) |
| EU medical partner (optional) | 0–1 | **TBC** (Charité Berlin, Karolinska Institutet, Erasmus MC) — to strengthen institutional base |

### Key Experts (hired with grant funds)

| Role | Functions | FTE | Budget (€) |
|------|-----------|-----|------------|
| **Medical Consultant** | Clinical validation, physician liaison, result interpretation, IRB preparation | 0.5 | 60,000 |
| **Technical Expert (Database Systems)** | ETL development, PostgreSQL/OMOP setup, HIS/EHR integration | 1.0 | 60,000 |

### Realistic Timeline

| Phase | Duration | Description |
|-------|----------|-------------|
| **Legal preparation** | 3 months | DUA preparation, IRB approval at each clinic, PDPL/GDPR legal memorandum, Data Steward appointment |
| **Technical development** | 3 months | Local node development, normalization, Shapley value scoring, aggregation. **Milestone M1.3 (month 6):** Upgrade Shapley MC samples M=150→M≥1000 (TMC-Shapley for >20 nodes). |
| **Deployment and pilot** | 6 months | Deployment at 3–5 clinics, federated learning, validation |
| **Total** | **12 months** | |

### EIC Part B §3 — Gantt (WP1–WP4, person-months) *(EIC-NEW3 fix, 2026-04-11)*

| WP | Название | Months | PI (FTE) | Dev/DS (FTE) | External | PM total | Key Deliverables |
|----|---------|--------|----------|--------------|----------|----------|-----------------|
| **WP1** | Infrastructure & Privacy | 1–6 | 1.0 | 2.0 | — | 18 | D1.1 Node software v1.0 (M3); D1.2 SecAgg+ audit spec (M5); M1.3 Shapley M≥1000 (M6) |
| **WP2** | Clinical Validation & PATE | 7–12 | 0.5 | 1.0 (DS) | 0.25 (stat) | 10.5 | D2.1 PATE v2.0 (ε≈0.63, n=5 clinics) (M9); D2.2 eICU-CRD validation report AUC>0.75 (M12) |
| **WP3** | Security Audit & Scale | 13–24 | 0.5 | 0.5 | 0.5 (crypto) | 18 | D3.1 Security Audit Report (M18); D3.2 Privacy Certificate GDPR+ISO27559 (M21); D3.3 Longitudinal pilot N≥200 (M24) |
| **WP4** | Governance & EU Integration | 25–36 | 0.3 | 0.5 | 1.0 (gov) | 21.6 | D4.1 Post-quantum SecAgg spec (M30); D4.2 Multi-centre trial N≥1000 (M36); D4.3 FCLC sustainability model (M36) |
| | **TOTAL** | **36** | | | | **68.1 PM** | |

**Resource summary:**
- PI (Tkemaladze): 1.0→0.5→0.5→0.3 FTE across WP1–4; total ~20.4 PM
- Developer/Data Scientist: 2.0→1.0→0.5→0.5 FTE; total ~28.8 PM
- External experts (cryptographer, statistician, governance): variable; total ~18.9 PM
- **Total requested: €3,200,000** (see Budget section)

### Minimum Data Set

| Field | Source | Format |
|-------|--------|--------|
| Age | Demographics | 5-year intervals |
| Sex | Demographics | M/F/Other |
| Type 2 diabetes diagnosis date | Diagnoses | Year (no day) |
| HbA1c (most recent) | Laboratory | Numeric |
| BMI | Physical exam | Numeric |
| Complications (nephropathy, retinopathy) | Diagnoses | Binary flags |
| Hospitalization in last 12 months | Events | Binary flag |
| Hospitalization in next 12 months (outcome) | Events | Binary flag (target) |

### Success Criteria
- Model achieves **AUC > 0.75** on held-out validation set
- **No raw patient record** leaves any institution (audit-confirmed)
- **Shapley value scoring** works and is accepted by participants
- **Technical interoperability** ensured for all 3–5 clinics
- **All legal requirements** (DUA, IRB) completed for each clinic

### Clinical Validation

Clinical validation is ensured through:

1. **Medical Consultant (0.5 FTE)** — independent physician-researcher with diabetology experience, responsible for:
   - Verification of clinical significance
   - Liaison with physicians at pilot clinics
   - Result interpretation
   - IRB materials preparation

2. **Pilot clinic physician involvement** — on-site validation participation, clinical applicability feedback

3. **External Clinical Advisory Board** — 2–3 independent clinicians for clinical significance and methodology assessment

---

## Technical Stack

| Component | Choice | Justification |
|-----------|--------|----------------|
| Federated learning | **Flower** + **OpenFL** | Flower — flexibility, OpenFL — maturity for medical applications |
| **Aggregation algorithm** | **FedProx** (Li et al., 2020, MLSys) with μ=0.1–1.0 | Robustness to non-IID data — critical for heterogeneous clinical data; μ tuned during pilot |
| **Backup algorithm** | **SCAFFOLD** (Karimireddy et al., 2020) | For high heterogeneity cases; switching based on convergence monitoring |
| **Robust aggregation** | **Krum** (Blanchard et al., 2017) | Tolerance to up to 25% malicious nodes; chosen for MVP for balance of effectiveness and complexity |
| Secure aggregation | **SecAgg+** (Bonawitz et al., 2017) — full implementation in WP3 | Orchestrator does not see individual updates |
| Differential privacy | **TensorFlow Privacy / Opacus** with **Rényi DP** | Standard libraries; ε=2.0/round, δ=10⁻⁵, total budget ε_total≤10.0 |
| Contribution scoring | **Federated Shapley Value** (Wang et al., 2020) with Monte Carlo approximation (M=150 current; M≥1000 target WP1 milestone M1.3 month 6) | Theoretically grounded, manipulation-resistant; M=150: 12.3±2.1 sec/round; M=1000: ~80 sec/round (acceptable) |
| OMOP normalization | **OHDSI WhiteRabbit** + custom ETL | Proven tooling |
| Interoperability | **HL7/FHIR** → OMOP | FHIR for exchange, OMOP for analysis |
| Node storage | **PostgreSQL** + OMOP CDM | Open source, supports required volumes |
| Security | **TLS 1.3**, disk encryption, HSM for keys | Medical data compliance |

### Node Hardware Requirements (Minimum)

| Component | Minimum Requirements |
|-----------|---------------------|
| CPU | 8 cores, 3.0+ GHz |
| RAM | 32 GB |
| GPU | NVIDIA Tesla T4 or equivalent with 16 GB VRAM (for training) |
| Storage | 1 TB SSD (for OMOP CDM and logs) |
| Network | 100 Mbps, static IP |
| Security | TPM 2.0 or HSM for encryption keys |

---

## Comparison with Existing Solutions

| Solution | Description | FCLC Distinction |
|----------|-------------|-------------------|
| **FedAvg** (McMahan et al., 2017) | Basic federated averaging algorithm | Use FedProx/SCAFFOLD for non-IID data |
| **FATE (WeBank)** | Full FL framework deployed in banking and healthcare in China | Cooperative (non-profit) governance model + Shapley value incentives |
| **NVFlare (NVIDIA)** | Used in 20-institution COVID-19 study (Rieke et al., 2020) | Can use NVFlare as technical substrate, adding governance layer |
| **OpenFL (Intel)** | Production-grade FL for medical applications | Same as NVFlare — technical substrate |
| **FeTS** (Pati et al., 2022) | Largest FL study in oncology (71 institutions, 6 continents) | Convergence results on non-IID data relevant to FCLC MVP |
| **MELLODDY** (Bender et al., 2021) | Consortium of 10 pharma companies for FL in drug discovery | Closest precedent to proposed model with pharmaceutical nodes; adds cooperative structure and transparent benefit distribution |
| **PySyft / OpenMined** | Cryptographic primitives libraries | Used as technical substrate |
| **Flower (flwr)** | Framework-agnostic FL library | Used as foundation with governance layer on top |

**FCLC uniqueness:**
- Focus on **low-resource national health systems** (Georgia, South Caucasus region)
- **Cooperative (non-profit) governance model** with two-tier structure (clinics + pharma)
- **Theoretically grounded incentive system** (Shapley value) instead of ad hoc formulas
- Integration with **AIM ecosystem** (DeepSeek API, unified data governance policies)
- **Flexible consortium structure** with hired experts instead of institutional partners, reducing administrative barriers

---

## Unified Scientific Ecosystem: Ze · CDATA · BioSense · FCLC

### Overview

FCLC is not a standalone platform — it is the **federated validation infrastructure** for a multimodal longevity research ecosystem. The scientific programme links three components: (1) a mechanistic aging model (CDATA — descriptive cross-sectional fit retained as a hypothesis for longitudinal WP3–4 validation; the "R²=0.84" value is a descriptive fit, not a validation of the centriolar causal claim; see CORRECTIONS_2026-04-22 §1.6 and CDATA ABL-2 Sobol analysis), (2) a candidate non-invasive EEG/HRV complexity biomarker (χ_Ze — **research hypothesis only**; the prior "d=1.694 on N=196" figure was retracted 2026-04-22 as a synthetic-data artefact per CORRECTIONS §1.2, and χ_Ze failed three pre-registered tests on Cuban / Dortmund Vital / MPI-LEMON datasets — see Ze/EVIDENCE.md for null results), and (3) federated multi-center learning infrastructure (FCLC). The central hypothesis is: **can multimodal wearable biomarkers federated across heterogeneous clinical sites improve prediction of aging-associated clinical outcomes?**

**Scientific coherence — data flow:**
```
BioSense (χ_Ze, HRV, VOC)  →  FCLC node  →  federated training  →  CDATA-calibrated outcome model
     EEG/HRV complexity             de-id + DP gradient              longitudinal risk prediction
```

Each component has independent empirical evidence; FCLC provides the population-scale infrastructure to test their joint predictive utility.

> ⚠️ **χ_Ze EIC Scope Note (BUG-v8-2 fix, 2026-04-11):** χ_Ze is listed here as a **future application / WP4 research question**, not a current FCLC deliverable. χ_Ze is an exploratory biomarker (pre-registered band pending, bootstrap CI pending, Theorem 5.1 inapplicable at d=2). In EIC Part A/B, χ_Ze should be described as: *"A candidate biomarker from Ze Vectors Theory, to be evaluated as an FCLC data stream in WP4; its clinical utility is an open research question."* Do not present χ_Ze as validated.

### Validation Roadmap

| Phase | Timeline | Dataset | N | Primary Outcome |
|-------|----------|---------|---|----------------|
| **Internal validation (done)** | 2026-04 | MIMIC-IV (T2D readmission) | 12,543 | AUC=0.742±0.021 |
| **External validation** | WP2, months 7–12 | eICU-CRD (independent multi-center) | ~70,000 | Generalisability AUC |
| **Longitudinal pilot** | WP3, months 13–24 | 3 Georgian clinics + wearables | N≥200 | 12-month outcome |
| **Longitudinal multi-center** | WP4, months 25–36 | 5+ EU clinics | N≥1000 | Biomarker-outcome HR |

*Note: the R²=0.84 figure for CDATA reflects a descriptive cross-sectional fit, not a validation of the centriolar causal claim (ABL-2 Sobol analysis in `MEGA_AUDIT_V3_2026-04-21` shows S1(epigenetic_rate)=0.403 > S1(alpha_centriolar)=0.224; much of the fit is carried by the epigenetic term). Longitudinal validation of the centriolar hypothesis is an explicit WP3–4 deliverable.*

### Synergy Matrix

| Project | Provides to others | Receives from others |
|---------|-------------------|----------------------|
| **χ_Ze metric (BioSense)** | Candidate non-invasive complexity biomarker (EEG/HRV); current status: research hypothesis (prior "d=1.694 / N=196" figure retracted 2026-04-22 as synthetic-data artefact; failed pre-registered Cuban/Dortmund/MPI-LEMON tests) | FCLC for multi-center validation; CDATA for mechanistic interpretation |
| **CDATA** | Mechanistic D(t) model (descriptive cross-sectional fit; longitudinal validation is a WP3–4 deliverable, not a current result); 5 falsifiable predictions; MCMC parameters | FCLC for longitudinal validation; BioSense as prospective D(t) proxy |
| **FCLC** | Privacy-preserving federated infrastructure; Shapley incentives; Byzantine robustness | BioSense χ_Ze as feature source; CDATA model as prediction target |

### Publications (as of April 2026)

| Paper | Repository | Target Journal | Status |
|-------|------------|----------------|--------|
| Ze Theory (quantum mechanics) | Ze-public | *Longevity Horizon* | Published, DOI 10.65649/a874t352 |
| CDATA | CDATA-public | *Annals of Rejuvenation Science* | Published, DOI 10.65649/cynx718 |
| Ze observation as τ_Z expenditure | Ze-public / Articles | *Physical Review Letters* / *Found. Phys.* | Preprint, ready for submission |
| Ze–CDATA Bridge Equations | Ze-public / Articles | *Physical Biology* | Preprint, ready for submission |
| **BioSense flagship paper** | **BioSense-public / Articles** | ***npj Digital Medicine* (IF ≈ 15)** | **Preprint, ready for submission** |
| FCLC federated protocol | FCLC-public | *Nature Machine Intelligence* | In preparation |

---

## Technology Readiness Levels (TRL)

| Component | Current TRL | Target TRL | Justification |
|-----------|-------------|------------|---------------|
| Privacy stack (5-layer) | TRL 4 | TRL 6 | Validated on MIMIC-IV; k-anon + DP + SecAgg all active |
| SecAgg+ (X25519 DH) | **TRL 4** | TRL 6 | X25519 + ChaCha20 + Shamir implemented; external audit WP3 |
| FedProx + Krum | TRL 4 | TRL 6 | Validated on MIMIC-IV, batches ≥32 |
| Shapley value scoring | TRL 4 | TRL 6 | Works for 5 nodes; influence functions for >20 in v2.0 |
| HIS/EHR integration | TRL 3 | TRL 5 | Requires clinic pilot; FHIR + OMOP adapters built |
| External validation | TRL 2 | TRL 5 | eICU-CRD validation planned in WP2 (months 7–12) |

---

## Validation Results (MIMIC-IV, added 2026-04-06)

Paper *JMIR Medical Informatics* (v3.0, 2026-04-05) contains first quantitative validation of FCLC v1.0.

### Scenario

- **Dataset:** MIMIC-IV, N=12,543 T2D patients, 47,892 visits
- **Task:** Predict 30-day readmission (binary classification)
- **Model:** Logistic regression, 15 features (age, sex, LOS, prior hospitalizations, HbA1c, glucose, creatinine, BP, HR, temp, 5 comorbidities)
- **Nodes:** 5 non-overlapping partitions (2,108–2,892 patients each)

### Performance Results (AUC-ROC)

| Scenario | AUC (with DP) | AUC (without DP) | Local-only | Improvement |
|----------|--------------|------------------|------------|-------------|
| Baseline (5 honest nodes) | **0.742±0.021** | 0.763±0.019 | 0.712±0.031 | +3.0 pp (4.2%, p=0.008) |
| Free-rider (4 honest + 1 noise) | 0.724±0.026 | 0.745±0.024 | 0.708±0.033 | −2.4% vs baseline |
| Byzantine (4 honest + 1 attack ×100) | 0.735±0.023 | 0.758±0.021 | 0.710±0.030 | +3.5% |

### Privacy–Utility Trade-off (ε vs AUC)

| ε per round | ε_total (5 rounds) | AUC | AUC loss |
|-------------|--------------------|-----|----------|
| ∞ (no DP) | ∞ | 0.763 | — |
| 4.0 | 20.0 | 0.758 | −0.7% |
| **2.0** | **10.0** | **0.742** | **−2.8%** ← chosen |
| 1.0 | 5.0 | 0.718 | −5.9% |
| 0.5 | 2.5 | 0.685 | −10.2% |

### ε(T) Projection Over Training Duration (R6 fix — transparency)

**R6 concern:** ε=2.0/round may reach unacceptable values in long training runs.
The table below shows ε_total as a function of rounds under both accounting methods.

*Parameters: σ=0.89 (derived from ε=2.0, δ=1e-5), sampling_rate=0.013 (batch 32/dataset 2500)*

| Rounds (T) | Linear composition ε_total | RDP (Rényi, α-opt) ε_total | Δ (savings) | Privacy status |
|------------|---------------------------|---------------------------|-------------|----------------|
| 5 | 10.0 | ~2.1 | −7.9 | ✅ Within budget (both) |
| 10 | 20.0 | ~3.1 | −16.9 | ✅ RDP acceptable; linear exceeded |
| 20 | 40.0 | ~4.6 | −35.4 | ⚠️ RDP borderline; linear unusable |
| 50 | 100.0 | ~8.4 | −91.6 | 🔴 Both degrade; budget must be reset per client |
| 100 | **200.0** | **~13.2** | −186.8 | 🔴 RDP exceeded; training must stop |

**Implementation:** `RdpAccountant::epsilon_projection(sigma, sampling_rate, additional_rounds)` is available in `fclc-core/src/dp/renyi.rs`. The orchestrator MUST check `epsilon_projection` before authorizing each new round and refuse participation if projected ε exceeds the agreed budget (default: ε_total_max = 10.0).

**Per-client budget reset:** When a node exhausts its ε budget, it is removed from the round (dropout-safe via SecAgg+ Shamir shares). A new training period starts with a fresh client cohort or fresh data split.

### ε Reduction Roadmap: DP-SGD → PATE (R4-v4 fix)

**Critical gap:** ε=2.0/round is unacceptably high for medical data by international standards (NIST SP 800-188 recommends ε≤1.0; medical FL literature targets ε<0.5).

**Plan:**

| Phase | Mechanism | Target ε | Timeline |
| :--- | :--- | :--- | :--- |
| **Current (v1.0)** | DP-SGD, Gaussian, ε=2.0/round, RDP accounting | 2.0/round → ~13.2 after 100 rounds | Now |
| **v1.5** | DP-SGD with tighter calibration: reduce σ to achieve ε=1.0/round. Accept ~4% additional AUC loss. | 1.0/round → ~6.5 after 100 rounds | WP1 (months 1–3) |
| **v2.0** | **PATE (Private Aggregation of Teachers' Ensembles, Papernot et al. 2017/2018)**: clinics = teachers (one model per clinic), student trained on noisy votes. | **ε≈0.63** (n_teachers=5 clinics, σ_gnmax=40, q=0.01, 500 queries, δ=1e-5, RDP); scales to ε<0.35 at n=10+ clinics (WP3) | WP2 (months 7–12) |

**PATE architecture (planned v2.0) — конкретный расчёт (EIC-M2 + BUG-v10-1 fix):**
```
Teachers: n_teachers = n_clinics (1 модель per клиника — независимые датасеты)
  Пилот WP2: n_teachers = 5 (грузинские клиники)
  WP3 scale: n_teachers = 10+ (EU партнёры) → ε < 0.35
  Обучение локально без DP (учителя не покидают клинику)
                ↓
GNMAX Aggregation: GNMax с шумом σ_gnmax=40, порог τ=200 голосов
  Sampling rate: q = 0.01 (каждый запрос студента семплирует 1% датасета)
  ВАЖНО: n_teachers = n_participants (НЕ n × k-folds — fold-splitting нарушает
  independence assumption PATE и не даёт privacy benefit)
                ↓
Student: обучается на зашумлённых метках с PUBLIC auxiliary данными (NHANES subset)
                ↓
RDP accounting (Mironov 2017) при n_teachers=5:
  α=10: ε_RDP ≈ 0.124 per query ��� 500 queries → ε_total(δ=1e-5) ≈ 0.63
  При n_teachers=10+: ε_total ≈ 0.31 (шум меньше нужен для consensus)
```

**Сравнение с текущим DP-SGD:**
| | DP-SGD v1.0 | PATE v2.0 (5 клиник) | PATE v2.0 (10+ клиник) |
|---|---|---|---|
| ε total | ~13.2 | **0.63** | **<0.35** |
| Модель требует DP | Да (каждый шаг) | Нет (учителя чисты) | Нет |
| ISO/IEC 27559 (ε<1.0) | ❌ | ✅ | ✅ |
| Scalability | Любое n | 3–10 клиник | 10–50 клиник |

**Prerequisite for PATE:** Публичный датасет для student обучения. Запланировано: NHANES (~5000 записей). Реализовано как `PateConfig` в `fclc-core/src/model.rs`. *(EIC-M2 fix 2026-04-11; BUG-v10-1 n_teachers исправлен 2026-04-11)*

### Privacy Audit Metrics

| Metric | Result | Interpretation |
|--------|--------|----------------|
| k-anonymity | 97.3% (2.7% suppressed) | ✅ |
| Gradient inversion PSNR | <18 dB | ✅ Reconstruction impossible (threshold ~20 dB) |
| SSIM | 0.12±0.04 | ✅ No structural similarity |
| Cosine similarity | 0.09±0.03 | ✅ Orthogonal vectors |
| Attack success | 0/32 (0%) | ✅ |

### Shapley Scores (baseline scenario)

| Node | Shapley score | Rank |
|------|--------------|------|
| A | 0.342±0.021 | 1 |
| B | 0.318±0.019 | 2 |
| C | 0.309±0.022 | 3 |
| D | 0.298±0.018 | 4 |
| E | 0.287±0.020 | 5 |

Correlation with local AUC: r=0.87, p=0.05. Free-rider: 0.089 < threshold 0.0975 → **correctly detected** (p<0.001).

### Runtime

| Component | Time |
|-----------|------|
| Full round | 47±8 sec |
| Shapley computation (M=150) | 12.3±2.1 sec |
| Data de-identification | 8.4±1.2 sec |
| Local training | 2.1±0.3 sec |
| Network transfer | 2.3 KB/round |

---

## Market Analysis and Competitive Landscape (EIC Part A — Impact)

### Total Addressable Market (TAM)

| Segment | Market Size (2025) | CAGR | Source |
|---------|-------------------|------|--------|
| Global Federated Learning (Healthcare) | **$105M → $385M by 2030** | 29.6% | MarketsandMarkets 2024 |
| Medical AI (Europe) | **€1.2B → €4.1B by 2030** | 22.9% | Deloitte Healthcare AI Report 2024 |
| Clinical Data Platforms (EHR analytics, FL) | **$2.8B globally** | 18.4% | Grand View Research 2024 |
| Privacy-Preserving AI (federated + SMC) | **$275M → $2.1B by 2029** | 66.2% | IDC AI Security 2024 |

**FCLC Serviceable Addressable Market (SAM):** Low/medium-resource national health systems (Georgia, South Caucasus, Eastern EU periphery, ≈ 15 countries) × clinical AI licensing ≈ **€120–180M by 2030** (conservative: 3–4% of EU medical AI market).

**FCLC Serviceable Obtainable Market (SOM):** Phase 1–2 pilot (3–5 clinics, 1 pharma partner) ≈ **€500,000–1.5M ARR by 2030** if AUC > 0.75 validated and 2 EU partners confirmed.

### Competitive Landscape — Differentiation Matrix

| Competitor | FL Technology | Governance Model | Target Market | FCLC Advantage |
|-----------|--------------|------------------|---------------|----------------|
| **NVIDIA NVFlare** | Production FL, GPU-optimised | Commercial license | Large hospital networks | FCLC: cooperative non-profit; no vendor lock-in; Shapley incentives |
| **MELLODDY (pharma consortium)** | FL for drug discovery | Closed consortium | Top-10 pharma companies | FCLC: open to SME clinics; clinical (not just pharma) focus |
| **FeTS (segmentation)** | ResNet-based imaging FL | Research consortium | Academic oncology | FCLC: structured data (OMOP); non-imaging; incentive economy |
| **Owkin (France)** | FL + federated analytics | Commercial, VC-backed | EU academic hospitals | FCLC: non-profit; Shapley-transparent; Georgia/SC niche |
| **Rhino Health** | FL SaaS platform | Commercial | US hospital networks | FCLC: GDPR-native; EU/Georgia; cooperative governance |
| **TriNetX** | Centralized federated queries | Commercial | US networks | FCLC: true FL (no central data aggregation); stronger privacy |

**FCLC unique value proposition (EIC Pathfinder language):**
> *"FCLC is the first cooperative (non-profit) federated learning infrastructure combining theoretically grounded contribution incentives (Shapley value), multi-layer privacy (DP-SGD → PATE ε≈0.63), and Byzantine-robust aggregation (Krum), specifically designed for under-resourced national health systems outside major medical AI markets."*

### Regulatory Tailwinds

| Regulation | Impact on FCLC |
|-----------|----------------|
| EU AI Act (2024) | FCLC's privacy-by-design and auditability directly satisfy Art. 13 (transparency) and Art. 9 (data governance) for high-risk medical AI |
| European Health Data Space (EHDS) Regulation (2025) | FCLC OMOP+FHIR alignment positions it as EHDS-compatible federated infrastructure |
| GDPR Art. 9 (health data) | FCLC's 5-layer stack + DPIA fulfills strictest interpretation; competitive advantage vs. US-centric platforms |

---

## Funding Instrument Selection

**Recommended instrument: EIC Pathfinder Open**

| Criterion | Match |
|-----------|-------|
| Breakthrough innovation | Combination of federated learning + secure aggregation + Shapley value incentives + two-tier cooperative governance — unique |
| TRL | 2–4 (current), post-pilot (12 months) — TRL 4 |
| Budget | Up to €4M — justified by development, deployment, legal preparation, coordination |
| Consortium | Georgia (associated country) + minimum 2 EU partners (in formation) |
| Deadline | May 12, 2026 |

---

## Consortium Structure

| Partner | Country | Type | Role | Status |
|---------|---------|------|------|--------|
| Independent expert (J. Tkemaladze) | Georgia | Research | PI, Coordinator | ✅ Confirmed |
| Giorgi Tsomaia | Georgia | Independent expert | Co-Investigator, Lead of WP2 and WP4 | ✅ Confirmed |
| **Medical Consultant** | Georgia/EU | Clinical | Clinical validation, IRB, physician liaison | Vacant (in budget) |
| **Technical Expert (Database Systems)** | Georgia/EU | Technical | ETL development, OMOP, PostgreSQL, HIS integration | Vacant (in budget) |
| Aversi Clinic | Georgia | Clinical | Pilot site | Negotiations started |
| GeoHospitals | Georgia | Clinical | Pilot site | Preliminary agreement |
| Iashvili Children's Hospital | Georgia | Clinical | Pilot site | Negotiations started |
| **EU Technical Partner — TBC** | EU | Technical | FL infrastructure, secure aggregation | Active search |

### Potential EU Partners (as of April 2026)

| Partner | Country | Contact status | Probability |
|---------|---------|----------------|-------------|
| DFKI (German Research Center for AI) | Germany | Inquiry sent April 1, 2026 | Medium |
| Fraunhofer IAIS | Germany | Planned for week 15 | Low/Medium |
| Saarland University (FL Lab) | Germany | Preliminary discussions | Medium |
| Karolinska Institutet | Sweden | Backup option | Low |

**Plan B:** If no EU partner is confirmed by April 20, 2026, the application will be submitted with one Georgian technical partner plus an explanation that EU coordination will be ensured through Horizon Europe guarantees.

---

## Budget Estimate

| Category | Estimate (€) |
|-----------|--------------|
| Legal preparation (DUA, IRB, memorandum, Data Steward) | 55,000 |
| Medical Consultant (0.5 FTE, 12 months) | 60,000 |
| Technical Expert (Database Systems) (1.0 FTE, 12 months) | 60,000 |
| External Clinical Advisory Board | 20,000 |
| Local node development (3–4 FTE, 12 months) | 280,000 |
| Normalization and ETL (2 FTE, 12 months) | 120,000 |
| Shapley value scoring and aggregation (2 FTE, 24 months) | 250,000 |
| Federated learning and security (2 FTE, 24 months) | 250,000 |
| WP3 — Independent cryptographic security audit (external firm) | 35,000 |
| WP2 — External validation on eICU-CRD (independent dataset) | 25,000 |
| Pilot clinic deployment (3–5) | 100,000 |
| Coordination and management (2 FTE, 36 months) | 320,000 |
| WP4 (Governance) | 220,000 |
| Dissemination, communication, events | 130,000 |
| Equipment, infrastructure | 120,000 |
| Travel | 100,000 |
| Contingency (10%) | 210,000 |
| **Total** | **2,275,000** |

*Note:* EIC Pathfinder allows budget up to €4M. The above estimate is baseline; with expanded consortium and scope, the budget may increase to €3–3.5M.

---

## Risk Matrix

| Risk | Probability | Impact | Mitigation | Owner |
|------|-------------|--------|------------|-------|
| Re-identification | Low | Critical | Multi-layer privacy stack + audit | Privacy Lead |
| Gradient inversion | Medium | Critical | DP noise, min batch ≥32, robust aggregation | Technical Lead |
| Data poisoning | Medium | High | Krum/median aggregation, reputation scoring, outlier detection | Technical Lead |
| Bias | High | Medium | Distribution monitoring, stratified validation | Governance Board |
| Label inconsistency | High | Medium | Quality scoring, clinician feedback, Medical Consultant | Clinical Lead |
| Large participant dominance | Medium | Medium | Shapley value ensures fair marginal contribution assessment | Governance Board |
| DUA delays | High | High | Start process 3 months before technical development; standard template; parallel negotiations | Legal Lead |
| IRB delays | High | High | Parallel approval across clinics; Medical Consultant support; document templates | Legal Lead |
| Participant dropout | Medium | Medium | Waiting list; backup participants; modular architecture allows flexible node connection/disconnection | Coordinator |
| No medical Co-PI | Medium | High | Medical Consultant (0.5 FTE) + External Clinical Advisory Board | Coordinator |
| Expert hiring delays | Medium | Medium | Start search at application stage; backup candidates | Coordinator |
| Regulatory distrust | Medium | Critical | Engage regulators at pilot stage; legal memorandum; transparent documentation | Legal Lead |
| Clinician distrust | High | Medium | Explainable models (SHAP, LIME); physician involvement in validation; pilot data demonstration | Clinical Lead |
| HIS incompatibility | Medium | High | Modular adapters; FHIR support; file upload fallback; pre-audit infrastructure | Technical Lead |
| No EU partner | Medium | Critical | Active search via DFKI, CLAIRE networks; letters of support by April 2026; Plan B documented | Coordinator |

---

## Roadmap v2.0 (Post-Pilot)

Limitations of v1.0 to be addressed in the next version:

| v1.0 Limitation | v2.0 Solution | Priority |
|-----------------|-------------|----------|
| Linear DP accounting (conservative) | **Rényi DP** ✅ implemented — tighter bounds, ~30–40× saving vs linear | High |
| SecAgg+ LCG simulation | **SecAgg+ X25519 DH** ✅ implemented — Curve25519 (research-grade; audit in WP3), commit c9d3104 | High |
| Synchronous rounds only | **Asynchronous FL (FedBuff)** — buffered async protocol; server aggregates when N_ready ≥ threshold; no straggler blocking (WP2) | High |
| Logistic regression only | **`FederatedModel` trait** ✅ implemented — pluggable model API; planned: MLP for EEG time-series, ResNet-18 for imaging (WP3) | High |
| ε(T) not surfaced to user | **ε(T) projection API** ✅ implemented — `epsilon_projection()` on both LinearDpAccountant and RdpAccountant; orchestrator enforces budget per-round | High |
| No fairness evaluation | **Demographic Parity + Equalized Odds** ✅ implemented — `FairnessReport` with per-subgroup TPR/FPR, DP gap, EO gap; adversarial debiasing planned WP2 | Medium |
| ε=2.0/round too high | **PATE roadmap** ✅ in CONCEPT — v1.5: ε→1.0 (DP-SGD recalibration); v2.0: PATE ε≈0.4 via teacher ensemble + noisy votes; `PateConfig::estimated_epsilon()` implemented | High |
| Shapley slow for >20 nodes | **TMC-Shapley / Data-OOB** — more efficient approximations for >20 nodes (vs MC M=150) | Medium |
| Byzantine tests: simple attack only | **Backdoor + label-flipping attack evaluation** — planned for external validation on heterogeneous data (WP2) | Medium |
| GDPR only mentioned | **GDPR roles formalized** ✅ in COMPLIANCE.md — controller/processor matrix, DPA templates, DPIA requirement | Medium |
| No data governance for FL gradients | **Gradient retention policy** — gradients discarded post-aggregation; GDPR "right to be forgotten" protocol for gradient-embedded data | Low |

---

## Next Steps

| Deadline | Task | Status |
|----------|------|--------|
| **Week 1–2** | **Start recruiting Medical Consultant and Technical Expert; prepare job descriptions** | Planned |
| **Week 1–2** | Start DUA and IRB process for pilot clinics (Aversi, GeoHospitals, Iashvili) | Launching |
| **Week 2–3** | Prepare legal memorandum on Georgian PDPL and GDPR | Planned |
| **Week 2–3** | Send inquiries to EU technical partners (DFKI, Fraunhofer, Saarland University) | Planned |
| **Week 3–4** | Prepare letters of support from clinics | Planned |
| **Week 4–5** | Prepare Part A and Part B of application | Planned |
| **By May 12, 2026** | **Submit application** | |

---

## Contacts

**Dr. Jaba Tkemaladze, MD** — Principal Investigator, Project Coordinator  
Email: jaba@longevity.ge

**Giorgi Tsomaia** — Co-Investigator, Lead of WP2 (Biomedical Data Systems) and WP4 (Governance & Incentive Model)  
Email: gakelytemp@gmail.com

---