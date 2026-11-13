use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Version {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub release_date: Option<NaiveDate>,
    pub status: String, // UNRELEASED, RELEASED, ARCHIVED
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    // Roll-up statistics
    pub total_issues: Option<i64>,
    pub todo_issues: Option<i64>,
    pub in_progress_issues: Option<i64>,
    pub done_issues: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionPayload {
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub release_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVersionPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub release_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseVersionPayload {
    pub release_date: Option<NaiveDate>,
    pub move_unreleased_to_version_id: Option<i64>, // None means leave unreleased issues
}

