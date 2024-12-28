use std::collections::HashMap;
use std::sync::Arc;

use ratatui::text::Line;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Block, Widget, WidgetRef},
};
use tokio::sync::Mutex;

pub struct CurrentStatusWidget {
    queue: Arc<Mutex<Vec<String>>>,
    ongoing: HashMap<String, String>,
}

impl CurrentStatusWidget {
    pub fn new(queue: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            queue,
            ongoing: HashMap::new(),
        }
    }
}

impl WidgetRef for CurrentStatusWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Status ").red().bold());

        block.render(area, buf);
    }
}
