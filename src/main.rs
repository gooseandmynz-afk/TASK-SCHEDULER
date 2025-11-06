// ---------- Imports ----------
use anyhow::{anyhow, Context};
use chrono::{DateTime, Local};
use directories::ProjectDirs;
use iced::keyboard;
use iced::theme::Theme;
use iced::widget::{
    button, checkbox, column, container, horizontal_space, pick_list, row, scrollable, text,
    text_input,
};
use iced::Command;
use iced::{executor, time, Element, Length, Subscription};
use iced::{Alignment, Application};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::{error, info};
use tracing_appender::{non_blocking, rolling};

// ---------- Messages ----------
#[derive(Debug, Clone)]
enum Message {
    SwitchTo(Screen),
    ToggleSort,
    ToastTick,
    TaskCheckComplete(Vec<(usize, bool)>),
    TaskNameChanged(String),
    IntervalChanged(Interval),
    AddTask,
    DeletePressed(usize),
    ConfirmDelete,
    CancelDelete,
    Save,
    SaveResult(Result<(), String>),
    Load,
    LoadResult(Result<Vec<Task>, String>),

    ToggleEnable(usize),
    Tick,
    StartBackground,
    BackgroundTick,
    ToggleTheme,
    KeyPress {
        key: keyboard::Key,
        modifiers: keyboard::Modifiers,
    },
}

// ---------- Screens ----------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Overview,
    NewTask,
    History,
}

// ---------- Interval ----------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Interval {
    Hourly,
    Daily,
    Weekly,
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Hourly => write!(f, "Hourly"),
            Interval::Daily => write!(f, "Daily"),
            Interval::Weekly => write!(f, "Weekly"),
        }
    }
}

// ---------- Task ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    name: String,
    interval: Interval,
    last_run: Option<DateTime<Local>>,
    enabled: bool,
}

impl Task {
    fn new(name: impl Into<String>, interval: Interval) -> Self {
        Self {
            name: name.into(),
            interval,
            last_run: None,
            enabled: false, // Tasks are disabled by default
        }
    }
}

// ---------- Toast ----------
// ---------- App ----------
struct TaskScheduler {
    screen: Screen,
    tasks: Vec<Task>,
    task_name: String,
    interval: Interval,
    history: Vec<String>,
    is_saving: bool,
    is_loading: bool,
    pending_delete: Option<usize>,
    // Toast notifications: (message, created_at)
    toasts: Vec<(String, Instant)>,
    // Keep the tracing-appender guard alive so the non-blocking worker can flush on drop
    log_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
    sort_asc: bool,
    // Background task state (simulated long-running job)
    background_running: bool,
    background_progress: f32, // 0.0..=1.0
    // Theming
    is_dark: bool,
}

// ---------- Helper Functions ----------
async fn check_tasks(tasks: Vec<Task>) -> Vec<(usize, bool)> {
    let now = Local::now();
    let mut results = Vec::new();

    for (idx, task) in tasks.iter().enumerate() {
        if !task.enabled {
            continue;
        }

        let should_run = if let Some(last) = task.last_run {
            match task.interval {
                Interval::Hourly => now.signed_duration_since(last).num_hours() >= 1,
                Interval::Daily => now.signed_duration_since(last).num_days() >= 1,
                Interval::Weekly => now.signed_duration_since(last).num_weeks() >= 1,
            }
        } else {
            true // Never run before
        };

        if should_run {
            results.push((idx, true));
        }
    }

    results
}

