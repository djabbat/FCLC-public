
# CLAUDE.md — FCLC (Federated Clinical Learning Cooperative)

## Project Identity

**FCLC** — Federated Clinical Learning Cooperative
**Purpose:** Privacy-preserving federated learning platform for clinical AI
**Version:** 0.1.0-alpha | **CONCEPT v6.0** | **EIC Pathfinder deadline: 12 May 2026**
**Location:** `~/Desktop/FCLC/`
**Repo:** private `djabbat/FCLC`

---

## Source of Truth

**CONCEPT.md is the authoritative document.**
All code, parameters, and documentation must match CONCEPT.md.
If there is a conflict between code and CONCEPT.md, fix the code.

---

## Architecture

```
fclc-core/        Rust library — DP engine, Shapley, FedProx/Krum, OMOP schema, de-identification
fclc-node/        Rust binary + egui GUI — local clinic node (3 tabs: Dashboard/Data/Training)
fclc-server/      Rust binary + Axum REST API — central orchestrator + PostgreSQL
fclc-web/         Elixir/Phoenix LiveView — web dashboard (✅ scaffolded)

docs/             README.md, additional documentation
```

---

## Critical Privacy Constraints (from CONCEPT.md)

### 5-Layer Privacy Stack — must be preserved in all code
1. Direct identifier removal: name → ∅, exact_dob → year, MRN → ∅, address → ∅
2. Quasi-identifier generalization: age → 5-yr bin, rare Dx → "other" if count < 5
3. k-anonymity check: k ≥ 5; groups below threshold suppressed
4. DP-SGD (Gaussian mechanism): ε=2.0/round, δ=1e-5, σ derived automatically
5. SecAgg+: orchestrator never sees individual node updates

### Key parameter values (must not change without biological/privacy justification)
| Parameter | Value | Source |
|-----------|-------|--------|
| DP ε per round | 2.0 | CONCEPT.md §Privacy |
| DP δ | 1e-5 | CONCEPT.md §Privacy |
| DP total budget | 10.0 | CONCEPT.md §Privacy |
| Gradient max_norm | 1.0 | CONCEPT.md §DP-SGD |
| FedProx μ | 0.1 | CONCEPT.md §Aggregation |
| Krum Byzantine fraction | 0.25 | CONCEPT.md §Robustness |
| Shapley MC samples | 150 | CONCEPT.md §Scoring |
| k-anonymity k | 5 | CONCEPT.md §Privacy |

Full parameter reference: `PARAMETERS.md`

---

## Build & Run

```bash
# Full build
bash run.sh build

# Run tests
bash run.sh test

# Run server (orchestrator) — requires PostgreSQL running
bash run.sh server

# Run node GUI
bash run.sh node

# Run web dashboard (Phoenix) — requires mix deps
bash run.sh web
```

---

## Workspace Structure

```toml
# Cargo.toml
[workspace]
members = ["fclc-core", "fclc-node", "fclc-server"]
```

- `fclc-core`: pure library, no binary; depends on serde, rand, rand_distr, uuid, thiserror
- `fclc-node`: binary + egui; entry point `src/main.rs` → calls `app::FclcNodeApp`
- `fclc-server`: binary + Axum; entry point `src/main.rs`; requires DATABASE_URL env var
- `fclc-web`: Phoenix/Elixir, separate from Cargo workspace (not in Cargo.toml)

---

## REST API Contract (fclc-server endpoints)

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/nodes/register` | Register clinic node |
| POST | `/api/nodes/:id/update` | Submit gradient update |
| GET | `/api/model/current` | Download global model |
| GET | `/api/rounds` | List rounds |
| GET | `/api/rounds/:id` | Round details |
| GET | `/api/nodes/:id/score` | Shapley score history |
| GET | `/api/metrics` | Aggregated metrics (for fclc-web) |

---

## What NOT to change without user approval
1. CONCEPT.md
2. Privacy layer implementation (de-identification, DP, k-anonymity)
3. DP ε/δ default values
4. Krum Byzantine tolerance threshold (0.25)
5. Shapley MC sample count (150)

---

## DeepSeek Rule
Route ALL non-trivial text tasks through DeepSeek API (`~/.aim_env → DEEPSEEK_API_KEY`).
Models: `deepseek-chat` (fast) · `deepseek-reasoner` (complex reasoning)

---

## Git Push Rule
- Push to private only: `djabbat/FCLC` (rule updated 2026-04-02: no public repos)
