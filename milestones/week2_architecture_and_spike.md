# Week 2 - Architecture & Spike

## Architecture Documentation

### State Model

```
App State
├── Screen (Overview | NewTask | History)
├── Tasks[]
│   ├── name: String
│   ├── interval: Interval
│   ├── last_run: Option<DateTime>
│   └── enabled: bool
├── UI State
│   ├── task_name: String
│   ├── interval: Interval
│   ├── is_saving: bool
│   └── is_loading: bool
└── Notifications
    └── toasts: Vec<(String, Instant)>
```

### Message Flow

1. User Actions → Messages
2. Messages → State Updates
3. State Updates → View Updates
4. Background Tasks → Message Generation

### Data Sources

1. File System

   - Tasks persistence (JSON)
   - Configuration
   - Logs

2. System Time
   - Task scheduling
   - Last run tracking

### Error Handling Strategy

1. User-facing Errors

   - Toast notifications
   - Status messages
   - Dialog boxes

2. System Errors
   - Structured logging
   - Error context preservation
   - Graceful degradation

## Spike Implementation Status

- [x] Iced shell application builds
- [x] Overview screen with mock data
- [x] Background task integration proof
- [x] Crate selection finalized

### Selected Crates

- iced: UI framework
- serde: Serialization
- chrono: Time management
- anyhow: Error handling
- tracing: Logging
