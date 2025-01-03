use std::{collections::HashMap, io, sync::Arc, time::Duration};

use crossterm::event::{self, poll, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::WidgetRef,
    DefaultTerminal,
};
use tokio::sync::Mutex;

use crate::{
    utils,
    widgets::{
        current_status::CurrentStatusWidget, hardware::HardwareUsageWidget, journalctl::LogWidget,
        podman::PodmanWidget, systemctl_stats::SystemctlWidget,
    },
};

pub struct App {
    terminal: DefaultTerminal,
    hw_usage: HardwareUsageWidget,
    status: CurrentStatusWidget,
    // logs: LogWidget,
}

impl App {
    pub fn new(
        mut terminal: DefaultTerminal,
        socket_messages: Arc<Mutex<HashMap<String, String>>>,
        process_updates: Arc<Mutex<HashMap<String, i64>>>,
    ) -> io::Result<Self> {
        terminal.clear()?;
        Ok(Self {
            terminal,
            hw_usage: HardwareUsageWidget::new(),
            status: CurrentStatusWidget::new(socket_messages, process_updates),
            // logs: LogWidget::new(),
        })
    }

    pub async fn run(&mut self) -> io::Result<()> {
        loop {
            // self.logs.poll();
            self.status.process_queue().await;
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
            let master_layout = utils::layout::make_layout(Direction::Vertical, 2);

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)]);

            let layout_ver = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)]);

            let blocks: Vec<Box<dyn WidgetRef>> = vec![
                Box::new(SystemctlWidget::new()),
                Box::new(PodmanWidget::new()),
            ];

            let [upper_area, hardware_area] = master_layout.areas(frame.area());
            let [stat_areas, log_area]: [Rect; 2] = layout.areas(upper_area);
            let status_areas: [Rect; 2] = layout_ver.areas(stat_areas);

            assert!(blocks.len() == status_areas.len());

            for i in 0..blocks.len() {
                blocks[i].render_ref(status_areas[i], frame.buffer_mut());
            }

            // self.logs.render(log_area, frame.buffer_mut());
            self.status.render_ref(log_area, frame.buffer_mut());
            self.hw_usage.render_ref(hardware_area, frame.buffer_mut());
        })?;

        Ok(())
    }
}
