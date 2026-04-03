# FedClinAI — Federated Clinical Learning Cooperative
## Concept Note v6.0 (English)

**Author:** Jaba Tkemaladze, MD — Principal Investigator
**Date:** April 2, 2026

---

**Federated Clinical Consortium (FCLC)**
**A Private Infrastructure for Training Medical AI Without Transferring Raw Patient Data**

**Version 6.0 — Final**
**Date: April 2, 2026**
**Status: READY FOR SUBMISSION**

---

### **Executive Summary**

This concept note proposes the establishment of a **Federated Clinical Consortium (FCLC)**, a non-profit partnership creating a secure, privacy-by-design infrastructure for training medical AI models. The core innovation lies in enabling multi-institutional collaboration **without the need to pool or transfer raw, identifiable patient data**.

**The Problem:** Vast amounts of clinical data exist across hospitals and pharmaceutical companies but remain siloed due to heterogeneous formats (HIS, EHR, PACS), stringent legal restrictions (GDPR, national laws), and legitimate commercial/reputational concerns. This impedes the development of robust, generalizable AI tools that could improve patient care.

**The Solution:** Each participant hosts a **local node** that: 1) connects to internal systems via HL7/FHIR adapters, 2) anonymizes data using a multi-layered privacy stack (identifier removal, k-anonymity, differential privacy with ε=2.0/round, δ=10⁻⁵, total budget ε_total≤10.0), 3) normalizes it to the OMOP Common Data Model, and 4) shares only encrypted model updates (gradients/weights). A central orchestrator aggregates these updates using privacy-preserving techniques like **Secure Aggregation (SecAgg+)** and robust algorithms (**FedProx, Krum** for Byzantine tolerance up to 25% malicious nodes). Data never leaves the hospital firewall—only training signals do.

**Governance & Incentives:** A two-tier governance model accommodates both clinical and industrial partners. A theoretically sound **Federated Shapley Value** mechanism (Monte Carlo approximated, ~10 min/round overhead for 10 nodes) measures each participant's marginal contribution to model performance. Contributions are transparently scored and converted into benefits (model access, revenue share). A free-rider prevention mechanism denies access to participants whose contribution falls below a set threshold (e.g., 5% of average).

**Proposed Collaboration:** We seek to formalize a consortium for an **EIC Pathfinder Open** application (deadline: May 12, 2026). We propose that **BCN-AIM Lab (University of Barcelona)** joins as the **EU Co-Principal Investigator and Technical Lead for the FL Architecture**, with a key role in validating the approach against emerging European Health Data Space (EHDS) standards. Furthermore, we envision **fedder.ai** as a potential **EU technical partner**, contributing expertise in scalable federated learning deployment.

**MVP & Timeline:** A 12-month pilot will predict 12-month hospitalization risk for Type 2 Diabetes patients across 3-5 Georgian clinics. Success is defined by model performance (AUC >0.75), verified adherence to the "no raw data export" principle, and a functional contribution scoring system. The total budget estimate is €2.275M, expandable within the EIC Pathfinder ceiling of €4M.

---

## **Full Concept Note**

### **One-Sentence Pitch**

We unite medical data from clinics and pharmaceutical companies to train AI without transferring raw patient data, with measurable contributions from each participant and transparent benefit sharing.

---

### **The Problem**

Data exists everywhere, but it is:
*   **In different formats** — HIS, EHR, PACS, LIS, unstructured notes.
*   **Under different legal constraints** — GDPR, national laws, internal policies.
*   **Nobody wants to give it away** — due to reputational, legal, and commercial risk.

**Result:** Large medical AI models are trained on narrow datasets, while real-world clinical data remains untapped. Clinicians continue to work without AI tools that could be built from existing data.

---

### **The Solution**

Each participant deploys a **local node** that:
1.  **Connects** to HIS/EHR/PACS via adapters (HL7/FHIR, proprietary APIs).
2.  **Anonymizes data** with guarantees (identifier removal, quasi-identifier generalization, differential privacy).
3.  **Normalizes** to a unified schema (OMOP CDM).
4.  **Sends** only anonymized aggregates or model updates (gradients, weights).

