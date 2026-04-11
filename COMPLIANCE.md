# FCLC — Compliance & Ethics

## Overview

FCLC processes clinical patient data from multiple jurisdictions. All operations must comply with GDPR (EU), and be designed to be compatible with HIPAA (US) and equivalent national laws. This document tracks compliance status and requirements.

---

## Regulatory Frameworks

| Framework | Jurisdiction | Applies to FCLC? | Status |
|-----------|-------------|-----------------|--------|
| GDPR (General Data Protection Regulation) | EU/EEA | ✅ Yes — EU clinic partners | 🟡 Design phase |
| HIPAA | USA | ⚠️ If US partners join | 🔵 Future |
| UK GDPR | United Kingdom | ⚠️ If UK partners join | 🔵 Future |
| Georgian PDL (Law on PD Protection) | Georgia | ✅ Yes — PI location | 🟡 Review needed |
| ISO/IEC 27001 | International | 🎯 Target certification | 🔵 Future |
| ICH E6 GCP | Clinical research | ✅ Yes — clinical AI | 🟡 Design phase |

---

## Privacy Stack — Compliance Mapping

| Layer | Mechanism | GDPR Article | Status |
|-------|-----------|-------------|--------|
| L1: Direct identifier removal | name/MRN/address/exact_dob → ∅ | Art. 4(1), 25 | ✅ Implemented (`deidentify.rs`) |
| L2: Quasi-identifier generalization | age → 5yr bin; rare Dx → "other" | Art. 5(1)(c) | ✅ Implemented |
| L3: k-anonymity (k≥5) | Groups < k suppressed | Art. 89 | ✅ Implemented |
| L4: DP-SGD | ε=2.0/round, δ=1e-5, Rényi accounting | Art. 25 (privacy by design) | ✅ Implemented (`dp/mod.rs`, `renyi.rs`) |
| L5: SecAgg+ | Orchestrator never sees individual updates | Art. 5(1)(f) | ✅ X25519 DH + ChaCha20 PRG (upgraded 2026-04-09, commit c9d3104); research-grade implementation; semi-honest adversary model; independent audit planned WP3 |

**SecAgg+ status (updated 2026-04-09):** LCG simulation replaced with real X25519 Curve25519 Diffie-Hellman (`x25519-dalek v2`). Pairwise seeds now derived from 128-bit shared secrets. Independent cryptographic audit planned in WP3 (months 4–6).

### SecAgg+ Formal Adversarial Model (v10 fix, 2026-04-10)

*R1 v9: "SecAgg+ lacks a formal security proof under a realistic adversarial model."*

The SecAgg+ implementation provides security under the following explicitly stated threat model:

| Parameter | Value |
|-----------|-------|
| **Adversary type** | Semi-honest (honest-but-curious) orchestrator server |
| **Corruption threshold** | Up to f = ⌊0.25n⌋ Byzantine nodes (Krum-filtered) |
| **Dropout tolerance** | Up to f nodes may drop out; shares reconstructed via Shamir (threshold=2) |
| **Colluding nodes** | No node coalition assumed (non-colluding clinic nodes) |
| **Communication model** | Authenticated channels (HTTPS/TLS between nodes and orchestrator) |
| **Cryptographic assumptions** | CDH hardness for X25519; ChaCha20 as PRG (IETF RFC 7539); GF(2⁸) Shamir information-theoretically secure for threshold ≥ 2 |

**Security guarantee:** The orchestrator learns only the aggregate gradient vector, not any individual node's update. Formally: `View(server) = {SUM(masked_i + mask_cancel) for i in nodes}` where masks cancel by construction.

**Known limitations (not covered by this model):**
- Active adversary (malicious orchestrator injecting crafted aggregated updates): NOT protected — requires verifiable aggregation (WP3 extension)
- Gradient inversion after aggregation: protected by DP only (L4); strong DP (ε << 1) required for full protection
- Node key compromise: X25519 private keys stored on node hardware; no hardware security module (HSM) used

**✅ DELTA DISCREPANCY RESOLVED (2026-04-11):**
ConceptNote_AubreyDeGrey.docx was corrected: δ=10⁻⁸ → **δ=1e-5**. Canonical value δ=1e-5 is now consistent across all documents (CONCEPT.md, COMPLIANCE.md, ConceptNote_Eden.md, ConceptNote_AubreyDeGrey.docx). Code remains authoritative.

