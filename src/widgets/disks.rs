use ratatui::text::Line;
use ratatui::widgets::{Block, Widget};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, widgets::WidgetRef};

pub struct DisksWidget {}

impl DisksWidget {
    pub fn new() -> Self {
        Self {}
    }

    pub fn poll(&mut self) {}
}

impl WidgetRef for DisksWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Disks ").blue().bold());

        block.render(area, buf);
    }
}
