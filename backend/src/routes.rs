use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    handlers::{
        activities, auth, automation, comments, components, issues, labels, notifications,
        projects, reports, seed, sprints, time_logs, versions, workspaces,
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
        // Sprint routes (Scrum Framework)
        .route("/api/projects/:pid/sprints", get(sprints::list_project_sprints).post(sprints::create_sprint))
        .route("/api/projects/:pid/sprints/move-issues", post(sprints::move_issues_to_sprint))
        .route("/api/sprints/:id", put(sprints::update_sprint).delete(sprints::delete_sprint))
        .route("/api/sprints/:id/start", post(sprints::start_sprint))
        .route("/api/sprints/:id/complete", post(sprints::complete_sprint))
        // Component routes (Project Subsystems)
        .route("/api/projects/:pid/components", get(components::list_project_components).post(components::create_component))
        .route("/api/components/:id", put(components::update_component).delete(components::delete_component))
        // Version routes (Releases & Versions)
        .route("/api/projects/:pid/versions", get(versions::list_project_versions).post(versions::create_version))
        .route("/api/versions/:id", put(versions::update_version).delete(versions::delete_version))
        .route("/api/versions/:id/release", post(versions::release_version))
        .route("/api/versions/:id/archive", post(versions::archive_version))
        // Label routes (Jira Ad-hoc Tagging)
        .route("/api/projects/:pid/labels", get(labels::list_project_labels).post(labels::create_label))
        .route("/api/labels/:id", delete(labels::delete_label))
        .route("/api/issues/:id/labels", get(labels::get_issue_labels).post(labels::set_issue_labels))
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
        // Reports & Analytics routes
        .route("/api/projects/:pid/reports/overview", get(reports::get_overview))
        .route("/api/projects/:pid/reports/velocity", get(reports::get_velocity))
        .route("/api/projects/:pid/reports/cumulative-flow", get(reports::get_cumulative_flow))
        .route("/api/projects/:pid/reports/timesheet", get(reports::get_timesheet))
        .route("/api/projects/:pid/reports/epic-progress", get(reports::get_epic_progress))
        // Seed database
        .route("/api/seed", post(seed::seed_database))
        .layer(cors)
        .with_state(state)
}
