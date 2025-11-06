# User Guide

## Overview

The Task Scheduler is a desktop application that helps you manage and track recurring tasks. This guide covers all features and how to use them effectively.

## Screens

### Overview Screen

The main screen shows all your tasks in a sortable list:

- **Status**: Toggle task enabled/disabled
- **Task Name**: Name of the task
- **Interval**: How often the task should run (Hourly/Daily/Weekly)
- **Last Run**: When the task was last executed
- **Actions**: Delete button for task removal

Features:

- Sort tasks by name (click Sort ↑/↓)
- Add new tasks (+ New Task button)
- Delete tasks (with confirmation)

### New Task Screen

Create new tasks with:

- Task name input
- Interval selection (Hourly/Daily/Weekly)
- Add/Cancel buttons

### History Screen

View a chronological log of:

- Task additions
- Task deletions
- Task checks
- Save/Load operations

## Features

### Task Management

1. **Adding Tasks**:

   - Click "+ New Task" or press Ctrl+N
   - Enter task name
   - Select interval
   - Click "Add"

2. **Enabling/Disabling Tasks**:

   - Click checkbox in Status column
   - Disabled tasks won't be checked automatically

3. **Deleting Tasks**:
   - Click "Delete" button
   - Confirm in dialog

### Automatic Checking

- Tasks are checked every minute
- Only enabled tasks are checked
- Tasks run based on their interval since last run

### Theme Support

- Toggle between Light/Dark theme
- Theme persists between sessions

### Progress Tracking

- Background task progress shown in status bar
- ASCII progress bar indicates completion
- Non-blocking operation

### Notifications

- Toast notifications for important events
- Appears in top-right
- Auto-dismisses after 3 seconds

### Keyboard Shortcuts

| Shortcut | Action          |
| -------- | --------------- |
| Ctrl+S   | Save tasks      |
| Ctrl+N   | New task        |
| Ctrl+H   | View history    |
| Ctrl+O   | Overview screen |

### Persistence

- Tasks auto-save after changes
- Manual save available (Save button)
- Load button to refresh from disk
- Stored in user config directory

## Troubleshooting

1. **Task not running**:

   - Check if task is enabled
   - Verify last run time
   - Ensure interval is appropriate

2. **Changes not saving**:

   - Check history for save errors
   - Try manual save
   - Verify write permissions

3. **UI issues**:
   - Try toggling theme
   - Restart application
   - Check log files

## Logs

Log files are stored in:

- Windows: `%APPDATA%\task_scheduler_gui\logs`
- macOS: `~/Library/Application Support/task_scheduler_gui/logs`
- Linux: `~/.config/task_scheduler_gui/logs`