---

## IRB / Ethics Requirements

### Requirements by Study Phase

| Phase | Requirement | Status |
|-------|------------|--------|
| Software development | No IRB needed | ✅ Current phase |
| Synthetic data testing | No IRB needed | ✅ |
| Pilot with real clinic data | **IRB required at each site** | 🔴 Not started |
| Multi-site trial | **Multi-site IRB + DUA** | 🔴 Future |

### Documents Needed for Clinical Pilot

- [ ] IRB protocol — template at `docs/IRB_protocol_template.md`
- [ ] Data Use Agreement (DUA) — template at `docs/DUA_template.md`
- [ ] Informed consent waiver (retrospective data → may qualify)
- [ ] Data Processing Agreement (DPA) per GDPR Art. 28
- [ ] Risk assessment / DPIA (Data Protection Impact Assessment)

---

## Data Governance

### What FCLC Does NOT Store

| Data type | Reason |
|-----------|--------|
| Patient names, MRN, exact DOB | Removed at L1 (de-identification) |
| Raw gradient updates | SecAgg+ ensures orchestrator only sees aggregate |
| Individual node weights (pre-aggregation) | By SecAgg+ design |
| IP addresses of clinic nodes | Not logged |

### What FCLC Stores

| Data | Location | Retention | Encryption |
|------|----------|-----------|-----------|
| Aggregated global model weights | PostgreSQL `rounds` table | 100 rounds | DB-level (TLS in transit) |
| Round audit log (hash-chain) | PostgreSQL `audit_log` | Permanent | SHA-256 integrity |
| Shapley scores (per-node) | PostgreSQL `shapley_scores` | 100 rounds | DB-level |
| Per-node DP epsilon spent | PostgreSQL `nodes` table | Permanent | DB-level |
| Anonymized update metadata (loss, AUC, record_count) | PostgreSQL `updates` | 100 rounds | DB-level |

### Node Registration

Nodes register with a UUID. No clinic name or location is stored by default. Clinic-to-UUID mapping is maintained offline by the administrator.

---

## GDPR Roles and Responsibilities (R7 fix)

GDPR Art. 4(7)/(8) requires explicit designation of controller and processor roles. In federated learning, these are distributed — this must be formalized before any clinical pilot.

| Entity | GDPR Role | Scope | Required Document |
|--------|-----------|-------|-------------------|
| **PI / CommonHealth platform** | Joint Controller (Art. 26) | Defines FL purpose, model architecture, DP parameters | Joint Controller Agreement (JCA) |
| **Each participating clinic** | Joint Controller | Controls patient data within its node; decides on participation | JCA + local DPA approval |
| **FCLC orchestrator server** | Data Processor (Art. 28) | Aggregates masked updates; stores round metadata | Data Processing Agreement (DPA) per clinic |
| **Cloud infrastructure provider** | Sub-processor | Hosts orchestrator | DPA with sub-processor clause |

**Key principle (FL-specific):** Patient data never leaves the clinic node. However, gradient updates may contain residual information about training data (membership inference risk). SecAgg+ mitigates this by ensuring orchestrator sees only the aggregate — but this must be documented explicitly in each DPA.

**International transfers:** If clinic nodes are outside the EU/EEA, Standard Contractual Clauses (SCCs, 2021 version) are required. Assessed per partner country before onboarding.

---

## Data Protection Impact Assessment (DPIA) — Status

GDPR Art. 35 requires a DPIA for systematic processing of health data at scale.

| Component | DPIA Required? | Status |
|-----------|---------------|--------|
| FCLC clinical pilot (≥3 clinics) | ✅ Yes — health data at scale | 🔴 Not started |
| MIMIC-IV dev/testing (de-identified) | ⚠️ Borderline — check T&C | 🟡 Under review |
| CommonHealth Ze·Profile (user-submitted) | ✅ Yes — health data + AI profiling | 🔴 Not started |
| BioSense sensor data collection | ✅ Yes — biometric + health data | 🔴 Not started (no prototype) |

**Action required before any clinical pilot:** DPIA must be completed and submitted to the relevant supervisory authority (e.g., Data Exchange Agency, Georgia; or national DPA of EU partner).

### DPIA Action Plan (v8 fix, 2026-04-10)

