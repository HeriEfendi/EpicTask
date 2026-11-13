# EpicTask — Jira Standards Gap Analysis & Implementation Roadmap

> **Status:** Active / In-Progress  
> **Standard Reference:** Atlassian Jira Software Enterprise & Cloud Standards  
> **Arsitektur:** Hybrid Monorepo (Rust/Axum, MariaDB, Vue 3 Composition API, Pinia, Tailwind CSS)  
> **Pedoman Arsitektur:** Senior Software Architect, Zero God-File Policy, Single Responsibility Principle, Tailwind-Only Styling.

Dokumen ini berfungsi sebagai *living document* dan *single source of truth* untuk seluruh implementasi standarisasi Jira pada EpicTask. Setiap fitur yang sedang atau selesai diimplementasikan akan diperbarui status, struktur basis data, endpoint API, dan komponen antarmukanya di sini.

---

## 📊 Matriks Keseluruhan (Master Feature Tracker)

| No | Fitur Standar Jira | Prioritas | Status | Modul Backend | Komponen Frontend | Keterangan |
|:---|:---|:---:|:---:|:---|:---|:---|
| **1** | **Scrum Backlog & Sprint Management** | P0 (Kritis) | 🟢 **Done** | `models/sprint.rs`<br>`handlers/sprints.rs` | `components/backlog/*`<br>`stores/sprint.js` | Siklus sprint (PLANNED, ACTIVE, COMPLETED), Backlog View, Start/Complete/Create Sprint modal, roll-up Story Points 3-pill (To Do, In Progress, Done), drag-and-drop tiket antar sprint/backlog. |
| **2** | **Components (Sub-Sistem / Modul)** | P0 (Kritis) | 🟢 **Done** | `models/component.rs`<br>`handlers/components.rs` | `components/backlog/ComponentManagerModal.vue`<br>`components/issue/IssueMetaSidebar.vue`<br>`stores/component.js` | Modul fungsional proyek, Component Lead, filter komponen di Backlog, penugasan komponen pada tiket & badge visual pada kartu Kanban. |
| **3** | **Versions / Releases (Release Hub)** | P1 (Tinggi) | 🟢 **Done** | `models/version.rs`<br>`handlers/versions.rs` | `components/releases/*`<br>`stores/version.js` | Pelacakan rilis software (UNRELEASED, RELEASED, ARCHIVED), target tanggal rilis, progress bar 3-fase (Done, In Progress, To Do), modal rilis dengan migrasi isu belum selesai, filter rilis di ViewTabs & ListView, badge rilis pada kartu Kanban. |
| **4** | **Board Swimlanes (Grouping Horizontal)** | P1 (Tinggi) | 🟢 **Done** | N/A (Frontend Agregasi) | `components/kanban/*`<br>`stores/project.js` | Pembagian horizontal Kanban: Group by Assignee, Group by Epic, Group by Priority (Expedite row darurat), header sticky kolom, roll-up story point per baris, dan dropzone sel mandiri. |
| **5** | **Issue Labels (Tagging Fleksibel)** | P2 (Menengah) | 🟢 **Done** | `models/label.rs`<br>`handlers/labels.rs` | `components/issue/IssueLabelsEditor.vue`<br>`stores/label.js` | Tagging bebas lintas tiket dengan autocomplete interaktif, chips badge warna-warni di Kanban & Backlog, filter label di ViewTabs, dan kolom + ekspor CSV di ListView. |
| **6** | **Issue Resolution (Alasan Selesai)** | P2 (Menengah) | ⚪ *Next (Planned)* | Kolom `resolution` pada issues | `components/issue/` | Pembedaan status DONE dengan Resolution (`Fixed`, `Won't Fix`, `Duplicate`, `Cannot Reproduce`). |
| **7** | **Quick Filter Bar di Atas Board** | P2 (Menengah) | ⚪ *Planned* | N/A (Frontend Client) | `components/common/` | Filter 1-klik di atas board: "Tiket Saya", "Hanya Bug", "Prioritas Tertinggi", "Belum Ditugaskan". |
| **8** | **Issue Watchers (Pengamat Tiket)** | P3 (Pelengkap) | ⚪ *Planned* | `models/watcher.rs`<br>`handlers/watchers.rs` | `components/issue/` | Fitur "Watch" tiket untuk notifikasi otomatis pembaruan tiket ke anggota terkait. |

---

## 🎯 Detail & Spesifikasi Eksekusi

