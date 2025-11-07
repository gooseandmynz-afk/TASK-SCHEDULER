# Week 4 — Core UI & Actions

Objectives

- Implement the main table view with sort and filter capabilities.
- Add a detail pane that shows task metadata when a row is selected.
- Implement one state-changing action (Delete with confirmation) and proper error handling.

Table view

- Columns: Status (checkbox), Task Name, Interval, Last Run, Actions
- Sorting: toggle ascending/descending by Name; future: sort by Last Run.
- Filtering: simple substring filter on task name (search box).

Detail pane

- Shows Name, Interval, Last Run, Enabled status, and buttons for Edit/Delete.

State-changing action

- Delete flow:
  - User clicks Delete -> `Message::DeletePressed(idx)` -> set `pending_delete = Some(idx)`
  - Show modal confirmation dialog
  - On confirm -> remove task from in-memory list and attempt `save_tasks_cmd`
  - On save error -> restore removed item and display error toast

Error handling & UX

- Dialogs and toasts should provide clear instructions.
- Long-running actions should show progress and never block the UI thread.
