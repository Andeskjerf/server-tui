use ratatui::text::Line;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

use crate::api::systemctl as systemctl_api;

pub struct SystemctlWidget {}

impl SystemctlWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl WidgetRef for SystemctlWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title(Line::from(" Systemctl ").blue().bold());

        let all_units = systemctl_api::get_units("");
        let running = systemctl_api::get_units("running");
        let failed = systemctl_api::get_units("failed");

        let mut paragraphs: Vec<Paragraph> = vec![
            Paragraph::new(format!("All services: {}", print_unit_length(all_units))),
            Paragraph::new(format!("Running services: {}", print_unit_length(running))),
        ];

        if let Some(res) = failed {
            if res.is_empty() {
                paragraphs.push(
                    Paragraph::new(format!("FAILED SERVICES: {}", res.len()))
                        .style(Style::default().bold().red()),
                );
            }
        }

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

fn print_unit_length(units: Option<Vec<String>>) -> String {
    if units.is_none() {
        "ERR".to_owned()
    } else {
        format!("{}", units.unwrap().len())
    }
}