### 1. Scrum Backlog & Sprint Management (🟢 SELESAI)
* **Tujuan:** Menyediakan perencanaan iterasi tangkas (Sprint) dan penampung Backlog sebelum tugas dipindahkan ke Sprint aktif.
* **Database Schema (MariaDB):**
  - Tabel `sprints`:
    - `id BIGINT AUTO_INCREMENT PRIMARY KEY`
    - `project_id BIGINT NOT NULL (FK projects.id)`
    - `name VARCHAR(100) NOT NULL`
    - `goal TEXT`
    - `status VARCHAR(20) NOT NULL DEFAULT 'PLANNED'` (`PLANNED`, `ACTIVE`, `COMPLETED`)
    - `start_date DATE DEFAULT NULL`
    - `end_date DATE DEFAULT NULL`
    - `created_at TIMESTAMP`, `updated_at TIMESTAMP`
  - Relasi `issues`:
    - Kolom `sprint_id BIGINT DEFAULT NULL` (foreign key referensi `sprints.id`)
* **Backend Endpoints:**
  - `GET /api/projects/:pid/sprints` : Mendapatkan seluruh sprint proyek beserta agregasi 3-pill story points (To Do, In Progress, Done)
  - `POST /api/projects/:pid/sprints` : Membuat sprint baru
  - `PUT /api/sprints/:id` : Memperbarui nama, target, tanggal sprint
  - `DELETE /api/sprints/:id` : Menghapus sprint (mengembalikan isu ke backlog dengan aman)
  - `POST /api/sprints/:id/start` : Memulai sprint (`ACTIVE`, validasi hanya 1 active sprint per project)
  - `POST /api/sprints/:id/complete` : Menyelesaikan sprint (`COMPLETED`, memindahkan tiket yang belum selesai ke Backlog atau ke Sprint terencana berikutnya)
  - `POST /api/projects/:pid/sprints/move-issues` : Memindahkan daftar isu ke dalam sprint atau ke backlog (`null`)
* **Frontend Modules & Components:**
  - `stores/sprint.js` : Pinia store reaktif untuk sprint dengan integrasi live WebSocket
  - `components/backlog/BacklogView.vue` : Tampilan induk Backlog View
  - `components/backlog/SprintContainer.vue` : Kontainer masing-masing sprint dengan drag-and-drop target
  - `components/backlog/SprintHeader.vue` : Header sprint dengan kontrol collapse, status badge, 3-pill points, aksi Mulai/Selesaikan/Edit/Hapus
  - `components/backlog/BacklogContainer.vue` : Penampung Product Backlog dengan drag-and-drop dropzone
  - `components/backlog/BacklogIssueItem.vue` : Baris item tiket dengan drag handle, tipe icon, key, judul, badge epic & component, status, prioritas, estimasi poin, dan avatar assignee
  - `components/backlog/CreateSprintModal.vue` : Modal pembuatan sprint terencana
  - `components/backlog/StartSprintModal.vue` : Modal memulai sprint dengan preset durasi (1, 2, 3, 4 minggu, custom) dan target sprint
  - `components/backlog/CompleteSprintModal.vue` : Modal penyelesaian sprint dengan rekap tiket selesai vs belum selesai serta opsi pemindahan tiket yang tersisa

---

### 2. Components / Sub-Sistem Proyek (🟢 SELESAI)
* **Tujuan:** Mengelompokkan pekerjaan teknis ke dalam sub-sistem fungsional (Backend, Frontend, Payment, Auth, dll) dengan kepemilikan teknis (*Component Lead*).
* **Database Schema (MariaDB):**
  - Tabel `components`:
    - `id BIGINT AUTO_INCREMENT PRIMARY KEY`
    - `project_id BIGINT NOT NULL (FK projects.id)`
    - `name VARCHAR(100) NOT NULL`
    - `description TEXT`
    - `lead_user_id BIGINT DEFAULT NULL (FK users.id)`
    - `created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`
  - Relasi `issues`:
    - Kolom `component_id BIGINT DEFAULT NULL` (foreign key referensi `components.id`)
* **Backend Endpoints:**
  - `GET /api/projects/:pid/components` : Daftar komponen proyek beserta nama lead dan jumlah tiket
  - `POST /api/projects/:pid/components` : Buat komponen baru
  - `PUT /api/components/:id` : Update komponen
  - `DELETE /api/components/:id` : Hapus komponen (otomatis meng-unassign komponen dari tiket terkait)
* **Frontend Modules & Components:**
  - `stores/component.js` : Pinia store reaktif untuk komponen proyek dengan live WebSocket sync
  - `components/backlog/ComponentManagerModal.vue` : Modal CRUD manajemen komponen dan penunjukan Component Lead
  - `components/issue/IssueMetaSidebar.vue` : Dropdown pemilihan Komponen saat pembuatan tiket baru maupun detail tiket
  - `components/kanban/KanbanCard.vue` : Tampilan badge komponen (cyan pill `🧩 Component Name`) pada kartu board
  - `components/backlog/BacklogView.vue` : Filter cepat berdasarkan komponen di toolbar Backlog

---

