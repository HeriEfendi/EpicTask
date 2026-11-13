use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IssueWatcher {
    pub user_id: i64,
    pub full_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AddWatcherPayload {
    pub user_id: Option<i64>,
}

