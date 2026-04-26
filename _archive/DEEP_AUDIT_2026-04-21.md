# FCLC EIC Pathfinder Deep Audit
**Date:** 2026-04-21
**Deadline:** 2026-05-12 (21 days remaining)
**Auditor:** Claude Opus 4.7 (independent post-fix review)
**Scope:** Verify whether recent fixes (CORRECTIONS_2026-04-22, FIXES_V3) have lifted FCLC above the previous v10 rejection score (1.86/5).
**Verdict headline:** **DO NOT SUBMIT on 2026-05-12.** Multiple FATAL issues remain. Defer to 2027 cycle as already recommended in `../EIC_CONSORTIUM_STRUCTURE_2026-04-21.md`.

---

## §1 Eligibility check

### 1.1 Call rules (verified against eic.ec.europa.eu, April 2026)

EIC Pathfinder Open 2026 formal requirements:
- **Consortium:** minimum **3 independent legal entities**, each in a **different** eligible country; **≥1 in an EU Member State**; **≥2 others** in different EU Member States or Associated Countries.
- **Budget range:** €1M – €4M per project; total call envelope €166M.
- **TRL:** 1–4 target (proof of concept of a novel technology).
- **Deadline:** 2026-05-12, 17:00 Brussels.
- Georgia is an Associated Country (eligible as participant).

### 1.2 Current FCLC / CommonHealth consortium — status

Source: `FCLC/CONCEPT.md §Consortium Structure` and `EIC_CONSORTIUM_STRUCTURE_2026-04-21.md`:

| Partner | Country | Status | Meets EIC rule? |
|---|---|---|---|
| J. Tkemaladze (NGO Georgia Longevity Alliance, coordinator) | GE | Confirmed | GE = eligible Associated Country |
| Giorgi Tsomaia (independent expert) | GE | Confirmed | Same country as coordinator — **does not count as a second independent entity for the 3-country rule** |
| Aversi Clinic | GE | "Negotiations started" | Same country; **no signed LoI** |
| GeoHospitals | GE | "Preliminary agreement" | Same country; **no signed LoI** |
| Iashvili Children's Hospital | GE | "Negotiations started" | Same country; **no signed LoI** |
| DFKI (DE) | DE | "Inquiry sent 2026-04-01" | **UNCONFIRMED** |
| Fraunhofer IAIS (DE) | DE | "Planned for week 15" | **NOT CONTACTED** |
| Saarland University | DE | "Preliminary discussions" | **UNCONFIRMED** |
| Karolinska | SE | "Backup option" | **NOT CONTACTED** |
| EU Medical partner (Charité/Karolinska/Erasmus) | — | "TBC" | **UNCONFIRMED** |
| EU Technical partner | — | "TBC" | **UNCONFIRMED** |
| Pharmaceutical partner | — | "active search" | **UNCONFIRMED** |

### 1.3 Eligibility verdict

🔴 **FAIL — FATAL.** To be eligible, FCLC needs signed LoI/commitment letters from **at least 2 independent EU legal entities in 2 different Member States**, and the coordinator must be an EU-eligible legal entity (NGO Georgia Longevity Alliance qualifies as Associated Country participant, but the 3-country rule still requires 1 EU MS + 1 other EU/Associated). As of 2026-04-21:
- 0 signed LoI from EU entities.
- DFKI inquiry was sent 20 days before deadline; Horizon Europe LoI process typically takes 4–8 weeks.
- The "Plan B" in CONCEPT.md §Consortium (submit with only a Georgian partner + "explanation") is **not a valid EIC submission strategy** — it would be rejected at eligibility check, not scored.

**FCLC's own documents admit this:** `EIC_CONSORTIUM_STRUCTURE_2026-04-21.md` already states submission is **DEFERRED to 2027–2028** due to "Fictitious consortium ('MPI-AGE or equivalent' not acceptable)". The CLAUDE.md root still shows the old 2026-05-12 target and €3.0M Variant C — **internal inconsistency between governance documents**.