### 3. Versions / Releases (Release Hub) (🟢 SELESAI)
* **Tujuan:** Pelacakan versi rilis software (misal `v1.0.0`, `v1.1.0`), target tanggal peluncuran, pengelompokan pekerjaan berdasarkan target deployment (*Fix Version*), dan pemantauan rasio penyelesaian tiket (Done / In Progress / To Do).
* **Database Schema (MariaDB):**
  - Tabel `versions`:
    - `id BIGINT AUTO_INCREMENT PRIMARY KEY`
    - `project_id BIGINT NOT NULL (FK projects.id)`
    - `name VARCHAR(100) NOT NULL`
    - `description TEXT`
    - `start_date DATE DEFAULT NULL`
    - `release_date DATE DEFAULT NULL`
    - `status VARCHAR(20) NOT NULL DEFAULT 'UNRELEASED'` (`UNRELEASED`, `RELEASED`, `ARCHIVED`)
    - `created_at TIMESTAMP`, `updated_at TIMESTAMP`
  - Relasi `issues`:
    - Kolom `fix_version_id BIGINT DEFAULT NULL` (foreign key referensi `versions.id`)
* **Backend Endpoints:**
  - `GET /api/projects/:pid/versions` : Daftar versi rilis proyek dengan agregasi total tiket, todo, in progress, dan done
  - `POST /api/projects/:pid/versions` : Membuat versi baru
  - `PUT /api/versions/:id` : Memperbarui nama, deskripsi, tanggal mulai, dan target rilis
  - `POST /api/versions/:id/release` : Merilis versi (`RELEASED`), menetapkan tanggal rilis aktual, dan menyediakan opsi memindahkan tiket yang belum selesai ke versi berikutnya atau melepasnya dari versi
  - `POST /api/versions/:id/archive` : Toggle arsip versi (`ARCHIVED` / `UNRELEASED`)
  - `DELETE /api/versions/:id` : Menghapus versi (otomatis mengosongkan referensi `fix_version_id` pada tiket terkait)
* **Frontend Modules & Components:**
  - `stores/version.js` : Pinia store reaktif untuk versi rilis dengan live WebSocket sync
  - `components/releases/ReleasesView.vue` : Tampilan induk Release Hub dengan 4 kartu metrik (Total, Belum Dirilis, Dirilis, Diarsipkan), filter tab, dan pencarian cepat
  - `components/releases/ReleaseCard.vue` : Kartu versi komprehensif menampilkan status badge, indikator keterlambatan (overdue), tanggal, deskripsi, dan tombol aksi
  - `components/releases/ReleaseProgressBar.vue` : Bar progres 3 segmen warna (Hijau: Done, Biru: In Progress, Abu-abu: To Do) disertai persentase dan legenda
  - `components/releases/CreateVersionModal.vue` : Modal formulir pembuatan & pengeditan versi rilis
  - `components/releases/ReleaseVersionModal.vue` : Modal konfirmasi rilis standar Jira dengan deteksi tiket belum selesai dan opsi target migrasi
  - `components/issue/IssueMetaSidebar.vue` : Dropdown atribut `Fix Version (Rilis)` pada pembuatan tiket baru dan panel detail tiket
  - `components/kanban/KanbanCard.vue` : Badge visual rilis (indigo pill `🚀 v1.0.0`) pada kartu Kanban
  - `components/common/ViewTabs.vue` : Tab navigasi "Rilis" dan dropdown filter cepat berdasarkan Fix Version
  - `components/list/ListView.vue` : Kolom versi rilis dengan inline selector dan ekspor CSV yang menyertakan data Fix Version
  - `components/common/Sidebar.vue` : Pintasan "Rilis & Versi" pada sidebar proyek

---

### 4. Board Swimlanes (Grouping Horizontal) (🟢 SELESAI)
* **Tujuan:** Mengelompokkan kartu Kanban secara horizontal berdasarkan dimensi tertentu (Assignee, Epic, Priority/Expedite) untuk meningkatkan visibilitas beban kerja dan pemblokir kritis sesuai standar papan Jira Software.
* **Dimensi Swimlane:**
  1. **Tanpa Swimlane (Default):** Kolom alur kerja vertikal standar.
  2. **Berdasarkan Assignee:** Baris horizontal terpisah untuk setiap anggota tim beserta avatar dan jumlah tiket + baris penampung "Belum Ditugaskan (Unassigned)".
  3. **Berdasarkan Epic:** Baris horizontal untuk setiap Epic aktif + baris "Tiket Tanpa Epic".
  4. **Berdasarkan Prioritas (Expedite):** Baris teratas dengan styling visual khusus *rose/red alert* (`🚨 Expedite`) untuk tiket `HIGHEST` (jalur cepat tanggap darurat), disusul baris Prioritas Tinggi, Sedang, dan Rendah.
