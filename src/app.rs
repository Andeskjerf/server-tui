use std::{io, time::Duration};

use crossterm::event::{self, poll, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, WidgetRef},
    DefaultTerminal,
};

use crate::widgets::{hardware::HardwareUsageWidget, systemctl_stats::SystemctlWidget};

pub struct App {
    terminal: DefaultTerminal,
    hw_usage: HardwareUsageWidget,
}

impl App {
    pub fn new(mut terminal: DefaultTerminal) -> io::Result<Self> {
        terminal.clear()?;
        Ok(Self {
            terminal,
            hw_usage: HardwareUsageWidget::new(),
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.hw_usage.poll_usage();

            self.draw()?;

            if poll(Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
        }
    }

    fn draw(&mut self) -> io::Result<()> {
        self.terminal.draw(|frame| {
            let master_layout = make_layout(Direction::Vertical, 2);
            let layout = make_layout(Direction::Horizontal, 2);
            let blocks: Vec<Box<dyn WidgetRef>> = vec![
                Box::new(SystemctlWidget::new()),
                Box::new(draw_block("Podman".to_owned())),
            ];

            let [status_area, hardware_area] = master_layout.areas(frame.area());
            let status_areas: [Rect; 2] = layout.areas(status_area);

            assert!(blocks.len() == status_areas.len());

            for i in 0..blocks.len() {
                blocks[i].render_ref(status_areas[i], frame.buffer_mut());
            }

            self.hw_usage.render_ref(hardware_area, frame.buffer_mut());
        })?;

        Ok(())
    }
}

fn make_layout(dir: Direction, count: u16) -> Layout {
    let percentage = 100 / count;
    Layout::default()
        .direction(dir)
        .constraints(vec![Constraint::Percentage(percentage); count as usize])
}

fn draw_block<'a>(title: String) -> Block<'a> {
    Block::bordered().title(title)
}