GDPR Art. 35 mandates DPIA before processing health data at scale. The following 5-step plan operationalizes the skeleton into a concrete work programme.

| Step | Action | Responsible | Deadline | Output |
|------|--------|-------------|----------|--------|
| **D1** | Engage GDPR-qualified legal counsel (at least 10h review engagement) | PI | 2026-05-15 | Legal engagement letter |
| **D2** | Complete DPIA_skeleton.md: processing description, data flows, data subjects | PI + Tech Lead | 2026-05-30 | Draft DPIA v0.1 |
| **D3** | Risk identification workshop: membership inference, re-identification, de-anonymisation | PI + Legal + EU partner DPO | 2026-06-15 | Risk register |
| **D4** | Mitigation mapping: match SecAgg+, DP, k-anonymity to identified risks | Tech Lead | 2026-06-30 | DPIA v1.0 (complete draft) |
| **D5** | Submit DPIA to supervisory authority (Data Exchange Agency, Georgia + EU partner national DPA) | PI + Legal | 2026-07-15 | DPIA submission receipt |

**Prior consultation (Art. 36):** If residual risk remains HIGH after D4, prior consultation with supervisory authority is mandatory — add 6–8 weeks to timeline.

### IRB / Ethics Status per Dataset (v8 fix, 2026-04-10)

All datasets must have explicit ethics approval or documented exemption before use.

| Dataset | IRB Status | Exemption Basis | Blocks | Action Required |
|---------|-----------|----------------|--------|-----------------|
| **Synthea (synthetic)** | ✅ EXEMPT | Fully synthetic — no real patients | Nothing | None |
| **MIMIC-IV Demo** | ✅ EXEMPT | PhysioNet de-identified; per-user credentialing | Nothing | Each user must complete PhysioNet credentialing |
| **Cuban EEG Dataset** | 🔴 NOT STARTED | — | v*_active CI; Ze validation | IRB at KIU + data provider agreement with CNEURO |
| **CDATA cohort** | 🔴 NOT STARTED | — | CDATA-Ze regression; Φ(D) selection | IRB at collection institution |
| **FCLC clinical pilot** | 🔴 NOT STARTED | — | Any clinical data processing | Multi-site IRB + DUA per COMPLIANCE.md; DPIA required first |
| **UK Biobank** | 🟡 PLANNED Q4 2026 | — | χ_Ze vs aging clocks benchmark | UK Biobank application (category: aging biomarkers) |

**Enforcement rule:** Any dataset with `NOT STARTED` status must NOT be used in any code path outside of `Exempt` contexts. The `dataset_ethics_catalogue()` function in `fclc-core` provides compile-time tracking of this status.

**DPIA template initiated (v4.0 fix):** A skeleton DPIA document is available at `docs/DPIA_skeleton.md`. It covers: processing description, necessity/proportionality assessment, risk identification, and mitigation measures. A GDPR-qualified lawyer must review and complete it before any clinical data processing.

---

## Informed Consent — FL-Specific Protocol (v4.0 fix)

Standard informed consent is insufficient for FL. Patients must understand that:
1. Their data **does not leave** the clinic node in raw form
2. A mathematical **model update (gradient)** is derived from their data and aggregated with others
3. The gradient is protected by **cryptographic masking (SecAgg+)** and **noise addition (DP)**
4. Despite these protections, a residual **membership inference risk** exists
5. They can **withdraw** participation; however, gradients already incorporated into past rounds **cannot be removed** (technical limitation — must be explicitly disclosed)

**Template:** `docs/IRB_protocol_template.md` already exists. Add FL-specific sections (points 1–5 above) before any clinical pilot submission to an Ethics Committee.

**Plain-language summary for patients (to be included in consent form):**
> "This study uses a privacy-preserving technique called Federated Learning. Your medical records stay on your hospital's computers. Only a scrambled mathematical summary — not your actual records — is ever shared, and even that summary is protected by encryption and random noise before leaving the hospital. The researchers will only ever see the combined result from all hospitals together, never anything that identifies you individually."

---

## BioSense — MDR 2017/745 Classification (R7 fix)

BioSense collects EEG + PPG + olfactory data to estimate biological age and brain health state. This falls under the EU Medical Device Regulation (MDR 2017/745).

