use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::*;

// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

// Query parameters for pagination
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

// Health check endpoint
async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
}

// Participant routes
async fn list_participants(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Participant>>, ApiError> {
    let participants = Participant::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(participants))
}

async fn get_participant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Participant>, ApiError> {
    let participant = Participant::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(participant))
}

async fn create_participant(
    State(state): State<AppState>,
    Json(payload): Json<CreateParticipant>,
) -> Result<Json<Participant>, ApiError> {
    let participant = Participant::create(&state.db_pool, payload).await?;
    Ok(Json(participant))
}

async fn update_participant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateParticipant>,
) -> Result<Json<Participant>, ApiError> {
    let participant = Participant::update(&state.db_pool, &id, payload).await?;
    Ok(Json(participant))
}

async fn delete_participant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Participant::delete(&state.db_pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Node routes
async fn list_nodes(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Node>>, ApiError> {
    let nodes = Node::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(nodes))
}

async fn get_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Node>, ApiError> {
    let node = Node::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(node))
}

async fn create_node(
    State(state): State<AppState>,
    Json(payload): Json<CreateNode>,
) -> Result<Json<Node>, ApiError> {
    let node = Node::create(&state.db_pool, payload).await?;
    Ok(Json(node))
}

async fn update_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNode>,
) -> Result<Json<Node>, ApiError> {
    let node = Node::update(&state.db_pool, &id, payload).await?;
    Ok(Json(node))
}

async fn delete_node(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Node::delete(&state.db_pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Model routes
async fn list_models(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Model>>, ApiError> {
    let models = Model::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(models))
}

async fn get_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Model>, ApiError> {
    let model = Model::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(model))
}

async fn create_model(
    State(state): State<AppState>,
    Json(payload): Json<CreateModel>,
) -> Result<Json<Model>, ApiError> {
    let model = Model::create(&state.db_pool, payload).await?;
    Ok(Json(model))
}

async fn update_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateModel>,
) -> Result<Json<Model>, ApiError> {
    let model = Model::update(&state.db_pool, &id, payload).await?;
    Ok(Json(model))
}

async fn delete_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Model::delete(&state.db_pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// TrainingRound routes
async fn list_training_rounds(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<TrainingRound>>, ApiError> {
    let rounds = TrainingRound::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(rounds))
}

async fn get_training_round(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<TrainingRound>, ApiError> {
    let round = TrainingRound::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(round))
}

async fn create_training_round(
    State(state): State<AppState>,
    Json(payload): Json<CreateTrainingRound>,
) -> Result<Json<TrainingRound>, ApiError> {
    let round = TrainingRound::create(&state.db_pool, payload).await?;
    Ok(Json(round))
}

async fn update_training_round(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTrainingRound>,
) -> Result<Json<TrainingRound>, ApiError> {
    let round = TrainingRound::update(&state.db_pool, &id, payload).await?;
    Ok(Json(round))
}

async fn delete_training_round(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    TrainingRound::delete(&state.db_pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ModelUpdate routes
async fn list_model_updates(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<ModelUpdate>>, ApiError> {
    let updates = ModelUpdate::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(updates))
}

async fn get_model_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ModelUpdate>, ApiError> {
    let update = ModelUpdate::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(update))
}

async fn create_model_update(
    State(state): State<AppState>,
    Json(payload): Json<CreateModelUpdate>,
) -> Result<Json<ModelUpdate>, ApiError> {
    let update = ModelUpdate::create(&state.db_pool, payload).await?;
    Ok(Json(update))
}

async fn delete_model_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    ModelUpdate::delete(&state.db_pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Contribution routes
async fn list_contributions(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Contribution>>, ApiError> {
    let contributions = Contribution::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(contributions))
}

async fn get_contribution(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Contribution>, ApiError> {
    let contribution = Contribution::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(contribution))
}

async fn create_contribution(
    State(state): State<AppState>,
    Json(payload): Json<CreateContribution>,
) -> Result<Json<Contribution>, ApiError> {
    let contribution = Contribution::create(&state.db_pool, payload).await?;
    Ok(Json(contribution))
}

// PrivacyBudget routes
async fn list_privacy_budgets(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<PrivacyBudget>>, ApiError> {
    let budgets = PrivacyBudget::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(budgets))
}

async fn get_privacy_budget(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PrivacyBudget>, ApiError> {
    let budget = PrivacyBudget::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(budget))
}

async fn create_privacy_budget(
    State(state): State<AppState>,
    Json(payload): Json<CreatePrivacyBudget>,
) -> Result<Json<PrivacyBudget>, ApiError> {
    let budget = PrivacyBudget::create(&state.db_pool, payload).await?;
    Ok(Json(budget))
}

async fn update_privacy_budget(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePrivacyBudget>,
) -> Result<Json<PrivacyBudget>, ApiError> {
    let budget = PrivacyBudget::update(&state.db_pool, &id, payload).await?;
    Ok(Json(budget))
}

// SecureAggregationSession routes
async fn list_secure_sessions(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<SecureAggregationSession>>, ApiError> {
    let sessions = SecureAggregationSession::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(sessions))
}

async fn get_secure_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SecureAggregationSession>, ApiError> {
    let session = SecureAggregationSession::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(session))
}

async fn create_secure_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateSecureAggregationSession>,
) -> Result<Json<SecureAggregationSession>, ApiError> {
    let session = SecureAggregationSession::create(&state.db_pool, payload).await?;
    Ok(Json(session))
}

async fn update_secure_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSecureAggregationSession>,
) -> Result<Json<SecureAggregationSession>, ApiError> {
    let session = SecureAggregationSession::update(&state.db_pool, &id, payload).await?;
    Ok(Json(session))
}

