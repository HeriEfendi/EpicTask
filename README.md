# EpicTask — High-Performance Modern Project & Issue Tracking Platform

[![Tech Stack](https://img.shields.io/badge/Frontend-Vue%203%20%7C%20Pinia%20%7C%20TailwindCSS-42b883?style=flat-square)](https://vuejs.org/)
[![Backend](https://img.shields.io/badge/Backend-Rust%20%7C%20Axum%20%7C%20SQLx-dea584?style=flat-square)](https://www.rust-lang.org/)
[![Database](https://img.shields.io/badge/Database-MariaDB-003545?style=flat-square)](https://mariadb.org/)
[![Desktop](https://img.shields.io/badge/Desktop-Tauri%20v2-24c8db?style=flat-square)](https://tauri.app/)
[![CI/CD](https://img.shields.io/badge/Release-Automated%20CI%2FCD-blueviolet?style=flat-square)](https://github.com/HeriEfendi/EpicTask/actions)

**EpicTask** adalah platform manajemen proyek agile dan pelacakan isu berkinerja tinggi yang dirancang untuk tim rekayasa perangkat lunak modern. EpicTask menggabungkan fungsionalitas esensial pelacakan kerja (Hierarki Isu, Custom Workflows, Multi-View Board, No-Code Automation, dan Enterprise Analytics Reporting) dengan antarmuka yang cepat, responsif, dan fleksibel untuk Web Application maupun Native Cross-Platform Desktop App (Linux, macOS, Windows via Tauri).

---

## 🏛️ Arsitektur Sistem

EpicTask menggunakan arsitektur **Hybrid Monorepo**: backend Rust Axum menyajikan RESTful API, Analytics Engine, dan WebSocket real-time broadcast hub, yang melayani frontend Vue 3 SPA baik di peramban web maupun di dalam window native Tauri Desktop.

```mermaid
graph TD
    subgraph Clients ["Klien Multi-Platform"]
        Web["🌐 Web Client (SPA)<br/>http://localhost:1420"]
        Desktop["🖥️ Desktop Client (Tauri v2)<br/>Linux (.deb, .rpm, .AppImage, .pkg.tar.zst) | Win (.exe, .msi) | macOS (.dmg)"]
    end

    subgraph FrontendApp ["Vue 3 SPA (Composition API)"]
        Pinia["Pinia Stores<br/>(Auth, Project, Sprint, Component, Report, Automation, Notification)"]
        Views["Multi-View Workspace<br/>Kanban | Backlog & Sprint | Timeline Gantt | List Spreadsheet | Laporan & Analytics"]
        RichEditor["RichTextEditor (Word-like WYSIWYG)<br/>Tables, Image Paste, Code Blocks"]
        ToastLayer["Vue-Toastification Layer<br/>Sleek Floating Notifications"]
        WSClient["WebSocket Client<br/>(Auto Reconnect & Live Sync)"]
    end

    subgraph Backend ["Rust High-Performance Service (Axum)"]
        Router["Axum HTTP Router & Middleware<br/>Port 8088"]
        AuthLayer["JWT Auth & Bcrypt Security"]
        SprintEngine["Sprint & Component Engine<br/>(Lifecycle, Points Rollup, Subsystems)"]
        AnalyticsEngine["Analytics & Report Handlers<br/>(Velocity, CFD, Timesheets)"]
        WSHub["tokio::sync::broadcast<br/>Real-Time Multi-Peer Broadcast"]
        AutoEngine["No-Code Automation Engine<br/>Status Triggers & Cascades"]
    end

    subgraph Database ["MariaDB Enterprise Database"]
        DB[(epictask DB<br/>Port 3307 / 3306<br/>InnoDB utf8mb4 | LONGTEXT Specs)]
    end

    Web --> Views
    Desktop --> Views
    Views --> RichEditor
    Views --> ToastLayer
    Views --> Pinia
    Pinia --> Router
    WSClient <--> WSHub
    Router --> AuthLayer
    Router --> SprintEngine
    Router --> AnalyticsEngine
    Router --> AutoEngine
    AuthLayer --> DB
    SprintEngine --> DB
    AnalyticsEngine --> DB
    AutoEngine --> DB
    Router --> DB
```

---

## 🚀 Fitur Unggulan (Core Features)

### 1. User, Workspace & Role Management

- **JWT Authentication:** Registrasi, Login, dan Logout menggunakan JWT bearer tokens serta hashing password aman dengan `bcrypt`.
- **Multi-Workspace Support:** Satu akun dapat memiliki dan mengelola beberapa ruang kerja mandiri.
- **Role-Based Access Control (RBAC):** Peran terstruktur tingkat Workspace: `OWNER`, `ADMIN`, `MEMBER`, dan `VIEWER`.

### 2. Multi-View Project Workspace

- **Agile Backlog & Sprint Planning (Scrum Framework):**
  - **Pemisahan Backlog & Active Sprint:** Antarmuka khusus perencanaan iterasi kerja tangkas ala Jira Software.
  - **Siklus Hidup Sprint Lengkap:** Status `PLANNED` (Direncanakan) $\to$ `ACTIVE` (Sedang Berjalan) $\to$ `COMPLETED` (Selesai & Diarsipkan).
  - **Agregasi Estimasi 3-Pill Jira:** Indikator visual akumulasi Story Points real-time (Abu: *To Do*, Biru: *In Progress*, Hijau: *Done*).
  - **Mulai & Selesaikan Sprint Interaktif:**
    - Modal *Mulai Sprint* dengan preset durasi 1, 2, 3, 4 minggu atau kustom, tanggal mulai/selesai, serta target sasaran (*Sprint Goal*).
    - Modal *Selesaikan Sprint* yang menampilkan ringkasan performa penyelesaian dan opsi otomatis memindahkan isu yang belum selesai ke *Product Backlog* atau ke Sprint berikutnya.
  - **Drag-and-Drop Terintegrasi:** Memindahkan isu antar-sprint atau dari Product Backlog dengan animasi mulus dan pembaruan instan via WebSocket.
- **Project Components (Sub-Sistem / Modul):**
  - Pengelompokan isu berdasarkan domain teknis (contoh: *Backend & API*, *Frontend UI*, *QA & Automation*, *Core Infra*).
  - Penunjukan *Component Lead* untuk penanggung jawab teknis tiap subsistem.
  - Filter satu-klik per komponen di Backlog dan kartu Kanban.
  - Badge visual komponen (`🧩 Nama Komponen`) pada setiap kartu board dan detail tiket.
- **Kanban View:** Kolom status dinamis dengan Drag-and-Drop beranimasi halus, indikator WIP (Work-in-Progress), dan drop guides.
- **Timeline / Gantt View:** Diagram batang horizontal interaktif yang memvisualisasikan rentang tanggal mulai (_Start Date_) dan tenggat (_Due Date_), navigasi waktu, dan status milestone.
- **Spreadsheet List View:** Tampilan tabular dengan kemampuan inline-editing instan untuk Status, Priority, Assignee, Points, dan Due Date tanpa perlu membuka modal.
- **Laporan & Analytics Suite:** Dasbor pelaporan lengkap mencakup Executive Summary, Sprint Velocity Chart & Burndown, Cumulative Flow Diagram (CFD), dan Timesheet Log kerja teragregasi.
- **Custom Workflows & Transition Guards:**
  - Penambahan status/kolom dinamis (kategori `TODO`, `IN_PROGRESS`, `DONE` dengan custom color picker).
  - **Workflow Transition Rules:** Admin dapat mengunci alur perpindahan status (misal: tiket di _Backlog_ dilarang langsung digeser ke _Done_ sebelum melewati _To Do_ & _In Progress_). Sistem otomatis menolak perpindahan ilegal dengan pesan guard yang jelas.

### 3. Issue Lifecycle & Rich Word-Like Editor

- **Issue Hierarchy:** Dukungan hierarki lengkap: **Epic** (inisiatif besar), **Story / Task** (pekerjaan standar), dan **Subtask** (sub-pekerjaan).
- **Atomic Issue Key Generation:** Penomoran tiket (`PROJECT-1`, `PROJECT-2`, dst.) bebas race-condition dengan transaksi terisolasi `SELECT ... FOR UPDATE` pada MariaDB.
- **Word-Grade Rich Text Editor:**
  - Formatting teks lengkap (Bold, Italic, Underline, Heading 1-3, Quote, Code blocks, Text Color, Highlight).
  - **Tabel Spesifikasi Dinamis:** Pembuatan dan manipulasi tabel langsung (+ Baris, + Kolom, Hapus Baris, Hapus Tabel) dengan floating bar kontekstual yang otomatis muncul hanya saat kursor aktif di dalam sel tabel.
  - **Lampiran Gambar Instan:** Dukungan paste gambar langsung dari clipboard (`Ctrl+V`) serta file upload picker.
  - Penyimpanan berkapasitas besar menggunakan tipe kolom `LONGTEXT`.
- **Unified Modal (Single Source of Truth):** Modal terpadu lebar 1440px responsif ([IssueDetailModal.vue](file:///home/lenovo/www/EpicTask/frontend/src/components/issue/IssueDetailModal.vue)) yang melayani proses pembuatan tiket baru maupun pengeditan detail secara seamless.

### 4. Tracking, Collaboration & Notifications

- **Time Tracking (Work Log):** Pencatatan jam kerja nyata per user dan per tiket dengan riwayat catatan pekerjaan.
- **Interactive Comments & @Mentions:** Kolom komentar interaktif dengan deteksi otomatis `@username` yang langsung mengirim notifikasi instan.
- **Subtask Checklist:** Progress bar interaktif yang otomatis menghitung persentase subtask selesai, dilengkapi toggle centang satu-klik.
- **Vue-Toastification:** Seluruh feedback aksi, peringatan alur kerja, dan error ditangani oleh notifikasi toast modern menggantikan dialog alert browser bawaan.

### 5. No-Code Automation Engine

- **Visual Rule Builder (IF-THEN):** Antarmuka intuitif untuk menyusun otomasi tanpa kode.
- **Triggers:** Status Changed (misal: saat tiket berubah menjadi `DONE`), Due Date Alert (< 24 jam).
- **Actions:** Cascade Subtasks (otomatis menyelesaikan semua subtask), Reassign to Reporter, Notify Assignee.

### 6. Real-time Live Synchronization

- **WebSocket Multi-Peer Broadcasting:** Setiap perubahan (geser kartu, edit, hapus, komentar, time log) disiarkan instan ke seluruh pengguna yang terhubung tanpa perlu refresh halaman.

### 7. Release Hub & Version Management (Jira Standard)

- **Siklus Hidup Rilis:** Pengelolaan versi software (`UNRELEASED`, `RELEASED`, `ARCHIVED`) dengan target peluncuran terstruktur.
- **Progress Bar 3-Segmen Realtime:** Visualisasi rasio penyelesaian tiket (Hijau: Selesai, Biru: Sedang Dikerjakan, Abu-abu: To Do) disertai persentase.
- **Jira-Style Release Confirmation Modal:** Menghitung otomatis tiket yang belum selesai saat rilis dan menyediakan opsi migrasi tiket ke versi terencana berikutnya atau melepasnya dari versi.
- **Atribut & Filter Fix Version:** Dropdown Fix Version pada modal pembuatan/detail tiket, kolom inline pada spreadsheet ListView, badge visual `🚀` pada kartu Kanban, dan filter 1-klik di header board.

---

## 🎯 Standarisasi Jira (Jira Enterprise Standards & Master Roadmap)

EpicTask terus mengadopsi standar industri terbaik dari **Atlassian Jira Software** untuk menghadirkan alur kerja rekayasa perangkat lunak enterprise yang matang dan terstruktur. Seluruh kemajuan standarisasi dicatat secara transparan pada berkas [JIRA_STANDARDIZATION_ROADMAP.md](file:///home/lenovo/www/EpicTask/JIRA_STANDARDIZATION_ROADMAP.md).

| No | Fitur Standar Jira | Prioritas | Status | Modul & Komponen Terkait | Keterangan Fungsional |
|:---:|:---|:---:|:---:|:---|:---|
| **1** | **Scrum Backlog & Sprint Management** | P0 (Kritis) | 🟢 **Selesai** | `handlers/sprints.rs`<br>`components/backlog/*`<br>`stores/sprint.js` | Siklus sprint (PLANNED $\to$ ACTIVE $\to$ COMPLETED), Backlog View terdedikasi, Start/Complete Sprint modal, Story Points 3-Pill rollup, dan drag-and-drop antar sprint/backlog. |
| **2** | **Components (Sub-Sistem / Modul)** | P0 (Kritis) | 🟢 **Selesai** | `handlers/components.rs`<br>`ComponentManagerModal.vue`<br>`stores/component.js` | Pemecahan modul teknis proyek, penunjukan *Component Lead*, penugasan komponen pada tiket, filter cepat, dan visual badge `🧩`. |
| **3** | **Versions / Releases (Release Hub)** | P1 (Tinggi) | 🟢 **Selesai** | `handlers/versions.rs`<br>`components/releases/*`<br>`stores/version.js` | Pelacakan versi rilis software (`UNRELEASED`, `RELEASED`, `ARCHIVED`), target tanggal rilis, progress bar 3-fase, modal rilis dengan migrasi isu belum selesai, dan filter Fix Version di board & list. |
| **4** | **Board Swimlanes (Grouping Horizontal)** | P1 (Tinggi) | 🟢 **Selesai** | `components/kanban/*`<br>`stores/project.js` | Pembagian horizontal Kanban: *Group by Assignee*, *Group by Epic*, atau *Group by Priority (Expedite)* dengan baris collapsible, roll-up story point, dan header kolom sticky. |
| **5** | **Issue Labels (Tagging Fleksibel)** | P2 (Menengah) | 🟢 **Selesai** | `handlers/labels.rs`<br>`IssueLabelsEditor.vue`<br>`stores/label.js` | Tag bebas lintas tiket dengan autocomplete interaktif, chips badge warna-warni di Kanban & Backlog, filter label di ViewTabs, dan kolom + ekspor CSV di ListView. |
| **6** | **Issue Resolution (Alasan Selesai)** | P2 (Menengah) | 🟢 **Selesai** | `handlers/issues/transition.rs`<br>`ResolutionModal.vue`<br>`IssueMetaSidebar.vue` | Standar audit Jira: pembedaan status DONE dengan alasan resolusi (`Fixed`, `Done`, `Won't Fix`, `Duplicate`, `Cannot Reproduce`, `Incomplete`), modal transisi otomatis saat geser ke DONE, coret teks key `~~KEY-123~~`, dan ekspor CSV. |
| **7** | **Quick Filter Bar di Atas Board** | P2 (Menengah) | 🟢 **Selesai** | `KanbanQuickFilters.vue`<br>`KanbanToolbar.vue`<br>`stores/project.js` | Filter 1-klik di toolbar atas board: "Tiket Saya", "Hanya Bug", "Prioritas Tinggi", "Belum Ditugaskan", "Due Minggu Ini", "Belum Selesai", tombol Reset, dan multi-select toggle. |
| **8** | **Issue Watchers (Pengamat Tiket)** | P3 (Pelengkap) | ⚪ *Next* | `models/watcher.rs`<br>`handlers/watchers.rs` | Fitur pengamat (*watcher*) tiket untuk menerima notifikasi otomatis pembaruan tanpa harus menjadi Assignee/Reporter. |

---

## 👥 Akun Demo & Data Uji Historis 1 Tahun

Database MariaDB telah dilengkapi generator data uji otomatis mencakup 1 tahun riwayat aktivitas realistis (Januari 2025 – 2026) untuk kebutuhan visualisasi grafik velocity, CFD, dan timesheet.

Anda dapat langsung login menggunakan tombol **1-Click Quick Login** di antarmuka atau menggunakan kredensial:

| Akun                               | Email                | Password      | Role       |
| ---------------------------------- | -------------------- | ------------- | ---------- |
| **Sarah Jenkins** (Tech Lead)      | `sarah@epictask.dev` | `password123` | **OWNER**  |
| **Alex Morgan** (Senior Fullstack) | `alex@epictask.dev`  | `password123` | **MEMBER** |
| **Elena Rostova** (QA Specialist)  | `elena@epictask.dev` | `password123` | **MEMBER** |
| **David Chen** (Product Owner)     | `david@epictask.dev` | `password123` | **ADMIN**  |

---

## 🛠️ Panduan Menjalankan & Rilis Otomatis

### 1. Menjalankan Backend (Rust Axum)

```bash
cd backend
cargo run
```

- REST API: `http://127.0.0.1:8088`
- WebSocket: `ws://127.0.0.1:8088/ws`

### 2. Menjalankan Frontend (Vue 3 + Vite)

```bash
cd frontend
npm install
npm run dev
```

- Akses aplikasi web di: `http://localhost:1420`

### 3. Menjalankan Aplikasi Desktop (Tauri v2)

```bash
cd frontend
npm run tauri dev
```

### 4. 🚀 Satu Perintah Rilis Otomatis Multi-Platform (`npm run release`)

Proyek ini dilengkapi script otomatisasi rilis terpadu (`scripts/release.mjs`) dan CI/CD GitHub Actions:

```bash
# Jalankan dari root direktori proyek:
npm run release 0.17.1
```

**Proses yang dijalankan secara otomatis:**

1. Sinkronisasi branch Git dan update histori commit.
2. Generate changelog rilis otomatis ke `CHANGELOG.md`.
3. Sinkronisasi nomor versi di `package.json`, `frontend/package.json`, `tauri.conf.json`, dan `Cargo.toml`.
4. Pembuatan Git Commit & Git Tag `v0.17.1`.
5. Otomatis `git push` ke GitHub dan memicu GitHub Actions Matrix Workflow untuk kompilasi multi-platform:
   - 🐧 **Linux:** `.deb`, `.rpm`, `.AppImage`, `.pkg.tar.zst` (Arch Linux package)
   - 🪟 **Windows:** `.exe` (NSIS Installer), `.msi`
   - 🍏 **macOS:** `.dmg` (Universal / Apple Silicon & Intel)

---

## 📁 Struktur Direktori Monorepo

```
EpicTask/
├── .github/
│   └── workflows/
│       └── release.yml              # GitHub Actions CI/CD matrix build multi-platform
├── scripts/
│   ├── release.mjs                  # Script rilis 1-perintah (npm run release <ver>)
│   └── create-arch-pkg.mjs          # Generator paket Arch Linux (.pkg.tar.zst)
├── CHANGELOG.md                     # Histori perubahan otomatis
├── Cargo.toml                       # Root Cargo workspace (backend + frontend/src-tauri)
├── package.json                     # Monorepo scripts
├── EpicTask_PRD.md                  # PRD Spesifikasi Produk
├── JIRA_STANDARDIZATION_ROADMAP.md  # Master roadmap & gap analysis standar Jira
├── README.md                        # Dokumentasi Lengkap Sistem
├── backend/                         # Rust Axum Backend
│   ├── Cargo.toml
│   ├── .env                         # Konfigurasi Database & Port (Port 8088)
│   └── src/
│       ├── main.rs                  # Entrypoint, DB pool, Background workers, Router
│       ├── config.rs                # Environment loader
│       ├── db.rs                    # Skema otomatis MariaDB (InnoDB, sprints, components, LONGTEXT)
│       ├── routes.rs                # Definisi route REST & WebSocket
│       ├── auth/
│       │   ├── jwt.rs               # Pembuatan & validasi JWT token
│       │   └── middleware.rs        # Axum extractor untuk AuthUser
│       ├── models/                  # Struct model data (Issue, Sprint, Component, Version, User, Reports)
│       │   ├── sprint.rs            # Struct model & payload siklus Sprint (Scrum)
│       │   ├── component.rs         # Struct model subsistem & modul proyek
│       │   ├── version.rs           # Struct model versi software & rilis
│       │   ├── label.rs             # Struct model label & ad-hoc tags
│       │   └── issue.rs             # Model tiket dengan sprint_id, component_id, fix_version_id & labels
│       ├── handlers/                # HTTP Controller handlers
│       │   ├── sprints.rs           # Siklus sprint, start/complete, 3-pill story point rollup
│       │   ├── components.rs        # CRUD komponen & kepemilikan Component Lead
│       │   ├── versions.rs          # Release Hub, lifecycle versi & issue migration
│       │   ├── labels.rs            # Manajemen label & sinkronisasi multi-tag
│       │   ├── auth.rs              # Login, Register, Me
│       │   ├── workspaces.rs        # Workspace & Member management
│       │   ├── projects.rs          # Project, Statuses, Workflow transitions
│       │   ├── issues/              # Atomic key generator, Move guard, Detail
│       │   ├── reports.rs           # Executive metrics, Velocity, CFD, Timesheets
│       │   ├── comments.rs          # Komentar dengan @mention parser
│       │   ├── time_logs.rs         # Log work time tracking
│       │   ├── automation.rs        # No-code rule configurations
│       │   ├── notifications.rs     # Pusat notifikasi
│       │   ├── activities.rs        # Activity log audit stream
│       │   └── seed.rs              # Generator data demo (users, sprints, components, issues)
│       ├── ws/
│       │   └── mod.rs               # WebSocket broadcast hub
│       └── automation/
│           └── engine.rs            # Execution engine untuk trigger & cascade action
└── frontend/                        # Vue 3 Frontend & Tauri Desktop Wrapper
    ├── package.json
    ├── vite.config.js               # Proxy ke Axum API & WS (Port 8088)
    ├── tailwind.config.js           # Desain token & tema modern
    ├── index.html                   # HTML entrypoint dengan Inter typography
    ├── src/
    │   ├── main.js                  # App bootstrap, Pinia & Vue-Toastification setup
    │   ├── App.vue                  # Root layout & view orchestrator
    │   ├── assets/
    │   │   ├── main.css             # Glassmorphism, toast styles, badges & custom scrollbars
    │   │   └── logo.svg             # Ikon logo EpicTask
    │   ├── utils/
    │   │   └── toast.js             # Unified toast notification helper
    │   ├── services/
    │   │   ├── api.js               # HTTP client dengan Bearer token
    │   │   ├── websocket.js         # WS client dengan auto-reconnect
    │   │   └── tauri.js             # Bridge integrasi desktop Tauri
    │   ├── stores/
    │   │   ├── auth.js              # State autentikasi & workspace aktif
    │   │   ├── sprint.js            # State sprint aktif, terencana, selesai & live WS
    │   │   ├── component.js         # State subsistem proyek & lead
    │   │   ├── version.js           # State versi rilis software & Release Hub
    │   │   ├── label.js             # State label & ad-hoc tag proyek
    │   │   ├── project.js           # State papan, multi-assignee filter, DnD, & isu aktif
    │   │   ├── report.js            # State analytics, velocity, CFD, & timesheet
    │   │   ├── automation.js        # State rule otomasi
    │   │   └── notification.js      # State notifikasi & badge unread
    │   └── components/
    │       ├── common/
    │       │   ├── Navbar.vue       # Header navigasi, status live WS, profile
    │       │   ├── ViewTabs.vue     # Switcher 2 baris (Tabs + Otomasi/Workflow + Multi-filter)
    │       │   ├── RichTextEditor.vue # Editor Word-like WYSIWYG, tabel pintar, image paste
    │       │   ├── Sidebar.vue      # Sidebar navigasi proyek & workspace
    │       │   └── SettingsModal.vue # Pengaturan workspace, profil, reset seed data
    │       ├── backlog/
    │       │   ├── BacklogView.vue  # Tampilan orkestrator Backlog & Sprint Jira
    │       │   ├── SprintContainer.vue # Kontainer kartu sprint dengan drag-drop
    │       │   ├── SprintHeader.vue # Header sprint, 3-pill story points, kontrol aksi
    │       │   ├── BacklogContainer.vue # Penampung Product Backlog
    │       │   ├── BacklogIssueItem.vue # Baris tiket terstruktur dengan tags & points
    │       │   ├── CreateSprintModal.vue # Modal buat sprint terencana
    │       │   ├── StartSprintModal.vue # Modal mulai sprint aktif
    │       │   ├── CompleteSprintModal.vue # Modal penyelesaian sprint & rollover isu
    │       │   └── ComponentManagerModal.vue # Modal CRUD komponen & lead
    │       ├── releases/
    │       │   ├── ReleasesView.vue # Tampilan induk Release Hub Jira
    │       │   ├── ReleaseCard.vue  # Kartu versi, status badge, overdue & actions
    │       │   ├── ReleaseProgressBar.vue # Bar progres 3-fase (Done, In Progress, To Do)
    │       │   ├── CreateVersionModal.vue # Modal pembuatan & edit versi rilis
    │       │   └── ReleaseVersionModal.vue # Modal konfirmasi rilis & rollover tiket
    │       ├── kanban/
    │       │   ├── KanbanBoard.vue  # Board orkestrator (standard columns & swimlanes)
    │       │   ├── KanbanToolbar.vue # Toolbar header board & dropdown switcher swimlane
    │       │   ├── KanbanColumn.vue # Komponen kolom vertikal atomik
    │       │   ├── KanbanSwimlaneView.vue # Orkestrator horizontal swimlane & header sticky
    │       │   ├── KanbanSwimlaneRow.vue # Baris swimlane collapsible, roll-up point & sel dropzone
    │       │   └── KanbanCard.vue   # Kartu isu, subtask checklist bar, component & epic badges
    │       ├── timeline/
    │       │   └── TimelineView.vue # Horizontal Gantt chart interaktif
    │       ├── list/
    │       │   └── ListView.vue     # Tabular spreadsheet dengan inline-edit
    │       ├── reports/
    │       │   └── ReportsView.vue  # Dasbor laporan, velocity burndown, CFD, timesheet
    │       ├── automation/
    │       │   └── AutomationModal.vue # Rule builder IF-THEN visual
    │       ├── project/
    │       │   └── WorkflowModal.vue   # Manajemen kolom & transition guards
    │       ├── issue/
    │       │   ├── IssueDetailModal.vue # Modal terpadu (Create + Edit), subtask, komentar, time log
    │       │   ├── IssueMetaSidebar.vue # Atribut tiket: tipe, status, priority, sprint, component, labels
    │       │   └── IssueLabelsEditor.vue # Multi-tag input dengan autocomplete pintar & pastel badges
    │       ├── notification/
    │       │   └── NotificationPopover.vue # Popover notifikasi interaktif
    │       └── auth/
    │           └── AuthModal.vue    # Formulir Login, Registrasi & 1-Click Demo
    └── src-tauri/                   # Tauri v2 Desktop Wrapper
        ├── Cargo.toml
        ├── tauri.conf.json
        ├── build.rs
        └── src/
            ├── main.rs
            └── lib.rs               # Native commands (desktop notification, app info)
```

---

## 🧪 Pengujian API Backend (CLI Curl Reference)

### 1. Login & Ambil Token

```bash
TOKEN=$(curl -s -X POST http://127.0.0.1:8088/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"sarah@epictask.dev","password":"password123"}' | grep -o '"token":"[^"]*' | cut -d'"' -f4)
```

### 2. Uji Workflow Transition Guard

```bash
# Perpindahan ilegal: Backlog (Status 1) -> Done (Status 5) -> DITOLAK (HTTP 400)
curl -s -X POST http://127.0.0.1:8088/api/issues/8/move \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"target_status_id": 5}'

# Output: {"error":"Workflow rule violation: Transition directly from 'Backlog' to 'Done' is not allowed."}
```

### 3. Uji Endpoint Sprints & Components (Standar Jira)

```bash
# Ambil daftar Sprint dengan agregasi 3-Pill Story Points
curl -s http://127.0.0.1:8088/api/projects/1/sprints \
  -H "Authorization: Bearer $TOKEN"

# Mulai Sprint aktif (durasi 14 hari)
curl -s -X POST http://127.0.0.1:8088/api/sprints/2/start \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"name":"EPIC Sprint 2","goal":"Fokus pada modul analitik dan rilis versi"}'

# Ambil daftar subsistem & komponen proyek beserta lead
curl -s http://127.0.0.1:8088/api/projects/1/components \
  -H "Authorization: Bearer $TOKEN"
```

### 4. Uji Endpoint Analytics & Laporan

```bash
# Ambil ringkasan metrik proyek
curl -s http://127.0.0.1:8088/api/projects/1/reports/overview \
  -H "Authorization: Bearer $TOKEN"

# Ambil data Cumulative Flow Diagram (CFD)
curl -s http://127.0.0.1:8088/api/projects/1/reports/cumulative-flow \
  -H "Authorization: Bearer $TOKEN"
```

---

## ⚡ Ringkasan Kinerja & Kualitas Rekayasa

1. **Atomic Concurrency:** Transaksi MariaDB `SELECT ... FOR UPDATE` menjamin penomoran Issue Key selalu unik dan berurutan secara konsisten.
2. **Keamanan Data:** Enkripsi password menggunakan `bcrypt`, otentikasi JWT Bearer token, dan prepared statement `sqlx`.
3. **Penyimpanan Spesifikasi Luas:** Kolom deskripsi mendukung format dokumen panjang dan tabel kompleks (`LONGTEXT`).
4. **Waktu Muat Cepat:** Bundle frontend produksi teroptimasi Vite dengan waktu kompilasi < 4 detik dan muat awal di bawah 300 ms.
5. **Real-time Synchronization:** State terdistribusi secara live via Axum WebSocket broadcast channel.

---

_Selamat berkarya bersama EpicTask! Buka http://localhost:1420 di browser atau jalankan `npm run tauri dev` untuk desktop app._
