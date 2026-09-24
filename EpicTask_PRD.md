# PRODUCT REQUIREMENT DOCUMENT (PRD)

## Project Name: EpicTask (Jira Clone Alternative)

**Target Output:** Web Application & Cross-Platform Desktop App (Windows, macOS, Linux) via Tauri
**Tech Stack:** Vue 3 (Frontend), Rust / Axum / Diesel or SQLx (Backend), Tauri (Desktop Wrapper), mariadb (Database)

---

## 1. Executive Summary & Project Goal

EpicTask adalah platform manajemen proyek dan pelacakan tugas (task tracking) modern yang mengadopsi nilai inti dari Jira Core/Software. Fokus utama aplikasi ini adalah menyediakan fungsionalitas esensial Jira (Hierarki Tugas, Custom Workflows, Multi-View Board, dan No-Code Automation) dengan performa tinggi, UI/UX yang lebih intuitif, dan fleksibilitas deployment ganda (Web App dan Desktop App Native via Tauri).

---

## 2. Core Constraints & Architecture Requirements

1. Frontend (FE): Vue 3 menggunakan Composition API, Pinia (State Management), dan Tailwind CSS untuk styling.
2. Backend (BE): Rust menggunakan HTTP framework Axum dan ORM Diesel / SQLx untuk kecepatan tinggi dan keamanan alokasi memori (memory safety).
3. Desktop Wrapper: Tauri untuk melakukan kompilasi dari frontend Vue ke executable format (.exe, .dmg, .deb/.appimage) dengan footprint memori yang sangat kecil.
4. Hybrid Capability: Backend Rust didesain sebagai RESTful API dan WebSocket server. Frontend Vue terhubung via HTTP biasa (untuk versi Web) maupun via IPC/Tauri Commands (untuk versi Desktop).
5. Real-time Synchronization: Setiap mutasi data papan (board) wajib disiarkan secara instan ke seluruh klien yang terhubung melalui WebSocket.

---

## 3. Epics & Functional Features Breakdown

### EPIC 1: User, Workspace & Role Management (Fondasi)

- FR-1.1 Auth System: Registrasi, Login, dan Logout menggunakan JWT (JSON Web Tokens) untuk enkripsi sesi pada web, dan secure storage keyring untuk desktop client.
- FR-1.2 Workspace Management: Pengguna dapat membuat, keluar, atau bergabung ke beberapa ruang kerja (Workspaces) terpisah. Satu workspace mendukung kolaborasi multi-user.
- FR-1.3 Role-Based Access Control (RBAC): Sistem peran bawaan tingkat Workspace dan Proyek: Owner (Pemilik), Admin (Pengelola aturan/workflow), Member (Pengembang/Pelaksana tugas), dan Viewer (Hanya baca).

### EPIC 2: Project & Board Management (Fitur Unggulan)

- FR-2.1 Project Types: Kemampuan membuat proyek dengan template metodologi Kanban or Scrum. Setiap proyek memiliki kode unik (Project Key) sendiri.
- FR-2.2 Multi-View Board:
  - Kanban View: Kolom status interaktif dengan fitur Drag-and-Drop (menggunakan library Vue Draggable / @hello-pangea/dnd port).
  - Timeline/Gantt View: Diagram batang horizontal yang menunjukkan hubungan tanggal mulai (Start Date) dan tenggat (Due Date).
  - List View: Tampilan tabular seperti spreadsheet untuk pengeditan teks tugas secara instan.
- FR-2.3 Custom Workflows:
  - Admin dapat membuat status baru secara dinamis (Contoh: Under Review, QA Testing).
  - Admin dapat menentukan aturan transisi status (Workflow Transition Rule). Contoh: Tiket di kolom To Do tidak boleh langsung digeser ke Done sebelum melewati status In Progress.

### EPIC 3: Issue/Task Lifecycle (Fitur Utama - Pembuatan Tugas)

