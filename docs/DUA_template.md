# Data Use Agreement (DUA) — FCLC Federated Clinical Learning Cooperative

**Version:** 1.0 · **Date:** April 2026
**Classification:** Legal Template — Fill in [BRACKETS] before use

---

## PARTIES

**Data Provider ("Clinic"):**
Name: [FULL LEGAL NAME OF INSTITUTION]
Address: [STREET, CITY, COUNTRY]
Registration number: [INSTITUTIONAL REGISTRATION OR VAT ID]
Principal Investigator: [NAME, MD/PhD, TITLE]
Contact: [EMAIL] · [PHONE]

**Data Recipient ("Coordinator"):**
Name: [Jaba Tkemaladze / FCLC Research Group]
Address: [ADDRESS, TBILISI, GEORGIA]
ORCID: 0000-0001-8651-7243
Contact: [EMAIL]

---

## 1. PURPOSE

1.1 The Clinic agrees to allow its anonymized, de-identified clinical data to be processed locally as part of the **Federated Clinical Learning Cooperative (FCLC)**, a federated machine learning platform for clinical AI research.

1.2 **No patient-level data leaves the Clinic's premises.** Only privacy-protected gradient updates (mathematical vectors) are transmitted to the FCLC central coordinator. These vectors contain no patient-identifiable information and cannot be reversed-engineered to reconstruct individual records (see §4 Technical Safeguards).

1.3 The purpose of data use is limited to:
- Training and validation of federated AI models for hospitalization risk prediction
- Computation of fairness scores (Shapley values) per contributing institution
- Aggregate statistical analysis and publication of results

---

## 2. PERMITTED DATA TYPES

2.1 The following OMOP CDM–compatible data fields may be processed **locally at the Clinic node**:

| Field | OMOP Concept Domain | Retention after processing |
|-------|---------------------|---------------------------|
| Age (5-year bin) | Person | None — gradient only |
| Sex (generalized) | Person | None — gradient only |
| Primary diagnosis year (decade) | Condition | None — gradient only |
| HbA1c last value (1 decimal) | Measurement | None — gradient only |
| BMI (integer) | Measurement | None — gradient only |
| Nephropathy indicator | Condition | None — gradient only |
| Retinopathy indicator | Condition | None — gradient only |
| Hospitalization flag (12-month) | Visit | None — gradient only |

2.2 **Prohibited data:** Full name, exact date of birth, medical record number (MRN), address, phone, email, national ID, insurance ID, or any other direct identifier. The FCLC de-identification pipeline (Layer 1) removes all such fields before any computation.

---

## 3. DE-IDENTIFICATION AND ANONYMIZATION PROTOCOL

3.1 Before any machine learning processing, all records are processed through the **FCLC 5-Layer Privacy Stack**:

| Layer | Mechanism | Standard |
|-------|-----------|---------|
| 1 | Direct identifier removal (name, MRN, DOB, address) | HIPAA Safe Harbor |
| 2 | Quasi-identifier generalization (age bins, decade dates) | GDPR Art. 89 |
| 3 | k-Anonymity enforcement (k≥5 per age/sex cell) | ISO 29101:2018 |
| 4 | Differential Privacy noise injection (ε=2.0/round, δ=1e-5) | Dwork & Roth (2014) |
| 5 | Secure Aggregation masking (pairwise gradient masks) | Bonawitz et al. (2017) |

3.2 The Clinic's local FCLC Node software displays a **de-identification preview** before any data is processed. The Clinic's designated data controller must review and approve this preview.

3.3 The DP budget per Clinic is capped at ε_total = 10.0. After exhaustion, the Clinic's node is automatically excluded from further rounds until budget renewal (pending IRB and DPA approval for new cohort).

---

## 4. TECHNICAL SAFEGUARDS

4.1 **Federated architecture:** No raw or de-identified patient records are transmitted. Only gradient vectors (lists of floating-point numbers) are sent from the Clinic to the FCLC coordinator.

4.2 **Differential Privacy (DP-SGD):** Each gradient vector is perturbed by calibrated Gaussian noise (ε=2.0, δ=1e-5 per round) before transmission. The noise level is set so that the presence or absence of any individual patient cannot be statistically detected by an adversary with probability greater than e^ε ≈ 7.4 (standard DP guarantee).

