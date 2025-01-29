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

use crate::services::event_bus::EventBus;
use crate::utils;

use super::controllers::current_status::CurrentStatusController;

pub struct CurrentStatusWidget {
    controller: CurrentStatusController,
}

impl CurrentStatusWidget {
    pub async fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        Self {
            controller: CurrentStatusController::new(event_bus),
        }
    }
}

impl WidgetRef for CurrentStatusWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Status ").red().bold());

        let active_messages = self.controller.get_message_lock();
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
