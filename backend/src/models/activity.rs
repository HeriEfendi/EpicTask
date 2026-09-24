use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActivityLog {
    pub id: i64,
    pub project_id: i64,
    pub issue_id: Option<i64>,
    pub user_id: Option<i64>,
    pub action: String,
    pub details: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub user_name: Option<String>,
    pub issue_key: Option<String>,
}
