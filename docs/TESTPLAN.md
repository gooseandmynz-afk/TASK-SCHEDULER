### Task Tests
- Make a new task
- Try making a task with no name (should fail)
- Check default values (enabled = false, last run = none)
- Save task to file (JSON)
- Load task from file
- Check all intervals (Hourly, Daily, Weekly)
- Make sure disabled tasks don’t run
- App State Tests
- Check messages work
- Check sorting tasks works
- Switch theme and see it stays


### File Tests
- Save tasks to file
- Load tasks from file
- Try loading missing or broken files


### UI Tests
- Switch screens
-  Use keyboard shortcuts
-   Show notification messages (toasts)


## Integration Tests
- Make task, enable it, see it run automatically, disable it, delete it
- Save multiple tasks, restart app, check tasks are still there


## Performance Tests
- Try with 100+ tasks, check sorting and scrolling
- Make sure saving in the background doesn’t block the app


## Acceptance Tests
- Do full task workflow: add, run, delete tasks
- Switch theme
- Check history
- Test on Windows, Linux, macOS


## Running Tests
# Run all tests
cargo test
# Run one test
cargo test test_name
# Run tests with output
cargo test -- --nocapture
