# Week 1 — Proposal & Planning

## Project proposal (1–2 pages)

Project: Task Scheduler (desktop GUI)

Purpose
- Provide a small, cross-platform desktop application that lets users define simple recurring tasks (Hourly/Daily/Weekly), enable or disable them, persist task state, and run a background check that marks tasks as run or signals the user.

Scope (MVP)
- Create/list/delete tasks with: name, interval (Hourly/Daily/Weekly), enabled flag, last_run timestamp.
- Background checker that decides which tasks are due and updates last_run when they run.
- Persist tasks to disk as JSON in a platform-appropriate config directory.
- Provide a desktop GUI implemented with Iced: overview table, detail pane, new-task dialog, settings screen.

Out-of-scope
- Remote synchronization, user accounts, complex cron-like expressions, plugins.

Non-functional requirements
- Cross-platform support: Windows 10+, macOS, Linux (x86_64). Focus development on Windows.
- Responsiveness: UI must remain responsive while background checks and I/O run.
- Durability: Saves must be atomic to avoid data corruption.

Risks and mitigations
- UI/Platform differences: Keep layout simple and responsive; test on each target OS early.
- Blocking background work: Use iced subscriptions and async commands; never run I/O on the UI thread.
- Data corruption on crash: Save to temp file and rename atomically; keep backups if save fails.

Success criteria
- Users can add tasks, enable/disable them, and the background checker should identify due tasks and update last_run accordingly.
- Data persists across restarts and recovers gracefully when the file is missing or malformed.

## Wireframes (rough sketches)

Overview / Main

-------------------------------------------------------------
| NAV: [Overview] [New Task] [History] [Settings]            |
-------------------------------------------------------------
| Tasks                                         | Details     |
| -------------------------------------------- | ----------- |
| [ ] Task A (Daily)    | Last run: 2025-11-07 10:00  | Name: Task A |
| [x] Task B (Hourly)   | Enabled: Yes                 | Interval: ...|
| [ ] Task C (Weekly)   |                             | Last run: ...|
| ...                                            | Actions: ... |
-------------------------------------------------------------

New Task dialog

-----------------------------
| Task name: [____________]  |
| Interval: (Daily) v        |
| [Add]     [Cancel]         |
-----------------------------

Settings

----------------------------------
| Check interval: [ 60s ]         |
| Theme: (Light) v                |
| Log level: (Info) v             |
| [Save]                          |
----------------------------------

## Work plan (week-by-week with hour estimates)

Note: estimates are rough and assume a single developer working part-time on the project.

- Week 1 — Proposal & Planning (6–10h)
  - Produce this proposal, wireframes, and a week-by-week plan.
  - Deliverable: `milestones/week1_proposal_and_planning.md` (this file).

- Week 2 — Architecture & Spike (12–16h)
  - Choose crates: `iced`, `serde`, `serde_json`, `chrono`, `directories`, `anyhow`, `tracing` (+ `tracing_appender` for file logs).
  - Implement an Iced shell app that builds and shows one screen with mock data.
  - Deliverable: `ARCHITECTURE.md`, spike that proves background task integration.

- Week 3 — Data Layer & Scanning (12–18h)
  - Implement `Task` model and async persistence helpers (load/save tasks) with tolerant parsing.
  - Add unit tests for load/save using temp dirs; initialize basic structured logging.
  - Deliverable: data-layer module and unit tests.

- Week 4 — Core UI & Actions (18–24h)
  - Implement the table view with sorting and simple filtering.
  - Add a detail pane and one state-changing action (Delete) with confirmation dialog and error handling.
  - Deliverable: GUI with table, detail pane, delete flow.

- Week 5 — Persistence & Polish (12–18h)
  - Add settings screen and atomic save to `tasks.json` under platform config dir.
  - Add progress indicators for save/load and ensure long work runs asynchronously.
  - Deliverable: settings UI and persistence robustness improvements.

- Week 6 — Testing & Hardening (12–18h)
  - Add integration tests (where possible), fault-injection tests for persistence I/O errors.
  - Improve error messages and add keyboard shortcuts/accessibility basics.
  - Deliverable: test reports and fixes.

- Week 7 — Beta & Documentation (6–12h)
  - Tag beta release, create `USER_GUIDE.md` and `TEST_PLAN.md` with results/perf notes.
  - Deliverable: beta tag and documentation.

## Acceptance checklist for Week 1
- [ ] Proposal & wireframes committed in `milestones/`.
- [ ] Week plan added to repository and linked from milestone entry.

---

If you want this written or formatted differently (for GitHub milestone description, PDF, or a short README), tell me the target format and I will produce it.
