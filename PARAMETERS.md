# FCLC — PARAMS: System Parameters Reference

## Part 1: Differential Privacy Parameters

| Parameter | Symbol | Default | Range | Unit | Description |
|-----------|--------|---------|-------|------|-------------|
| `dp_epsilon_per_round` | ε | 2.0 | 0.1 – 10.0 | — | DP privacy budget per training round (Gaussian mechanism) |
| `dp_delta` | δ | 1e-5 | 1e-8 – 1e-4 | — | Failure probability; must be ≪ 1/dataset_size |
| `dp_epsilon_total` | ε_total | 10.0 | ε_per_round – 100.0 | — | Lifetime privacy budget per node; node excluded when exceeded |
| `dp_noise_multiplier` | σ_mult | auto | — | — | σ = sensitivity × √(2 ln(1.25/δ)) / ε; derived, not set directly |
| `dp_sensitivity` | Δf | 1.0 | 0.1 – 5.0 | — | L2 sensitivity of gradient (= max_grad_norm after clipping) |
| `dp_accounting` | — | basic | {basic, Rényi, moments} | — | Composition rule for multi-round budget tracking (current: linear/basic; Rényi DP is TODO) |

**Notes:**
- At ε=2.0/round: budget exhausted after 5 rounds (standard); with Rényi DP subsampling at rate q, effective ε ≈ 0.2/round → ~50 rounds
- δ < 1/n_patients is required for meaningful privacy guarantee
- ε < 1.0: strong privacy, significant utility loss; ε = 2.0–8.0: practical medical FL range

---

## Part 2: Gradient Clipping

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `max_grad_norm` | 1.0 | 0.1 – 10.0 | Maximum L2 norm of gradient before DP noise injection |
| `clip_strategy` | `per_sample` | {`per_sample`, `global`} | Per-sample clipping (standard for DP-SGD); global for speed |

---

## Part 3: Federated Aggregation — FedProx

| Parameter | Symbol | Default | Range | Description |
|-----------|--------|---------|-------|-------------|
| `fedprox_mu` | μ | 0.1 | 0.0 – 1.0 | Proximal penalty coefficient; 0 = FedAvg; higher = closer to global model |
| `local_epochs` | E | 3 | 1 – 20 | Local training epochs per round at each node |
| `local_batch_size` | B | 32 | 8 – 256 | Mini-batch size for local SGD |
| `local_learning_rate` | η_local | 0.01 | 1e-4 – 0.1 | Learning rate for local optimizer (SGD) |
| `global_learning_rate` | η_global | 1.0 | 0.1 – 2.0 | Server-side aggregation learning rate (FedAvg: 1.0) |
| `participation_fraction` | C | 1.0 | 0.1 – 1.0 | Fraction of nodes participating each round (C=1.0: all nodes) |

---

## Part 4: Byzantine Robustness — Krum

| Parameter | Symbol | Default | Range | Description |
|-----------|--------|---------|-------|-------------|
| `krum_byzantine_fraction` | f_ratio | 0.25 | 0.0 – 0.33 | Maximum assumed fraction of Byzantine nodes: f = ⌊f_ratio × n⌋ |
| `krum_select_m` | m | auto | 1 – n | Number of updates to retain: m = n − 2f; computed from f |
| `krum_distance_metric` | — | `l2` | {`l2`, `cosine`} | Distance metric for neighbor score computation |

**Constraint:** For Krum to guarantee robustness: f < n/2 (and typically f < n/4 in practice).

---

## Part 5: Shapley Value Scoring

| Parameter | Symbol | Default | Range | Description |
|-----------|--------|---------|-------|-------------|
| `shapley_mc_samples` | M | 150 | 50 – 500 | Monte Carlo permutation samples; trade-off: accuracy vs. compute |
| `shapley_validation_fraction` | — | 0.2 | 0.05 – 0.5 | Fraction of orchestrator data used as held-out validation set |
| `shapley_metric` | — | `AUC` | {`AUC`, `F1`, `accuracy`} | Performance metric for Shapley marginal contribution calculation |
| `shapley_baseline` | v(∅) | 0.5 | 0.0 – 1.0 | Value of empty coalition (AUC of random classifier = 0.5) |
| `shapley_smoothing` | α | 0.3 | 0.0 – 1.0 | EMA smoothing of scores across rounds: score_t = α×score_{t-1} + (1-α)×raw |

