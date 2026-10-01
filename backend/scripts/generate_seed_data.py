#!/usr/bin/env python3
"""
EpicTask 1-Year Historical Dummy Data Generator (Jan 2026 - Oct 2026)
Generates rich, realistic project tracking data for future reporting & analytics testing.
"""

import sys
import random
from datetime import datetime, date, timedelta

# Users mapping
USERS = [
    {"id": 1, "name": "Sarah Jenkins", "role": "Tech Lead"},
    {"id": 2, "name": "Alex Morgan", "role": "Senior Fullstack"},
    {"id": 3, "name": "Elena Rostova", "role": "QA Specialist"},
    {"id": 4, "name": "David Chen", "role": "Product Owner"},
]

# Statuses mapping
STATUS_BACKLOG = 1     # TODO
STATUS_TODO = 2        # TODO
STATUS_IN_PROGRESS = 3 # IN_PROGRESS
STATUS_IN_REVIEW = 4   # IN_PROGRESS
STATUS_DONE = 5        # DONE

def escape_sql(s):
    if s is None:
        return "NULL"
    return "'" + str(s).replace("'", "''").replace("\\", "\\\\") + "'"

def format_dt(dt):
    return dt.strftime("%Y-%m-%d %H:%M:%S")

def format_date(d):
    return d.strftime("%Y-%m-%d")

