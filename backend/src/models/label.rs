use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Label {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub color: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub issue_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLabelPayload {
    pub name: String,
    pub color: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdateLabelPayload {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetIssueLabelsPayload {
    pub labels: Vec<String>,
}