**Preliminary classification: Class IIa** (Rule 10 — active diagnostic devices for physiological parameters).

| Compliance Requirement | MDR Article | Status | Timeline |
|------------------------|-------------|--------|----------|
| Clinical evaluation (PMCF study) | Art. 61 | 🔴 Not started | WP4+ |
| Technical documentation (TD) | Annex II | 🔴 Not started | WP3+ |
| Quality Management System (ISO 13485) | Art. 10(9) | 🔴 Not started | WP3+ |
| CE marking (Notified Body assessment) | Art. 52 | 🔴 Not started | Post-pilot |
| EU Authorized Representative (if non-EU manufacturer) | Art. 11 | 🔴 Not started | Before CE |
| EUDAMED registration | Art. 29 | 🔴 Not started | Before CE |

**Budget implication:** MDR compliance for Class IIa device typically requires €200k–€500k and 2–4 years. This is NOT reflected in the current FCLC grant budget. A separate device certification budget line is required in any grant application mentioning BioSense hardware.

**Safe harbor:** BioSense as a research-use-only (RUO) tool during pilot phase avoids MDR if clearly labeled "For research use only — not for clinical decision making" and not sold/distributed to end users. This label must be enforced in all materials.

---

## Audit Trail

FCLC implements a hash-chain audit log (see `src/db.rs → insert_audit_entry`):

```
entry_hash = SHA-256(round_id ‖ round_number ‖ gradient_hash ‖ prev_hash)
```

This provides:
- **Integrity:** Any tampering with a past round changes all subsequent hashes
- **Non-repudiation:** Each round is cryptographically linked to its predecessor
- **Transparency:** Audit log can be provided to regulators on request

---

## Security Checklist

- [x] DP noise injection (Gaussian mechanism, calibrated to ε, δ)
- [x] Gradient clipping (L2 norm ≤ 1.0) before noise
- [x] k-anonymity enforcement before any data leaves clinic node
- [x] TLS required for all REST API communications (enforced by Axum + rustls)
- [x] Audit log with hash-chain integrity
- [x] SecAgg+: X25519 DH + ChaCha20 PRG (completed 2026-04-09)
- [ ] Independent cryptographic security audit (WP3, months 4–6)
- [ ] Penetration testing of REST API endpoints
- [ ] Rate limiting on `/api/nodes/:id/update` (prevent gradient flooding)
- [ ] Node authentication (currently UUID only — add HMAC or TLS client certs)
- [ ] Database encryption at rest
- [ ] Automated DP budget alerting (warn node when ε_remaining < 2.0)

---

## Right to Explanation — GDPR Art. 22 + EU AI Act Art. 13 (v7 fix, 2026-04-10)

GDPR Art. 22 grants data subjects the right not to be subject to solely automated decisions with significant effects. EU AI Act Art. 13 requires transparency for high-risk AI systems. FCLC must address both.

### Applicability to FCLC

| Scenario | GDPR Art. 22 applies? | AI Act Art. 13 applies? | Action required |
|----------|----------------------|------------------------|-----------------|
| Model predicts readmission risk → shown to clinician → clinician decides | ⚠️ Borderline — human in the loop may exempt | ✅ Yes — high-risk AI system | Explanation interface required |
| Model score used to prioritise patient list automatically | ✅ Yes — automated significant effect | ✅ Yes | Human oversight + explanation mandatory |
| Shapley score used to allocate data contribution rewards | ⚠️ Not significant effect | ❌ No | Best-practice disclosure |

### Explanation Requirements (DP + FL Specific)

The technical challenge: federated learning + differential privacy make individual-level explanations difficult:
- **SHAP/LIME** require local model access — available at node level, NOT at aggregated model level
- **DP noise** distorts feature importances at the individual level
- **SecAgg+** means the server never sees individual updates — explanation must be generated at the node

**Adopted strategy (v7):**
1. **Node-level explanations** (pre-SecAgg+): SHAP values computed locally on the node's training data before DP noise injection; returned alongside the gradient update (optional, privacy-preserving)
2. **Global feature importance** from aggregated model (post-aggregation): computed by the orchestrator on synthetic probe data; available via `GET /api/model/explanations`
3. **Clinician-facing UI**: explanation shown as top-3 contributing features; confidence interval derived from DP noise level (σ); labeled "AI-assisted — not a clinical decision"
4. **Patient-facing explanation** (consent form language): "The system identified patterns in data from multiple hospitals. We cannot tell you exactly which feature drove the result for you individually, because of privacy protection applied to all data."

