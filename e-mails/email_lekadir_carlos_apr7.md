# Email: Meeting Confirmation + Concept Note
**To:** Professor Karim Lekadir, Carlos (fedder.ai)  
**From:** Dr. Jaba Tkemaladze  
**Date:** April 3, 2026  
**Subject:** FedClinAI — Zoom Call Confirmed April 7 + Full Concept Note v6.0

---

Dear Professor Lekadir and Carlos,

Thank you again for your positive response and willingness to discuss the FedClinAI project.

We are delighted to propose a video call for **Tuesday, April 7, at 2:00 PM Barcelona time (4:00 PM Tbilisi time)**.

**Topic:** Federated Clinical Learning Cooperative  
**Time:** Apr 7, 2026 — 04:00 PM Tbilisi / 02:00 PM Barcelona  

**Join Zoom Meeting:**  
https://us04web.zoom.us/j/76842989164?pwd=cy5TXogseaq7fAtZJAQ8nCJbu4u5PG.1  
Meeting ID: 768 4298 9164  
Passcode: eNZe38

**Agenda (30 minutes):**
1. Brief overview of FedClinAI (5 min) — problem, solution, current consortium
2. Technical architecture (10 min) — FedProx with Krum, differential privacy (ε=2.0/round), Shapley value contribution scoring
3. Role for BCN-AIM Lab / fedder.ai (10 min) — EU Co-Principal Investigator, Technical Lead for FL Architecture, validation against EHDS standards
4. Next steps for EIC Pathfinder submission (5 min) — what we need for the May 12 deadline

Please let me know if this time works for you, or if you would prefer an alternative.

As promised, I am attaching the **full Concept Note v6.0** below. This is our final, submission-ready document.

Looking forward to speaking with you.

Best regards,

**Dr. Jaba Tkemaladze, MD**  
Principal Investigator, FedClinAI Project  
Email: djabbat@gmail.com

**Giorgi Tsomaia**  
Co-Investigator, WP2 & WP4 Lead  
Email: gakelytemp@gmail.com

---

---

# FedClinAI — FULL CONCEPT NOTE v6.0

**Federated Clinical Consortium (FCLC)**  
**A Private Infrastructure for Training Medical AI Without Transferring Raw Patient Data**

**Version 6.0 — Final | Date: April 2, 2026 | Status: READY FOR SUBMISSION**

---

## Executive Summary

This concept note proposes the establishment of a **Federated Clinical Consortium (FCLC)**, a non-profit partnership creating a secure, privacy-by-design infrastructure for training medical AI models. The core innovation lies in enabling multi-institutional collaboration **without the need to pool or transfer raw, identifiable patient data**.

**The Problem:** Vast amounts of clinical data exist across hospitals and pharmaceutical companies but remain siloed due to heterogeneous formats (HIS, EHR, PACS), stringent legal restrictions (GDPR, national laws), and legitimate commercial/reputational concerns. This impedes the development of robust, generalizable AI tools that could improve patient care.

**The Solution:** Each participant hosts a **local node** that: (1) connects to internal systems via HL7/FHIR adapters, (2) anonymizes data using a multi-layered privacy stack (identifier removal, k-anonymity, differential privacy with ε=2.0/round, δ=10⁻⁵, total budget ε_total≤10.0), (3) normalizes to the OMOP Common Data Model, and (4) shares only encrypted model updates. A central orchestrator aggregates these updates using **SecAgg+** and robust algorithms (**FedProx + Krum** for Byzantine tolerance up to 25% malicious nodes). **Data never leaves the hospital firewall — only training signals do.**

**Governance & Incentives:** A two-tier governance model accommodates both clinical and industrial partners. A theoretically sound **Federated Shapley Value** mechanism measures each participant's marginal contribution to model performance. Contributions are transparently scored and converted into benefits (model access, revenue share). A free-rider prevention mechanism denies access to participants whose contribution falls below 5% of the average.

**Proposed Role for BCN-AIM Lab / fedder.ai:** We propose that **BCN-AIM Lab (University of Barcelona)** joins as the **EU Co-Principal Investigator and Technical Lead for the FL Architecture**, with a key role in validating the approach against European Health Data Space (EHDS) standards. **fedder.ai** is envisioned as a potential **EU technical partner**, contributing expertise in scalable federated learning deployment.

**MVP & Timeline:** A 12-month pilot predicting hospitalization risk for Type 2 Diabetes patients across 3–5 Georgian clinics. Target: AUC >0.75, zero raw data export (audited). Total budget: **€2.275M** (EIC Pathfinder ceiling: €4M).

**EIC Pathfinder Deadline: May 12, 2026.**

---

## One-Sentence Pitch

We unite medical data from clinics and pharmaceutical companies to train AI without transferring raw patient data, with measurable contributions from each participant and transparent benefit sharing.

---

## The Problem

