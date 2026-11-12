use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Component {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub lead_user_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,

    // Joined fields
    pub lead_name: Option<String>,
    pub issue_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateComponentPayload {
    pub name: String,
    pub description: Option<String>,
    pub lead_user_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateComponentPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub lead_user_id: Option<i64>,
}