### 1.4 TRL claim check
`CONCEPT.md §TRL` table lists TRL 2–4 for components (appropriate for Pathfinder). ✅ Passes.

### 1.5 Variant C validity
The 5-WP Variant C (FCLC €0.6M / Ze €0.5M / CDATA €0.8M / BioSense €0.6M / Aqtivirebuli €0.5M) is **explicitly annulled** by CORRECTIONS_2026-04-22 §1.4. The CLAUDE.md of CommonHealth still advertises Variant C → must be synchronised with CORRECTIONS before any document leaves the building.

---

## §2 Technical claims verification

### 2.1 χ_Ze "validated" claim (v10 blocker)

**Previous misrepresentation:** χ_Ze as "validated clinical biomarker with d=1.694, R²=0.84".

**Current status after fixes:**
- CORRECTIONS_2026-04-22 §1.2 formally retracts χ_Ze as a validated biomarker ("R²=0.84 получено на синтетических данных"; failed pre-registered MPI-LEMON / Dortmund Vital / Cuban tests).
- `FCLC/CONCEPT.md` L444 now contains a scope note: *"χ_Ze is an exploratory biomarker... Do not present χ_Ze as validated."* ✅
- **BUT** the same CONCEPT.md L434 still contains: *"a non-invasive EEG/HRV complexity biomarker (χ_Ze, exploratory pilot on N=196, d=1.694, p<0.0001 — see caveat below)"* — **this is contradictory**. The d=1.694 figure comes from the synthetic-data pipeline retracted by CORRECTIONS §1.2. Any reviewer doing 5 minutes of Google will find the conflict between "retracted" in CORRECTIONS and "d=1.694 p<0.0001" in CONCEPT.

🔴 **PARTIAL FIX, still FATAL.** Delete the d=1.694 / N=196 figures entirely from FCLC CONCEPT until Dortmund Vital / MPI-LEMON pre-registered replication succeeds.

### 2.2 CDATA R²=0.84 claim

**Current claim (FCLC CONCEPT L456):** *"R²=0.84 for CDATA is cross-sectional. Longitudinal validation is an explicit deliverable in WP3–4."*

**Problem:** FCLC/CONCEPT still presents `CDATA (R²=0.84 cross-sectional)` in the ecosystem Synergy Matrix as a confirmed property. The CDATA Mega-Audit (`MEGA_AUDIT_V3_2026-04-21/06_PEER_REVIEW_CDATA.md`, cross-referenced in CORRECTIONS §1.6) shows **ABL-2 Sobol inversion**: S1(epigenetic_rate)=0.403 > S1(alpha_centriolar)=0.224 — i.e. the centriolar mechanism is NOT the dominant driver in CDATA's own simulation. R²=0.84 may largely be carried by the epigenetic term, not the causal claim that makes CDATA distinctive.

🟠 **HIGH.** Remove R²=0.84 from FCLC CONCEPT entirely or re-phrase as "descriptive cross-sectional fit, not validation of the centriolar hypothesis."

### 2.3 δ discrepancy (v10 blocker)

**Previous blocker:** different δ values in different files.

**Current status:**
- CONCEPT.md uses δ = 10⁻⁵ consistently (L64, Privacy-Utility table L516, PATE section L549).
- PARAMETERS.md: `δ_round = 10⁻⁵` (row 15).
- THEORY.md §3.2: δ = 10⁻⁵.
- EVIDENCE.md: does not quote a specific δ.

✅ **δ discrepancy resolved.** One value, consistently 1e-5.

### 2.4 ε claim — still the biggest scientific weakness

**Current state:** ε_round = 2.0, ε_total ≤ 10.0 (linear composition over 5 rounds).

CONCEPT.md admits (L64): *"ISO/IEC 27559:2022 recommends ε_total<1.0 for health data. WP2 target: redesign to ε≤0.5/round via PATE."*

