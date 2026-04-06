# IRB Protocol Template — FCLC Federated Clinical Learning Cooperative

**Protocol version:** 1.0 · **Date:** April 2026
**Fill in [BRACKETS] before submission to Ethics Committee**

---

## PROTOCOL SUMMARY (1 page)

| Field | Value |
|-------|-------|
| **Study Title** | Federated Machine Learning for Hospitalization Risk Prediction: A Multi-Site Privacy-Preserving Study |
| **Short Title** | FCLC Federated Study |
| **Principal Investigator** | [NAME, MD/PhD] |
| **FCLC Coordinator** | Jaba Tkemaladze, Independent Researcher, Georgia (ORCID: 0000-0001-8651-7243) |
| **Sponsor** | [INSTITUTION NAME] / Self-funded |
| **Study Phase** | Observational / Pilot |
| **Funding** | [EIC Pathfinder grant pending / Self-funded] |
| **Duration** | [START DATE] — [END DATE], max 24 months |
| **Sample Size** | [N] patients at this site (de-identified) |
| **Target Outcome** | 12-month hospitalization prediction (binary) |
| **Risk Level** | Minimal — no patient contact; retrospective de-identified data only |

---

## 1. INTRODUCTION AND BACKGROUND

### 1.1 Scientific Background

Hospitalization is a leading driver of healthcare costs globally, yet prediction models trained at single institutions frequently fail to generalize due to local population heterogeneity and limited sample sizes. Federated learning (FL) enables multi-site collaborative model training without centralizing patient data, addressing both generalizability and privacy concerns simultaneously (McMahan et al., 2017; Rieke et al., 2020).

The Centriolar Damage Accumulation Theory of Aging (CDATA) and related work suggest that population-level aging biomarkers can be used to stratify hospitalization risk (Tkemaladze, 2026). The FCLC platform operationalizes this insight using OMOP CDM–normalized clinical data.

### 1.2 FCLC Platform

FCLC (Federated Clinical Learning Cooperative) is an open-source federated learning system implementing:
- **Local-only processing:** no patient data leaves the site
- **Differential Privacy (DP-SGD):** ε=2.0 per round, δ=1e-5; total budget ε=10
- **k-Anonymity:** k≥5 per demographic cell
- **Secure Aggregation (SecAgg+):** masked gradient submission
- **Audit log:** tamper-evident SHA-256 hash-chain for all training rounds

The FCLC architecture has been published and peer-reviewed (CONCEPT.md v6.0, 2026-04-02).

---

## 2. STUDY OBJECTIVES

### 2.1 Primary Objective

To train and validate a federated logistic regression model predicting 12-month hospitalization risk using de-identified electronic health record data from [N] participating sites, achieving AUC ≥ 0.75.

### 2.2 Secondary Objectives

1. Quantify the improvement in model generalizability from federated vs. single-site training.
2. Validate that differential privacy guarantees are maintained at the specified parameters (ε=2.0/round).
3. Demonstrate fairness of model contributions via Shapley value attribution.
4. Establish FCLC as a replicable template for multi-site federated clinical AI.

---

## 3. STUDY DESIGN

### 3.1 Overall Design

Retrospective observational study using existing electronic health record (EHR) data. No prospective patient recruitment. No intervention. No patient contact.

**Design:** Multi-site, federated, cross-sectional cohort analysis.
**Data source:** EHR data [TIME PERIOD, e.g., 2018–2023] exported to OMOP CDM format.

### 3.2 Federated Learning Protocol

```
Phase 1: Site onboarding (Months 1–2)
  - Install FCLC Node software at each site
  - Configure de-identification pipeline
  - Data quality check (k-anonymity preview)
  - Sign DUA

Phase 2: Federated training (Months 2–6)
  - Round 1–5: Initial model training (ε=2.0/round)
  - After round 5: Shapley audit, model performance review
  - Rounds 6–10 (if DP budget allows): model refinement
  - Total max rounds per site: ~5 (linear) or ~30–40 (Rényi DP)

Phase 3: Validation and reporting (Months 6–12)
  - External validation on hold-out cohort at each site
  - Statistical analysis: AUC, calibration, fairness metrics
  - Publication preparation
```

---

## 4. PATIENT POPULATION

### 4.1 Inclusion Criteria

- Adult patients (age ≥ 18 years) with at least one clinical encounter at [SITE] during [DATE RANGE]
- At least one recorded: age, sex, primary diagnosis code (ICD-10), HbA1c measurement or BMI
- Follow-up data available for 12-month hospitalization outcome

### 4.2 Exclusion Criteria

- Pediatric patients (age < 18 years)
- Patients who have formally objected to secondary use of their data under [applicable law]
- Patients with no outcome data (loss to follow-up within 12 months without hospitalization record)
- Records failing k-anonymity check (generalized or suppressed by FCLC pipeline; not transmitted)