Setiap tugas (Issue) direpresentasikan sebagai objek terstruktur di database dengan aturan berikut:

- FR-3.1 Issue Hierarchy:
  - Epic: Penampung inisiatif besar berdurasi panjang (berwarna unik).
  - Task / Story: Tiket standar penampung pekerjaan utama.
  - Subtask: Tiket anak yang terikat langsung ke satu Parent Task dan tidak bisa berdiri mandiri.
- FR-3.2 Create Issue Form Details:
  - Summary (Title): String, maks 255 karakter (Wajib diisi).
  - Issue Key: Penomoran otomatis berformat [KEY]-[COUNTER] (Contoh: PROJ-101) yang digenerate secara atomic di backend.
  - Description: Rich text editor (menggunakan TipTap / Quill Vue).
  - Assignee & Reporter: Dropdown relasi user terdaftar pada proyek.
  - Priority: Enum (Highest, High, Medium, Low).
  - Dates: Start Date, Due Date, Created At, Updated At.
  - Estimation: Satuan Jam (Hours) atau Story Points (Angka Fibonacci).
  - Attachments: Maksimal file 10MB per unggahan (disimpan ke S3/MinIO atau Local Directory via Tauri FS API).
  - Issue Linking: Relasi ketergantungan antar tiket (Blocks, Is Blocked By, Relates To).

### EPIC 4: Execution & Tracking (Fitur Utama - Menjalankan Tugas)

- FR-4.1 Drag-and-Drop Workflow Execution: Menggeser kartu issue antar kolom otomatis memvalidasi aturan transisi workflow, mengubah status issue di database, dan memicu reload parsial via WebSocket ke user lain.
- FR-4.2 Time Tracking (Log Time): Input data manual konsumsi waktu kerja nyata. Field: Time Spent dan Remaining Estimate.
- FR-4.3 Interactive Comments & Mentions: Kolom komentar di bagian bawah detail issue dengan fitur deteksi @username untuk memicu notifikasi internal.
- FR-4.4 Subtask Checklist: Progress bar otomatis yang menghitung persentase subtask yang berstatus DONE terhadap total subtask pada parent task utama.

### EPIC 5: No-Code Automation Engine

- FR-5.1 Rule Builder UI: Tampilan visual berbasis logika IF-THEN-THAT di Vue untuk menyusun otomatisasi tanpa koding.
- FR-5.2 Core Triggers:
  - Trigger Status: Ketika status parent issue berubah menjadi Done.
  - Trigger Time: Ketika Due Date dari suatu tiket tersisa kurang dari 24 jam.
- FR-5.3 Core Actions:
  - Action Cascade: Ubah seluruh status subtask di bawahnya menjadi Done secara otomatis.
  - Action Notify/Assign: Mengirim notifikasi otomatis atau mengubah Assignee menjadi Reporter asal.

### EPIC 6: Collaboration & Notification System

- FR-6.1 Real-time State Synchronization: Menggunakan WebSockets di Axum untuk menyiarkan (broadcast) pembaruan papan secara langsung tanpa perlu refresh halaman.
- FR-6.2 Activity Stream (Audit Log): Sistem mencatat riwayat perubahan penting pada tiket (Contoh: "Budi mengubah prioritas PROJ-10 dari Medium ke High").
- FR-6.3 Notification Center: Pusat notifikasi dalam aplikasi (bell icon) dan sistem desktop OS (via Tauri Notification API) untuk memberi tahu pengguna jika:
  - Nama mereka dimention dalam komentar.
  - Ditunjuk sebagai Assignee baru pada suatu isu.
  - Isu yang mereka laporkan (Reported) mengalami perubahan status atau selesai.

---

## 4. Technical Architecture & Database Schema

Sistem ini menggunakan basis data mariadb untuk menjamin integritas relasi tabel data proyek berskala besar.

