// Week 6: GUI start with Iced
// Minimal Iced application skeleton that will become the final GUI
// See `week6_TESTING.md` for testing, fault-injection and accessibility plans.

// Week 6: GUI start with Iced
// Minimal Iced application skeleton that will become the final GUI
// See `week6_TESTING.md` for testing, fault-injection and accessibility plans.

use iced::{
    button, executor, scrollable, Application, Button, Column, Command, Element, Length, Settings,
    Subscription, Text,
};

// Import the model + persistence snapshots so this snapshot clearly builds on
// the work from weeks 2 and 3. These `mod` statements are illustrative — when
// integrating into a crate you'd wire modules or copy code accordingly.
mod rust_week2_model;
mod rust_week3_persistence;

use rust_week2_model::{Interval, Task};
use rust_week3_persistence::{load_tasks, save_tasks};

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<Vec<Task>, String>),
    NoOp,
}

struct App {
    tasks: Vec<Task>,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Load tasks asynchronously on startup; this demonstrates using
        // Command::perform with the persistence layer (week 3).
        (
            App { tasks: vec![] },
            Command::perform(
                async { load_tasks().await.map_err(|e| e.to_string()) },
                Message::Loaded,
            ),
        )
    }

    fn title(&self) -> String {
        "Task Scheduler - Week 6".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok(list)) => {
                self.tasks = list;
                Command::none()
            }
            Message::Loaded(Err(_)) => {
                // On load error we keep an empty list; real app shows toast
                self.tasks = vec![Task::new("Demo Task", Interval::Daily)];
                Command::none()
            }
            Message::NoOp => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut col = Column::new()
            .spacing(8)
            .push(Text::new("Week 6 — GUI (mock data)"));
        if self.tasks.is_empty() {
            col = col.push(Text::new("No tasks loaded"));
        } else {
            let list = self
                .tasks
                .iter()
                .map(|t| Text::new(format!("- {} ({})", t.name, t.interval)))
                .collect::<Vec<_>>();
            let mut s = scrollable::Scrollable::new(0);
            for item in list {
                s = s.push(item);
            }
            col = col.push(s.height(Length::FillPortion(2)));
        }
        col.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