Clinical data exists everywhere, but it is:
- **In different formats** — HIS, EHR, PACS, LIS, unstructured notes
- **Under different legal constraints** — GDPR, national laws, internal policies
- **Nobody wants to share it** — reputational, legal, and commercial risk

**Result:** Large medical AI models are trained on narrow datasets, while real-world clinical data remains untapped. Clinicians continue without AI tools that could be built from existing data.

---

## The Solution

Each participant deploys a **local node** that:
1. **Connects** to HIS/EHR/PACS via HL7/FHIR adapters
2. **Anonymizes** with guarantees (identifier removal, quasi-identifier generalization, differential privacy)
3. **Normalizes** to OMOP Common Data Model
4. **Sends** only anonymized model updates (gradients, weights)

**Central Orchestrator:** collects updates, aggregates (federated averaging + SecAgg+), maintains audit logs, calculates Shapley contribution scores.

---

## Privacy — By Architecture, Not by Promise

| Level | Mechanism |
|-------|-----------|
| 1 | Removal of direct identifiers (name, ID, address, exact date) |
| 2 | Quasi-identifier generalization (age groups, rare diagnoses → suppression) |
| 3 | Record-level re-identification risk (k-anonymity, l-diversity) |
| 4 | **Differential Privacy: ε=2.0/round, δ=10⁻⁵, ε_total≤10.0, Gaussian mechanism, Rényi DP accounting** |
| 5 | Secure Aggregation (SecAgg+): orchestrator sees only aggregated result |

**Result:** Even if the orchestrator is compromised or traffic intercepted, patient data cannot be reconstructed.

**Threat Model:**
- Orchestrator: honest-but-curious
- Up to 25% Byzantine (malicious) nodes — handled by Krum robust aggregation
- External adversary: TLS 1.3 + disk encryption + HSM

---

## Contribution Measurement — Federated Shapley Value

Contribution assessed via **approximated Federated Shapley Value** (Wang et al. 2020):
- Monte Carlo approximation (100–200 iterations)
- Computational overhead: ~10 min/round for 10 nodes [O(n² × M)]
- Theoretically sound: satisfies efficiency, symmetry, linearity, null-player axioms (Shapley 1953)
- Manipulation-resistant: marginal contribution is hard to artificially inflate
- Free-rider protection: access denied if contribution < 5% of average

---

## MVP

**Use Case:** Predicting 12-month hospitalization risk for Type 2 Diabetes patients.

**Why this use case:**
- T2DM is prevalent; data available in most clinics
- Outcome (hospitalization) clearly defined and recorded
- High clinical value — reduced costs, improved outpatient management

**Pilot Sites (3–5 Georgian clinics):**
- Aversi Clinic (negotiations started)
- GeoHospitals (preliminary agreement)
- Iashvili Children's Hospital, diabetology unit (negotiations started)

**Minimal Dataset:**
Age (5-yr bins), sex, T2DM diagnosis year, HbA1c (last), BMI, complications (binary flags), hospitalization in past 12 months, hospitalization in next 12 months (target variable).

**Success Criteria:**
- Model AUC > 0.75 on held-out validation set
- Zero raw patient records left the institution (audited)
- Shapley scoring functional and accepted by participants
- Technical interoperability across all 3–5 clinics
- All DUA/IRB requirements met

**Timeline:** 3 months legal prep → 3 months technical development → 6 months deployment + pilot = **12 months total**.

---

## Technical Stack

| Component | Choice | Justification |
|-----------|--------|---------------|
| FL Framework | Flower + OpenFL | Flexibility + medical application maturity |
| **Aggregation** | **FedProx** (μ=0.1–1.0) | Resilience to non-IID heterogeneous clinical data |
| **Robust Aggregation** | **Krum** | Byzantine tolerance ≤25% malicious nodes |
| Secure Aggregation | **SecAgg+** | Orchestrator never sees individual updates |
| Differential Privacy | TF Privacy / Opacus + Rényi DP | ε=2.0/round, δ=10⁻⁵, ε_total≤10.0 |
| Contribution Scoring | Federated Shapley Value (MC, M=100–200) | Theoretically sound, ~10 min/round |
| OMOP Normalization | OHDSI WhiteRabbit + custom ETL | Proven toolkit |
| Interoperability | HL7/FHIR → OMOP | FHIR for exchange, OMOP for analysis |
| Storage | PostgreSQL + OMOP CDM | Open source |
| Security | TLS 1.3, disk encryption, HSM | Medical data compliance |

**Minimum Node Hardware:** 8-core CPU, 32 GB RAM, NVIDIA T4 GPU (16 GB VRAM), 1 TB SSD, 100 Mbps static IP, TPM 2.0 / HSM.

---

## Governance Model

**Legal structure:** Non-profit partnership (Georgia or EU state). Participants sign a Consortium Agreement.

**Two-tier model:**

| Tier | Participants | Rights | Obligations |
|------|-------------|--------|-------------|
| Clinical | Hospitals, diagnostic centers | Full vote on clinical matters; model access | Anonymized clinical data; ethical compliance |
| Industrial | Pharma, CRO | Observer on clinical; full vote on commercial | Trial data; financial support (Phase 2–3) |