**Compute estimate:** O(n² × M) marginal evaluations; at n=10 nodes, M=150: ~1,500 evaluations × AUC time ≈ 10 min/round.

---

## Part 6: Secure Aggregation (SecAgg+)

| Parameter | Default | Description |
|-----------|---------|-------------|
| `secagg_threshold` | ⌈2n/3⌉ | Minimum surviving nodes for reconstruction to succeed |
| `secagg_key_size` | 256 | Pairwise mask key size (bits) |
| `secagg_dropout_tolerance` | ⌊n/3⌋ | Maximum nodes that can drop without aborting round |

---

## Part 7: Privacy / De-identification

| Parameter | Default | Description |
|-----------|---------|-------------|
| `k_anonymity_k` | 5 | Minimum equivalence class size; records in groups < k are suppressed |
| `age_bin_width` | 5 | Age generalization bucket width (years) |
| `rare_dx_threshold` | 5 | Diagnosis code count below which it is mapped to "other" |
| `dob_precision` | `year` | Date-of-birth precision retained after de-identification |
| `suppression_columns` | name, MRN, address, exact_dob | Fields removed entirely before any processing |

---

## Part 8: Model Architecture (Logistic Regression baseline)

| Parameter | Default | Range | Description |
|-----------|---------|-------|-------------|
| `model_type` | `logistic` | {`logistic`, `mlp`} | Global model architecture |
| `input_dim` | 9 | 9 – 4096 | OMOP feature dimensions: 8 clinical features + 1 bias (OmopRecord::FEATURE_DIM + 1) |
| `output_dim` | 1 | 1 – n_classes | Prediction targets (binary: 1 = adverse outcome) |
| `mlp_hidden_layers` | [64, 32] | — | Hidden layer sizes if model_type = `mlp` |
| `regularization` | `l2` | {`l1`, `l2`, `none`} | Regularization applied during local training |
| `lambda_reg` | 0.001 | 0.0 – 0.1 | Regularization coefficient |

---

## Part 9: Round / Protocol Timing

| Parameter | Default | Description |
|-----------|---------|-------------|
| `round_timeout_sec` | 3600 | Time before orchestrator closes the collection window for a round |
| `min_nodes_per_round` | 3 | Minimum participating nodes to proceed with aggregation |
| `max_rounds` | 50 | Hard stop on training rounds (budget and goal dependent) |
| `round_interval_sec` | 86400 | Minimum gap between rounds (24 h default for async clinics) |

---

## Part 10: Database / Persistence

| Parameter | Default | Description |
|-----------|---------|-------------|
| `db_pool_size` | 10 | PostgreSQL connection pool size (sqlx) |
| `model_checkpoint_rounds` | 5 | Save global model weights every N rounds |
| `score_history_retain_rounds` | 100 | How many rounds of Shapley scores to retain in DB |

---

## Part 11: Node Registration & Access Control

| Parameter | Default | Description |
|-----------|---------|-------------|
| `min_contribution_score` | 0.05 | Minimum Shapley score over last 3 rounds to retain access |
| `probation_rounds` | 3 | Rounds below threshold before automatic suspension |
| `max_budget_fraction_per_round` | 0.5 | A single node cannot spend more than 50% of remaining budget in one round |

---

## Part 12: Parameter Sensitivity Summary

| Parameter | Impact on Privacy | Impact on Accuracy | Impact on Compute |
|-----------|------------------|-------------------|-------------------|
| ε ↑ | Weaker | Higher | No change |
| σ (noise) ↑ | Stronger | Lower | No change |
| M (Shapley) ↑ | No change | Scores more accurate | O(M) linear |
| f_ratio ↑ | No change | Lower (fewer updates) | No change |
| μ (FedProx) ↑ | No change | More stable non-IID | No change |
| k ↑ | Stronger | Lower (more suppression) | No change |
| E (local epochs) ↑ | Weaker (more data seen) | Higher | O(E) linear |
