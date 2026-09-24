use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i64,
    pub workspace_id: i64,
    pub name: String,
    pub key: String,
    pub project_type: String, // KANBAN or SCRUM
    pub current_issue_counter: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Status {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub category: String, // TODO, IN_PROGRESS, DONE
    pub position: i32,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkflowTransition {
    pub id: i64,
    pub project_id: i64,
    pub from_status_id: i64,
    pub to_status_id: i64,
    pub from_status_name: Option<String>,
    pub to_status_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectPayload {
    pub name: String,
    pub key: String,
    pub project_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStatusPayload {
    pub name: String,
    pub category: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransitionPayload {
    pub from_status_id: i64,
    pub to_status_id: i64,
}
