use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    handlers::{
        activities, auth, automation, comments, issues, notifications, projects, seed, time_logs,
        workspaces,
    },
    ws::ws_handler,
    AppState,
};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // WebSocket endpoint
        .route("/ws", get(ws_handler))
        // Auth routes
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/me", get(auth::me))
        .route("/api/users", get(auth::list_users))
        // Workspace routes
        .route("/api/workspaces", get(workspaces::list_workspaces).post(workspaces::create_workspace))
        .route("/api/workspaces/:id", get(workspaces::get_workspace))
        .route("/api/workspaces/:id/members", post(workspaces::add_member))
        .route("/api/workspaces/:id/members/:uid", put(workspaces::update_member_role))
        // Project routes
        .route("/api/workspaces/:wid/projects", get(projects::list_projects).post(projects::create_project))
        .route("/api/projects/:id", get(projects::get_project))
        .route("/api/projects/:id/statuses", get(projects::list_statuses).post(projects::create_status))
        .route("/api/projects/:id/transitions", get(projects::list_transitions).post(projects::create_transition))
        .route("/api/projects/:id/transitions/:tid", delete(projects::delete_transition))
        // Issue routes
        .route("/api/projects/:pid/issues", get(issues::list_project_issues).post(issues::create_issue))
        .route("/api/issues/:id", get(issues::get_issue).put(issues::update_issue).delete(issues::delete_issue))
        .route("/api/issues/:id/move", post(issues::move_issue))
        .route("/api/issues/:id/links", post(issues::create_issue_link))
        // Time Log routes
        .route("/api/issues/:id/timelogs", get(time_logs::list_time_logs).post(time_logs::create_time_log))
        // Comment routes
        .route("/api/issues/:id/comments", get(comments::list_comments).post(comments::create_comment))
        // Automation routes
        .route("/api/projects/:pid/automations", get(automation::list_automations).post(automation::create_automation))
        .route("/api/projects/:pid/automations/:id", put(automation::update_automation).delete(automation::delete_automation))
        // Notification routes
        .route("/api/notifications", get(notifications::list_notifications))
        .route("/api/notifications/:id/read", put(notifications::mark_as_read))
        .route("/api/notifications/read-all", post(notifications::mark_all_as_read))
        // Activity Log routes
        .route("/api/projects/:pid/activities", get(activities::list_activities))
        // Seed database
        .route("/api/seed", post(seed::seed_database))
        .layer(cors)
        .with_state(state)
}
