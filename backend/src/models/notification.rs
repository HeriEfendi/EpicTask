use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: i64,
    pub recipient_id: i64,
    pub sender_id: Option<i64>,
    pub issue_id: Option<i64>,
    pub title: String,
    pub message: String,
    pub action_type: String, // MENTION, ASSIGNED, STATUS_CHANGE, AUTOMATION
    pub is_read: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub sender_name: Option<String>,
    pub issue_key: Option<String>,
}