**Analysis:**
- The current ε_total = 10.0 for a **5-round pilot** is defensible only as an "exploratory research setting". But the ε(T) projection table (L528) shows linear composition reaches ε=200 at 100 rounds — completely unusable.
- The proposed PATE solution (L549) calculates ε ≈ 0.63 at n_teachers=5. This is **not yet implemented** (`PateConfig` is a stub; no empirical AUC at ε=0.63).
- The medical-FL literature standard target is ε < 1.0. A proposal reaching EIC with ε=10 and a "we plan to fix it" roadmap is, as Reviewer C noted, a **fatal ethical flaw** for any biomedical funder.

🔴 **UNRESOLVED BLOCKER.** Until `PateConfig` is end-to-end benchmarked on MIMIC-IV with AUC > 0.70 at ε ≤ 1.0 and a short paper deposited on arXiv, any claim of privacy-preserving medical FL is aspirational. This cannot be fixed in 21 days.

### 2.5 SecAgg+ "research-grade" vs "validated" claim

**Current state (CONCEPT §Privacy threat model L77):** honest caveat — *"ACTIVE ADVERSARY LIMITATION: Current implementation is proven secure against a semi-honest (honest-but-curious) orchestrator only."* ✅

**44/44 tests passing** (EVIDENCE §2.1) is internally consistent and factually true for the code in `fclc-core/aggregation/secagg/`. But: 44 unit/integration tests ≠ cryptographic audit. The CONCEPT honestly says external audit is WP3 M4-M6. ✅

### 2.6 "Validated on MIMIC-IV" — honest?

MIMIC-IV results (CONCEPT §Validation Results L491): 5 non-overlapping partitions of N=12,543 T2D patients, AUC=0.742±0.021 with DP. Free-rider and Byzantine scenarios reported.

**Issue:** MIMIC-IV is a **single US hospital** (BIDMC). "5 nodes" were artificial partitions of one dataset, not 5 real sites. For an EIC panel this is a **simulation**, not a federation. The wording *"External validation... eICU-CRD"* is correctly framed as WP2 deliverable. But the narrative framing (Synergy Matrix, Market Analysis) implies more maturity than 5 simulated partitions of a single public dataset warrants.

🟠 **HIGH.** Reframe as "single-dataset partitioned simulation" throughout, not "federated validation".

---

## §3 Budget realism

### 3.1 What FCLC/CommonHealth is asking — inconsistent across files

| Source | Total FCLC ask | Total CommonHealth ask |
|---|---|---|
| `FCLC/CONCEPT.md` L714 (budget table) | €2,275,000 | — |
| `FCLC/CONCEPT.md` L341 (Gantt, "Total requested") | **€3,200,000** | — |
| `CommonHealth/CLAUDE.md` (Variant C) | €0.6M FCLC | **€3.0M total** |
| `EIC_CONSORTIUM_STRUCTURE_2026-04-21.md` (deferred 2027) | €0.7M FCLC | **€3.3M total** |
| `CORRECTIONS_2026-04-22.md` §1.4 (Variant B) | €0.5M FCLC | **€2.2M total** |

🔴 **CRITICAL INCONSISTENCY.** FCLC alone has **4 different budget figures** in 4 official files. Any EIC reviewer or evaluator will find this instantly. This alone is a FATAL at eligibility/quality check.

### 3.2 Defensibility of the individual lines (using CONCEPT.md budget table)

