# LINKS.md — FCLC: Ecosystem Connections

## Internal Ecosystem Links

### FCLC ↔ AIM (central medical AI hub)
- **Direction:** FCLC feeds improved models back to AIM; AIM is the primary beneficiary of trained models
- **Mechanism:** After each federated training cycle, the global model (logistic regression / MLP weights) can be exported and integrated into AIM's `diagnosis_engine.py` or `treatment_recommender.py`
- **AIM role:** AIM at DrJaba clinic is itself a potential pilot node (`fclc-node` running locally)
- **Data flow:** AIM patient records (anonymized, OMOP-normalized via FCLC's de-identification pipeline) → local training → gradient update → orchestrator
- **Benefit for AIM:** Models trained on multi-clinic data outperform single-clinic models; AIM's diagnostic accuracy improves with each federated round
- **Benefit for FCLC:** AIM is a guaranteed first-participant node with real clinical data (Dr. Tkemaladze's integrative medicine practice)

### FCLC ↔ DrJaba (platform for participant clinics)
- **Direction:** DrJaba.com is the public face and coordination point for FCLC participant clinics
- **Use case:** Clinics that use the DrJaba platform (or are known to Dr. Tkemaladze's network) are the primary recruitment targets for FCLC pilot
- **Infrastructure:** FCLC orchestrator (`fclc-server`) may be hosted under `drjaba.com` umbrella infrastructure
- **Domain (planned):** `fclc.drjaba.com` or similar for the `fclc-web` Phoenix dashboard

### FCLC ↔ WLRAbastumani (potential first participant clinic)
- **Direction:** WLRAbastumani (Abastumani Wellness & Longevity Resort) as candidate first external participant node
- **Rationale:** Within Dr. Tkemaladze's professional network; has patient records that could contribute to T2DM or longevity-outcome models
- **Status:** Not yet confirmed; requires IRB/DUA agreement templates (Task #63 in TODO.md)
- **Action needed:** Approach after fclc-node binary is demo-ready; show de-identification UI (Task #25)

### FCLC ↔ CDATA (aging simulation data as training source)
- **Direction:** CDATA's aging simulation datasets are a candidate training/validation data source for FCLC
- **Use case:** CDATA generates synthetic or modeled aging-related biological data; this can serve as a held-out validation set on the orchestrator (avoiding the need for real patient data at the orchestrator level)
- **Benefit for FCLC:** Reduces dependency on real clinic data for validation; allows Shapley scoring to proceed with synthetic reference
- **Benefit for CDATA:** FCLC models trained on real clinical data can validate CDATA's simulation outputs
- **Data type alignment:** CDATA → aging biomarkers, telomere dynamics; FCLC pilot → T2DM or oncology; overlap in metabolic and aging-related endpoints

---

## External Collaborators

### Giorgi Tsomaia
- **Role:** External co-author (scientific/methodological)
- **Contribution:** Conceptual and scientific contributions to federated learning methodology and privacy architecture
- **Co-authorship:** To be listed on all publications, preprints, and grant applications arising from FCLC
- **Contact:** Via Dr. Tkemaladze directly

---

## External Frameworks & References

### Open-Source Federated Learning Frameworks
| Framework | Language | Notes |
|-----------|----------|-------|
| **Flower (flwr)** | Python | Most widely adopted FL framework; FCLC's Rust implementation is custom but Flower is the reference architecture |
| **PySyft** | Python | Privacy-preserving ML; pioneered SecAgg concepts used in FCLC |
| **TensorFlow Federated (TFF)** | Python | Google's FL framework; good reference for FedAvg/FedProx implementations |
| **OpenFL** | Python | Intel's FL framework; focuses on medical imaging |

### Standards & Regulatory
- **OMOP CDM:** https://ohdsi.github.io/CommonDataModel/ — normalization schema used by FCLC
- **GDPR Article 9:** Special categories of personal data (health data); basis for federated approach
- **Georgian PDPL:** Personal Data Protection Law of Georgia; mirrors GDPR
- **HL7 FHIR:** https://hl7.org/fhir/ — data interchange format supported by `fclc-node` connector

### GitHub Repositories
- **Private:** https://github.com/djabbat/FCLC — full codebase including CLAUDE.md, TODO.md, PARAMETERS.md, MAP.md
- **Public:** https://github.com/djabbat/FCLC-public — excludes CLAUDE.md, TODO.md, PARAMETERS.md, MAP.md

### Grant Target
- **EIC Pathfinder:** https://eic.ec.europa.eu/eic-funding-opportunities/eic-pathfinder_en
- **Deadline:** 10 May 2026
