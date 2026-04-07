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
| L5: SecAgg+ | Orchestrator never sees individual updates | Art. 5(1)(f) | ⚠️ Demo mode — LCG PRG; needs ChaCha20 before production |

**Critical gap:** SecAgg+ currently uses LCG + DefaultHasher instead of ChaCha20-Poly1305 + DH key exchange. Must be replaced before clinical deployment.

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
- [ ] SecAgg+: replace LCG with ChaCha20 PRG (**before any clinical use**)
- [ ] Penetration testing of REST API endpoints
- [ ] Rate limiting on `/api/nodes/:id/update` (prevent gradient flooding)
- [ ] Node authentication (currently UUID only — add HMAC or TLS client certs)
- [ ] Database encryption at rest
- [ ] Automated DP budget alerting (warn node when ε_remaining < 2.0)

---

## Responsible Disclosure

Security issues should be reported to the PI (J. Tkemaladze) via private channel.  
No public bug bounty program at this stage (pre-clinical software).

---

*Updated: 2026-04-06*  
*Next review: before first clinical pilot (IRB submission date)*
