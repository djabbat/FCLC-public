use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use uuid::Uuid;
use validator::Validate;

use crate::error::ApiError;

// Participant - клиника или фармацевтическая компания
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Participant {
    pub id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub participant_type: String, // "clinic", "pharma", "research"
    pub contact_email: String,
    pub country_code: String,
    pub reputation_score: f64,
    pub contribution_credits: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateParticipant {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub participant_type: String,
    #[validate(email)]
    pub contact_email: String,
    pub country_code: String,
    pub reputation_score: Option<f64>,
    pub contribution_credits: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateParticipant {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub participant_type: Option<String>,
    #[validate(email)]
    pub contact_email: Option<String>,
    pub country_code: Option<String>,
    pub reputation_score: Option<f64>,
    pub contribution_credits: Option<f64>,
    pub is_active: Option<bool>,
}

impl Participant {
    pub async fn list(
        pool: &PgPool,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Self>, ApiError> {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).max(1).min(100);
        let offset = ((page - 1) * per_page) as i64;
        
        let participants = sqlx::query_as(
            r#"
            SELECT * FROM participants
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(participants)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Self, ApiError> {
        let participant = sqlx::query_as(
            r#"
            SELECT * FROM participants
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(participant)
    }
    
    pub async fn create(pool: &PgPool, data: CreateParticipant) -> Result<Self, ApiError> {
        data.validate()?;
        
        let participant = sqlx::query_as(
            r#"
            INSERT INTO participants (
                name, participant_type, contact_email, country_code,
                reputation_score, contribution_credits, is_active
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.participant_type)
        .bind(data.contact_email)
        .bind(data.country_code)
        .bind(data.reputation_score.unwrap_or(1.0))
        .bind(data.contribution_credits.unwrap_or(0.0))
        .bind(data.is_active.unwrap_or(true))
        .fetch_one(pool)
        .await?;
        
        Ok(participant)
    }
    
    pub async fn update(pool: &PgPool, id: &Uuid, data: UpdateParticipant) -> Result<Self, ApiError> {
        if let Some(email) = &data.contact_email {
            validator::validate_email(email)?;
        }
        
        let participant = sqlx::query_as(
            r#"
            UPDATE participants
            SET 
                name = COALESCE($2, name),
                participant_type = COALESCE($3, participant_type),
                contact_email = COALESCE($4, contact_email),
                country_code = COALESCE($5, country_code),
                reputation_score = COALESCE($6, reputation_score),
                contribution_credits = COALESCE($7, contribution_credits),
                is_active = COALESCE($8, is_active),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.name)
        .bind(data.participant_type)
        .bind(data.contact_email)
        .bind(data.country_code)
        .bind(data.reputation_score)
        .bind(data.contribution_credits)
        .bind(data.is_active)
        .fetch_one(pool)
        .await?;
        
        Ok(participant)
    }
    
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            DELETE FROM participants
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// Node - локальный узел участника
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Node {
    pub id: Uuid,
    pub participant_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub node_url: String,
    pub api_key_hash: String,
    pub is_online: bool,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub software_version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateNode {
    pub participant_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(url)]
    pub node_url: String,
    pub api_key: String,
    pub software_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateNode {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(url)]
    pub node_url: Option<String>,
    pub api_key: Option<String>,
    pub is_online: Option<bool>,
    pub software_version: Option<String>,
}

impl Node {
    pub async fn list(
        pool: &PgPool,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Self>, ApiError> {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).max(1).min(100);
        let offset = ((page - 1) * per_page) as i64;
        
        let nodes = sqlx::query_as(
            r#"
            SELECT * FROM nodes
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(nodes)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Self, ApiError> {
        let node = sqlx::query_as(
            r#"
            SELECT * FROM nodes
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(node)
    }
    
    pub async fn create(pool: &PgPool, data: CreateNode) -> Result<Self, ApiError> {
        data.validate()?;
        
        // Hash the API key
        let api_key_hash = format!("{:x}", md5::compute(&data.api_key));
        
        let node = sqlx::query_as(
            r#"
            INSERT INTO nodes (
                participant_id, name, node_url, api_key_hash,
                software_version, is_online
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(data.participant_id)
        .bind(data.name)
        .bind(data.node_url)
        .bind(api_key_hash)
        .bind(data.software_version)
        .bind(false)
        .fetch_one(pool)
        .await?;
        
        Ok(node)
    }
    
    pub async fn update(pool: &PgPool, id: &Uuid, data: UpdateNode) -> Result<Self, ApiError> {
        let api_key_hash = data.api_key.as_ref().map(|key| format!("{:x}", md5::compute(key)));
        
        let node = sqlx::query_as(
            r#"
            UPDATE nodes
            SET 
                name = COALESCE($2, name),
                node_url = COALESCE($3, node_url),
                api_key_hash = COALESCE($4, api_key_hash),
                is_online = COALESCE($5, is_online),
                software_version = COALESCE($6, software_version),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.name)
        .bind(data.node_url)
        .bind(api_key_hash)
        .bind(data.is_online)
        .bind(data.software_version)
        .fetch_one(pool)
        .await?;
        
        Ok(node)
    }
    
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            DELETE FROM nodes
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// Model - глобальная модель для обучения
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Model {
    pub id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub model_type: String, // "classification", "regression", "survival"
    pub architecture: serde_json::Value,
    pub current_version: i32,
    pub is_active: bool,
    pub total_rounds: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateModel {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub model_type: String,
    pub architecture: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateModel {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub model_type: Option<String>,
    pub architecture: Option<serde_json::Value>,
    pub current_version: Option<i32>,
    pub is_active: Option<bool>,
    pub total_rounds: Option<i32>,
}

impl Model {
    pub async fn list(
        pool: &PgPool,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Self>, ApiError> {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).max(1).min(100);
        let offset = ((page - 1) * per_page) as i64;
        
        let models = sqlx::query_as(
            r#"
            SELECT * FROM models
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(models)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Self, ApiError> {
        let model = sqlx::query_as(
            r#"
            SELECT * FROM models
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(model)
    }
    
    pub async fn create(pool: &PgPool, data: CreateModel) -> Result<Self, ApiError> {
        data.validate()?;
        
        let model = sqlx::query_as(
            r#"
            INSERT INTO models (name, model_type, architecture, current_version, total_rounds)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.model_type)
        .bind(data.architecture)
        .bind(1)
        .bind(0)
        .fetch_one(pool)
        .await?;
        
        Ok(model)
    }
    
    pub async fn update(pool: &PgPool, id: &Uuid, data: UpdateModel) -> Result<Self, ApiError> {
        let model = sqlx::query_as(
            r#"
            UPDATE models
            SET 
                name = COALESCE($2, name),
                model_type = COALESCE($3, model_type),
                architecture = COALESCE($4, architecture),
                current_version = COALESCE($5, current_version),
                is_active = COALESCE($6, is_active),
                total_rounds = COALESCE($7, total_rounds),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.name)
        .bind(data.model_type)
        .bind(data.architecture)
        .bind(data.current_version)
        .bind(data.is_active)
        .bind(data.total_rounds)
        .fetch_one(pool)
        .await?;
        
        Ok(model)
    }
    
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            DELETE FROM models
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// TrainingRound - раунд федеративного обучения
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TrainingRound {
    pub id: Uuid,
    pub model_id: Uuid,
    pub round_number: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String, // "pending", "running", "completed", "failed"
    pub aggregation_method: String, // "fedavg", "fedprox", "krum"
    pub dp_epsilon_used: f64,
    pub dp_delta_used: f64,
    pub min_updates_required: i32,
    pub actual_updates: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTrainingRound {
    pub model_id: Uuid,
    pub round_number: i32,
    pub aggregation_method: String,
    pub dp_epsilon_used: f64,
    pub dp_delta_used: f64,
    pub min_updates_required: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTrainingRound {
    pub status: Option<String>,
    pub end_time: Option<DateTime<Utc>>,
    pub actual_updates: Option<i32>,
}

impl TrainingRound {
    pub async fn list(
        pool: &PgPool,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Self>, ApiError> {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).max(1).min(100);
        let offset = ((page - 1) * per_page) as i64;
        
        let rounds = sqlx::query_as(
            r#"
            SELECT * FROM training_rounds
            ORDER BY round_number DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(rounds)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Self, ApiError> {
        let round = sqlx::query_as(
            r#"
            SELECT * FROM training_rounds
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(round)
    }
    
    pub async fn create(pool: &PgPool, data: CreateTrainingRound) -> Result<Self, ApiError> {
        data.validate()?;
        
        let round = sqlx::query_as(
            r#"
            INSERT INTO training_rounds (
                model_id, round_number, aggregation_method,
                dp_epsilon_used, dp_delta_used, min_updates_required,
                status, start_time
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            RETURNING *
            "#
        )
        .bind(data.model_id)
        .bind(data.round_number)
        .bind(data.aggregation_method)
        .bind(data.dp_epsilon_used)
        .bind(data.dp_delta_used)
        .bind(data.min_updates_required)
        .bind("pending")
        .fetch_one(pool)
        .await?;
        
        Ok(round)
    }
    
    pub async fn update(pool: &PgPool, id: &Uuid, data: UpdateTrainingRound) -> Result<Self, ApiError> {
        let round = sqlx::query_as(
            r#"
            UPDATE training_rounds
            SET 
                status = COALESCE($2, status),
                end_time = COALESCE($3, end_time),
                actual_updates = COALESCE($4, actual_updates)
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.status)
        .bind(data.end_time)
        .bind(data.actual_updates)
        .fetch_one(pool)
        .await?;
        
        Ok(round)
    }
    
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            DELETE FROM training_rounds
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// ModelUpdate - обновление модели от узла
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ModelUpdate {
    pub id: Uuid,
    pub round_id: Uuid,
    pub node_id: Uuid,
    pub update_hash: String,
    pub gradient_norm: f64,
    pub num_samples: i32,
    pub computation_time_sec: f64,
    pub dp_noise_added: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateModelUpdate {
    pub round_id: Uuid,
    pub node_id: Uuid,
    pub update_hash: String,
    pub gradient_norm: f64,
    pub num_samples: i32,
    pub computation_time_sec: f64,
    pub dp_noise_added: bool,
}

impl ModelUpdate {
    pub async fn list(
        pool: &PgPool,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Self>, ApiError> {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).max(1).min(100);
        let offset = ((page - 1) * per_page) as i64;
        
        let updates = sqlx::query_as(
            r#"
            SELECT * FROM model_updates
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(updates)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Self, ApiError> {
        let update = sqlx::query_as(
            r#"
            SELECT * FROM model_updates
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(update)
    }
    
    pub async fn create(pool: &PgPool, data: CreateModelUpdate) -> Result<Self, ApiError> {
        data.validate()?;
        
        let update = sqlx::query_as(
            r#"
            INSERT INTO model_updates (
                round_id, node_id, update_hash, gradient_norm,
                num_samples, computation_time_sec, dp_noise_added
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(data.round_id)
        .bind(data.node_id)
        .bind(data.update_hash)
        .bind(data.gradient_norm)
        .bind(data.num_samples)
        .bind(data.computation_time_sec)
        .bind(data.dp_noise_added)
        .fetch_one(pool)
        .await?;
        
        Ok(update)
    }
    
    pub async fn delete(pool: &PgPool, id: &Uuid) -> Result<(), ApiError> {
        sqlx::query(
            r#"
            DELETE FROM model_updates
            WHERE id = $1
            "#
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// Contribution - вклад участника (Shapley value)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contribution {
    pub id: Uuid,
    pub participant_id: Uuid,
    pub round_id: Uuid,
    pub shapley_value: f64,
    pub marginal_contribution: f64,
    pub monte_carlo_samples: i32,
    pub computation_time_sec: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateContribution {
    pub participant_id: Uuid,
    pub round_id: Uuid,
    pub shapley_value: f64,
    pub marginal_contribution: f64,
    pub monte_carlo_samples: i32,
    pub computation_time_sec: f64,
}

impl Contribution {
    pub async fn list(
        pool: &PgPool,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Self>, ApiError> {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).max(1).min(100);
        let offset = ((page - 1) * per_page) as i64;
        
        let contributions = sqlx::query_as(
            r#"
            SELECT * FROM contributions
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(contributions)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Self, ApiError> {
        let contribution = sqlx::query_as(
            r#"
            SELECT * FROM contributions
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(contribution)
    }
    
    pub async fn create(pool: &PgPool, data: CreateContribution) -> Result<Self, ApiError> {
        data.validate()?;
        
        let contribution = sqlx::query_as(
            r#"
            INSERT INTO contributions (
                participant_id, round_id, shapley_value,
                marginal_contribution, monte_carlo_samples,
                computation_time_sec
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(data.participant_id)
        .bind(data.round_id)
        .bind(data.shapley_value)
        .bind(data.marginal_contribution)
        .bind(data.monte_carlo_samples)
        .bind(data.computation_time_sec)
        .fetch_one(pool)
        .await?;
        
        Ok(contribution)
    }
}

// PrivacyBudget - отслеживание бюджета дифференциальной приватности
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PrivacyBudget {
    pub id: Uuid,
    pub node_id: Uuid,
    pub epsilon_total: f64,
    pub epsilon_used: f64,
    pub delta: f64,
    pub total_rounds: i32,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePrivacyBudget {
    pub node_id: Uuid,
    pub epsilon_total: f64,
    pub epsilon_used: f64,
    pub delta: f64,
    pub total_rounds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePrivacyBudget {
    pub epsilon_used: Option<f64>,
    pub total_rounds: Option<i32>,
}

impl PrivacyBudget {
    pub async fn list(
        pool: &PgPool,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Self>, ApiError> {
        let page = page.unwrap_or(1).max(1);
        let per_page = per_page.unwrap_or(20).max(1).min(100);
        let offset = ((page - 1) * per_page) as i64;
        
        let budgets = sqlx::query_as(
            r#"
            SELECT * FROM privacy_budgets
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(per_page as i64)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        Ok(budgets)
    }
    
    pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Self, ApiError> {
        let budget = sqlx::query_as(
            r#"
            SELECT * FROM privacy_budgets
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(budget)
    }
    
    pub async fn create(pool: &PgPool, data: CreatePrivacyBudget) -> Result<Self, ApiError> {
        data.validate()?;
        
        let budget = sqlx::query_as(
            r#"
            INSERT INTO privacy_budgets (
                node_id, epsilon_total, epsilon_used,
                delta, total_rounds
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(data.node_id)
        .bind(data.epsilon_total)
        .bind(data.epsilon_used)
        .bind(data.delta)
        .bind(data.total_rounds)
        .fetch_one(pool)
        .await?;
        
        Ok(budget)
    }
    
    pub async fn update(pool: &PgPool, id: &Uuid, data: UpdatePrivacyBudget) -> Result<Self, ApiError> {
        let budget = sqlx::query_as(
            r#"
            UPDATE privacy_budgets
            SET 
                epsilon_used = COALESCE($2, epsilon_used),
                total_rounds = COALESCE($3, total_rounds),
                last_updated = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.epsilon_used)
        .bind(data.total_rounds)
        .fetch_one(pool)
        .await?;
        
        Ok(budget)
    }
}

// SecureAggregationSession - сессия безопасной агрегации SecAgg+
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SecureAggregationSession {