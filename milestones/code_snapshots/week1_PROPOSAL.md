# Week 1 — Proposal & Planning

Project: Task Scheduler (Iced GUI)

Summary (1–2 pages)

- Goal: Build a cross-platform desktop Task Scheduler that allows users to define lightweight recurring tasks (Hourly/Daily/Weekly), persist them, and run background checks to perform or signal task runs. The UI will be implemented with Iced; persistence will use JSON config under platform-appropriate config dirs.
- Scope: Core features for MVP:
  - Create/list/delete tasks with a name and interval
  - Enable/disable tasks
  - Background checker that decides which tasks are due
  - Persist tasks to disk (atomic save)
  - Basic GUI with task table, detail pane, and settings
- Out of scope for MVP: remote sync, advanced scheduling expressions, plugin system.

Target OS

- Primary: Windows 10+ (developer workstation). Secondary: macOS and Linux (x86_64). Use cross-platform crates (iced, directories, serde).

Risks & Mitigations

- UI complexity / layout differences across platforms — target a simple responsive layout, test on each platform weekly.
- Background task concurrency causing UI freeze — use iced subscriptions and async commands; keep heavy work off the UI thread.
- Data corruption on crash while saving — write to a temp file and rename atomically.

Wireframes (rough ASCII)

## Overview screen

| Tasks      | Details                   |
| ---------- | ------------------------- |
| [x] Task A | Name: Task A              |
| [ ] Task B | Interval: Daily           |
| ...        | Last run: 2025-11-07 10:0 |

---

## New Task dialog

| Task name: [________] |
| Interval: (Daily) v |
| [Add] [Cancel] |

---

Work plan (week-by-week, rough hours)

- Week 1 — Proposal & Planning: 6–10h (proposal, wireframes, week plan)
- Week 2 — Architecture & Spike: 12–16h (choose crates, iced shell, background task spike)
- Week 3 — Data Layer & Scanning: 12–18h (async data retrieval, unit tests, logging)
- Week 4 — Core UI & Actions: 18–24h (table view, detail pane, one state-changing action)
- Week 5 — Persistence & Polish: 12–18h (settings, atomic save, progress indicators)
- Week 6 — Testing & Hardening: 12–18h (integration tests, fault injection, accessibility)
- Week 7 — Beta & Documentation: 6–12h (tag release, docs, test results, performance notes)

Acceptance criteria for Week 1

- Signed-off proposal document here.
- Wireframes included.
- A realistic week-by-week plan with hour estimates.