async fn save_tasks_cmd(tasks: Vec<Task>) -> Result<(), anyhow::Error> {
    let path = project_file_path().ok_or_else(|| anyhow!("no config dir"))?;
    let json = serde_json::to_string_pretty(&tasks).context("serializing tasks")?;
    // Write atomically: write to temp file and then rename
    let mut tmp = path.clone();
    tmp.set_extension("json.tmp");
    fs::write(&tmp, &json).with_context(|| format!("writing temp file {}", tmp.display()))?;
    fs::rename(&tmp, &path)
        .with_context(|| format!("renaming {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}

async fn load_tasks_cmd() -> Result<Vec<Task>, anyhow::Error> {
    let path = project_file_path().ok_or_else(|| anyhow!("no config dir"))?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(&path)
        .with_context(|| format!("reading tasks file {}", path.display()))?;
    // Try direct deserialization first (normal case)
    if let Ok(list) = serde_json::from_str::<Vec<Task>>(&data) {
        return Ok(list);
    }

    // Fallback: be tolerant of older/variant formats. Try parsing manually.
    let v: serde_json::Value = serde_json::from_str(&data).context("parsing tasks JSON")?;
    let arr = v
        .as_array()
        .ok_or_else(|| anyhow!("expected an array of tasks"))?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let name = item
            .get("name")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();

        let interval = match item.get("interval").and_then(|s| s.as_str()) {
            Some("Hourly") | Some("hourly") => Interval::Hourly,
            Some("Weekly") | Some("weekly") => Interval::Weekly,
            _ => Interval::Daily,
        };

        let enabled = item
            .get("enabled")
            .and_then(|b| b.as_bool())
            .unwrap_or(false);

        let last_run = item.get("last_run").and_then(|lr| {
            if lr.is_null() {
                None
            } else if let Some(s) = lr.as_str() {
                // Try parsing as RFC3339 first
                if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                    Some(dt.with_timezone(&Local))
                } else {
                    None
                }
            } else {
                None
            }
        });

        out.push(Task {
            name,
            interval,
            last_run,
            enabled,
        });
    }

    Ok(out)
}

fn project_file_path() -> Option<PathBuf> {
    if let Some(dirs) = ProjectDirs::from("com", "example", "task_scheduler_gui") {
        let dir = dirs.config_dir();
        let _ = fs::create_dir_all(dir);
        let mut path = dir.to_path_buf();
        path.push("tasks.json");
        Some(path)
    } else {
        None
    }
}

// ---------- View Implementations ----------
impl TaskScheduler {
    fn view_overview(&self) -> Element<'_, Message> {
        // Responsive header: title takes larger portion, actions are grouped on the right
        let header_actions = row![
            button(if self.sort_asc {
                "Sort ↓"
            } else {
                "Sort ↑"
            })
            .on_press(Message::ToggleSort),
            button("+ New Task").on_press(Message::SwitchTo(Screen::NewTask)),
        ]
        .spacing(8);

        let page_header = row![
            container(text("Tasks").size(32)).width(Length::FillPortion(3)),
            horizontal_space().width(Length::FillPortion(1)),
            header_actions,
        ]
        .padding(10)
        .spacing(10)
        .width(Length::Fill);

        // Column headers
        let headers = row![
            text("Status").width(Length::FillPortion(1)),
            text("Task Name").width(Length::FillPortion(4)),
            text("Interval").width(Length::FillPortion(1)),
            text("Last Run").width(Length::FillPortion(2)),
            text("Actions").width(Length::FillPortion(1))
        ]
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center);

        // Create a sorted copy of task indices
        let mut task_indices: Vec<usize> = (0..self.tasks.len()).collect();
        task_indices.sort_by(|&a, &b| {
            let ordering = self.tasks[a].name.cmp(&self.tasks[b].name);
            if self.sort_asc {
                ordering
            } else {
                ordering.reverse()
            }
        });

        let tasks: Vec<Element<Message>> = task_indices
            .iter()
            .map(|&idx| {
                let task = &self.tasks[idx];
                container(
                    row![
                        checkbox("", task.enabled)
                            .width(Length::FillPortion(1))
                            .on_toggle(move |_| Message::ToggleEnable(idx)),
                        text(&task.name).width(Length::FillPortion(4)),
                        text(&task.interval.to_string()).width(Length::FillPortion(1)),
                        text(if let Some(last) = task.last_run {
                            last.format("%Y-%m-%d %H:%M:%S").to_string()
                        } else {
                            "Never".into()
                        })
                        .width(Length::FillPortion(2)),
                        button("Delete")
                            .width(Length::FillPortion(1))
                            .on_press(Message::DeletePressed(idx))
                    ]
                    .spacing(10)
                    .padding(10)
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .into()
            })
            .collect();

        let tasks_list = column(tasks).spacing(5);

        let tasks_container: Element<Message> = if self.tasks.is_empty() {
            container(
                text("No tasks yet. Click '+ New Task' to create one.")
                    .size(16)
                    .style(iced::theme::Text::Color([0.5, 0.5, 0.5].into())),
            )
            .width(Length::Fill)
            .center_x()
            .padding(40)
            .into()
        } else {
            scrollable(container(tasks_list).width(Length::Fill).padding(1))
                .height(Length::Fill)
                .into()
        };

        let content = column![page_header, headers, tasks_container]
            .spacing(0)
            .width(Length::Fill);

        let mut page = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x();

        // Add delete confirmation dialog if needed
        if let Some(idx) = self.pending_delete {
            if idx < self.tasks.len() {
                // Center the dialog with a semi-transparent overlay
                let dialog = container(
                    container(
                        column![
                            text(format!("Delete task '{}'?", self.tasks[idx].name)).size(18),
                            row![
                                button(text("Cancel"))
                                    .style(iced::theme::Button::Secondary)
                                    .on_press(Message::CancelDelete),
                                button(text("Delete"))
                                    .style(iced::theme::Button::Destructive)
                                    .on_press(Message::ConfirmDelete)
                            ]
                            .spacing(10)
                            .padding(10)
                        ]
                        .spacing(20)
                        .align_items(Alignment::Center),
                    )
                    .style(iced::theme::Container::Box)
                    .padding(20)
                    .max_width(400) // Limit dialog width
                    .center_x()
                    .center_y(),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y();

                page = container(column![page, dialog])
                    .width(Length::Fill)
                    .height(Length::Fill);
            }
        }

        page.into()
    }

    fn view_new_task(&self) -> Element<'_, Message> {
        let content = container(
            column![
                text("New Task").size(24),
                text_input("Task name", &self.task_name).on_input(Message::TaskNameChanged),
                pick_list(
                    vec![Interval::Hourly, Interval::Daily, Interval::Weekly],
                    Some(self.interval),
                    Message::IntervalChanged
                ),
                row![
                    button("Cancel").on_press(Message::SwitchTo(Screen::Overview)),
                    button("Add").on_press(Message::AddTask)
                ]
                .spacing(10)
            ]
            .spacing(10)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding(20)
        .center_x();

        content.into()
    }

    fn view_history(&self) -> Element<'_, Message> {
        let content = container(
            column![
                text("History").size(24),
                scrollable(
                    column(
                        self.history
                            .iter()
                            .map(|h| text(h).into())
                            .collect::<Vec<Element<Message>>>(),
                    )
                    .spacing(5)
                )
                .height(Length::Fill),
                button("Back").on_press(Message::SwitchTo(Screen::Overview))
            ]
            .spacing(10)
            .width(Length::Fill),
        )
        .width(Length::Fill)
        .padding(20)
        .center_x();

        content.into()
    }
}