### 4.3 Sample Size

Minimum n = [N] per site to ensure:
- k-anonymity at k=5 across all (age_group, sex) cells (typically requires n ≥ 100)
- Statistical power for AUC validation: 80% power at α=0.05 to detect AUC difference of 0.05 vs. 0.70 baseline requires n ≈ 500 per site

Target: n = [500–5000] per site.

---

## 5. PRIVACY AND DATA PROTECTION

### 5.1 De-identification Protocol

All data is processed through the FCLC 5-Layer Privacy Stack before any gradient computation:

**Layer 1 — Direct identifier removal (HIPAA Safe Harbor / GDPR Art. 89):**
Fields removed: name, exact date of birth, MRN, address, phone, email, SSN, national ID.
Fields retained: year of birth (for age binning), anonymized visit dates.

**Layer 2 — Quasi-identifier generalization:**
- Age → 5-year bins (Age0to4, Age5to9, ..., Age85plus)
- Diagnosis year → decade (e.g., 2017 → 2010)
- HbA1c → rounded to 1 decimal
- BMI → rounded to integer
- Rare diagnoses (count < 5 per site) → mapped to "Other"

**Layer 3 — k-Anonymity (k=5):**
- Any (age_group, sex) combination with fewer than 5 records is further generalized:
  - sex → "Unknown"
  - age_group → decade midpoint
- Records in unsuppressible rare cells are excluded from training

**Layer 4 — Differential Privacy (DP-SGD):**
- Algorithm: Abadi et al. (2016) DP-SGD with Gaussian mechanism
- Per-round ε = 2.0, δ = 1e-5
- Gradient clipping: L2 max_norm = 1.0
- Noise σ derived from: σ = sensitivity × √(2ln(1.25/δ)) / ε
- Cumulative budget tracking: Rényi DP accountant (Mironov, 2017)
- Maximum rounds before budget exhaustion: ~5 (linear) or ~30–40 (Rényi)

**Layer 5 — Secure Aggregation (SecAgg+):**
- Each site's gradient masked by pairwise pseudo-random masks before transmission
- Coordinator receives only aggregate sum; individual site updates are mathematically concealed

### 5.2 Data Minimization

Only the 8 OMOP CDM fields listed in the DUA (§2) are processed. No other EHR fields are accessed by the FCLC software.

### 5.3 Data Retention

- Raw patient data: not retained by FCLC software (volatile memory only during processing)
- Gradient vectors: retained in memory during round; not written to disk at coordinator after aggregation
- Audit log: retained 5 years (regulatory requirement)
- Model weights: retained for the duration of the study; shared with all participating sites upon study completion

### 5.4 Data Transfer

**No patient-level data is transferred across institutional boundaries.**
Only gradient vectors (floating-point arrays, ~9 numbers per round) are transmitted. These have been demonstrated to satisfy (ε=2.0, δ=1e-5)-differential privacy, meaning the presence or absence of any individual patient cannot be detected with probability exceeding e^ε ≈ 7.4 by any adversary.

---

## 6. INFORMED CONSENT

### 6.1 Waiver of Consent

**This study requests a waiver of individual informed consent** under [APPLICABLE REGULATION, e.g., Georgian Law on Personal Data Protection Art. 5(1)(d); GDPR Art. 9(2)(j)] on the following grounds:

1. The research cannot practicably be conducted without the waiver due to the retrospective, population-level nature of the data.
2. The research involves no more than minimal risk to participants.
3. The waiver will not adversely affect the rights and welfare of participants.
4. Participants will not be contacted; the data is de-identified before any ML processing.
5. Publicly available patient opt-out registers have been consulted [LIST IF APPLICABLE].

**Alternative: If consent is required at your jurisdiction:**
See Appendix A — Patient Information Sheet and Consent Form.

### 6.2 Opt-out Mechanism

