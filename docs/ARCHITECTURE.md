## Overview
Task Scheduler is a desktop app built with Rust using the Iced GUI framework.
The app works by listening to what you do, like clicks or typing, and updates the screen or tasks accordingly.


## Main Parts
### 1. App State (`TaskScheduler`)
Stores all the current app data:
- screen – which screen is showing (overview, new task, history)
- tasks – list of all tasks
- task_name / interval** – input for new tasks
- history – log of actions
- toasts – notifications that pop up
- theme – light or dark mode
- Background_running / progress – shows task checking progress


### 2. Messages
Messages are how the app reacts to user actions or events:
- Navigation: Switch screens
- Task Management: Add, delete, enable/disable tasks
- Operations: Save, load tasks
- UI: Toggle theme, sort tasks, show notifications
- Background: Check tasks automatically


Example flow:
User clicks Add Task -> Message::AddTask -> tasks list updates -> screen refreshes

### 3. Features
- Task management: Add, delete, enable/disable tasks
- Automatic checking: Tasks check every minute
- Sorting & Theme: Sort tasks and switch light/dark mode
- Notifications: Shows messages for important events
- History: Keeps track of actions


### 4. Persistence
Tasks and settings are saved to a JSON file in your user directory:
- Windows: `%APPDATA%\task_scheduler_gui\tasks.json`
- Mac: `~/Library/Application Support/task_scheduler_gui/tasks.json`
- Linux: `~/.config/task_scheduler_gui/tasks.json`


### 5. Error Handling
- Problems saving or loading show a toast message
- Logs are stored in the same folder as tasks
- The app tries to recover from temporary issues automatically


### 6. Data Flow
Simple steps the app follows:
User action -> Message -> Update app state -> Refresh screen -> Save/Load as needed


### 7. Performance
- Tasks checking and saving run in the background
- UI stays responsive
- Works even with many tasks


### 8. Security
- Only writes files in your own user folder
- Checks task names and file contents for safety


### 9. Future Ideas
- More themes and settings
- Connect to calendars or notifications
- Add more task intervals
