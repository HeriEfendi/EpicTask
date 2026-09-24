use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Issue {
    pub id: i64,
    pub project_id: i64,
    pub parent_id: Option<i64>,
    pub epic_id: Option<i64>,
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub issue_type: String, // EPIC, TASK, STORY, SUBTASK, BUG
    pub status_id: i64,
    pub priority: Option<String>, // LOWEST, LOW, MEDIUM, HIGH, HIGHEST
    pub assignee_id: Option<i64>,
    pub reporter_id: Option<i64>,
    pub story_points: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub position: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    // Joined fields
    pub status_name: Option<String>,
    pub status_category: Option<String>,
    pub status_color: Option<String>,
    pub assignee_name: Option<String>,
    pub assignee_avatar: Option<String>,
    pub reporter_name: Option<String>,
    pub epic_summary: Option<String>,
    pub parent_key: Option<String>,
    pub parent_summary: Option<String>,
    pub subtask_count: Option<i64>,
    pub subtask_done_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIssuePayload {
    pub summary: String,
    pub description: Option<String>,
    pub issue_type: Option<String>,
    pub status_id: Option<i64>,
    pub priority: Option<String>,
    pub assignee_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub epic_id: Option<i64>,
    pub story_points: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIssuePayload {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub issue_type: Option<String>,
    pub status_id: Option<i64>,
    pub priority: Option<String>,
    pub assignee_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub epic_id: Option<i64>,
    pub story_points: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct MoveIssuePayload {
    pub target_status_id: i64,
    pub target_position: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IssueLink {
    pub id: i64,
    pub source_issue_id: i64,
    pub target_issue_id: i64,
    pub link_type: String, // BLOCKS, IS_BLOCKED_BY, RELATES_TO
    pub created_at: Option<DateTime<Utc>>,
    pub target_key: Option<String>,
    pub target_summary: Option<String>,
    pub target_status_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIssueLinkPayload {
    pub target_issue_id: i64,
    pub link_type: String,
}
