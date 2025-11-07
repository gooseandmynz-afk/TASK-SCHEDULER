# Week 5 — Persistence & Polish

Goals

- Add robust configuration persistence and a settings screen.
- Improve UX polish: progress indicators and non-blocking behavior.

Persistence

- Store `tasks.json` under `directories::ProjectDirs::config_dir()`.
- Save atomically: write to a temp file and rename.
- When saving, keep a retry/backoff strategy for transient I/O errors.

Settings screen

- Allow user to change: check interval (seconds/minutes), theme preference (light/dark), log level.
- Changes should persist to a separate `settings.json`.

Progress indicators

- Show small "Saving..."/"Loading..." indicators in the footer.
- Use `Command::perform` and show progress until the future resolves.

Non-blocking UX

- Any blocking system operations must run via `Command::perform` and not on the main thread.