def generate_sql():
    random.seed(42) # Deterministic for reproducibility
    sql_lines = []
    sql_lines.append("-- =========================================================")
    sql_lines.append("-- EpicTask 1-Year Reporting & Analytics Seed Data")
    sql_lines.append("-- Timespan: 2026-01-01 to 2026-10-01 (Current Date)")
    sql_lines.append("-- =========================================================")
    sql_lines.append("START TRANSACTION;\n")

    # 1. Update Workspace and Project initial creation timestamps to 2026-01-01
    sql_lines.append("UPDATE workspaces SET created_at = '2026-01-01 08:00:00' WHERE id = 1;")
    sql_lines.append("UPDATE projects SET created_at = '2026-01-01 08:30:00' WHERE id = 1;\n")

    # Epics definition spanning the year
    epics_data = [
        {
            "key_suffix": 10,
            "summary": "Cloud-Native Infrastructure, Docker & MariaDB High Availability",
            "desc": "Establish robust infrastructure, connection pooling, migration automation, and health checks.",
            "month_start": 1, "month_end": 2,
            "points": 21, "status": STATUS_DONE, "assignee": 1, "reporter": 4, "priority": "HIGHEST"
        },
        {
            "key_suffix": 11,
            "summary": "Enterprise Identity, JWT Auth & Granular RBAC Permissions",
            "desc": "Secure token management, workspace level roles (Owner, Admin, Member, Viewer) and bcrypt hashing.",
            "month_start": 1, "month_end": 3,
            "points": 34, "status": STATUS_DONE, "assignee": 2, "reporter": 1, "priority": "HIGH"
        },
        {
            "key_suffix": 12,
            "summary": "Interactive Kanban Engine & Multi-view Board Architecture",
            "desc": "Performant drag-and-drop board with WIP limits, column headers, and optimistic state updates.",
            "month_start": 2, "month_end": 4,
            "points": 34, "status": STATUS_DONE, "assignee": 2, "reporter": 4, "priority": "HIGHEST"
        },
        {
            "key_suffix": 13,
            "summary": "Gantt Timeline Engine, Milestone Markers & Critical Path",
            "desc": "SVG/Canvas based timeline visualizer with issue date dragging, milestone indicators, and link connectors.",
            "month_start": 3, "month_end": 5,
            "points": 21, "status": STATUS_DONE, "assignee": 3, "reporter": 1, "priority": "HIGH"
        },
        {
            "key_suffix": 14,
            "summary": "Real-time Axum WebSocket Broadcast Hub & Collaboration Presence",
            "desc": "Low-latency WebSocket channel for instant board synchronization, user active typing, and online avatars.",
            "month_start": 4, "month_end": 6,
            "points": 21, "status": STATUS_DONE, "assignee": 1, "reporter": 2, "priority": "HIGHEST"
        },
        {
            "key_suffix": 15,
            "summary": "No-Code Automation Engine & Custom Workflow Guards",
            "desc": "IF-THEN rule engine supporting status cascade, automatic assignment, and transition validation rules.",
            "month_start": 5, "month_end": 7,
            "points": 34, "status": STATUS_DONE, "assignee": 2, "reporter": 4, "priority": "HIGH"
        },
        {
            "key_suffix": 16,
            "summary": "Cross-Platform Desktop Client (Tauri v2 Native Wrapper)",
            "desc": "Compiling Vue frontend into ultra-lightweight desktop binaries with OS tray, shortcuts, and offline store.",
            "month_start": 6, "month_end": 8,
            "points": 21, "status": STATUS_DONE, "assignee": 1, "reporter": 4, "priority": "HIGH"
        },
        {
            "key_suffix": 17,
            "summary": "Time Tracking, Worklogs & Resource Allocation Suite",
            "desc": "Comprehensive log-time dialog, remaining estimate calculations, and team productivity logs.",
            "month_start": 7, "month_end": 9,
            "points": 13, "status": STATUS_DONE, "assignee": 3, "reporter": 4, "priority": "MEDIUM"
        },
        {
            "key_suffix": 18,
            "summary": "Enterprise Reporting, Velocity Charts & Executive Dashboards",
            "desc": "Multi-dimensional reports for sprint velocity, cumulative flow, cycle time, and timesheet exports.",
            "month_start": 9, "month_end": 10,
            "points": 34, "status": STATUS_IN_PROGRESS, "assignee": 2, "reporter": 4, "priority": "HIGHEST"
        }
    ]

    epic_records = []
    # Store inserted issues for references: (issue_id, key, summary, issue_type, status_id, assignee_id, created_dt, due_dt)
    all_issues = []
    time_logs = []
    activity_logs = []
    comments = []
    issue_links = []

    current_issue_id = 9 # Existing count is 9

    # Generate Epics
    for ep in epics_data:
        current_issue_id += 1
        key = f"EPIC-{current_issue_id}"
        created_dt = datetime(2026, ep["month_start"], random.randint(2, 6), random.randint(8, 11), random.randint(10, 50))
        start_d = date(2026, ep["month_start"], 8)
        due_d = date(2026, min(ep["month_end"], 10), min(25, 28))
        updated_dt = datetime(2026, min(ep["month_end"], 10), random.randint(18, 26), 17, 30)

        ep_rec = {
            "id": current_issue_id,
            "epic_id": None,
            "parent_id": None,
            "key": key,
            "summary": ep["summary"],
            "desc": ep["desc"],
            "issue_type": "EPIC",
            "status_id": ep["status"],
            "priority": ep["priority"],
            "assignee_id": ep["assignee"],
            "reporter_id": ep["reporter"],
            "points": ep["points"],
            "start_date": start_d,
            "due_date": due_d,
            "created_at": created_dt,
            "updated_at": updated_dt
        }
        epic_records.append(ep_rec)
        all_issues.append(ep_rec)

    # Monthly theme definitions for granular Stories, Tasks, Bugs, and Subtasks
    monthly_themes = [
        # Month 1: Jan 2026
        {
            "month": 1, "epic_idx": 0, "sprint": "Sprint 1 & 2 (Jan 2026)",
            "items": [
                ("STORY", "Database Connection Pooling & SQLx Migration Pipeline", 8, "HIGHEST", 1, 4, 3, 14, STATUS_DONE),
                ("TASK", "Configure MariaDB InnoDB utf8mb4 collation and foreign key cascades", 3, "HIGH", 2, 1, 5, 10, STATUS_DONE),
                ("TASK", "Docker Compose multi-stage build setup for Backend and MariaDB", 5, "MEDIUM", 1, 4, 8, 16, STATUS_DONE),
                ("BUG", "Fix deadlocks in simultaneous table creation on MariaDB restart", 0, "HIGHEST", 1, 3, 12, 14, STATUS_DONE),
                ("TASK", "Implement Axum health check and liveness probe endpoints", 2, "LOW", 2, 1, 15, 19, STATUS_DONE),
                ("STORY", "JWT Token Signing & RS256 Verification Middleware", 5, "HIGHEST", 2, 1, 15, 24, STATUS_DONE),
                ("TASK", "Workspace Membership RBAC Validator (Owner/Admin/Member)", 5, "HIGH", 1, 4, 18, 26, STATUS_DONE),
                ("BUG", "Token expiration returning 500 instead of standard 401 Unauthorized", 0, "HIGH", 2, 3, 22, 23, STATUS_DONE),
                ("TASK", "Seed basic test fixtures for integration test harness", 3, "LOW", 3, 1, 25, 29, STATUS_DONE),
            ]
        },
        # Month 2: Feb 2026
        {
            "month": 2, "epic_idx": 1, "sprint": "Sprint 3 & 4 (Feb 2026)",
            "items": [
                ("STORY", "Atomic Ticket Key Generation with MariaDB SELECT FOR UPDATE", 8, "HIGHEST", 1, 4, 1, 8, STATUS_DONE),
                ("TASK", "Issue CRUD RESTful endpoints with input validation", 5, "HIGH", 2, 1, 4, 12, STATUS_DONE),
                ("BUG", "Issue Key gap when transaction rolls back during creation", 0, "HIGH", 1, 3, 9, 11, STATUS_DONE),
                ("STORY", "Pinia Issue Store with reactive caching and optimistic updates", 5, "HIGH", 2, 4, 10, 18, STATUS_DONE),
                ("TASK", "TipTap Rich Text Editor Integration for Issue Descriptions", 5, "MEDIUM", 3, 4, 12, 20, STATUS_DONE),
                ("TASK", "Custom Markdown parser for bullet lists, tables and code blocks", 3, "LOW", 2, 1, 16, 22, STATUS_DONE),
                ("BUG", "TipTap toolbar styling clipping inside modal container", 0, "MEDIUM", 3, 3, 18, 19, STATUS_DONE),
                ("STORY", "Custom Status Creator and Ordering Position Controller", 5, "HIGH", 1, 4, 20, 27, STATUS_DONE),
                ("TASK", "Define Default Kanban Statuses (Backlog, Todo, In Progress, Review, Done)", 2, "LOW", 2, 1, 22, 26, STATUS_DONE),
            ]
        },
        # Month 3: Mar 2026
        {
            "month": 3, "epic_idx": 2, "sprint": "Sprint 5 & 6 (Mar 2026)",
            "items": [
                ("STORY", "Interactive Kanban Board Drag-and-Drop Column Reordering", 8, "HIGHEST", 2, 4, 2, 10, STATUS_DONE),
                ("TASK", "Vue Draggable integration with ghost element animations", 5, "HIGH", 2, 1, 4, 12, STATUS_DONE),
                ("BUG", "Card drop event misfiring on rapid mouse release between columns", 0, "HIGHEST", 3, 2, 8, 10, STATUS_DONE),
                ("STORY", "Workflow Transition Rule Validation Guard in Backend", 8, "HIGHEST", 1, 4, 9, 18, STATUS_DONE),
                ("TASK", "UI Toast Notification for disallowed workflow transitions", 3, "MEDIUM", 2, 1, 14, 20, STATUS_DONE),
                ("TASK", "Column WIP (Work In Progress) limit badges and visual indicators", 3, "LOW", 3, 4, 16, 22, STATUS_DONE),
                ("BUG", "Column issue count badge not updating after drag-and-drop", 0, "MEDIUM", 3, 3, 18, 19, STATUS_DONE),
                ("STORY", "Hierarchical Subtask Checklist with progress bar on cards", 5, "HIGH", 2, 4, 21, 28, STATUS_DONE),
                ("TASK", "Atomic Subtask Counter calculation on Issue Detail Modal", 3, "MEDIUM", 1, 2, 23, 29, STATUS_DONE),
                ("BUG", "Subtask completion percent division by zero when subtask count is 0", 0, "LOW", 3, 3, 25, 26, STATUS_DONE),
            ]
        },
        # Month 4: Apr 2026
        {
            "month": 4, "epic_idx": 3, "sprint": "Sprint 7 & 8 (Apr 2026)",
            "items": [
                ("STORY", "Timeline / Gantt View Horizon Engine with Responsive Day Zoom", 8, "HIGHEST", 3, 4, 2, 11, STATUS_DONE),
                ("TASK", "Interactive Date Range Picker with Start Date and Due Date constraints", 5, "HIGH", 2, 1, 5, 14, STATUS_DONE),
                ("BUG", "Timeline bar left offset shifting across UTC and local timezones", 0, "HIGHEST", 1, 3, 10, 12, STATUS_DONE),
                ("STORY", "SVG Dependency Connectors between blocked and blocking issues", 8, "HIGH", 2, 4, 11, 20, STATUS_DONE),
                ("TASK", "Issue Linking API (Blocks, Is Blocked By, Relates To)", 5, "HIGH", 1, 4, 13, 22, STATUS_DONE),
                ("TASK", "Detect and prevent circular dependency loops in issue links", 5, "HIGHEST", 1, 1, 16, 24, STATUS_DONE),
                ("BUG", "SVG connector arrows detaching when scrolling horizontally", 0, "MEDIUM", 3, 2, 21, 23, STATUS_DONE),
                ("STORY", "Milestone Markers and Today Indicator line on Timeline", 3, "LOW", 2, 4, 22, 28, STATUS_DONE),
                ("TASK", "Keyboard shortcuts for Timeline navigation (left/right pan, zoom)", 3, "LOW", 3, 1, 24, 29, STATUS_DONE),
            ]
        },
        # Month 5: May 2026
        {
            "month": 5, "epic_idx": 4, "sprint": "Sprint 9 & 10 (May 2026)",
            "items": [
                ("STORY", "Axum WebSocket Channel with Topic-Based Room Subscriptions", 8, "HIGHEST", 1, 4, 2, 10, STATUS_DONE),
                ("TASK", "Client-side WebSocket Auto-reconnection with Exponential Backoff", 5, "HIGH", 2, 1, 5, 13, STATUS_DONE),
                ("BUG", "Zombie WebSocket connection keeping database pool connection open", 0, "HIGHEST", 1, 3, 9, 11, STATUS_DONE),
                ("STORY", "Real-Time Issue Update Broadcast across connected browser clients", 8, "HIGHEST", 2, 4, 11, 19, STATUS_DONE),
                ("TASK", "Conflict Detection Protocol when two users edit the same issue", 5, "HIGH", 1, 4, 14, 22, STATUS_DONE),
                ("TASK", "Live Typing Indicators on Issue Detail Comments", 3, "LOW", 3, 2, 17, 24, STATUS_DONE),
                ("BUG", "Typing status stuck after user abruptly closes browser tab", 0, "MEDIUM", 3, 3, 20, 21, STATUS_DONE),
                ("STORY", "In-App Notification Bell System with unread counter badge", 5, "HIGH", 2, 4, 22, 29, STATUS_DONE),
                ("TASK", "Notification action types: MENTION, ASSIGNED, STATUS_CHANGE", 3, "MEDIUM", 1, 4, 24, 30, STATUS_DONE),
            ]
        },
        # Month 6: Jun 2026
        {
            "month": 6, "epic_idx": 5, "sprint": "Sprint 11 & 12 (Jun 2026)",
            "items": [
                ("STORY", "Visual No-Code Automation Rule Builder with IF-THEN triggers", 8, "HIGHEST", 2, 4, 2, 11, STATUS_DONE),
                ("TASK", "Rule Trigger: When Issue Status changes to DONE", 5, "HIGH", 1, 4, 5, 14, STATUS_DONE),
                ("TASK", "Rule Action: Cascade auto-complete to all nested Subtasks", 5, "HIGH", 1, 2, 8, 17, STATUS_DONE),
                ("BUG", "Cascade trigger creating infinite loop when subtask triggers parent", 0, "HIGHEST", 1, 3, 12, 14, STATUS_DONE),
                ("STORY", "Scheduled Background Task for 24h Due Date Alerts", 5, "HIGH", 1, 4, 15, 23, STATUS_DONE),
                ("TASK", "Rule Action: Reassign issue to original Reporter on review reject", 3, "MEDIUM", 2, 4, 18, 25, STATUS_DONE),
                ("BUG", "Due Date alert sending duplicate notifications on every minute tick", 0, "HIGHEST", 3, 3, 21, 22, STATUS_DONE),
                ("STORY", "Activity Stream Audit Log recorder for compliance tracking", 5, "MEDIUM", 3, 4, 23, 29, STATUS_DONE),
                ("TASK", "Filter Activity Stream by Project, User, and Date Range", 3, "LOW", 2, 1, 25, 30, STATUS_DONE),
            ]
        },
        # Month 7: Jul 2026
        {
            "month": 7, "epic_idx": 6, "sprint": "Sprint 13 & 14 (Jul 2026)",
            "items": [
                ("STORY", "Tauri v2 Core Setup & Linux/macOS App Bundle Configurations", 8, "HIGHEST", 1, 4, 2, 12, STATUS_DONE),
                ("TASK", "Native OS Window Controls and Frameless Glass Titlebar", 5, "HIGH", 2, 1, 6, 15, STATUS_DONE),
                ("TASK", "Desktop System Tray with Quick Issue Creation and Status Glance", 5, "HIGH", 1, 4, 10, 19, STATUS_DONE),
                ("BUG", "System tray menu not opening on Wayland Linux desktops", 0, "HIGH", 3, 1, 14, 16, STATUS_DONE),
                ("STORY", "Offline-First SQLite Cache Layer for Desktop App", 8, "HIGH", 2, 1, 15, 24, STATUS_DONE),
                ("TASK", "Delta Synchronization Protocol when network reconnects", 5, "HIGH", 1, 2, 18, 26, STATUS_DONE),
                ("BUG", "Offline edits overwriting newer cloud changes without conflict prompt", 0, "HIGHEST", 3, 3, 22, 24, STATUS_DONE),
                ("STORY", "Native Desktop Notifications via Tauri Plugin Notification", 3, "MEDIUM", 3, 4, 25, 30, STATUS_DONE),
                ("TASK", "Global Desktop Keyboard Shortcut (Ctrl+Alt+T) to summon Quick Task", 3, "LOW", 2, 4, 26, 31, STATUS_IN_REVIEW),
            ]
        },
        # Month 8: Aug 2026
        {
            "month": 8, "epic_idx": 7, "sprint": "Sprint 15 & 16 (Aug 2026)",
            "items": [
                ("STORY", "Time Tracking Engine: Log Work Dialog with Time Spent & Estimates", 8, "HIGHEST", 3, 4, 2, 10, STATUS_DONE),
                ("TASK", "Parse human duration strings (e.g. 2h 30m, 1d 4h, 45m)", 3, "HIGH", 2, 1, 5, 12, STATUS_DONE),
                ("BUG", "Time log parser crash on non-ASCII whitespace characters", 0, "MEDIUM", 3, 3, 9, 11, STATUS_DONE),
                ("STORY", "Timesheet Summary View by User, Sprint, and Date Range", 8, "HIGH", 2, 4, 11, 20, STATUS_DONE),
                ("TASK", "Time Log CSV & Excel Export Engine for billing and payroll", 5, "MEDIUM", 3, 4, 14, 23, STATUS_DONE),
                ("TASK", "Sprint Burndown Chart Data Aggregator (Story Points vs Days)", 5, "HIGH", 1, 4, 17, 25, STATUS_DONE),
                ("BUG", "Burndown chart displaying negative remaining points on scope additions", 0, "HIGH", 3, 2, 22, 24, STATUS_DONE),
                ("STORY", "List View Inline Cell Editing with keyboard navigation (Tab/Enter)", 5, "HIGH", 2, 4, 23, 30, STATUS_DONE),
                ("TASK", "List View Column Sorting and Multi-Criteria Filtering", 5, "MEDIUM", 3, 1, 25, 31, STATUS_IN_REVIEW),
            ]
        },
        # Month 9: Sep 2026
        {
            "month": 9, "epic_idx": 8, "sprint": "Sprint 17 & 18 (Sep 2026)",
            "items": [
                ("STORY", "Sprint Velocity Engine: Points Committed vs Completed over 6 Sprints", 8, "HIGHEST", 1, 4, 1, 10, STATUS_DONE),
                ("TASK", "Cumulative Flow Diagram (CFD) Data Pipeline with Daily Status Snapshots", 8, "HIGHEST", 2, 4, 5, 15, STATUS_DONE),
                ("BUG", "CFD band height jumping when tickets are moved backwards to To Do", 0, "HIGH", 3, 2, 12, 14, STATUS_DONE),
                ("STORY", "Cycle Time & Lead Time Scatter Plot with 85th Percentile Confidence", 8, "HIGH", 2, 4, 14, 24, STATUS_IN_REVIEW),
                ("TASK", "Team Workload & Capacity Heatmap across Active Sprints", 5, "MEDIUM", 3, 4, 17, 26, STATUS_IN_REVIEW),
                ("TASK", "Sprint Health Gauge: Completed, Scope Creep, and Blocked tickets", 5, "HIGH", 1, 4, 20, 28, STATUS_IN_PROGRESS),
                ("BUG", "Scope creep count including subtasks of existing stories incorrectly", 0, "MEDIUM", 3, 3, 24, 26, STATUS_DONE),
                ("STORY", "Epic Progress Roadmap with nested story completion percentage", 8, "HIGH", 2, 4, 22, 29, STATUS_IN_PROGRESS),
                ("TASK", "Automated PDF Report Generator for Weekly Stakeholder Status", 5, "LOW", 3, 4, 25, 30, STATUS_IN_PROGRESS),
            ]
        },
        # Month 10: Oct 2026 (Current Month - Up to Oct 1, 2026)
        {
            "month": 10, "epic_idx": 8, "sprint": "Sprint 19 (Oct 2026 - Active Sprint)",
            "items": [
                ("STORY", "Executive Analytics Dashboard with Real-Time KPI Cards", 8, "HIGHEST", 2, 4, 1, 8, STATUS_IN_PROGRESS),
                ("TASK", "Interactive Drilldown from Velocity Chart into filtered List View", 5, "HIGH", 1, 4, 1, 9, STATUS_TODO),
                ("TASK", "Sprint Forecasting Model based on Monte Carlo Simulation", 8, "MEDIUM", 1, 4, 1, 14, STATUS_BACKLOG),
                ("BUG", "Fix race condition in concurrent report aggregation queries", 0, "HIGHEST", 3, 1, 1, 3, STATUS_IN_PROGRESS),
                ("STORY", "Custom Report Builder with Drag-and-Drop Chart Widgets", 8, "HIGH", 2, 4, 1, 15, STATUS_TODO),
                ("TASK", "Scheduled Email/Slack digest for Sprint Milestone completions", 3, "LOW", 3, 4, 1, 12, STATUS_BACKLOG),
                ("TASK", "Export Board to High-Res PNG / SVG Vector Graphic", 3, "LOW", 2, 4, 1, 10, STATUS_BACKLOG),
            ]
        }
    ]

    # Insert items
    for theme in monthly_themes:
        month = theme["month"]
        epic_rec = epic_records[theme["epic_idx"]]
        epic_id = epic_rec["id"]

        for item in theme["items"]:
            current_issue_id += 1
            itype, summary, points, priority, assignee, reporter, day_start, day_due, status = item
            key = f"EPIC-{current_issue_id}"

            created_hour = random.randint(8, 16)
            created_min = random.randint(0, 59)
            created_dt = datetime(2026, month, min(day_start, 28), created_hour, created_min)
            start_d = date(2026, month, min(day_start, 28))
            
            # Due date
            if month == 10 and day_due > 1:
                due_d = date(2026, 10, min(day_due, 28))
            else:
                due_d = date(2026, month, min(day_due, 28))

            if status == STATUS_DONE:
                updated_dt = datetime(2026, month, min(day_due, 28), random.randint(14, 18), random.randint(0, 59))
            elif status in (STATUS_IN_PROGRESS, STATUS_IN_REVIEW):
                updated_dt = datetime(2026, month, min(max(day_start, 1), 28), 16, 30)
            else:
                updated_dt = created_dt

            desc = f"{summary}. Implemented as part of {theme['sprint']}. Critical for cross-platform and standard task workflow."

            rec = {
                "id": current_issue_id,
                "epic_id": epic_id,
                "parent_id": None,
                "key": key,
                "summary": summary,
                "desc": desc,
                "issue_type": itype,
                "status_id": status,
                "priority": priority,
                "assignee_id": assignee,
                "reporter_id": reporter,
                "points": points,
                "start_date": start_d,
                "due_date": due_d,
                "created_at": created_dt,
                "updated_at": updated_dt
            }
            all_issues.append(rec)

            # Generate 1-2 Subtasks for Stories
            if itype == "STORY":
                parent_id = current_issue_id
                subtask_defs = [
                    (f"API contracts & backend logic for {summary[:40]}", 2, assignee),
                    (f"Vue components & responsive design for {summary[:40]}", 2, 2 if assignee != 2 else 3),
                ]
                for sub_summary, sub_pts, sub_assignee in subtask_defs:
                    current_issue_id += 1
                    sub_key = f"EPIC-{current_issue_id}"
                    sub_status = status if status == STATUS_DONE else (STATUS_DONE if random.random() < 0.5 else STATUS_IN_PROGRESS)
                    sub_created = created_dt + timedelta(hours=random.randint(1, 4))
                    sub_rec = {
                        "id": current_issue_id,
                        "epic_id": epic_id,
                        "parent_id": parent_id,
                        "key": sub_key,
                        "summary": sub_summary,
                        "desc": f"Subtask breakdown under {key}",
                        "issue_type": "SUBTASK",
                        "status_id": sub_status,
                        "priority": priority,
                        "assignee_id": sub_assignee,
                        "reporter_id": assignee,
                        "points": sub_pts,
                        "start_date": start_d,
                        "due_date": due_d,
                        "created_at": sub_created,
                        "updated_at": updated_dt
                    }
                    all_issues.append(sub_rec)

    # Output Issues SQL
    sql_lines.append("-- 2. Insert Issues (Epics, Stories, Tasks, Bugs, Subtasks)")
    for iss in all_issues:
        parent_val = str(iss["parent_id"]) if iss["parent_id"] is not None else "NULL"
        epic_val = str(iss["epic_id"]) if iss["epic_id"] is not None else "NULL"
        sql_lines.append(
            f"INSERT INTO issues (id, project_id, parent_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position, created_at, updated_at) "
            f"VALUES ({iss['id']}, 1, {parent_val}, {epic_val}, {escape_sql(iss['key'])}, {escape_sql(iss['summary'])}, {escape_sql(iss['desc'])}, "
            f"{escape_sql(iss['issue_type'])}, {iss['status_id']}, {escape_sql(iss['priority'])}, {iss['assignee_id']}, {iss['reporter_id']}, {iss['points']}, "
            f"{escape_sql(format_date(iss['start_date']))}, {escape_sql(format_date(iss['due_date']))}, 0, "
            f"{escape_sql(format_dt(iss['created_at']))}, {escape_sql(format_dt(iss['updated_at']))});"
        )

    # 3. Generate Rich Time Logs across the entire year
    sql_lines.append("\n-- 3. Insert Time Logs (Worklogs from Jan 2026 to Oct 2026)")
    worklog_descriptions = [
        "Initial technical analysis and architecture design",
        "Implemented database schema and indexes",
        "Frontend component development and state management",
        "Unit testing, edge case handling and refactoring",
        "Code review revisions and transition validation",
        "Performance profiling and query optimization",
        "Bug investigation, root cause fix and regression test",
        "Cross-platform verification on Linux, macOS and Web",
        "API contract validation and schema synchronization",
        "Documentation, accessibility audits and styling polish"
    ]

    time_log_id = 1 # existing count is 1
    for iss in all_issues:
        if iss["issue_type"] in ("STORY", "TASK", "BUG", "SUBTASK"):
            # If Done, log between 2 and 4 worklog entries
            # If In Progress, log 1-2 entries
            num_logs = 0
            if iss["status_id"] == STATUS_DONE:
                num_logs = random.randint(2, 4)
            elif iss["status_id"] in (STATUS_IN_PROGRESS, STATUS_IN_REVIEW):
                num_logs = random.randint(1, 2)

            for i in range(num_logs):
                time_log_id += 1
                hours = random.choice([1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 7.5])
                seconds = int(hours * 3600)
                user_id = iss["assignee_id"] if (i % 2 == 0) else random.choice([1, 2, 3, 4])
                
                # log date within the task window
                delta_days = max(1, (iss["due_date"] - iss["start_date"]).days)
                log_day_offset = min(i, delta_days - 1)
                log_date = iss["start_date"] + timedelta(days=log_day_offset)
                log_dt = datetime(log_date.year, log_date.month, log_date.day, random.randint(10, 17), random.randint(0, 59))
                if log_dt > datetime(2026, 10, 1, 11, 0, 0):
                    log_dt = datetime(2026, 10, 1, 10, random.randint(10, 50))

                desc = random.choice(worklog_descriptions)
                sql_lines.append(
                    f"INSERT INTO time_logs (issue_id, user_id, time_spent_seconds, description, logged_at) "
                    f"VALUES ({iss['id']}, {user_id}, {seconds}, {escape_sql(desc)}, {escape_sql(format_dt(log_dt))});"
                )

    # 4. Generate Activity Logs (Audit Trail)
    sql_lines.append("\n-- 4. Insert Activity Logs (Full History Jan-Oct 2026)")
    for iss in all_issues:
        # Creation log
        k = iss["key"]
        sql_lines.append(
            f"INSERT INTO activity_logs (project_id, issue_id, user_id, action, details, created_at) "
            f"VALUES (1, {iss['id']}, {iss['reporter_id']}, 'CREATE_ISSUE', {escape_sql(f'Created ticket {k}')}, {escape_sql(format_dt(iss['created_at']))});"
        )
        # Move log if Done or In Progress
        if iss["status_id"] in (STATUS_IN_PROGRESS, STATUS_IN_REVIEW, STATUS_DONE):
            move_dt = iss["created_at"] + timedelta(hours=random.randint(4, 18))
            status_name = "In Progress" if iss["status_id"] == STATUS_IN_PROGRESS else ("In Review" if iss["status_id"] == STATUS_IN_REVIEW else "Done")
            sql_lines.append(
                f"INSERT INTO activity_logs (project_id, issue_id, user_id, action, details, created_at) "
                f"VALUES (1, {iss['id']}, {iss['assignee_id']}, 'MOVE_ISSUE', {escape_sql(f'Moved {k} to {status_name}')}, {escape_sql(format_dt(move_dt))});"
            )

    # 5. Generate Comments & Mentions
    sql_lines.append("\n-- 5. Insert Comments & Team Collaborations")
    comment_templates = [
        "Implementation verified and ready for review. @sarah please take a look at the transition logic.",
        "Tested across Chrome, Firefox and desktop wrapper. All integration tests passed! @david",
        "Found a minor edge case with high concurrency, patch applied in the latest commit.",
        "Story points estimation adjusted after backlog refinement session. @alex",
        "Great work team! This meets all sprint acceptance criteria and PRD specifications.",
        "Performance metrics look exceptional: API response time is under 15ms."
    ]

    for iss in all_issues:
        if iss["issue_type"] in ("STORY", "TASK", "BUG") and random.random() < 0.45:
            c_user = random.choice([1, 2, 3, 4])
            c_text = random.choice(comment_templates)
            c_dt = iss["created_at"] + timedelta(days=1, hours=random.randint(1, 5))
            if c_dt > datetime(2026, 10, 1, 11, 0):
                c_dt = datetime(2026, 10, 1, 9, 30)
            sql_lines.append(
                f"INSERT INTO comments (issue_id, user_id, body, created_at) "
                f"VALUES ({iss['id']}, {c_user}, {escape_sql(c_text)}, {escape_sql(format_dt(c_dt))});"
            )

    # 6. Generate Issue Links (Blocks, Relates To)
    sql_lines.append("\n-- 6. Insert Issue Links & Dependencies")
    # Link some tasks to epics and between stories
    linked_pairs = [
        (10, 11, "RELATES_TO"),
        (11, 12, "BLOCKS"),
        (12, 13, "BLOCKS"),
        (13, 14, "RELATES_TO"),
        (14, 15, "BLOCKS"),
        (15, 16, "RELATES_TO"),
        (16, 17, "BLOCKS"),
        (17, 18, "BLOCKS"),
    ]
    for src, tgt, ltype in linked_pairs:
        sql_lines.append(
            f"INSERT INTO issue_links (source_issue_id, target_issue_id, link_type, created_at) "
            f"VALUES ({src}, {tgt}, '{ltype}', '2026-03-01 10:00:00');"
        )

    # 7. Update current_issue_counter on projects table
    sql_lines.append(f"\n-- 7. Sync current_issue_counter to {current_issue_id}")
    sql_lines.append(f"UPDATE projects SET current_issue_counter = {current_issue_id} WHERE id = 1;")

    sql_lines.append("\nCOMMIT;")
    return "\n".join(sql_lines)

if __name__ == "__main__":
    sql = generate_sql()
    with open("backend/scripts/seed_1year_history.sql", "w") as f:
        f.write(sql)
    print(f"Generated seed SQL file with {sql.count('INSERT INTO issues')} issues and {sql.count('INSERT INTO time_logs')} time logs.")
