use std::collections::VecDeque;

use ratatui::text::Line;
use ratatui::widgets::{List, ListItem, ListState, StatefulWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget, WidgetRef},
};
use systemd::journal::{Journal, OpenOptions};
use systemd::JournalRecord;

pub struct LogWidget {
    journal: Journal,
    records: VecDeque<JournalRecord>,
    list_state: ListState,
    max_records: usize,
}

impl LogWidget {
    pub fn new() -> Self {
        Self {
            journal: OpenOptions::default()
                .local_only(true)
                .runtime_only(false)
                .system(true)
                .open()
                .expect("unable to open journal for reading"),
            records: VecDeque::new(),
            list_state: ListState::default(),
            max_records: 100,
        }
    }

    pub fn poll(&mut self) {
        while let Ok(Some(res)) = self.journal.next_entry() {
            self.records.push_back(res);
            if self.records.len() > self.max_records {
                self.records.pop_front();
            }
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // const ROW_COUNT: usize = 30;
        let block = Block::bordered().title_bottom(Line::from(" System Log ").white().bold());

        // let areas = Layout::vertical([Constraint::Max(1); ROW_COUNT]).split(block.inner(area));

        // block.render(area, buf);

        // for i in (0..ROW_COUNT).rev() {
        //     if let Some(record) = self.records.get(self.records.len() - i) {
        //         Paragraph::new(format!("{:?}", record.get("MESSAGE").unwrap()))
        //             .render(areas[i], buf);
        //     }
        // }

        let items: Vec<ListItem> = self
            .records
            .iter()
            .map(|r| {
                let msg = match r.get("MESSAGE") {
                    Some(res) => res.clone(),
                    None => String::from(""),
                };
                ListItem::from(msg)
            })
            .collect();

        let list = List::new(items).block(block);
        self.list_state.select_last();

        StatefulWidget::render(list, area, buf, &mut self.list_state)
    }
}