**Implementation status:**
- [x] SHAP architecture decision documented (node-level)
- [ ] SHAP integration in fclc-node (WP3, months 7–9)
- [ ] `/api/model/explanations` endpoint (WP3)
- [ ] Clinician UI explanation panel (WP3)
- [ ] Legal review: GDPR Art. 22 exemption documented (human oversight, not solely automated)

### Data Processing Agreement (DPA) Checklist — GDPR Art. 28 (v7 fix)

Each clinic–orchestrator relationship requires a signed DPA. This checklist ensures completeness.

**Required clauses per DPA (Art. 28(3)):**

- [ ] Processing purpose: FL model training for [specific clinical prediction task]
- [ ] Data categories: pseudonymized OMOP records (age bin, diagnosis codes, lab values — NO direct identifiers)
- [ ] Data subjects: patients of [Clinic Name], retrospective cohort [date range]
- [ ] Processing location: orchestrator hosted at [cloud provider + region]
- [ ] Sub-processor: [cloud provider] — requires sub-processor clause
- [ ] Retention: aggregated model weights 100 rounds; raw updates NEVER stored (SecAgg+)
- [ ] Security measures: SecAgg+ (X25519/ChaCha20), DP-SGD (ε=2.0/round, δ=1e-5), TLS 1.3, k-anonymity ≥5
- [ ] Audit rights: clinic can request audit log extract at any time
- [ ] Return/deletion: on contract termination, orchestrator deletes all node-derived data within 30 days
- [ ] Assistance: orchestrator assists controller in responding to data subject rights requests
- [ ] FL-specific disclosure: "Gradient updates may contain residual information; SecAgg+ ensures orchestrator only sees aggregate — documented as residual membership inference risk (mitigated)"
- [ ] International transfer clause (if clinic outside EU/EEA): SCCs 2021 attached

**Status:** DPA template at `docs/DUA_template.md` — needs FL-specific clauses above added before clinical pilot.

---

## Algorithmic Impact Assessment (AIA) — Skeleton (v6 fix, 2026-04-10)

An Algorithmic Impact Assessment is required before clinical deployment of any AI/FL model under the EU AI Act (Annex III high-risk systems: medical devices, health management) and is recommended by IEEE 7010 (Wellbeing of People). This is a skeleton — completion requires a GDPR-qualified reviewer and AI ethics board sign-off.

### AIA Status

| Component | Status | Owner | Deadline |
|-----------|--------|-------|---------|
| System description + intended use | 🟡 Draft (this document) | PI | Before clinical pilot |
| Risk classification (EU AI Act) | 🔴 Not started | Legal | Before clinical pilot |
| Bias & fairness assessment | 🔴 Not started | ML lead | WP2 |
| Transparency & explainability plan | 🔴 Not started | PI | WP2 |
| Human oversight mechanism | 🔴 Not started | PI + clinic leads | WP3 |
| Accuracy & robustness requirements | 🟡 Partial (fairness metrics in code) | ML lead | WP2 |
| Post-market monitoring plan | 🔴 Not started | PI | WP4 |

### Risk Classification (EU AI Act, preliminary)

FCLC falls under **Annex III — High-Risk AI Systems**:
- Category: AI systems intended to be used for health management and healthcare decision support
- Classification: **HIGH RISK** (Art. 6(2) + Annex III §5(a))
- Obligations: conformity assessment, technical documentation, human oversight, logging, transparency

**Action required before clinical pilot:** Register in EU AI Act database (Art. 51); complete conformity assessment.

### Fairness & Bias Mitigation

| Subgroup | Monitoring mechanism | Status |
|----------|---------------------|--------|
| Age groups (Under40/40–60/60–80/Over80) | `evaluate_age_group_fairness()` in fclc-core | ✅ Implemented |
| Sex/gender | Planned for WP2 (eICU-CRD dataset) | 🔴 Not started |
| Ethnicity | Planned for WP2 | 🔴 Not started |
| Geography (by clinic node) | Shapley scores proxy | 🟡 Partial |

