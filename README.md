# Task Scheduler

A desktop application built with Rust and Iced for managing recurring tasks with configurable intervals.

## Features

- Task management with hourly, daily, and weekly intervals
- Automatic task status tracking
- Live progress updates
- Dark/Light theme support
- Keyboard shortcuts
- Persistent storage
- Structured logging
- Toast notifications
- Responsive layout

## Prerequisites

- Rust (stable channel)
- Cargo package manager

## Building

```bash
# Clone the repository
git clone <repository-url>
cd task_scheduler

# Build the project
cargo build

# Run in debug mode
cargo run

# Build for release
cargo build --release
```

## Running

```bash
# Run the debug build
cargo run

# Run the release build
./target/release/iced_task_scheduler
```

## Keyboard Shortcuts

- `Ctrl+S`: Save tasks
- `Ctrl+N`: New task
- `Ctrl+H`: View history
- `Ctrl+O`: Overview screen

## Development

```bash
# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy lints
cargo clippy

# Run with logging enabled
$env:RUST_LOG="debug"  # PowerShell
cargo run
```

## Documentation

- [User Guide](./docs/userguide.md)
- [Test Plan](./docs/testplan.md)
- [Architecture](./docs/architecture.md)
