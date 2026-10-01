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
        Pinia["Pinia Stores<br/>(Auth, Project, Report, Automation, Notification)"]
        Views["Multi-View Workspace<br/>Kanban | Timeline Gantt | List Spreadsheet | Laporan & Analytics"]
        RichEditor["RichTextEditor (Word-like WYSIWYG)<br/>Tables, Image Paste, Code Blocks"]
        ToastLayer["Vue-Toastification Layer<br/>Sleek Floating Notifications"]
        WSClient["WebSocket Client<br/>(Auto Reconnect & Live Sync)"]
    end

    subgraph Backend ["Rust High-Performance Service (Axum)"]
        Router["Axum HTTP Router & Middleware<br/>Port 8088"]
        AuthLayer["JWT Auth & Bcrypt Security"]
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
    Router --> AnalyticsEngine
    Router --> AutoEngine
    AuthLayer --> DB
    AnalyticsEngine --> DB
    AutoEngine --> DB
    Router --> DB
```

---

## 🚀 Fitur Unggulan (Core Features)

### 1. User, Workspace & Role Management
* **JWT Authentication:** Registrasi, Login, dan Logout menggunakan JWT bearer tokens serta hashing password aman dengan `bcrypt`.
* **Multi-Workspace Support:** Satu akun dapat memiliki dan mengelola beberapa ruang kerja mandiri.
* **Role-Based Access Control (RBAC):** Peran terstruktur tingkat Workspace: `OWNER`, `ADMIN`, `MEMBER`, dan `VIEWER`.

### 2. Multi-View Project Workspace
* **Kanban View:** Kolom status dinamis dengan Drag-and-Drop beranimasi halus, indikator WIP (Work-in-Progress), dan drop guides.
* **Timeline / Gantt View:** Diagram batang horizontal interaktif yang memvisualisasikan rentang tanggal mulai (*Start Date*) dan tenggat (*Due Date*), navigasi waktu, dan status milestone.
* **Spreadsheet List View:** Tampilan tabular dengan kemampuan inline-editing instan untuk Status, Priority, Assignee, Points, dan Due Date tanpa perlu membuka modal.
* **Laporan & Analytics Suite:** Dasbor pelaporan lengkap mencakup Executive Summary, Sprint Velocity Chart & Burndown, Cumulative Flow Diagram (CFD), dan Timesheet Log kerja teragregasi.
* **Custom Workflows & Transition Guards:**
  * Penambahan status/kolom dinamis (kategori `TODO`, `IN_PROGRESS`, `DONE` dengan custom color picker).
  * **Workflow Transition Rules:** Admin dapat mengunci alur perpindahan status (misal: tiket di *Backlog* dilarang langsung digeser ke *Done* sebelum melewati *To Do* & *In Progress*). Sistem otomatis menolak perpindahan ilegal dengan pesan guard yang jelas.

### 3. Issue Lifecycle & Rich Word-Like Editor
* **Issue Hierarchy:** Dukungan hierarki lengkap: **Epic** (inisiatif besar), **Story / Task** (pekerjaan standar), dan **Subtask** (sub-pekerjaan).
* **Atomic Issue Key Generation:** Penomoran tiket (`PROJECT-1`, `PROJECT-2`, dst.) bebas race-condition dengan transaksi terisolasi `SELECT ... FOR UPDATE` pada MariaDB.
* **Jira/Word-Grade Rich Text Editor:**
  * Formatting teks lengkap (Bold, Italic, Underline, Heading 1-3, Quote, Code blocks, Text Color, Highlight).
  * **Tabel Spesifikasi Dinamis:** Pembuatan dan manipulasi tabel langsung (+ Baris, + Kolom, Hapus Baris, Hapus Tabel) dengan floating bar kontekstual yang otomatis muncul hanya saat kursor aktif di dalam sel tabel.
  * **Lampiran Gambar Instan:** Dukungan paste gambar langsung dari clipboard (`Ctrl+V`) serta file upload picker.
  * Penyimpanan berkapasitas besar menggunakan tipe kolom `LONGTEXT`.
* **Unified Modal (Single Source of Truth):** Modal terpadu lebar 1440px responsif ([IssueDetailModal.vue](file:///home/lenovo/www/EpicTask/frontend/src/components/issue/IssueDetailModal.vue)) yang melayani proses pembuatan tiket baru maupun pengeditan detail secara seamless.

### 4. Tracking, Collaboration & Notifications
* **Time Tracking (Work Log):** Pencatatan jam kerja nyata per user dan per tiket dengan riwayat catatan pekerjaan.
* **Interactive Comments & @Mentions:** Kolom komentar interaktif dengan deteksi otomatis `@username` yang langsung mengirim notifikasi instan.
* **Subtask Checklist:** Progress bar interaktif yang otomatis menghitung persentase subtask selesai, dilengkapi toggle centang satu-klik.
* **Vue-Toastification:** Seluruh feedback aksi, peringatan alur kerja, dan error ditangani oleh notifikasi toast modern menggantikan dialog alert browser bawaan.

### 5. No-Code Automation Engine
* **Visual Rule Builder (IF-THEN):** Antarmuka intuitif untuk menyusun otomasi tanpa kode.
* **Triggers:** Status Changed (misal: saat tiket berubah menjadi `DONE`), Due Date Alert (< 24 jam).
* **Actions:** Cascade Subtasks (otomatis menyelesaikan semua subtask), Reassign to Reporter, Notify Assignee.

### 6. Real-time Live Synchronization
* **WebSocket Multi-Peer Broadcasting:** Setiap perubahan (geser kartu, edit, hapus, komentar, time log) disiarkan instan ke seluruh pengguna yang terhubung tanpa perlu refresh halaman.

---

## 👥 Akun Demo & Data Uji Historis 1 Tahun

Database MariaDB telah dilengkapi generator data uji otomatis mencakup 1 tahun riwayat aktivitas realistis (Januari 2025 – 2026) untuk kebutuhan visualisasi grafik velocity, CFD, dan timesheet.

Anda dapat langsung login menggunakan tombol **1-Click Quick Login** di antarmuka atau menggunakan kredensial:

| Akun | Email | Password | Role |
|---|---|---|---|
| **Sarah Jenkins** (Tech Lead) | `sarah@epictask.dev` | `password123` | **OWNER** |
| **Alex Morgan** (Senior Fullstack) | `alex@epictask.dev` | `password123` | **MEMBER** |
| **Elena Rostova** (QA Specialist) | `elena@epictask.dev` | `password123` | **MEMBER** |
| **David Chen** (Product Owner) | `david@epictask.dev` | `password123` | **ADMIN** |

---

## 🛠️ Panduan Menjalankan & Rilis Otomatis

### 1. Menjalankan Backend (Rust Axum)
```bash
cd backend
cargo run
```
* REST API: `http://127.0.0.1:8088`
* WebSocket: `ws://127.0.0.1:8088/ws`

