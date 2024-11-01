use ratatui::text::Line;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

pub struct PodmanWidget {}

impl PodmanWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl WidgetRef for PodmanWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Podman ").magenta().bold());

        let mut paragraphs: Vec<Paragraph> = vec![
            Paragraph::new(format!("All containers: {}", 0)),
            Paragraph::new(format!("Running containers: {}", 0)),
        ];

        paragraphs.push(
            Paragraph::new(format!("FAILED CONTAINERS: {}", 0))
                .style(Style::default().bold().red().slow_blink())
                .slow_blink(),
        );

        let areas = Layout::vertical(
            paragraphs
                .iter()
                .map(|_| Constraint::Max(1))
                .collect::<Vec<Constraint>>(),
        )
        .split(block.inner(area));

        block.render(area, buf);

        for i in 0..paragraphs.len() {
            paragraphs.get(i).unwrap().render(areas[i], buf);
        }
    }
}
