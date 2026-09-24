use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AutomationRule {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub action_type: String,
    pub action_config: serde_json::Value,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAutomationRulePayload {
    pub name: String,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub action_type: String,
    pub action_config: serde_json::Value,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAutomationRulePayload {
    pub name: Option<String>,
    pub trigger_type: Option<String>,
    pub trigger_config: Option<serde_json::Value>,
    pub action_type: Option<String>,
    pub action_config: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}