**Threshold:** Demographic Parity gap < 0.1 (EEOC 4/5ths rule). Equalized Odds gap < 0.1.
**Action on violation:** Round is flagged; orchestrator notifies all nodes; training paused pending review.

### Human Oversight Mechanism

- All model outputs are **decision-support only** — no autonomous clinical decisions
- Clinician override always available; model output labeled "AI-assisted, unvalidated"
- Audit log provides full traceability of each FL round to regulatory bodies
- Emergency stop: orchestrator admin can pause all rounds via `/api/admin/pause` (to be implemented in WP3)

### Transparency & Explainability

- Architecture and DP parameters documented in CONCEPT.md and COMPLIANCE.md (this file)
- Plain-language patient summary provided in consent form (see "Informed Consent" section above)
- SHAP-based local explanations planned for WP3 (requires post-aggregation model access)
- Sex-based and age-based fairness metrics: `evaluate_sex_fairness()` + `evaluate_age_group_fairness()` implemented in fclc-core v9

---

## EU AI Act — Conformity Assessment Checklist (v9 fix, 2026-04-10)

*Required for HIGH RISK classification (Annex III §5(a) — health management & decision support). Checklist based on EU AI Act Title III, Chapter 2 (Arts. 8–15) and Annex IV (Technical Documentation).*

### Art. 9 — Risk Management System

| Requirement | Status | Action Required |
|-------------|--------|-----------------|
| Identify and analyse known/foreseeable risks | 🔴 Not started | Complete AIA §Risk matrix before clinical pilot |
| Implement risk control measures | 🟡 Partial (DP + SecAgg+ + fairness) | Add human override mechanism (WP3) |
| Residual risk evaluation against benefits | 🔴 Not started | Requires medical consultant sign-off |
| Post-market risk monitoring plan | 🔴 Not started | Define monitoring KPIs before WP4 |

### Art. 10 — Data and Data Governance

| Requirement | Status | Action Required |
|-------------|--------|-----------------|
| Training/validation/test data governance procedures | 🟡 Partial (dataset_ethics_catalogue() implemented) | Complete IRB for Cuban EEG + CDATA + clinical pilot |
| Data relevance, representativeness, freedom from errors | 🔴 Not validated | eICU-CRD bias analysis in WP2 |
| Examination for biases (demographic) | 🟡 Partial (FairnessReport implemented) | Age + sex + ethnicity bias study on real data |
| Data protection safeguards | ✅ GDPR-compliant 5-layer stack | Maintain current implementation |

### Art. 11 — Technical Documentation (Annex IV)

| Annex IV Section | Status | Owner |
|------------------|--------|-------|
| A1: System description & intended purpose | 🟡 Draft (CONCEPT.md) | PI |
| A2: Interaction with hardware/other AI | 🔴 Not started | ML lead |
| A3: Instructions for use | 🔴 Not started | PI |
| A4: Technical characteristics (training, architecture) | 🟡 Partial (CONCEPT.md §Architecture) | ML lead |
| A5: General description of capabilities/limitations | 🟡 Partial (CONCEPT.md §Limitations) | PI |
| A6: Risk management documentation | 🔴 Not started | Legal + PI |
| A7: Changes to the system | 🔴 Not started | ML lead (add to changelog) |
| A8: Standards applied | 🔴 Not started | Legal |
| A9: Conformity declaration | 🔴 Not started | PI (after completion) |

### Art. 12 — Record-keeping (Logging)

| Requirement | Status | Notes |
|-------------|--------|-------|
| Automatic logging of FL rounds | ✅ Audit log (`002_audit_log.sql`) | Logs: round_id, node_id, timestamp, ε_consumed |
| Log integrity (tamper-evident) | 🔴 Not started | Add HMAC signatures to audit log entries |
| Log retention (min 10 years for medical) | 🔴 Not started | Define PostgreSQL archive strategy |
| Access control for logs | 🟡 Partial (admin-only API endpoint planned) | Implement `/api/admin/logs` with JWT |

### Art. 13 — Transparency & Provision of Information

| Requirement | Status | Notes |
|-------------|--------|-------|
| Clear identification as AI system to users | 🔴 Not started | Add "AI-assisted, unvalidated" label to all outputs |
| Intended purpose disclosed to deployers | 🟡 Draft (CONCEPT.md §Intended Use) | Finalise IFU document |
| DP parameters disclosed to oversight bodies | ✅ Documented (ε, δ, σ in CONCEPT.md + code) | — |
| Model capabilities and limitations | 🟡 Partial | Add formal limitations section to IFU |