**Central Orchestrator:**
*   Collects model updates.
*   Aggregates them (federated averaging with secure aggregation).
*   Maintains audit logs and versioning.
*   Calculates each participant's contribution.

**Data never leaves the clinic — only training signals do.**

---

### **Privacy — By Architecture, Not by Promise**

| Level | Mechanism |
| :--- | :--- |
| 1 | Removal of direct identifiers (name, ID, address, exact date). |
| 2 | Generalization of quasi-identifiers (age groups, rare diagnoses → suppression). |
| 3 | Record-level re-identification risk assessment (k-anonymity, l-diversity). |
| 4 | **Differential Privacy (ε=2.0 per round, δ=10⁻⁵, total budget ε_total≤10.0, Gaussian mechanism, accounting via Rényi DP).** |
| 5 | Secure Aggregation (orchestrator sees only the aggregated result). |

**Result:** Even if the orchestrator is compromised or traffic is intercepted, patient data cannot be reconstructed.

---

### **Threat Model and Security**

**Threat Model:**
*   **Orchestrator:** Honest-but-curious — follows the protocol but may try to infer information.
*   **Nodes:** Up to 25% may be malicious (Byzantine) — sending incorrect updates to poison the model.
*   **External Attacker:** May intercept network traffic.

**Protective Measures:**
*   **Secure Aggregation (SecAgg+):** Orchestrator cannot see individual node updates.
*   **Differential Privacy:** Sufficient noise to prevent data reconstruction from gradients.
*   **Minimum Batch Size:** ≥32 records to mitigate gradient inversion attacks (Zhu et al. 2019).
*   **Robust Aggregation:** Use of **Krum** for resilience against up to 25% malicious nodes.
*   **Reputational Scoring:** Nodes with anomalous behavior are automatically excluded from training.

---

### **Unified Clinical Schema**

We employ the **OMOP Common Data Model**, the most widespread standard for observational studies. Adaptation:

| Domain | Standard |
| :--- | :--- |
| Diagnoses | ICD-10/11 |
| Laboratory | LOINC |
| Medications | ATC, RxNorm |
| Procedures | SNOMED CT, CPT |
| Demographics | Generalized categories |

The local node converts data from HIS/EHR to OMOP on-premise. **FHIR** is supported as an alternative input format.

---

### **Contribution Measurement**

**Contribution is assessed via approximated Federated Shapley Value.**
The marginal contribution of each participant to the global model's performance is computed. For scalability with 5–10 nodes, a Monte Carlo method (100–200 iterations) is used, providing acceptable accuracy with computational cost on the order of O(n² × M). For 10 nodes and 200 iterations, the added time per round is ~10 minutes, acceptable for asynchronous FL.

**Advantages:**
*   **Theoretical Soundness:** The only scheme satisfying axioms of efficiency, symmetry, linearity, and null player.
*   **Manipulation-Resistant:** Marginal contribution is hard to artificially inflate.
*   **Proportionality:** Credits strictly correspond to real impact on model quality.
*   **Resilience to Dominance:** Values marginal, not absolute, data volume.

Participants earn **contribution credits**, convertible into benefits.
**Free-rider protection:** Access to the global model is granted only to participants whose accumulated contribution score exceeds a set threshold (e.g., 5% of the average).

---

### **Benefits for Participants**

**Clinics:**
*   Access to best-in-class AI models without losing data control.
*   Licensing discounts.
*   Priority access to new versions.
*   Custom model fine-tuning on local data.
*   Anonymous benchmarking.

**Pharmaceutical Companies:**
*   Access to heterogeneous real-world data without jurisdictional risk.
*   Post-marketing effectiveness analysis.
*   Accelerated R&D via federated cohorts.

**All Participants:**
*   Proportional share of commercial revenue.
*   Participation in consortium governance.

---

### **Governance Model**