```sql
-- TABEL UTAMA FONDASI
CREATE TABLE workspaces (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(100) NOT NULL
);

-- JEMBATAN MANY-TO-MANY USER & WORKSPACE + RBAC
CREATE TABLE workspace_members (
    workspace_id INTEGER REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL DEFAULT 'MEMBER',
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (workspace_id, user_id)
);

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    workspace_id INTEGER REFERENCES workspaces(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    key VARCHAR(10) UNIQUE NOT NULL,
    project_type VARCHAR(20) NOT NULL,
    current_issue_counter INTEGER DEFAULT 0 NOT NULL
);

-- TABEL CUSTOM WORKFLOW & TRANSISI STATUS
CREATE TABLE statuses (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    category VARCHAR(20) NOT NULL
);

CREATE TABLE workflow_transitions (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    from_status_id INTEGER REFERENCES statuses(id) ON DELETE CASCADE,
    to_status_id INTEGER REFERENCES statuses(id) ON DELETE CASCADE,
    CONSTRAINT unique_transition UNIQUE(project_id, from_status_id, to_status_id)
);

-- TABEL DATA ISSUES (CORE TASK)
CREATE TABLE issues (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    parent_id INTEGER REFERENCES issues(id) ON DELETE SET NULL,
    epic_id INTEGER REFERENCES issues(id) ON DELETE SET NULL,
    key VARCHAR(20) UNIQUE NOT NULL,
    summary VARCHAR(255) NOT NULL,
    description TEXT,
    issue_type VARCHAR(20) NOT NULL,
    status_id INTEGER REFERENCES statuses(id),
    priority VARCHAR(20) DEFAULT 'MEDIUM',
    assignee_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    reporter_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    story_points INTEGER DEFAULT 0,
    due_date TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- TABEL PENDUKUNG (TIME LOG, COMMENT & NOTIFICATION)
CREATE TABLE time_logs (
    id SERIAL PRIMARY KEY,
    issue_id INTEGER REFERENCES issues(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    time_spent_seconds BIGINT NOT NULL,
    logged_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    issue_id INTEGER REFERENCES issues(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    recipient_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    sender_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    issue_id INTEGER REFERENCES issues(id) ON DELETE CASCADE,
    action_type VARCHAR(50) NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- TABEL LOGIKA NO-CODE AUTOMATION ENGINE
CREATE TABLE automation_rules (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    trigger_type VARCHAR(50) NOT NULL,
    trigger_config JSONB NOT NULL,
    action_type VARCHAR(50) NOT NULL,
    action_config JSONB NOT NULL,
    is_active BOOLEAN DEFAULT TRUE
);
```

---

## 5. Non-Functional Requirements & Performance

- Concurrency & Atomic Key Generation: Pembuatan Issue Key (misal: PROJ-101) wajib menggunakan operasi database terisolasi (SELECT FOR UPDATE pada baris tabel projects atau menggunakan database sequence) guna menghindari bentrok data akibat race condition saat beberapa pengguna membuat tiket secara bersamaan.
- Security: Enkripsi password menggunakan bcrypt atau argon2id pada backend Rust. Seluruh komunikasi REST API dan WebSocket wajib menggunakan protokol aman (HTTPS & WSS).
- Offline-First Cache Strategy (Tauri Desktop Feature): Khusus client desktop, jika koneksi internet terputus, aplikasi harus tetap dapat dibuka. Mutasi data lokal (seperti menggeser papan atau membuat note) disimpan sementara ke dalam cache browser (IndexedDB) atau SQLite lokal via plugin Tauri. Ketika koneksi terdeteksi online, sistem akan melakukan sinkronisasi delta (sinkronisasi dua arah) ke Central Cloud API.
- Performance Benchmark: Waktu muat awal (initial load) papan Kanban dengan kapasitas hingga 500 tiket aktif tidak boleh melebihi 1.5 detik. Implementasikan pagination tingkat database, lazy loading komponen di Vue 3, serta kompresi payload JSON gzip/brotli pada backend Rust.