### Art. 14 — Human Oversight

| Requirement | Status | Notes |
|-------------|--------|-------|
| Oversight by qualified persons | 🔴 Not started | Define "qualified clinician" criteria |
| Ability to pause/stop AI system | 🟡 Planned (`/api/admin/pause` WP3) | Not yet implemented |
| Ignore or override AI output | 🟡 Design intent (decision-support only) | Not enforced in UI yet |
| Full understanding of capabilities/limits by overseers | 🔴 Not started | Training programme for clinic users |

### Art. 15 — Accuracy, Robustness, Cybersecurity

| Requirement | Status | Notes |
|-------------|--------|-------|
| Defined accuracy metrics and benchmarks | 🟡 Partial (AUC proxy in tests) | Requires clinical validation (P1-F) |
| Robustness against adversarial inputs | 🟡 Partial (Krum Byzantine filter) | No adversarial robustness test suite |
| Cybersecurity measures | 🟡 Partial (X25519 + ChaCha20) | Independent audit required (P1-B) |
| Fallback behaviour for failure | 🔴 Not started | Define safe-fail mode |

**Overall Art. 15 status: ⚠️ INSUFFICIENT for regulatory submission — minimum: complete P1-B (security audit) + P1-F (clinical validation) before Annex III filing.**

---

## BioSense: RUO → Clinical Research Pathway (v9 fix, 2026-04-10)

*BioSense (EEG+HRV+olfaction hardware) currently operates as Research Use Only (RUO). This section documents the pathway to clinical research qualification.*

### Current Regulatory Status

| Component | Current Classification | Target Classification | Applicable Regulation |
|-----------|----------------------|----------------------|----------------------|
| BioSense hardware (EEG headset) | RUO — no CE mark | In Vitro Diagnostic (IVD) or General Medical Device | EU MDR 2017/745 or EU IVDR 2017/746 |
| χ_Ze algorithm (software) | RUO — not a medical device | Software as Medical Device (SaMD) | EU MDR Art.2(1) + MDCG 2019-11 |
| FCLC federated model | RUO — decision support prototype | Class IIa Medical Device (decision support) | EU MDR Annex VIII Rule 11 |

### IVD vs MDR Determination

The classification depends on primary intended use:

```
IF intended use = "measuring a physiological parameter for diagnostic purposes"
  → EU IVDR 2017/746 (In Vitro Diagnostics Regulation)
  → χ_Ze measuring EEG T/S events → NOT in vitro → IVDR does NOT apply

IF intended use = "health management decision support using processed physiological signals"
  → EU MDR 2017/745 (Medical Device Regulation)
  → BioSense: Class I (low risk, measuring function only) or Class IIa (diagnostic)
  → FCLC: Class IIa (Rule 11 — software for diagnosis/monitoring of chronic conditions)
```

**Preliminary classification: EU MDR 2017/745, Class IIa** (requires Notified Body involvement).

### RUO → Clinical Research Steps

| Step | Action | Regulatory Basis | Timeline |
|------|--------|-----------------|----------|
| R1 | Confirm MDR vs IVDR applicability with Notified Body | MDCG 2019-11 guidance | Before clinical pilot |
| R2 | Prepare technical file per MDR Annex II/III | EU MDR Art. 52 | WP3 |
| R3 | Clinical Evaluation per MEDDEV 2.7/1 rev.4 | EU MDR Art. 61 | WP3–WP4 |
| R4 | BioSense CE marking (Class I self-declaration or IIa Notified Body) | EU MDR Annex IX | WP4 |
| R5 | χ_Ze algorithm SaMD registration | EU MDR + IMDRF SaMD guidance | WP4 |
| R6 | Post-Market Clinical Follow-up (PMCF) plan | EU MDR Annex XIV Part B | Before commercial use |

**Current action required (before first clinical use of BioSense):**
- [ ] Obtain written legal opinion: MDR vs IVDR classification
- [ ] Add "RUO ONLY — Not for clinical diagnostic use" label to all BioSense output interfaces
- [ ] Ensure all studies with BioSense use IRB-approved protocols explicitly designating RUO status

