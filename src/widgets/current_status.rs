use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ratatui::layout::{Alignment, Constraint, Direction, Flex};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Block, Widget, WidgetRef},
};
// use tokio::sync::Mutex;

use crate::services::event_bus::EventBus;
use crate::utils;

pub struct CurrentStatusWidget {
    pub event_bus: Arc<Mutex<EventBus>>,
    pub ongoing: Arc<Mutex<HashMap<String, String>>>,
}

impl CurrentStatusWidget {
    pub async fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        let widget = Self {
            event_bus,
            ongoing: Arc::new(Mutex::new(HashMap::new())),
        };

        widget
            .ongoing
            .lock()
            .unwrap()
            .insert("All good!".to_string(), "Nothing happening".to_string());

        {
            let ongoing = Arc::clone(&widget.ongoing);
            widget
                .event_bus
                .lock()
                .unwrap()
                .subscribe("process_watcher", move |data| {
                    CurrentStatusWidget::on_event(Arc::clone(&ongoing), data);
                });
        }

        widget
    }

    async fn on_event(ongoing: Arc<Mutex<HashMap<String, String>>>, data: Vec<u8>) {
        let decoded = String::from_utf8(data).expect("unable to decode data");
        let split = decoded.split(',').collect::<Vec<&str>>();
        let mut lock = ongoing.lock().unwrap();
        match split[0] {
            "rm" => {
                lock.remove(split[1]);
            }
            "add" => {
                if lock.len() == 1 && lock.get("All good!").is_some() {
                    lock.remove("All good!");
                }
                lock.insert(
                    split[1].to_string(),
                    "Update from ProcessWatcher".to_string(),
                );
            }
            _ => println!("invalid command"),
        }

        if lock.is_empty() {
            lock.insert("All good!".to_string(), "Nothing happening".to_string());
        }
    }
}

impl WidgetRef for CurrentStatusWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Status ").red().bold());

        let ongoing = self.ongoing.lock().unwrap();
        let layout = utils::layout::make_layout(Direction::Horizontal, ongoing.len() as u16)
            .flex(Flex::Center)
            .split(block.inner(area));
        let paragraphs = ongoing
            .iter()
            .map(|(k, v)| {
                (
                    Paragraph::new(k.clone())
                        .bold()
                        .alignment(Alignment::Center),
                    Paragraph::new(v.clone()).alignment(Alignment::Center),
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