* **Frontend Modules & Components:**
  - `stores/project.js` : State `swimlaneMode` reaktif (`'none' | 'assignee' | 'epic' | 'priority'`).
  - `components/kanban/KanbanToolbar.vue` : Toolbar header board dengan penghitung tiket dan dropdown switcher swimlane 1-klik.
  - `components/kanban/KanbanColumn.vue` : Komponen kolom vertikal atomik untuk mode standar (bebas god-file).
  - `components/kanban/KanbanSwimlaneView.vue` : Orkestrator swimlane dengan header kolom sticky di bagian atas (menjaga label kolom tetap terlihat saat scroll ke bawah) dan grouping dinamis.
  - `components/kanban/KanbanSwimlaneRow.vue` : Baris swimlane horizontal yang dapat di-*collapse/expand*, menampilkan avatar/icon grup, badge judul, penghitung tiket, kalkulasi akumulasi Story Points per baris, dan sel dropzone mandiri per status.
  - `components/kanban/KanbanBoard.vue` : Integrasi terpusat dengan deteksi drag-and-drop antar sel swimlane dan alert penjaga workflow transition guard.

---

### 5. Issue Labels (Tagging Fleksibel) (🟢 SELESAI)
* **Tujuan:** Tagging bebas lintas tiket dan tipe (contoh: `security`, `frontend`, `hotfix`, `ux`, `api`) untuk kategorisasi ad-hoc tanpa membatasi hierarki modul atau versi.
* **Database Schema (MariaDB):**
  - Tabel `labels`:
    - `id BIGINT AUTO_INCREMENT PRIMARY KEY`
    - `project_id BIGINT NOT NULL (FK projects.id)`
    - `name VARCHAR(50) NOT NULL`
    - `color VARCHAR(20) DEFAULT '#6366f1'`
    - `created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP`
    - `UNIQUE KEY uk_project_label (project_id, name)`
  - Tabel pivot `issue_labels`:
    - `issue_id BIGINT NOT NULL (FK issues.id ON DELETE CASCADE)`
    - `label_id BIGINT NOT NULL (FK labels.id ON DELETE CASCADE)`
    - `PRIMARY KEY (issue_id, label_id)`
* **Backend Endpoints:**
  - `GET /api/projects/:pid/labels` : Daftar seluruh label aktif di proyek dengan jumlah tiket yang menggunakannya
  - `POST /api/projects/:pid/labels` : Membuat label baru atau mengembalikan label yang sudah ada
  - `DELETE /api/labels/:id` : Menghapus label (otomatis membersihkan referensi `issue_labels`)
  - `GET /api/issues/:id/labels` : Mengambil daftar label yang terpasang pada satu tiket
  - `POST /api/issues/:id/labels` : Menyinkronkan (bulk set) seluruh label pada tiket dengan auto-creation label baru jika belum ada
* **Frontend Modules & Components:**
  - `stores/label.js` : Pinia store reaktif untuk label dengan WebSocket sync
  - `components/issue/IssueLabelsEditor.vue` : Komponen atomik input multi-tag dengan autocomplete suggestions real-time, opsi "+ Buat label baru", tombol hapus cepat, dan warna pastel dinamis
  - `components/issue/IssueMetaSidebar.vue` : Integrasi editor label pada mode pembuatan tiket dan mode detail
  - `components/kanban/KanbanCard.vue` : Tampilan chips label (`🏷️ label`) pada kartu board
  - `components/backlog/BacklogIssueItem.vue` : Tampilan chips label pada baris item backlog
  - `components/common/ViewTabs.vue` : Dropdown filter label cepat di toolbar board
  - `components/list/ListView.vue` : Kolom data label dan ekspor CSV yang menyertakan informasi label

---

### 6. Issue Resolution (Alasan Selesai) (⚪ NEXT UP)
* **Tujuan:** Standar audit mutu software Jira — membedakan kapan suatu tiket masuk status berkategori `DONE` dengan alasan resolusi spesifik (`Fixed`, `Won't Fix`, `Duplicate`, `Cannot Reproduce`, `Incomplete`).
* **Database Schema (MariaDB):**
  - Kolom `resolution VARCHAR(50) DEFAULT NULL` pada tabel `issues`
  - Kolom `resolved_at TIMESTAMP DEFAULT NULL` pada tabel `issues`
* **Rencana Implementasi:**
  - `models/issue.rs` & `handlers/issues/transition.rs` (update transition logic untuk mengisi/menghapus resolution saat berpindah ke/dari status kategori DONE)
  - Modal atau dropdown pemilihan resolusi saat kartu digeser ke kolom DONE
  - Tampilan badge resolusi dan coret teks key (strikethrough `~~KEY-123~~`) saat tiket sudah memiliki resolution (standar Jira).

---

*(Catatan: Bagian 7 s.d. 8 akan dilanjutkan secara berurutan sesuai prioritas).*