### 2. Menjalankan Frontend (Vue 3 + Vite)
```bash
cd frontend
npm install
npm run dev
```
* Akses aplikasi web di: `http://localhost:1420`

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
   * 🐧 **Linux:** `.deb`, `.rpm`, `.AppImage`, `.pkg.tar.zst` (Arch Linux package)
   * 🪟 **Windows:** `.exe` (NSIS Installer), `.msi`
   * 🍏 **macOS:** `.dmg` (Universal / Apple Silicon & Intel)

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
├── README.md                        # Dokumentasi Lengkap Sistem
├── backend/                         # Rust Axum Backend
│   ├── Cargo.toml
│   ├── .env                         # Konfigurasi Database & Port (Port 8088)
│   └── src/
│       ├── main.rs                  # Entrypoint, DB pool, Background workers, Router
│       ├── config.rs                # Environment loader
│       ├── db.rs                    # Skema otomatis MariaDB (InnoDB, LONGTEXT specs)
│       ├── routes.rs                # Definisi route REST & WebSocket
│       ├── auth/
│       │   ├── jwt.rs               # Pembuatan & validasi JWT token
│       │   └── middleware.rs        # Axum extractor untuk AuthUser
│       ├── models/                  # Struct model data (Issue, Status, User, Workflows, Reports)
│       ├── handlers/                # HTTP Controller handlers
│       │   ├── auth.rs              # Login, Register, Me
│       │   ├── workspaces.rs        # Workspace & Member management
│       │   ├── projects.rs          # Project, Statuses, Workflow transitions
│       │   ├── issues.rs            # Atomic key generator, Move guard, Detail
│       │   ├── reports.rs           # Executive metrics, Velocity, CFD, Timesheets
│       │   ├── comments.rs          # Komentar dengan @mention parser
│       │   ├── time_logs.rs         # Log work time tracking
│       │   ├── automation.rs        # No-code rule configurations
│       │   ├── notifications.rs     # Pusat notifikasi
│       │   ├── activities.rs        # Activity log audit stream
│       │   └── seed.rs              # Generator data demo 1 tahun historis
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
    │       ├── kanban/
    │       │   ├── KanbanBoard.vue  # Board kolom DnD & transition violation alert
    │       │   └── KanbanCard.vue   # Kartu isu, subtask checklist bar, badges
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
    │       │   └── CreateIssueModal.vue # Proxy ke IssueDetailModal
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

### 3. Uji Endpoint Analytics & Laporan
```bash
# Ambil ringkasan metrik proyek
curl -s http://127.0.0.1:8088/api/projects/1/reports/overview \
  -H "Authorization: Bearer $TOKEN"

# Ambil data Cumulative Flow Diagram (CFD)
curl -s http://127.0.0.1:8088/api/projects/1/reports/cfd \
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

*Selamat berkarya bersama EpicTask! Buka http://localhost:1420 di browser atau jalankan `npm run tauri dev` untuk desktop app.*