| Category | Proposed | Realistic for EIC Pathfinder? |
|---|---|---|
| Legal prep (DUA/IRB × 3–5 Georgia clinics) | €55,000 | Low — EU legal counsel for multi-jurisdictional DUA is €150–300/hr; €55k buys ~200 hours. Underfunded if multi-country. |
| Medical Consultant 0.5 FTE × 12 mo | €60,000 | Plausible for Georgia rates; too low for EU rates. |
| Technical Expert 1.0 FTE × 12 mo | €60,000 | Severely below EU market rate (~€90–130k/yr total cost for senior Rust/FL engineer in DE). |
| Shapley + aggregation (2 FTE × 24 mo) | €250,000 | Low — 48 person-months at €5.2k/month is Georgia-only rate. |
| External cryptographic audit | €35,000 | Insufficient — independent FL/crypto audits by NCC Group, Trail of Bits, Kudelski run €80–150k for a full protocol audit. |
| eICU-CRD external validation | €25,000 | Low — data access is free, but 0.25 FTE × 6 mo of a biostatistician = €30–40k. |
| Coordination + management (2 FTE × 36 mo) | €320,000 | Below EU academia (≈€600k for a real grant manager + PI admin). |
| Contingency (10%) | €210,000 | Standard. |
| **Total** | **€2,275,000** | **Under-specified; likely €3.5–4.0M at realistic EU rates** |

### 3.3 Budget verdict

🔴 **BLOCKER.** The budget table is built on Georgia-only cost assumptions but the consortium-building text requires EU academic partners who must be budgeted at EU rates. The €2.275M figure is defensible only if 100% of work is done in Georgia; if 2 EU labs are partners they consume €600k–1M each at typical EU academic overhead rates. True defensible budget for the stated scope: **€3.5–4.0M** — right at the EIC ceiling.

Additionally, Reviewer B (MEGA_AUDIT peer review) verdict already flagged: *"Underfunded budget (€2.1M insufficient for scope)"* — this has **not** been corrected.

---

## §4 Competitor differentiation

### 4.1 Landscape (verified 2026-04-21)

| Project | Type | Scope overlap with FCLC | FCLC differentiator? |
|---|---|---|---|
| **TRUMPET** (Horizon Europe, 2022–2025) | HE research | Federated learning for health, privacy beyond GDPR, scalable FL service platform | **Direct competitor.** Already funded, publishing. FCLC has no stated differentiator vs TRUMPET. |
| **DataTools4Heart (DT4H)** (HE, 2022–2025) | HE research | Federated cardiology toolbox, OMOP-compatible, privacy-preserving | **Direct competitor on OMOP+FL.** Already in Phase 2 (ESC press release, 2025). |
| **FeatureCloud** (H2020 grant 826078, 2019–2023) | FL platform | FL as a Service, Docker-based, medical data | Finished but followed by spin-offs; 20+ apps deployed. FCLC's advantage: Shapley contribution + cooperative governance. |
| **ECRIN** | Clinical trial infra | Multi-country clinical network, not FL-focused | Not direct competitor; possible ally. |
| **MELLODDY** (IMI2, completed) | Pharma FL consortium | Drug discovery FL across 10 pharma | Different vertical. |
| **Owkin** (commercial, €200M+ funded) | Commercial FL | Medical FL, pharma partnerships | Well-funded competitor; FCLC differentiator is cooperative non-profit structure. |
| **NVIDIA FLARE / Intel OpenFL** | Industrial FL frameworks | Plumbing | FCLC is a **governance + incentives layer**, not a framework. Correct positioning. |
| **Rhino Health** | Commercial | US-focused | Not EU overlap. |

### 4.2 FCLC's stated differentiators (CONCEPT §Competitive Landscape L629)

1. **Cooperative non-profit governance with Shapley incentives** — genuinely novel relative to the listed competitors. ✅
2. **Focus on low-resource national health systems (Georgia / South Caucasus)** — valid niche, but EIC Pathfinder is a **pan-European breakthrough** instrument. Regional focus must be argued as a springboard, not the ceiling.
3. **Byzantine-robust (Krum) + PATE → ε<0.5** — same as TRUMPET and DataTools4Heart are building. **Not unique.**

### 4.3 Differentiation verdict