// ---------- Application Implementation ----------
impl Application for TaskScheduler {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        // Setup structured JSON logging to file if possible
        let mut log_guard: Option<tracing_appender::non_blocking::WorkerGuard> = None;
        if let Some(dirs) = ProjectDirs::from("com", "example", "task_scheduler_gui") {
            let log_dir = dirs.config_dir().join("logs");
            let _ = fs::create_dir_all(&log_dir);
            // Rolling daily file appender
            let file_appender = rolling::daily(log_dir, "task_scheduler.log");
            let (non_blocking, guard) = non_blocking(file_appender);
            // Use structured JSON output for file logs so they are machine-readable.
            // Keep the non-blocking guard alive on the app state so logs flush on drop.
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .json() // Emit structured JSON lines
                .with_thread_names(true)
                .with_thread_ids(true)
                .with_target(true)
                .with_ansi(false)
                .with_writer(non_blocking)
                .init();
            log_guard = Some(guard);
        } else {
            // Fallback to console (human readable) logging for development runs.
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_ansi(true)
                .with_thread_names(true)
                .with_thread_ids(true)
                .with_target(true)
                .init();
        }

        let app = Self {
            screen: Screen::Overview,
            tasks: vec![],
            task_name: String::new(),
            interval: Interval::Daily,
            history: vec!["App started.".into()],
            is_saving: false,
            is_loading: false,
            pending_delete: None,
            toasts: Vec::new(),
            log_guard,
            sort_asc: true,
            background_running: false,
            background_progress: 0.0,
            is_dark: false,
        };

