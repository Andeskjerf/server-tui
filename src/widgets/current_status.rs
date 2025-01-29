use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use ratatui::layout::{Alignment, Constraint, Direction, Flex};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Block, Widget, WidgetRef},
};

use crate::services::event_bus::EventBus;
use crate::utils;

type Messages = Arc<Mutex<HashMap<String, (String, i64)>>>;

pub struct CurrentStatusWidget {
    pub active_messages: Messages,
}

impl CurrentStatusWidget {
    pub async fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        let active_messages = Arc::new(Mutex::new(HashMap::new()));
        CurrentStatusWidget::cleanup_task(Arc::clone(&active_messages));
        CurrentStatusWidget::subscribe(Arc::clone(&event_bus), Arc::clone(&active_messages));
        Self { active_messages }
    }

    fn subscribe(event_bus: Arc<Mutex<EventBus>>, active_messages: Messages) {
        event_bus
            .lock()
            .unwrap()
            .subscribe("process_watcher", move |data| {
                CurrentStatusWidget::on_event(Arc::clone(&active_messages), data);
            });
    }

    fn cleanup_task(active_messages: Messages) {
        const CLEANUP_INTERVAL: u8 = 2;
        tokio::spawn(async move {
            loop {
                let mut lock = active_messages.lock().unwrap();
                let now = chrono::Utc::now().timestamp();
                (*lock).clone().into_iter().for_each(|(key, (_, ts))| {
                    if key == "All good!" {
                        return;
                    }

                    if now - ts >= CLEANUP_INTERVAL as i64 {
                        (*lock).remove(&key);
                    }
                });
                if lock.is_empty() {
                    lock.insert(
                        "All good!".to_string(),
                        ("Nothing happening".to_string(), 0),
                    );
                }
                drop(lock);
                sleep(Duration::from_millis(100));
            }
        });
    }

    fn on_event(active_messages: Messages, data: Vec<u8>) {
        let binding = String::from_utf8(data).expect("unable to decode data");
        let decoded = binding.split(',').collect::<Vec<&str>>();
        let mut lock = active_messages.lock().unwrap();
        match decoded[0] {
            "add" => {
                if lock.len() == 1 && lock.get("All good!").is_some() {
                    lock.remove("All good!");
                }
                lock.insert(
                    decoded[1].to_string(),
                    ("Running".to_string(), chrono::Utc::now().timestamp()),
                );
            }
            _ => println!("invalid command"),
        }
    }
}

impl WidgetRef for CurrentStatusWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Status ").red().bold());

        let active_messages = self.active_messages.lock().unwrap();
        let layout =
            utils::layout::make_layout(Direction::Horizontal, active_messages.len() as u16)
                .flex(Flex::Center)
                .split(block.inner(area));
        let paragraphs = active_messages
            .iter()
            .map(|(k, v)| {
                (
                    Paragraph::new(k.clone())
                        .bold()
                        .alignment(Alignment::Center),
                    Paragraph::new(v.0.clone()).alignment(Alignment::Center),
                )
            })
            .collect::<Vec<(Paragraph, Paragraph)>>();

        (0..paragraphs.len()).for_each(|i| {
            let sub_layout = utils::layout::make_layout(Direction::Vertical, 2)
                .constraints(vec![Constraint::Max(1); 2])
                .flex(Flex::Center);
            let areas: [Rect; 2] = sub_layout.areas(layout[i]);
            if let Some(elems) = paragraphs.get(i) {
                let elems = elems.clone();
                elems.0.render(areas[0], buf);
                elems.1.render(areas[1], buf);
            }
        });

        block.render(area, buf);
    }
}
