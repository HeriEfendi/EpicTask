use clap::{Args, Parser, Subcommand};
use sqlx::MySqlPool;

pub mod components;
pub mod doctor;
pub mod issues;
pub mod labels;
pub mod projects;
pub mod sprints;
pub mod users;
pub mod versions;
pub mod watchers;

#[derive(Parser, Debug)]
#[command(name = "epictask-cli")]
#[command(author = "EpicTask Team")]
#[command(version = "0.1.3")]
#[command(about = "EpicTask CLI — Developer & AI Agent Command Line Interface", long_about = None)]
pub struct Cli {
    #[arg(long, global = true, help = "Output raw JSON for agent or script parsing")]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage & inspect projects and workflows
    Projects(ProjectCommand),

    /// Manage & inspect issues (tickets)
    Issues(IssueCommand),

    /// Manage Scrum sprints and backlog
    Sprints(SprintCommand),

    /// Manage project components / subsystems
    Components(ComponentCommand),

    /// Manage software releases and versions
    Versions(VersionCommand),

    /// Manage labels and ad-hoc tagging
    Labels(LabelCommand),

    /// Manage issue watchers (collaborator observers)
    Watchers(WatcherCommand),

    /// Manage users and generate development tokens
    Users(UserCommand),

    /// Run full system & Jira features diagnostic audit
    Doctor,

    /// Seed development database with rich test data
    Seed,
}

#[derive(Args, Debug)]
pub struct ProjectCommand {
    #[command(subcommand)]
    pub action: ProjectAction,
}

#[derive(Subcommand, Debug)]
pub enum ProjectAction {
    /// List all projects
    List,
    /// List workflow statuses for a project
    Statuses {
        /// Project ID or Key (e.g. 1 or PROJ)
        project: String,
    },
}

#[derive(Args, Debug)]
pub struct IssueCommand {
    #[command(subcommand)]
    pub action: IssueAction,
}

#[derive(Subcommand, Debug)]
pub enum IssueAction {
    /// List issues in a project with optional filters
    List {
        /// Project ID or Key
        project: String,
        #[arg(long, help = "Filter by status ID, name, or category (e.g. 'TODO', 'In Progress', 'Done')")]
        status: Option<String>,
        #[arg(long, help = "Filter by assignee user ID")]
        assignee: Option<i64>,
        #[arg(long, help = "Filter by issue type (TASK, STORY, EPIC, BUG)")]
        r#type: Option<String>,
        #[arg(long, help = "Filter by priority (LOWEST, LOW, MEDIUM, HIGH, HIGHEST)")]
        priority: Option<String>,
        #[arg(long, help = "Filter by resolution (e.g. 'Fixed', 'Done', or 'unresolved')")]
        resolution: Option<String>,
        #[arg(long, help = "Filter by sprint ID")]
        sprint: Option<i64>,
        #[arg(long, help = "Filter by component ID")]
        component: Option<i64>,
        #[arg(long, help = "Filter by label name")]
        label: Option<String>,
        #[arg(short, long, help = "Search text in summary or key")]
        search: Option<String>,
        #[arg(long, help = "Limit results (default: 50)")]
        limit: Option<i64>,
        #[arg(long, help = "Offset for pagination")]
        offset: Option<i64>,
    },
    /// Inspect details of an issue (including subtasks & watchers)
    Get {
        /// Issue ID or Key (e.g. 42 or PROJ-15)
        issue: String,
    },
    /// Create a new issue
    Create {
        /// Project ID or Key
        project: String,
        #[arg(short, long)]
        summary: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short = 't', long)]
        r#type: Option<String>,
        #[arg(short = 'p', long)]
        priority: Option<String>,
        #[arg(short = 'a', long)]
        assignee: Option<i64>,
        #[arg(long)]
        sprint: Option<i64>,
        #[arg(long)]
        component: Option<i64>,
        #[arg(long)]
        fix_version: Option<i64>,
        #[arg(long)]
        points: Option<i32>,
        #[arg(long, help = "Due date in YYYY-MM-DD format")]
        due: Option<String>,
        #[arg(long, help = "Comma-separated labels (e.g. 'backend,api')")]
        labels: Option<String>,
    },
    /// Move issue to another workflow status (with optional resolution)
    Move {
        /// Issue ID or Key
        issue: String,
        #[arg(short, long, help = "Target status ID or name (e.g. 'Done' or 3)")]
        status: String,
        #[arg(short, long, help = "Resolution reason (e.g. 'Fixed', 'Won\'t Fix', 'Duplicate')")]
        resolution: Option<String>,
    },
    /// Delete an issue
    Delete {
        /// Issue ID or Key
        issue: String,
    },
}

#[derive(Args, Debug)]
pub struct SprintCommand {
    #[command(subcommand)]
    pub action: SprintAction,
}

