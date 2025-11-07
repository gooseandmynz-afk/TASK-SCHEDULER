# Week 7 — Beta & Documentation

Deliverables

- Beta release tag: create a `v0.1.0-beta` annotated tag and produce release notes.
- USER_GUIDE.md — installation, usage, troubleshooting (link to `docs/USER_GUIDE.md` if present).
- TEST_PLAN.md — include test results and a short performance analysis.

USER GUIDE (high level)

- Installation: cargo run (development) or provide prebuilt binaries for Windows/macOS/Linux.
- Usage: create tasks, enable them, and let background checker update `last_run`.
- Troubleshooting: check logs under config dir `/logs`, run `cargo run --features debug` to get verbose logs.

TEST PLAN (summary)

- Unit tests: data layer, serialization.
- Integration tests: end-to-end persistence and task checks.
- Fault injection: simulated save/load failures — results attached in `TEST_PLAN.md`.

Performance notes

- Measure background check latency with 100 tasks; ensure UI remains responsive; target <100ms scheduling check on typical workstation.
