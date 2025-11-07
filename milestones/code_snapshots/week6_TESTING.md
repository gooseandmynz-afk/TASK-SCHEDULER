# Week 6 — Testing & Hardening

Objectives

- Add integration tests and fault-injection tests to simulate I/O failures.
- Improve error messages and add basic accessibility considerations.

Integration tests

- Use `assert_cmd` or similar to run the CLI variant; for GUI, use unit tests of view-model logic.
- End-to-end tests: create temp config, run the scheduler check routine, validate `last_run` updates.

Fault injection

- Provide a test harness that substitutes an I/O layer which can fail on demand (e.g., return Err on save).
- Verify UI handles save failures gracefully (toast + retry suggestion).

Accessibility

- Use descriptive labels and provide keyboard shortcuts for primary actions (Ctrl+S, Ctrl+N).
- Ensure text sizes are legible; ensure high-contrast theme available.
