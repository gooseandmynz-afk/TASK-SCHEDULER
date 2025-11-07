# Week 2 — Architecture & Spike

Deliverables

- `ARCHITECTURE.md` style summary for the app state model, message flow, data sources, and error-handling strategy (this file is a focused week-2 artifact).
- Iced shell application that builds successfully and displays one screen with mock data.

State model

- Root application state: `TaskScheduler` { screen, tasks, ui_state, settings, history }
- Each `Task` contains: name, interval, last_run (Option<DateTime>), enabled (bool)
- UI state: selection index, pending delete, transient dialogs, toasts

Message flow

- Use an Elm-like message model: `Message` enum represents all user actions and background events.
- UI events generate Commands which perform async work and return a `Message` when complete.
- Background tick subscription periodically sends `Tick` messages to check tasks.

Data sources

- Local JSON file at platform config dir (`directories::ProjectDirs`) named `tasks.json`.
- In later weeks we may add process/file scans as additional sources behind async boundaries.

Error handling strategy

- Surface recoverable errors in the UI via toasts and history log.
- For persistence errors: show friendly message and keep existing in-memory state intact.
- For background job errors: log (tracing) and show an unobtrusive notification.

Spike checklist

- Finalize crate selection: iced, chrono, serde/serde_json, anyhow, directories, tracing (+tracing_appender)
- Create a minimal Iced app that compiles and shows a mock tasks list (see `rust_week6_gui_start.rs`) — include mock data hard-coded or via `Command::none()`.
- Prove background task integration: subscription/time::every or spawn an async task returning via Command::perform.

Notes

- Keep dependencies minimal. Use `iced` latest 0.x stable that works with the user's toolchain.