// Counter routes
async fn list_counters(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Counter>>, ApiError> {
    let counters = Counter::list(&state.db_pool, pagination.page, pagination.per_page).await?;
    Ok(Json(counters))
}

async fn get_counter(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Counter>, ApiError> {
    let counter = Counter::find_by_id(&state.db_pool, &id).await?;
    Ok(Json(counter))
}

async fn create_counter(
    State(state): State<AppState>,
    Json(payload): Json<CreateCounter>,
) -> Result<Json<Counter>, ApiError> {
    let counter = Counter::create(&state.db_pool, payload).await?;
    Ok(Json(counter))
}

async fn update_counter(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCounter>,
) -> Result<Json<Counter>, ApiError> {
    let counter = Counter::update(&state.db_pool, &id, payload).await?;
    Ok(Json(counter))
}

async fn delete_counter(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    Counter::delete(&state.db_pool, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Special endpoints
async fn compute_l_tissue() -> Result<Json<HashMap<String, f64>>, ApiError> {
    // Placeholder for MCOA L_tissue computation
    Ok(Json(HashMap::from([("status".to_string(), 0.0)])))
}

async fn start_secure_aggregation(
    State(state): State<AppState>,
    Json(payload): Json<SecureAggregationRequest>,
) -> Result<Json<SecureAggregationResponse>, ApiError> {
    // Placeholder for SecAgg+ protocol initiation
    let session = SecureAggregationSession::create(&state.db_pool, CreateSecureAggregationSession {
        round_id: payload.round_id,
        node_ids: payload.node_ids,
        threshold: payload.threshold,
        status: "initialized".to_string(),
        public_keys: vec![],
        shares: vec![],
        aggregated_result: None,
        dropout_recovered: false,
    }).await?;
    
    Ok(Json(SecureAggregationResponse {
        session_id: session.id,
        status: session.status,
        required_participants: session.node_ids.len(),
        threshold: session.threshold,
    }))
}

// Main router
pub fn app_router(db_pool: PgPool) -> Router {
    let state = AppState::new(db_pool);
    
    Router::new()
        .route("/health", get(health))
        // Participants
        .route("/participants", get(list_participants).post(create_participant))
        .route("/participants/:id", get(get_participant).put(update_participant).delete(delete_participant))
        // Nodes
        .route("/nodes", get(list_nodes).post(create_node))
        .route("/nodes/:id", get(get_node).put(update_node).delete(delete_node))
        // Models
        .route("/models", get(list_models).post(create_model))
        .route("/models/:id", get(get_model).put(update_model).delete(delete_model))
        // TrainingRounds
        .route("/training_rounds", get(list_training_rounds).post(create_training_round))
        .route("/training_rounds/:id", get(get_training_round).put(update_training_round).delete(delete_training_round))
        // ModelUpdates
        .route("/model_updates", get(list_model_updates).post(create_model_update))
        .route("/model_updates/:id", get(get_model_update).delete(delete_model_update))
        // Contributions
        .route("/contributions", get(list_contributions).post(create_contribution))
        .route("/contributions/:id", get(get_contribution))
        // PrivacyBudgets
        .route("/privacy_budgets", get(list_privacy_budgets).post(create_privacy_budget))
        .route("/privacy_budgets/:id", get(get_privacy_budget).put(update_privacy_budget))
        // SecureAggregationSessions
        .route("/secure_sessions", get(list_secure_sessions).post(create_secure_session))
        .route("/secure_sessions/:id", get(get_secure_session).put(update_secure_session))
        // Counters
        .route("/counters", get(list_counters).post(create_counter))
        .route("/counters/:id", get(get_counter).put(update_counter).delete(delete_counter))
        // Special endpoints
        .route("/mcoa/l_tissue", get(compute_l_tissue))
        .route("/secagg/start", post(start_secure_aggregation))
        .with_state(state)
}