#[derive(Subcommand, Debug)]
pub enum SprintAction {
    /// List sprints with 3-pill story points (To Do / In Progress / Done)
    List {
        project: String,
    },
    /// Create a new planned sprint
    Create {
        project: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        goal: Option<String>,
        #[arg(long, help = "Start date YYYY-MM-DD")]
        start: Option<String>,
        #[arg(long, help = "End date YYYY-MM-DD")]
        end: Option<String>,
    },
    /// Start a planned sprint
    Start {
        #[arg(help = "Sprint ID")]
        sprint: i64,
        #[arg(short, long, default_value = "2", help = "Duration in weeks")]
        weeks: i64,
        #[arg(short, long)]
        goal: Option<String>,
    },
    /// Complete an active sprint and rollover incomplete issues
    Complete {
        #[arg(help = "Sprint ID")]
        sprint: i64,
        #[arg(short, long, help = "Target sprint ID for incomplete issues (omit to move to Backlog)")]
        move_to: Option<i64>,
    },
}

#[derive(Args, Debug)]
pub struct ComponentCommand {
    #[command(subcommand)]
    pub action: ComponentAction,
}

#[derive(Subcommand, Debug)]
pub enum ComponentAction {
    /// List all components in a project
    List {
        project: String,
    },
    /// Create a component
    Create {
        project: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long, help = "Component lead user ID")]
        lead: Option<i64>,
    },
    /// Delete a component
    Delete {
        #[arg(help = "Component ID")]
        component: i64,
    },
}

#[derive(Args, Debug)]
pub struct VersionCommand {
    #[command(subcommand)]
    pub action: VersionAction,
}

#[derive(Subcommand, Debug)]
pub enum VersionAction {
    /// List releases and versions with issue progress
    List {
        project: String,
    },
    /// Create a release version
    Create {
        project: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(long, help = "Target release date YYYY-MM-DD")]
        date: Option<String>,
    },
    /// Mark a version as released
    Release {
        #[arg(help = "Version ID")]
        version: i64,
        #[arg(short, long, help = "Target version ID to migrate unresolved issues")]
        move_to: Option<i64>,
    },
    /// Toggle archive status of a version
    Archive {
        #[arg(help = "Version ID")]
        version: i64,
    },
}

#[derive(Args, Debug)]
pub struct LabelCommand {
    #[command(subcommand)]
    pub action: LabelAction,
}

#[derive(Subcommand, Debug)]
pub enum LabelAction {
    /// List all labels in a project
    List {
        project: String,
    },
    /// Add a label to an issue
    Add {
        issue: String,
        #[arg(short, long)]
        label: String,
        #[arg(short, long, help = "Hex color (e.g. #6366f1)")]
        color: Option<String>,
    },
    /// Remove a label from an issue
    Remove {
        issue: String,
        #[arg(short, long)]
        label: String,
    },
}

#[derive(Args, Debug)]
pub struct WatcherCommand {
    #[command(subcommand)]
    pub action: WatcherAction,
}

#[derive(Subcommand, Debug)]
pub enum WatcherAction {
    /// List all watchers for an issue
    List {
        issue: String,
    },
    /// Add a user as watcher
    Add {
        issue: String,
        #[arg(short, long, help = "User ID")]
        user: i64,
    },
    /// Remove a user from watchers
    Remove {
        issue: String,
        #[arg(short, long, help = "User ID")]
        user: i64,
    },
}

#[derive(Args, Debug)]
pub struct UserCommand {
    #[command(subcommand)]
    pub action: UserAction,
}

#[derive(Subcommand, Debug)]
pub enum UserAction {
    /// List all users
    List,
    /// Generate a JWT development bearer token for testing APIs
    Token {
        /// User email or ID
        user: String,
    },
    /// View performance metrics for a specific user
    Performance {
        /// User email or ID
        user: String,
        #[arg(short, long, default_value = "1", help = "Project ID")]
        project: i64,
        #[arg(long, help = "Start date YYYY-MM-DD")]
        start: Option<String>,
        #[arg(long, help = "End date YYYY-MM-DD")]
        end: Option<String>,
    },
}

