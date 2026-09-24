use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TimeLog {
    pub id: i64,
    pub issue_id: i64,
    pub user_id: Option<i64>,
    pub time_spent_seconds: i64,
    pub description: Option<String>,
    pub logged_at: Option<DateTime<Utc>>,
    pub user_name: Option<String>,
    pub user_avatar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTimeLogPayload {
    pub time_spent_seconds: i64,
    pub description: Option<String>,
}