**Multi-Stakeholder Board:** clinics (3–5), pharma (2–3), patient advocates (1–2, non-voting), data protection lawyers, ethicists, technical auditors.

**External Clinical Advisory Board:** 2–3 independent clinicians, meetings twice a year (budget: €20,000).

**Six Governing Principles:**
1. Raw data never leaves the clinic — architectural requirement, auditable
2. Every outgoing contribution verified for privacy
3. Every contribution weighted by utility (Shapley value)
4. Every participant receives transparent contribution scoring
5. Every model audited for bias before release
6. Commercial benefits distributed transparently by formula

---

## Legal Basis

- **Data Use Agreement (DUA):** Template prohibiting raw data transfer. Negotiations start 3 months before technical work.
- **IRB / Ethics Approval:** Each institution obtains local ethics committee approval. Supported by project Medical Consultant.
- **Data Steward:** Appointed at each organization.
- **GDPR:** Based on patient consent or national secondary use law (Art. 9(2)(i),(j)); DPIA required per EU partner.
- **Georgian PDPL (2011, amended 2023):** Harmonized with GDPR.

---

## IP & Financial Model

**IP:**
- Base model: Consortium joint property, licensed to members at preferential rates
- Fine-tunings: Exclusive property of the fine-tuning participant
- Methodology / publications: Open Access with consortium attribution

**Financial Phases:**
- **Phase 1 — Pilot (12 months):** Grant-funded (Horizon Europe / EIC Pathfinder). No fees.
- **Phase 2 — Scaling:** Entry fees for new members; 70% revenue → Shapley-proportional credits, 30% → platform development.
- **Phase 3 — Sustainability:** Annual fees (Clinics: from €5k; Pharma: from €50k); licensing as main income.

---

## Consortium Structure (Proposed)

| Partner | Country | Role | Status |
|---------|---------|------|--------|
| Phasis Academy (J. Tkemaladze) | Georgia | PI, Coordinator | ✅ Confirmed |
| Giorgi Tsomaia | Georgia | Co-I, WP2 & WP4 Lead | ✅ Confirmed |
| Medical Consultant | Georgia/EU | Clinical validation, IRB | Vacancy (in budget) |
| Technical Expert (DB Systems) | Georgia/EU | ETL, OMOP, HIS integration | Vacancy (in budget) |
| Aversi Clinic | Georgia | Pilot site | Negotiations started |
| GeoHospitals | Georgia | Pilot site | Preliminary agreement |
| Iashvili Children's Hospital | Georgia | Pilot site | Negotiations started |
| **BCN-AIM Lab (Univ. Barcelona)** | **Spain (EU)** | **EU Co-PI, FL Architecture Lead, EHDS Validation** | **To be invited** |
| **fedder.ai** | **EU** | **EU Technical Partner — scalable FL deployment** | **To be invited** |
| Pharma Partner | EU/Georgia | Validation, commercialization | Search active |

---

## Budget Estimate

| Category | € |
|----------|---|
| Legal preparation (DUA, IRB, memo, Data Stewards) | 55,000 |
| Medical Consultant (0.5 FTE × 12 months) | 60,000 |
| Technical Expert — Database Systems (1.0 FTE × 12 months) | 60,000 |
| External Clinical Advisory Board | 20,000 |
| Local node development (3–4 FTE × 12 months) | 280,000 |
| Normalization & ETL (2 FTE × 12 months) | 120,000 |
| Shapley scoring & aggregation (2 FTE × 24 months) | 250,000 |
| Federated learning & security (2 FTE × 24 months) | 250,000 |
| Deployment in pilot clinics | 100,000 |
| Coordination & management (2 FTE × 36 months) | 320,000 |
| WP4 — Governance | 220,000 |
| Dissemination, communication, events | 130,000 |
| Equipment & infrastructure | 120,000 |
| Travel | 100,000 |
| Contingency (10%) | 210,000 |
| **TOTAL** | **2,275,000** |

*EIC Pathfinder ceiling: €4M. Budget can be expanded to €3–3.5M with consortium growth.*

---

## Next Steps

| Timeline | Task |
|----------|------|
| Weeks 1–2 | Start Medical Consultant & Technical Expert search; begin DUA/IRB for pilot clinics |
| Weeks 2–3 | Legal memo on PDPL/GDPR; **formally invite BCN-AIM Lab and fedder.ai** |
| Weeks 3–4 | Letters of support from clinics |
| Weeks 4–5 | Prepare grant application (Part A & B) |
| **May 12, 2026** | **Submit to EIC Pathfinder Open** |

---

*Concept Note v6.0 — Final. Full peer-review cycle completed. Ready for EIC Pathfinder Open submission. April 2026.*

*Dr. Jaba Tkemaladze, MD — djabbat@gmail.com*