🟠 **INSUFFICIENT.** CONCEPT.md §Competitive Landscape table (L629) **does not mention TRUMPET or DataTools4Heart at all**. An EIC panel will score this as incomplete landscape analysis. Both projects directly occupy the FCLC technical space (federated + OMOP + privacy) and are already publishing. FCLC must either:
(a) demonstrate a concrete technical advance over TRUMPET/DT4H (e.g. cryptographically verified Shapley accounting that neither has), or
(b) position as a **complementary downstream layer** (governance + incentives on top of TRUMPET's FL infrastructure).

---

## §5 PMID / DOI verification

I verified via Crossref and PubMed the citations that appear in `FCLC/EVIDENCE.md §1` (the 5 core literature claims).

| Claim | Cited DOI / ID | Verified (Crossref/PubMed) | Status |
|---|---|---|---|
| McMahan et al., FedAvg (2016) | `10.48550/arXiv.1602.05629` | arXiv:1602.05629 title confirmed | ✅ OK (arXiv DOI valid) |
| Bonawitz et al., Practical Secure Aggregation (CCS 2017) | `10.1145/3133956.3133982` | Crossref returns: "Practical Secure Aggregation for Privacy-Preserving Machine Learning" by Bonawitz, Ivanov, Kreuter et al., 2017 ACM SIGSAC CCS | ✅ **VERIFIED** |
| Zhu et al., Deep Leakage from Gradients (NeurIPS 2019) | `10.1145/3321705.3329809` | Crossref returns: **"MPC Joins The Dark Side"** by Cartlidge, Smart, Talibi Alaoui — **completely different paper** (AsiaCCS 2019) | 🔴 **WRONG DOI — FABRICATED / MISMATCHED** |
| Dwork & Roth, Algorithmic Foundations of DP | ISBN 978-1-108-47457-8 | Real book (Now Publishers, 2014), ISBN shape correct | ⚠️ ISBN shape is for Cambridge; the Now Publishers monograph ISBN differs. Verify by hand against Now Publishers catalog. |
| Blanchard et al., Krum (NeurIPS 2017) | arXiv:1703.02757 | arXiv title matches | ✅ OK |

### 5.1 PMID verification

FCLC/EVIDENCE.md cites **zero PMIDs directly**. The Mega-Audit v3 (`CORRECTIONS_2026-04-22.md` §2.6) claims "PMID: 365/365 подтверждены (100%) — нет фабрикации" across the ecosystem, but this applies to CDATA / MCOA / BioSense bibliographies, not FCLC (which cites conference DOIs, not PubMed).

### 5.2 Citation verdict

🔴 **CITATION ERROR IN EVIDENCE.md.** The DOI `10.1145/3321705.3329809` attributed to Zhu et al. "Deep Leakage from Gradients" actually resolves to "MPC Joins The Dark Side" (AsiaCCS 2019). The correct identifier for DLG is **arXiv:1906.08935** and NeurIPS 2019 proceedings (no SIGSAC DOI). Zhu et al. 2019 is cited as the foundational gradient-inversion attack throughout CONCEPT.md (L89, L585 privacy audit metrics) — the ACM DOI is simply wrong.

This is the exact type of error that discredits a proposal in peer review. **Fix before submission to any venue, not just EIC.**

---

## §6 5 hardest reviewer questions + draft answers

### Q1 (Excellence, FATAL): "FCLC proposes no new FL algorithm. TRUMPET and DataTools4Heart have delivered OMOP-based federated health infrastructures. What is the breakthrough?"

**Honest draft answer:** The breakthrough is **not** a new FL algorithm. It is the first **cryptographically-auditable cooperative governance layer** combining: (i) Federated Shapley Value with verifiable Monte-Carlo seeds under SecAgg+; (ii) Byzantine-robust aggregation tied to Shapley reputation scoring; (iii) PATE-based privacy budget at ε < 0.5 with per-client ε(T) enforcement. TRUMPET and DT4H do not implement Shapley-based incentive distribution or cryptographically bound contribution accounting. **Weakness of answer:** this is an engineering integration, not a science breakthrough — inherently mid-tier for Pathfinder, which explicitly funds *science-driven technologies at TRL 1–4* rather than infrastructure layers.

### Q2 (Implementation, FATAL): "Your budget table shows €2.275M, your Gantt says €3.2M, your umbrella says €0.6M. Which is it?"

**Honest draft answer:** No defensible answer. This inconsistency alone is a procedural rejection risk. **Action:** reconcile all files to a single figure before submission.

### Q3 (Ethics/Regulatory, BLOCKING): "You claim ISO/IEC 27559 compliance but ε_total=10.0 is 10× above the recommended ceiling. Why should the EIC fund a health-data project that knowingly violates privacy norms?"

**Honest draft answer:** The ε=10.0 value is an exploratory upper bound for the v1.0 prototype pilot; the final deployable version (v2.0) uses PATE with ε≈0.63. CONCEPT.md §ε Reduction Roadmap quantifies the trajectory. **Weakness:** PATE is not yet implemented empirically. Until there is a runnable demonstration at ε < 1.0 with AUC > 0.70, the answer is aspirational and Reviewer C (Wellcome Trust) scored this as REJECT.

### Q4 (Consortium, FATAL): "Name your signed EU partners and show the Letters of Commitment."

**Honest draft answer (current):** None signed as of 2026-04-21. DFKI contacted 2026-04-01; response pending. Fraunhofer, Saarland, Karolinska not yet engaged beyond preliminary. **This is why the internal governance document (`EIC_CONSORTIUM_STRUCTURE_2026-04-21.md`) correctly recommends DEFERRAL to 2027**.

### Q5 (Impact, HIGH): "Your validation is on MIMIC-IV (a single US hospital) partitioned into 5 synthetic nodes. How does this generalise to real European multi-site federation?"

**Honest draft answer:** It does not — that is exactly what WP2 (eICU-CRD multi-centre external) and WP3 (Georgian clinical pilot N≥200) will test. The MIMIC-IV result is a sanity check of the software stack, not a generalisation claim. **Weakness:** CONCEPT.md §Validation Roadmap (L448) frames MIMIC-IV as an "internal validation" — reviewers will correctly read this as oversold. Re-label as "prototype sanity check".

---

## §7 Revised go / no-go recommendation

### 7.1 Blockers remaining (post-CORRECTIONS, post-FIXES_V3)

| # | Blocker | Fixable in 21 days? |
|---|---|---|
| B1 | No signed EU partner LoI (violates EIC eligibility rule) | 🔴 No — realistic LoI timeline is 4–8 weeks |
| B2 | Four different budget figures across governance docs | 🟢 Yes — editorial |
| B3 | ε_total=10 without empirical PATE ε<1 demonstration | 🔴 No — requires code + experiments + short paper |
| B4 | χ_Ze d=1.694 still cited in FCLC CONCEPT L434 despite CORRECTIONS §1.2 retraction | 🟢 Yes — editorial |
| B5 | Wrong DOI for Zhu 2019 "Deep Leakage from Gradients" | 🟢 Yes — editorial |
| B6 | Competitive landscape missing TRUMPET and DataTools4Heart | 🟢 Yes — editorial |
| B7 | "Validated on MIMIC-IV" framing oversells partitioned single-dataset simulation | 🟢 Yes — editorial |
| B8 | CDATA ABL-2 Sobol inversion unresolved; R²=0.84 claim still carried into FCLC | 🔴 No — this is CDATA's scientific problem, independent of FCLC |
| B9 | Internal inconsistency: CommonHealth/CLAUDE.md still advertises 2026-05-12 + Variant C; EIC_CONSORTIUM_STRUCTURE already says DEFERRED to 2027 | 🟢 Yes — but governance discipline problem |

**Six fixable editorial issues + three structural blockers (B1, B3, B8).**

### 7.2 Consolidated scoring (against v10's 1.86/5 baseline)

| Criterion | v10 | Post-fix (2026-04-21) | Target for submission |
|---|---|---|---|
| Excellence | 2.0 | 2.3 | ≥3.5 |
| Implementation | 1.8 | 2.1 | ≥3.5 |
| Impact | 1.8 | 2.0 | ≥3.5 |
| Eligibility | 0.5 | 0.5 | **must be 5.0 (binary)** |
| **Weighted avg** | **1.86** | **~2.0** | **≥3.5** |

The post-fix improvement is **marginal (+0.14)** and concentrated in editorial consistency, not the structural gaps.

### 7.3 Recommendation

🛑 **DO NOT SUBMIT ON 2026-05-12.**

This aligns with the already-written internal governance doc `EIC_CONSORTIUM_STRUCTURE_2026-04-21.md` (v1.0, 2026-04-21), which explicitly says *"EIC Pathfinder Open 2026-05-12 submission DEFERRED to 2027–2028 cycle per super-strict peer review verdict (score 2.0/5.0, Do Not Submit)."* **The audit you requested confirms that deferral is correct.**

The pieces of the organism (CLAUDE.md, CONCEPT.md, CORRECTIONS) have not yet been harmonised — the project is telling itself two contradictory stories (submit / defer) across different files. **First action (next 48 h):** collapse the 2026-05-12 variant across all files to match the DEFERRED-to-2027 canon.

### 7.4 21-day action plan if submission is nevertheless attempted (not recommended)

1. **24 h:** reconcile all budget figures to one value (€3.5M recommended).
2. **48 h:** delete χ_Ze d=1.694 and CDATA R²=0.84 numbers from FCLC files; replace with "exploratory prediction, not validated".
3. **72 h:** fix the Zhu 2019 DOI; add TRUMPET / DataTools4Heart / FeatureCloud to competitor matrix with explicit differentiation.
4. **Week 1:** secure at least 2 signed LoI from EU partners in 2 different Member States (otherwise stop here — eligibility fail).
5. **Week 2:** implement a minimal PATE demo on MIMIC-IV, target ε ≤ 0.7, AUC ≥ 0.70; archive as preprint.
6. **Week 3:** independent mock review (paid service, ~€3k); iterate.

Probability of funding if all 6 steps completed in 21 days: ~8–12% (base rate of EIC Pathfinder is 5–8%; editorial fixes lift only marginally). Probability if deferred to 2027 with the same steps plus real CDATA/ABL-2 resolution + real pilot data: 20–30% — which is why the deferral recommendation is correct.

---

## Sources

- [EIC Pathfinder Open 2026 — European Innovation Council](https://eic.ec.europa.eu/eic-funding-opportunities/eic-pathfinder/eic-pathfinder-open-0_en)
- [DataTools4Heart CORDIS fact sheet (101057849)](https://cordis.europa.eu/project/id/101057849)
- [TRUMPET project — federated learning for healthcare](https://trumpetproject.eu/the-future-of-digital-healthcare-with-federated-learning/)
- Bonawitz et al., Practical Secure Aggregation, CCS 2017 — DOI 10.1145/3133956.3133982 (verified via Crossref)
- Zhu, Liu, Han "Deep Leakage from Gradients" — [arXiv:1906.08935](https://arxiv.org/abs/1906.08935); NeurIPS 2019 (the DOI 10.1145/3321705.3329809 currently cited in FCLC/EVIDENCE.md is **wrong** — it resolves to a different paper)
- Internal: `FCLC/CONCEPT.md` (v6.2, 2026-04-11), `CORRECTIONS_2026-04-22.md`, `EIC_CONSORTIUM_STRUCTURE_2026-04-21.md`, `MEGA_AUDIT_V3_2026-04-21/06_PEER_REVIEW_FCLC.md` (2.75/5.0 consolidated verdict)