        (
            app,
            Command::perform(
                async { load_tasks_cmd().await.map_err(|e| e.to_string()) },
                Message::LoadResult,
            ),
        )
    }

    fn title(&self) -> String {
        format!("Rust Task Scheduler — {:?}", self.screen)
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subs: Vec<Subscription<Message>> = vec![
            time::every(Duration::from_secs(60)).map(|_| Message::Tick),
            keyboard::on_key_press(|key, modifiers| Some(Message::KeyPress { key, modifiers })),
        ];

        // Small tick used to drive toast expiration
        subs.push(time::every(Duration::from_millis(500)).map(|_| Message::ToastTick));

        // When a background job is running, also subscribe to a faster tick to drive progress
        if self.background_running {
            subs.push(time::every(Duration::from_millis(300)).map(|_| Message::BackgroundTick));
        }

        Subscription::batch(subs)
    }

    fn theme(&self) -> Self::Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let nav = row![
            button("Overview").on_press(Message::SwitchTo(Screen::Overview)),
            button("New Task").on_press(Message::SwitchTo(Screen::NewTask)),
            button("History").on_press(Message::SwitchTo(Screen::History)),
            button("Save").on_press(Message::Save),
            button("Load").on_press(Message::Load),
            button("Start Long Task").on_press(Message::StartBackground),
            button(if self.is_dark {
                "Light Theme"
            } else {
                "Dark Theme"
            })
            .on_press(Message::ToggleTheme)
        ]
        .spacing(8);

        let status = row![
            text(format!("Tasks: {}", self.tasks.len())),
            text(if self.is_saving { "Saving..." } else { "" }),
            text(if self.is_loading { "Loading..." } else { "" }),
            // Background progress indicator (simple textual + ascii bar)
            text(if self.background_running {
                let pct = (self.background_progress * 100.0).round();
                let filled = (self.background_progress * 20.0).round() as usize;
                let bar = format!(
                    "[{}{}] {}%",
                    "#".repeat(filled),
                    " ".repeat(20 - filled),
                    pct as i32
                );
                bar
            } else if (self.background_progress - 1.0).abs() < std::f32::EPSILON {
                "Background: Done".into()
            } else {
                String::new()
            })
        ]
        .spacing(16)
        .width(Length::Fill);

        let notification = if self.is_saving {
            text("Saving tasks...")
                .size(16)
                .style(iced::theme::Text::Color([0.0, 0.6, 0.0].into()))
        } else if self.is_loading {
            text("Loading tasks...")
                .size(16)
                .style(iced::theme::Text::Color([0.0, 0.0, 0.8].into()))
        } else {
            text("")
        };

        // Render active toasts on the right side
        let toasts_widget = column(
            self.toasts
                .iter()
                .rev()
                .map(|(m, _)| {
                    container(text(m))
                        .padding(6)
                        .style(iced::theme::Container::Box)
                        .into()
                })
                .collect::<Vec<Element<Message>>>(),
        )
        .spacing(6);

        let footer = row![status, notification, horizontal_space(), toasts_widget]
            .spacing(16)
            .padding(8);

        let body = match self.screen {
            Screen::Overview => self.view_overview(),
            Screen::NewTask => self.view_new_task(),
            Screen::History => self.view_history(),
        };

        container(
            column![nav, body, footer]
                .spacing(12)
                .padding(12)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Tick => {
                self.history.push(format!(
                    "[{}] Checking tasks...",
                    Local::now().format("%H:%M:%S")
                ));
                let tasks = self.tasks.clone();
                Command::perform(check_tasks(tasks), Message::TaskCheckComplete)
            }
            Message::TaskCheckComplete(updates) => {
                let mut any_updates = false;
                for (idx, _) in updates {
                    if idx < self.tasks.len() {
                        any_updates = true;
                        self.tasks[idx].last_run = Some(Local::now());
                        self.history.push(format!(
                            "[{}] Task '{}' checked",
                            Local::now().format("%H:%M:%S"),
                            self.tasks[idx].name
                        ));
                    }
                }
                if any_updates {
                    let tasks_clone = self.tasks.clone();
                    Command::perform(
                        async move { save_tasks_cmd(tasks_clone).await.map_err(|e| e.to_string()) },
                        Message::SaveResult,
                    )
                } else {
                    Command::none()
                }
            }
            Message::SwitchTo(s) => {
                self.screen = s;
                Command::none()
            }
            Message::TaskNameChanged(name) => {
                self.task_name = name;
                Command::none()
            }
            Message::ToggleSort => {
                self.sort_asc = !self.sort_asc;
                Command::none()
            }

            Message::ToggleTheme => {
                self.is_dark = !self.is_dark;
                Command::none()
            }

            Message::ToastTick => {
                // Remove expired toasts (3s lifetime)
                let now = Instant::now();
                self.toasts
                    .retain(|(_, t)| now.duration_since(*t) < Duration::from_secs(3));
                Command::none()
            }

            Message::IntervalChanged(interval) => {
                self.interval = interval;
                Command::none()
            }
            Message::AddTask => {
                if !self.task_name.trim().is_empty() {
                    let task = Task::new(self.task_name.trim(), self.interval);
                    self.tasks.push(task);
                    self.history.push(format!(
                        "[{}] Added '{}'",
                        Local::now().format("%H:%M:%S"),
                        self.task_name
                    ));
                    self.task_name.clear();
                    self.screen = Screen::Overview;
                    // show a toast and save
                    self.toasts.push(("Task added".into(), Instant::now()));
                    let tasks_clone = self.tasks.clone();
                    return Command::perform(
                        async move { save_tasks_cmd(tasks_clone).await.map_err(|e| e.to_string()) },
                        Message::SaveResult,
                    );
                } else {
                    Command::none()
                }
            }
            Message::DeletePressed(idx) => {
                self.pending_delete = Some(idx);
                Command::none()
            }
            Message::ConfirmDelete => {
                if let Some(idx) = self.pending_delete.take() {
                    if idx < self.tasks.len() {
                        let removed = self.tasks.remove(idx);
                        self.history.push(format!(
                            "[{}] Deleted '{}'",
                            Local::now().format("%H:%M:%S"),
                            removed.name
                        ));
                        // show toast for deletion and save
                        self.toasts.push(("Task deleted".into(), Instant::now()));
                        let tasks_clone = self.tasks.clone();
                        return Command::perform(
                            async move { save_tasks_cmd(tasks_clone).await.map_err(|e| e.to_string()) },
                            Message::SaveResult,
                        );
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
            Message::CancelDelete => {
                self.pending_delete = None;
                Command::none()
            }
            Message::Save => {
                self.is_saving = true;
                if let Some(p) = project_file_path() {
                    self.history.push(format!(
                        "[{}] Saving to {}...",
                        Local::now().format("%H:%M:%S"),
                        p.display()
                    ));
                } else {
                    self.history.push(format!(
                        "[{}] Saving: no config dir",
                        Local::now().format("%H:%M:%S")
                    ));
                }
                let tasks_clone = self.tasks.clone();
                Command::perform(
                    async move { save_tasks_cmd(tasks_clone).await.map_err(|e| e.to_string()) },
                    Message::SaveResult,
                )
            }
            Message::KeyPress { key, modifiers } => {
                if modifiers.control() || modifiers.command() {
                    match key {
                        keyboard::Key::Character(c) => {
                            let cmd = match c.to_string().as_str() {
                                "s" => Some(Message::Save),
                                "n" => Some(Message::SwitchTo(Screen::NewTask)),
                                "h" => Some(Message::SwitchTo(Screen::History)),
                                "o" => Some(Message::SwitchTo(Screen::Overview)),
                                _ => None,
                            };
                            if let Some(msg) = cmd {
                                return self.update(msg);
                            }
                        }
                        _ => {}
                    }
                } else if let keyboard::Key::Character(c) = key {
                    if c == "\u{7f}" {
                        // Delete key
                        if let Some(idx) = self.pending_delete {
                            return self.update(Message::DeletePressed(idx));
                        }
                    }
                }
                Command::none()
            }
            Message::SaveResult(res) => {
                self.is_saving = false;
                match res {
                    Ok(_) => {
                        info!("Save successful");
                        self.history.push(format!(
                            "[{}] Save successful",
                            Local::now().format("%H:%M:%S")
                        ));
                    }
                    Err(e) => {
                        error!("Save failed: {}", e);
                        self.history.push(format!(
                            "[{}] Save failed: {}",
                            Local::now().format("%H:%M:%S"),
                            e
                        ));
                    }
                }
                Command::none()
            }
            Message::Load => {
                self.is_loading = true;
                if let Some(p) = project_file_path() {
                    self.history.push(format!(
                        "[{}] Loading from {}...",
                        Local::now().format("%H:%M:%S"),
                        p.display()
                    ));
                } else {
                    self.history.push(format!(
                        "[{}] Loading: no config dir",
                        Local::now().format("%H:%M:%S")
                    ));
                }
                Command::perform(
                    async { load_tasks_cmd().await.map_err(|e| e.to_string()) },
                    Message::LoadResult,
                )
            }
            Message::LoadResult(res) => {
                self.is_loading = false;
                match res {
                    Ok(list) => {
                        let count = list.len();
                        self.tasks = list;
                        info!("Load successful ({} tasks)", count);
                        self.history.push(format!(
                            "[{}] Loaded tasks ({} items)",
                            Local::now().format("%H:%M:%S"),
                            count
                        ));
                    }
                    Err(e) => {
                        error!("Load failed: {}", e);
                        self.history.push(format!(
                            "[{}] Load failed: {}",
                            Local::now().format("%H:%M:%S"),
                            e
                        ));
                    }
                }
                Command::none()
            }
            Message::StartBackground => {
                if !self.background_running {
                    self.background_running = true;
                    self.background_progress = 0.0;
                    self.history.push(format!(
                        "[{}] Background task started",
                        Local::now().format("%H:%M:%S")
                    ));
                }
                Command::none()
            }
            Message::BackgroundTick => {
                if self.background_running {
                    // Advance simulated progress
                    self.background_progress += 0.05;
                    if self.background_progress >= 1.0 {
                        self.background_progress = 1.0;
                        self.background_running = false;
                        self.history.push(format!(
                            "[{}] Background task complete",
                            Local::now().format("%H:%M:%S")
                        ));
                    }
                }
                Command::none()
            }

            Message::ToggleEnable(idx) => {
                if idx < self.tasks.len() {
                    self.tasks[idx].enabled = !self.tasks[idx].enabled;
                    let tasks_clone = self.tasks.clone();
                    Command::perform(
                        async move { save_tasks_cmd(tasks_clone).await.map_err(|e| e.to_string()) },
                        Message::SaveResult,
                    )
                } else {
                    Command::none()
                }
            }
        }
    }
}

fn main() -> iced::Result {
    TaskScheduler::run(iced::Settings::default())
}