4.3 **Secure Aggregation:** Gradient vectors are masked with cryptographic pseudo-random masks before transmission. The coordinator receives only the aggregate sum; individual Clinic contributions are invisible.

4.4 **Byzantine robustness:** The FCLC server uses Krum-based outlier rejection (25% Byzantine tolerance) to prevent corrupted or adversarial updates from affecting the global model.

4.5 **Audit log:** All federated rounds are logged in a tamper-evident hash-chain audit log (SHA-256, append-only). The Clinic may request audit log excerpts for regulatory review.

4.6 **Data deletion:** The FCLC Node software does not store raw patient data beyond the local processing session. No patient data is written to disk by the FCLC software; all intermediate data exists only in volatile memory during processing.

---

## 5. DATA GOVERNANCE

5.1 **Legal basis for processing (GDPR Art. 6):** [SELECT ONE]
- [ ] Art. 6(1)(e) — Public interest / scientific research (preferred)
- [ ] Art. 6(1)(a) — Patient consent (for identifiable data, if used)
- [ ] Art. 6(1)(c) — Legal obligation

5.2 **Special category data (GDPR Art. 9):** Health data is processed under:
- [ ] Art. 9(2)(j) — Scientific research with appropriate safeguards
- [ ] Art. 9(2)(h) — Healthcare provision purposes

5.3 **Data Protection Officer:** [NAME, EMAIL of Clinic DPO]

5.4 **Data flows:** No cross-border data transfer of patient data occurs. The gradient vectors transmitted are not personal data within the meaning of GDPR (post-anonymization, post-DP).

5.5 **DPIA (Data Protection Impact Assessment):** A DPIA is [REQUIRED / NOT REQUIRED] under GDPR Art. 35 for this Clinic's participation. [If required: DPIA reference number: ______]

---

## 6. INTELLECTUAL PROPERTY

6.1 The global federated model produced by FCLC belongs to all contributing Clinics jointly, weighted by their Shapley contribution score.

6.2 Publications arising from FCLC research will acknowledge all participating institutions. Clinics contributing ≥5% Shapley score over the study period are entitled to co-authorship consideration per ICMJE criteria.

6.3 Clinic-specific data, local model weights, and local training statistics remain the property of the Clinic.

---

## 7. DURATION AND TERMINATION

7.1 This Agreement is effective from [START DATE] to [END DATE] (maximum 24 months; renewable).

7.2 Either party may terminate with 30 days written notice.

7.3 Upon termination, the Clinic's node is deregistered. No residual patient data exists at the coordinator. The global model retains aggregate contributions but cannot be disentangled by Clinic of origin.

7.4 Notwithstanding termination, audit log entries for completed rounds are retained for 5 years per regulatory requirement.

---

## 8. LIABILITY AND INDEMNIFICATION

8.1 The Coordinator is not liable for any privacy breach resulting from the Clinic's failure to correctly apply the de-identification pipeline or failure to restrict physical access to the FCLC Node software.

8.2 Each party is responsible for compliance with applicable data protection law in its jurisdiction.

8.3 The Clinic indemnifies the Coordinator against claims arising from use of data that the Clinic did not have authority to process.

---

## 9. GOVERNING LAW

This Agreement is governed by [Georgian law / EU law / local jurisdiction law as appropriate].
Disputes shall be resolved by [ARBITRATION INSTITUTION / COURT].

---

## SIGNATURES

**On behalf of the Clinic:**

| | |
|---|---|
| Name: | _________________________ |
| Title: | _________________________ |
| Date: | _________________________ |
| Signature: | _________________________ |
| Institutional Stamp: | _________________________ |

**On behalf of FCLC Coordinator:**

| | |
|---|---|
| Name: | Jaba Tkemaladze |
| Date: | _________________________ |
| Signature: | _________________________ |

---

*This template must be reviewed by institutional legal counsel before use.*
*Reference: GDPR (EU) 2016/679 · Georgian PDPL (2023) · HIPAA (if US data) · CONCEPT.md §Legal*
