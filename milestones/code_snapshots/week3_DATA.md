# Week 3 — Data Layer & Scanning

Goals

- Implement a real data retrieval layer behind async boundaries. For the scheduler this means reading/writing `tasks.json` and offering a test data source such as a filesystem or process list scan.
- Add unit tests for the data layer and initial structured logging.

Design

- `data::store` module — exposes `async fn load_tasks() -> Result<Vec<Task>>` and `async fn save_tasks(tasks: Vec<Task>) -> Result<()>`.
- Keep serialization with `serde` and tolerant deserialization (back-compat).
- Ensure I/O runs off the UI thread: `Command::perform(load_tasks_cmd(), Message::LoadResult)`.

Example: file scan source (stub)

```rust
async fn scan_files(path: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut out = Vec::new();
    for entry in tokio::fs::read_dir(path).await? {
        let e = entry?;
        out.push(e.path());
    }
    Ok(out)
}
```

Testing

- Unit tests for load/save should:
  - Use a tempdir to write a sample `tasks.json` and confirm round-trip.
  - Test tolerant parsing of older formats.

Logging

- Initialize `tracing_subscriber` in app startup; write human logs to stdout during development and JSON files for release runs.
