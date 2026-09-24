use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkspaceMember {
    pub workspace_id: i64,
    pub user_id: i64,
    pub role: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkspacePayload {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberPayload {
    pub email: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRolePayload {
    pub role: String,
}
