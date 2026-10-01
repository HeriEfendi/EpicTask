# EpicTask Changelog

## [v0.1.3] - 2026-10-01

### 🚀 Fitur Baru (Features)
- integrate vue-toastification for professional toast notifications (9e84937)
- standardize CreateIssueModal with RichTextEditor and clean up ListView inline row (a539462)

### ⚡ Peningkatan & Refactoring
- align description label and editor hint horizontally in IssueDetailModal (f042e25)
- unify IssueDetailModal and CreateIssueModal into single consolidated component (835b091)

## [v0.1.2] - 2026-10-01

### 🐛 Perbaikan Masalah (Bug Fixes)
- prevent grep pipefail exit on first release with no previous tags (fca2312)

## [v0.1.1] - 2026-10-01

### 🚀 Fitur Baru (Features)
- add automated release script and GitHub Actions CI/CD workflow (f7143af)
- add Word-like rich text editor with tables and image paste, expand edit modal to 1440px (c5c66ce)
- redesign 2-row layout with right-aligned tools and multi-assignee filter dropdown (9fd1448)
- add executive summary, velocity chart, CFD, and timesheet reports view (591b425)
- expand issue description to LONGTEXT and add 1-year historical seed dataset (6e02cb2)
- implement analytics & reporting endpoints (overview, velocity, CFD, timesheet) (c23ae45)
- add infinite scroll with 20-row batches and footer row counter (66baa9e)
- add sidebar, streamline header, and enlarge font sizes across app (c8badc1)
- refine light theme button tones, inputs, and components (37ca176)

### ⚡ Peningkatan & Refactoring
- navbar full-width on top, sidebar open/hide only (2-state), no navbar shift (6bcb610)
- require env vars in config, simplify navbar breadcrumb to project-only, add settings modal (2570bf3)
- streamline user avatar in navbar to image-only button (52ebbbb)

Semua perubahan dan histori rilis dicatat di sini secara otomatis.

