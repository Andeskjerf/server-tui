use ratatui::text::Line;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

use crate::api::systemctl as systemctl_api;

pub struct SystemctlWidget {
    all: Option<Vec<String>>,
    running: Option<Vec<String>>,
    failed: Option<Vec<String>>,
}

impl SystemctlWidget {
    pub fn new() -> Self {
        Self {
            all: None,
            running: None,
            failed: None,
        }
    }

    pub fn poll(&mut self) {
        // self.all = systemctl_api::get_units("");
        self.running = systemctl_api::get_units("running");
        self.failed = systemctl_api::get_units("failed");
    }
}

impl WidgetRef for SystemctlWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Systemctl ").blue().bold());

        let mut paragraphs: Vec<Paragraph> = vec![
            // Paragraph::new(format!("All services: {}", print_unit_length(&self.all))),
            // Paragraph::new(format!("Running services: {}", print_unit_length(&self.running))),
        ];

        // if let Some(res) = &self.failed {
        //     if res.is_empty() {
        //         paragraphs.push(
        //             Paragraph::new(format!("FAILED SERVICES: {}", res.len()))
        //                 .style(Style::default().bold().red()),
        //         );
        //     }
        // }

        let areas = Layout::vertical(
            paragraphs
                .iter()
                .map(|_| Constraint::Max(1))
                .collect::<Vec<Constraint>>(),
        )
        .split(block.inner(area));

        block.render(area, buf);

        // for i in 0..paragraphs.len() {
        //     paragraphs.get(i).unwrap().render(areas[i], buf);
        // }
    }
}

fn print_unit_length(units: &Option<Vec<String>>) -> String {
    if units.is_none() {
        "ERR".to_owned()
    } else {
        format!("{}", units.clone().unwrap().len())
    }
}
