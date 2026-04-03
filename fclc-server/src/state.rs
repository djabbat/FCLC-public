use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use sqlx::PgPool;

use crate::models::{RoundResult, UpdatePayload};

/// Dimension of the global logistic-regression model.
/// Must match OmopRecord::FEATURE_DIM + 1 (bias) = 9.
pub const MODEL_DIM: usize = 9;

/// Maximum cumulative epsilon allowed per node before exclusion.
pub const EPSILON_TOTAL: f64 = 10.0;

/// Minimum number of nodes that must submit updates before auto-aggregation.
pub const MIN_NODES_FOR_AGGREGATION: usize = 2;

/// Shared application state passed to every Axum handler via `State<Arc<AppState>>`.
pub struct AppState {
    /// PostgreSQL connection pool.
    pub pool: PgPool,

    /// Current global model weights (128-dim logistic regression).
    pub global_model: Arc<RwLock<Vec<f64>>>,

    /// Current federated round number (starts at 0, increments after each aggregation).
    pub current_round: Arc<RwLock<u64>>,

    /// Pending updates for the current round: (node_id, payload).
    /// Cleared after each successful aggregation.
    pub pending_updates: Arc<RwLock<Vec<(Uuid, UpdatePayload)>>>,

    /// Cumulative DP epsilon spent per node across all rounds.
    pub node_budgets: Arc<RwLock<HashMap<Uuid, f64>>>,

    /// History of completed rounds (in-memory mirror for fast reads).
    pub round_history: Arc<RwLock<Vec<RoundResult>>>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            global_model: Arc::new(RwLock::new(vec![0.0_f64; MODEL_DIM])),
            current_round: Arc::new(RwLock::new(0u64)),
            pending_updates: Arc::new(RwLock::new(Vec::new())),
            node_budgets: Arc::new(RwLock::new(HashMap::new())),
            round_history: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
