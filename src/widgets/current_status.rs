use std::collections::HashMap;
use std::sync::Arc;

use ratatui::layout::{Direction, Flex};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Block, Widget, WidgetRef},
};
use tokio::sync::Mutex;

use crate::utils;

pub struct CurrentStatusWidget {
    queue: Arc<Mutex<HashMap<String, String>>>,
    ongoing: HashMap<String, String>,
}

impl CurrentStatusWidget {
    pub fn new(queue: Arc<Mutex<HashMap<String, String>>>) -> Self {
        Self {
            queue,
            ongoing: HashMap::new(),
        }
    }

    pub async fn process_queue(&mut self) {
        self.ongoing.clear();

        // need to do this since we can't await lock during render
        self.queue.lock().await.iter().for_each(|(k, v)| {
            self.ongoing.insert(k.clone(), v.clone());
        });

        if self.ongoing.is_empty() {
            self.ongoing
                .insert("All good!".to_string(), "Nothing happening".to_string());
        }
    }
}

impl WidgetRef for CurrentStatusWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Status ").red().bold());

        let layout = utils::layout::make_layout(Direction::Horizontal, self.ongoing.len() as u16)
            .flex(Flex::Center)
            .split(block.inner(area));
        let paragraphs = self
            .ongoing
            .iter()
            .map(|(k, v)| (Paragraph::new(k.clone()), Paragraph::new(v.clone())))
            .collect::<Vec<(Paragraph, Paragraph)>>();

        (0..paragraphs.len()).for_each(|i| {
            let sub_layout = utils::layout::make_layout(Direction::Vertical, 2).flex(Flex::Center);
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