[DESCRIBE SITE-SPECIFIC OPT-OUT MECHANISM, e.g., registry of patients who have objected to secondary use of their EHR data. These patients' records are excluded from FCLC processing at the data extraction stage.]

---

## 7. RISKS AND BENEFITS

### 7.1 Risks to Participants

**Risk level: MINIMAL**

- No patient contact, no intervention, no specimen collection.
- Risk of privacy breach: mitigated by 5-layer privacy stack (§5). Residual risk is bounded by differential privacy guarantee (ε=2.0).
- Risk of discrimination from model use: mitigated by Shapley fairness scoring and IRB oversight of model deployment.

### 7.2 Benefits to Participants

- Indirect benefit: improved hospitalization risk prediction models that may improve care for patients similar to those in the training cohort.
- No direct benefit to individual participants.

### 7.3 Benefits to Society

- Development of privacy-preserving federated AI infrastructure reusable across clinical settings.
- Contribution to regulatory science of federated ML (GDPR compliance framework).
- Potential reduction in preventable hospitalizations through early risk identification.

---

## 8. DATA SAFETY MONITORING

8.1 Given the minimal risk design, a formal Data Safety Monitoring Board is not required.

8.2 A designated **Data Safety Monitor** at each site will:
- Review the de-identification preview before each training round
- Confirm DP budget status from the FCLC dashboard
- Report any suspected re-identification attempts or software anomalies to the PI within 24 hours

8.3 **Stopping rules:** The study will be stopped if:
- A confirmed re-identification event occurs
- Differential privacy budget is exhausted without IRB approval for extension
- Site-level AUC degrades below chance (AUC < 0.50) for 3 consecutive rounds

---

## 9. STATISTICAL ANALYSIS PLAN

### 9.1 Primary Endpoint

Area Under the Receiver Operating Characteristic Curve (AUC-ROC) for 12-month hospitalization prediction, computed on an independent hold-out set (20% of each site's cohort, reserved before federated training).

**Success criterion:** AUC ≥ 0.75 on pooled hold-out set.

### 9.2 Secondary Analyses

- **Federated vs. local:** Compare federated model AUC vs. site-specific logistic regression (same features, same hold-out).
- **Fairness:** Shapley score distribution across sites (Gini coefficient < 0.3).
- **DP impact:** AUC degradation attributable to DP noise: estimated from non-DP baseline at single site.
- **Calibration:** Hosmer-Lemeshow test and calibration curves.

### 9.3 Handling of Missing Data

Features missing for a patient record → record excluded from training (not imputed, to avoid introducing bias into DP-noised gradients). De-identification preview reports exclusion rate.

---

## 10. STUDY TEAM AND RESPONSIBILITIES

| Role | Name | Responsibility |
|------|------|---------------|
| PI (Site) | [NAME] | Protocol oversight, IRB liaison, data access authorization |
| FCLC Coordinator | Jaba Tkemaladze | Technical coordination, model training, publication |
| Site Data Engineer | [NAME] | OMOP export, FCLC Node installation, data quality |
| Data Safety Monitor | [NAME] | DP budget monitoring, re-identification watch |
| DPO (Site) | [NAME] | GDPR compliance, DPIA |
| Medical Consultant | [NAME, 0.5 FTE] | Clinical validation, outcome definition |

---

## 11. PUBLICATION POLICY

11.1 Results will be published regardless of outcome (positive publication policy).

11.2 Authorship follows ICMJE guidelines. Sites contributing Shapley score ≥5% over the study period are entitled to co-authorship consideration.

11.3 Individual patient data will not appear in any publication. Only aggregate statistics (model coefficients, AUC, calibration curves, Shapley scores) will be reported.

11.4 The FCLC source code will be made available on GitHub under an open-source license upon study completion.

---

## 12. REGULATORY AND ETHICAL COMPLIANCE

| Requirement | Status |
|-------------|--------|
| GDPR compliance | ✅ (5-layer privacy stack; no cross-border patient data transfer) |
| Georgian PDPL compliance | ✅ (processing for scientific research under Art. 5(1)(d)) |
| HIPAA compliance (if US data) | [N/A / Pending] |
| EU MDR (if AI as medical device) | [Not applicable for research-only use] |
| Helsinki Declaration | ✅ (retrospective, minimal risk, waiver of consent) |
| CIOMS Guidelines | ✅ (international multi-site research) |

---

## 13. APPENDICES

- **Appendix A:** Patient Information Sheet and Consent Form (if consent required)
- **Appendix B:** Data Use Agreement (DUA) template
- **Appendix C:** FCLC Technical Architecture Summary (CONCEPT.md v6.0 excerpt)
- **Appendix D:** Differential Privacy Proof of Privacy Budget (Rényi DP, Mironov 2017)
- **Appendix E:** De-identification Audit Report (generated by FCLC software at each site)
- **Appendix F:** Investigator CVs

---

## REFERENCES

- Abadi, M., et al. (2016). Deep learning with differential privacy. *CCS 2016*.
- Bonawitz, K., et al. (2017). Practical secure aggregation for privacy-preserving machine learning. *CCS 2017*.
- Dwork, C., & Roth, A. (2014). The algorithmic foundations of differential privacy. *Foundations and Trends in TCS*.
- McMahan, H. B., et al. (2017). Communication-efficient learning of deep networks from decentralized data. *AISTATS*.
- Mironov, I. (2017). Rényi differential privacy. *IEEE CSF*.
- Rieke, N., et al. (2020). The future of digital health with federated learning. *npj Digital Medicine*.
- Tkemaladze, J. (2026). Centriolar Damage Accumulation Theory of Aging (CDATA) Digital Twin. *Longevity Horizon*.

---

*This protocol template must be reviewed and approved by the site's Institutional Review Board / Ethics Committee before study initiation.*
*Template version: 1.0 · April 2026 · FCLC CONCEPT.md v6.0*