---

## GDPR Chapter V — Cross-Border Data Transfers (v10 fix, 2026-04-10)

*R7 v9: "Transfer of data to Cuban partners is not addressed from a GDPR Chapter V (Transfers to Third Countries) perspective."*

### Data Flow Map

| Data flow | Source | Destination | GDPR Chapter V applies? |
|-----------|--------|-------------|------------------------|
| Gradient updates (aggregated, DP-protected) | Georgian nodes | FCLC orchestrator (Georgia) | No (intra-Georgia; Georgian PDL applies) |
| Cuban EEG dataset (research data) | Cuba (CNEURO) | PI workstation (Georgia) | **⚠️ Yes** — Cuba is not an EU adequacy country |
| χ_Ze outputs (anonymised) | Georgia → EU partner | EU/EEA | No (adequate jurisdiction) |
| FCLC model (aggregated, no PD) | Georgia → LEVF (USA) | USA | **⚠️ Yes** — US has no general adequacy decision |

### Required Safeguards for Third-Country Transfers

| Transfer | Safeguard mechanism | Status |
|----------|---------------------|--------|
| Cuban EEG → Georgia (research) | **Art. 49(1)(d)** — transfer necessary for scientific research purposes + IRB approval at both sites | 🔴 IRB at CNEURO NOT STARTED |
| Georgia → USA (LEVF/ADG) | **Standard Contractual Clauses (SCCs)** per Commission Decision 2021/914 | 🔴 Not drafted |
| Any EU node → non-EU entity | **Art. 46** — SCCs or Binding Corporate Rules | 🔴 Not applicable until EU node exists |

**Action required before Cuban EEG analysis:**
1. Obtain IRB approval at CNEURO (Havana) for data re-use in FCLC context
2. Execute Art. 49(1)(d) transfer documentation with DPA notification
3. Restrict use to aggregated/anonymised gradients only — no raw EEG leaves Cuba

---

## ISO 13485 QMS — Skeleton (v10 fix, 2026-04-10)

*R7 v9: "Developing a medical device without initiating MDR QMS is a severe regulatory breach."*

ISO 13485:2016 (Quality Management System for Medical Devices) is mandatory for MDR Class IIa CE marking of BioSense + χ_Ze SaMD. This skeleton documents what must be established before clinical pilot.

### QMS Elements Status

| ISO 13485 Clause | Requirement | Status | Owner |
|-----------------|-------------|--------|-------|
| 4.1 | QMS scope definition | 🔴 Not started | PI |
| 4.2.3 | Medical device file (technical documentation) | 🔴 Not started | ML lead |
| 5.1 | Management commitment (Quality Policy) | 🔴 Not started | PI |
| 6.2 | Personnel competence records | 🔴 Not started | PI |
| 7.3 | Design and development (D&D plan) | 🟡 Informal (CONCEPT.md) | ML lead |
| 7.3.2 | D&D input requirements | 🟡 Partial (CLAUDE.md constraints) | ML lead |
| 7.3.3 | D&D outputs (code + tests) | 🟡 Partial (fclc-core 90 tests) | ML lead |
| 7.3.5 | D&D verification | 🟡 Partial (cargo test CI) | ML lead |
| 7.3.6 | D&D validation (clinical) | 🔴 Not started | Medical consultant |
| 7.5 | Production and service provision controls | 🔴 Not started | Technical expert |
| 8.2.1 | Customer feedback mechanism | 🔴 Not started | PI |
| 8.5.2 | Corrective action (CAPA) | 🔴 Not started | PI |

**Minimum viable QMS for clinical pilot:**
- [ ] Quality Policy document (1 page, PI signature)
- [ ] Medical Device File scope (2 pages: intended use + safety claims)
- [ ] D&D Plan formalised from CONCEPT.md
- [ ] CAPA procedure (corrective action for adverse events)
- [ ] ISO 13485-registered consultant engaged (estimated cost: €15–25k)

**Timeline:** QMS initiation must precede IRB submission for clinical pilot (P0-C → P1-A).

---

## Responsible Disclosure

Security issues should be reported to the PI (J. Tkemaladze) via private channel.  
No public bug bounty program at this stage (pre-clinical software).

---

*Updated: 2026-04-06*  
*Next review: before first clinical pilot (IRB submission date)*
