use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i64,
    pub issue_id: i64,
    pub user_id: Option<i64>,
    pub body: String,
    pub created_at: Option<DateTime<Utc>>,
    pub user_name: Option<String>,
    pub user_avatar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentPayload {
    pub body: String,
}
