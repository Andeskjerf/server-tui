use std::sync::{Arc, Mutex};

use log::trace;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

use crate::{services::event_bus::EventBus, utils};

use super::controllers::datetime::DateTimeController;

pub struct DateTimeWidget {
    controller: DateTimeController,
}

impl DateTimeWidget {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        Self {
            controller: DateTimeController::new(event_bus),
        }
    }
}

impl WidgetRef for DateTimeWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title_bottom(Line::from(" Time ").white().bold());

        let layout = utils::layout::make_layout(Direction::Horizontal, 1_u16)
            .flex(Flex::Center)
            .split(block.inner(area));

        let formatted_time = self.controller.get_formatted_time();
        trace!("formatted_time: {}", formatted_time);
        let time = Paragraph::new(format!("{}\n\n{}", formatted_time, formatted_time))
            .bold()
            .alignment(Alignment::Center);

        let sub_layout = utils::layout::make_layout(Direction::Vertical, 1_u16)
            .constraints(vec![Constraint::Max(1); 1])
            .flex(Flex::Center)
            .split(layout[0]);

        time.render(sub_layout[0], buf);
        block.render(area, buf);
    }
}
