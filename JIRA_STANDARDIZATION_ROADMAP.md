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
| **3** | **Versions / Releases (Release Hub)** | P1 (Tinggi) | ⚪ *Next (Planned)* | `models/version.rs`<br>`handlers/versions.rs` | `components/releases/` | Pelacakan rilis software, tanggal rilis, status released/unreleased, ringkasan progres tiket. |
| **4** | **Board Swimlanes (Grouping Horizontal)** | P1 (Tinggi) | ⚪ *Planned* | N/A (Frontend Agregasi) | `components/kanban/` | Pembagian horizontal Kanban: Group by Assignee, Epic, atau Priority (Expedite). |
| **5** | **Issue Labels (Tagging Fleksibel)** | P2 (Menengah) | ⚪ *Planned* | `models/label.rs`<br>`handlers/labels.rs` | `components/issue/` | Tag bebas lintas tiket (contoh: `security`, `frontend`, `hotfix`, `ux`) dengan autocomplete & filter. |
| **6** | **Issue Resolution (Alasan Selesai)** | P2 (Menengah) | ⚪ *Planned* | Kolom `resolution` pada issues | `components/issue/` | Pembedaan status DONE dengan Resolution (`Fixed`, `Won't Fix`, `Duplicate`, `Cannot Reproduce`). |
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

### 3. Versions / Releases (Release Hub) (⚪ NEXT UP)
* **Tujuan:** Pelacakan versi rilis software (misal `v1.0.0`, `v1.1.0`), target tanggal rilis, dan kemajuan penyelesaian tiket dalam rilis.
* **Database Schema (MariaDB):**
  - Tabel `versions`: `id`, `project_id`, `name`, `description`, `release_date`, `is_released`, `is_archived`, `created_at`.
  - Kolom `fix_version_id` pada `issues`.
* **Rencana Implementasi:**
  - `models/version.rs` dan `handlers/versions.rs`
  - `stores/version.js` dan tab / modal rilis pada antarmuka.

---

*(Catatan: Bagian 4 s.d. 8 akan dilanjutkan secara berurutan sesuai prioritas).*