**Legal Structure:** A **non-profit partnership** under the law of the host country (Georgia or an EU state). Participants sign a Consortium Agreement.
**Two-Tier Model:** Separates clinical and industrial participants with distinct rights and obligations.
**Multi-Stakeholder Board:** Includes voting members from clinics and pharma, plus non-voting patient advocates, data protection lawyers, ethicists, and technical auditors.
**External Clinical Advisory Board:** 2-3 independent clinicians for validation (budget: €20,000).
**Six Governing Principles:** Enforce data locality, privacy validation, contribution weighting, transparent scoring, bias auditing, and fair benefit distribution.

---

### **Legal Basis**

*   **Data Use Agreement (DUA):** Standard template prohibiting raw data transfer. Negotiations start 3 months prior to technical work.
*   **IRB Approval:** Each institution obtains local ethics committee approval. Process supported by the project's Medical Consultant.
*   **Data Steward:** Appointed at each participating organization to ensure compliance.
*   **GDPR Compliance:** Based on patient consent or national law for secondary use; DPIA required.
*   **Georgian PDPL Compliance:** Harmonized with GDPR; similar requirements apply.

---

### **Intellectual Property and Financial Model**

**IP Regime:**
*   Base Model: Consortium's joint property, licensed to participants at preferential rates.
*   Participant-specific fine-tunings: Participant's exclusive property.
*   Local components: Participant's property with open license to the consortium.
*   Methodology/Publications: Open Access with consortium attribution.

**Financial Model (Phased):**
*   **Phase 1 — Pilot (12 months):** Grant funding (e.g., Horizon Europe). No fees.
*   **Phase 2 — Scaling:** Entry fees for new members; external licensing. 70% revenue distributed via Shapley credits, 30% to platform development.
*   **Phase 3 — Sustainability:** Annual membership fees (Clinics: from €5k; Pharma: from €50k); licensing as main income.

---

### **MVP (Minimum Viable Product)**

**Use Case:** Predicting 12-month hospitalization risk for patients with Type 2 Diabetes.
**Pilot Participants:** 3-5 Georgian clinics (candidates identified), 1 Pharma partner (TBC), 1 EU technical partner (TBC).
**Key Hires (Grant-funded):** Medical Consultant (0.5 FTE, €60k), Technical Expert - Database Systems (1.0 FTE, €60k).
**Timeline:** 3 months legal prep, 3 months technical dev, 6 months deployment/pilot (Total: 12 months).
**Success Criteria:** AUC >0.75, zero raw data export (audited), functional Shapley scoring, technical interoperability, all legal/IRB requirements met.

---

### **Technical Stack**

| Component | Choice | Justification |
| :--- | :--- | :--- |
| FL Framework | **Flower** + **OpenFL** | Flexibility + medical application maturity. |
| **Aggregation Algorithm** | **FedProx** (μ=0.1–1.0) | Resilience to non-IID data. |
| **Robust Aggregation** | **Krum** | Byzantine tolerance (up to 25% malicious nodes). |
| Secure Aggregation | **SecAgg+** | Hides individual updates from orchestrator. |
| Differential Privacy | **TensorFlow Privacy / Opacus** with **Rényi DP** (ε=2.0/round, δ=10⁻⁵). | Standard libraries. |
| Contribution Eval. | **Federated Shapley Value** (Monte Carlo, M=100–200). | Theoretically sound, manipulation-resistant. |
| OMOP Normalization | **OHDSI WhiteRabbit** + custom ETL. | Proven toolkit. |
| Interoperability | **HL7/FHIR** → OMOP. | Standard exchange to analysis model. |
| Node Storage | **PostgreSQL** + OMOP CDM. | Open source, sufficient capacity. |
| Security | **TLS 1.3**, disk encryption, HSM for keys. | Medical data compliance. |

**Minimum Node Hardware:** 8-core CPU, 32GB RAM, NVIDIA T4 GPU (16GB), 1TB SSD, 100 Mbps static IP, TPM 2.0/HSM.

---

### **Risk Matrix**

Key risks include re-identification (Low Prob./Critical Impact, mitigated by privacy stack), data poisoning (Medium/Critical, mitigated by Krum), bias (High/Medium, mitigated by monitoring), DUA/IRB delays (High/High, mitigated by early start & templates), and lack of EU partner (Medium/Critical, mitigated by active search). Owners (Privacy Lead, Technical Lead, etc.) are assigned for each.

