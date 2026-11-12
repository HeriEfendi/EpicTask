use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Sprint {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub goal: Option<String>,
    pub status: String, // PLANNED, ACTIVE, COMPLETED
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    // Roll-up statistics
    pub total_issues: Option<i64>,
    pub todo_points: Option<i64>,
    pub in_progress_points: Option<i64>,
    pub done_points: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSprintPayload {
    pub name: String,
    pub goal: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSprintPayload {
    pub name: Option<String>,
    pub goal: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct StartSprintPayload {
    pub name: Option<String>,
    pub goal: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteSprintPayload {
    pub move_incomplete_to_sprint_id: Option<i64>, // None means move to Product Backlog
}

#[derive(Debug, Deserialize)]
pub struct MoveSprintIssuesPayload {
    pub issue_ids: Vec<i64>,
    pub target_sprint_id: Option<i64>, // None means move to Backlog
}