pub async fn run_cli(cli: Cli, pool: &MySqlPool, jwt_secret: &str) -> Result<(), String> {
    let json = cli.json;

    match cli.command {
        Commands::Projects(cmd) => match cmd.action {
            ProjectAction::List => projects::list_projects(pool, json).await,
            ProjectAction::Statuses { project } => projects::list_statuses(pool, &project, json).await,
        },
        Commands::Issues(cmd) => match cmd.action {
            IssueAction::List {
                project,
                status,
                assignee,
                r#type,
                priority,
                resolution,
                sprint,
                component,
                label,
                search,
                limit,
                offset,
            } => {
                let filters = issues::ListIssuesFilter {
                    status: status.as_deref(),
                    assignee_id: assignee,
                    issue_type: r#type.as_deref(),
                    priority: priority.as_deref(),
                    resolution: resolution.as_deref(),
                    sprint_id: sprint,
                    component_id: component,
                    label: label.as_deref(),
                    search: search.as_deref(),
                    limit,
                    offset,
                };
                issues::list_issues(pool, &project, filters, json).await
            }
            IssueAction::Get { issue } => issues::get_issue(pool, &issue, json).await,
            IssueAction::Create {
                project,
                summary,
                description,
                r#type,
                priority,
                assignee,
                sprint,
                component,
                fix_version,
                points,
                due,
                labels,
            } => {
                let label_vec = labels.map(|l| l.split(',').map(|s| s.trim().to_string()).collect());
                let input = issues::CreateIssueInput {
                    summary: &summary,
                    description: description.as_deref(),
                    issue_type: r#type.as_deref(),
                    priority: priority.as_deref(),
                    assignee_id: assignee,
                    sprint_id: sprint,
                    component_id: component,
                    fix_version_id: fix_version,
                    story_points: points,
                    due_date: due.as_deref(),
                    labels: label_vec,
                };
                issues::create_issue(pool, &project, input, json).await
            }
            IssueAction::Move {
                issue,
                status,
                resolution,
            } => issues::move_issue(pool, &issue, &status, resolution.as_deref(), json).await,
            IssueAction::Delete { issue } => issues::delete_issue(pool, &issue, json).await,
        },
        Commands::Sprints(cmd) => match cmd.action {
            SprintAction::List { project } => sprints::list_sprints(pool, &project, json).await,
            SprintAction::Create {
                project,
                name,
                goal,
                start,
                end,
            } => sprints::create_sprint(pool, &project, &name, goal.as_deref(), start.as_deref(), end.as_deref(), json).await,
            SprintAction::Start {
                sprint,
                weeks,
                goal,
            } => sprints::start_sprint(pool, sprint, Some(weeks), goal.as_deref(), json).await,
            SprintAction::Complete { sprint, move_to } => {
                sprints::complete_sprint(pool, sprint, move_to, json).await
            }
        },
        Commands::Components(cmd) => match cmd.action {
            ComponentAction::List { project } => components::list_components(pool, &project, json).await,
            ComponentAction::Create {
                project,
                name,
                description,
                lead,
            } => components::create_component(pool, &project, &name, description.as_deref(), lead, json).await,
            ComponentAction::Delete { component } => components::delete_component(pool, component, json).await,
        },
        Commands::Versions(cmd) => match cmd.action {
            VersionAction::List { project } => versions::list_versions(pool, &project, json).await,
            VersionAction::Create {
                project,
                name,
                description,
                date,
            } => versions::create_version(pool, &project, &name, description.as_deref(), date.as_deref(), json).await,
            VersionAction::Release { version, move_to } => {
                versions::release_version(pool, version, move_to, json).await
            }
            VersionAction::Archive { version } => versions::archive_version(pool, version, json).await,
        },
        Commands::Labels(cmd) => match cmd.action {
            LabelAction::List { project } => labels::list_labels(pool, &project, json).await,
            LabelAction::Add {
                issue,
                label,
                color,
            } => labels::add_label(pool, &issue, &label, color.as_deref(), json).await,
            LabelAction::Remove { issue, label } => labels::remove_label(pool, &issue, &label, json).await,
        },
        Commands::Watchers(cmd) => match cmd.action {
            WatcherAction::List { issue } => watchers::list_watchers(pool, &issue, json).await,
            WatcherAction::Add { issue, user } => watchers::add_watcher(pool, &issue, user, json).await,
            WatcherAction::Remove { issue, user } => watchers::remove_watcher(pool, &issue, user, json).await,
        },
        Commands::Users(cmd) => match cmd.action {
            UserAction::List => users::list_users(pool, json).await,
            UserAction::Token { user } => users::generate_token(pool, &user, jwt_secret, json).await,
            UserAction::Performance {
                user,
                project,
                start,
                end,
            } => users::show_user_performance(pool, &user, project, start, end, json).await,
        },
        Commands::Doctor => doctor::run_doctor(pool, json).await,
        Commands::Seed => {
            println!("🌱 Running database seed...");
            match crate::handlers::seed::run_seed(pool).await {
                Ok(_) => {
                    if json {
                        println!("{}", serde_json::json!({ "success": true }));
                    } else {
                        println!("✅ Database seeded successfully!");
                    }
                    Ok(())
                }
                Err(e) => Err(format!("Seed failed: {}", e)),
            }
        }
    }
}