---

### **Ecosystem**

The project integrates into the broader **AIM (Advanced Intelligence in Medicine)** ecosystem, leveraging shared infrastructure (e.g., DeepSeek API) and unified data policies.

---

### **Funding Instrument of Choice: EIC Pathfinder Open**

**Deadline: May 12, 2026.** The project aligns with criteria for breakthrough innovation (combining FL, secure aggregation, Shapley incentives, cooperative governance), target TRL (2-3 to 4), budget (<€4M), and consortium requirements (Georgia + min. 2 EU partners).

---

### **Consortium Structure (Proposed)**

| Partner | Country | Type | Proposed Role | Status |
| :--- | :--- | :--- | :--- | :--- |
| Phasis Academy (J. Tkemaladze) | Georgia | Research | PI, Coordinator | ✅ Confirmed |
| Giorgi Tsomaia | Georgia | Independent Expert | Co-I, WP2 & WP4 Lead | ✅ Confirmed |
| **Medical Consultant** | Georgia/EU | Clinical | Clinical Validation, IRB | Vacancy (in budget) |
| **Technical Expert (DB)** | Georgia/EU | Technical | ETL, OMOP, Integration | Vacancy (in budget) |
| Aversi Clinic | Georgia | Clinical | Pilot Site | Negotiations |
| GeoHospitals | Georgia | Clinical | Pilot Site | Preliminary Agreement |
| Iashvili Children's Hospital | Georgia | Clinical | Pilot Site | Negotiations |
| **[BCN-AIM Lab]** | **Spain (EU)** | **Technical/Research** | **EU Co-Principal Investigator, Technical Lead (FL Architecture), EHDS Validation** | **To be invited** |
| **[fedder.ai]** | **EU** | **Technical** | **Potential EU Technical Partner** | **To be invited** |
| [Pharma Partner — TBC] | EU/Georgia | Industrial | Validation, Commercialization | Search Active |

---

### **Budget Estimate**

| Category | Estimate (€) |
| :--- | :--- |
| Legal Preparation (DUA, IRB, memo, Data Steward) | 55,000 |
| Medical Consultant (0.5 FTE, 12 months) | 60,000 |
| Technical Expert - Database Systems (1.0 FTE, 12 months) | 60,000 |
| External Clinical Advisory Board | 20,000 |
| Local Node Development | 280,000 |
| Normalization & ETL | 120,000 |
| Shapley Scoring & Aggregation | 250,000 |
| Federated Learning & Security | 250,000 |
| Deployment in Pilot Clinics | 100,000 |
| Coordination & Management | 320,000 |
| WP4 (Governance) | 220,000 |
| Dissemination, Communication, Events | 130,000 |
| Equipment, Infrastructure | 120,000 |
| Travel | 100,000 |
| Contingency (10%) | 210,000 |
| **TOTAL** | **2,275,000** |

*Note: The EIC Pathfinder ceiling is €4M. This is a base estimate; it can be increased to €3–3.5M with consortium expansion.*

---

### **Next Steps & Timeline**

1.  **Weeks 1–2:** Initiate search for Medical Consultant & Technical Expert; begin DUA/IRB processes with pilot clinics.
2.  **Weeks 2–3:** Prepare legal memo on PDPL/GDPR; **formally invite BCN-AIM Lab and fedder.ai.**
3.  **Weeks 3–4:** Secure letters of support from clinics.
4.  **Weeks 4–5:** Prepare grant application (Part A & B).
5.  **By May 12, 2026:** **Submit final proposal to EIC Pathfinder Open.**

---

### **Contacts**

**Dr. Jaba Tkemaladze, MD** — Principal Investigator, Project Coordinator
Email: djabbat@gmail.com

**Giorgi Tsomaia** — Co-Investigator, Lead of WP2 (Biomedical Data Systems) and WP4 (Governance & Incentive Model)
Email: gakelytemp@gmail.com

---
*Version 6.0 — Final. This document has undergone full review and is ready for submission to EIC Pathfinder Open. April 2026.*