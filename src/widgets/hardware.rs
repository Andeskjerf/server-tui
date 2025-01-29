use std::sync::{Arc, Mutex};

use log::info;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols,
    widgets::{Axis, Chart, Dataset, GraphType, Widget, WidgetRef},
};

use crate::services::event_bus::EventBus;

use super::controllers::hardware::HardwareUsageController;

pub struct HardwareUsageWidget {
    controller: HardwareUsageController,
}

impl HardwareUsageWidget {
    pub fn new(event_bus: Arc<Mutex<EventBus>>) -> Self {
        Self {
            controller: HardwareUsageController::new(event_bus),
        }
    }
}

impl WidgetRef for HardwareUsageWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut cpu_data: Vec<(f64, f64)> = vec![];
        let mut ram_data: Vec<(f64, f64)> = vec![];
        info!("history: {}", self.controller.history as usize);
        for i in 0..=self.controller.history as usize {
            // break early if data hasn't been polled for that long
            if i + 1 > self.controller.cpu_lock().len() {
                break;
            }
            info!("printing");

            cpu_data.push((i as f64, self.controller.cpu_lock()[i]));
            ram_data.push((i as f64, self.controller.ram_lock()[i]));
        }
        info!("len cpu: {}", cpu_data.len());
        info!("len ram: {}", ram_data.len());

        let datasets = vec![
            Dataset::default()
                .name("CPU")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().blue().bold())
                .data(&cpu_data),
            Dataset::default()
                .name("Memory")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().red().bold())
                .data(&ram_data),
        ];

        let mut labels: Vec<String> = vec![];
        for i in 1..=2 {
            labels.push((self.controller.history / i as f64).to_string());
        }
        labels.push("0".to_owned());
        labels.reverse();

        // Create the X axis and define its properties
        let x_axis = Axis::default()
            .style(Style::default().white())
            .bounds([0.0, self.controller.history])
            .labels(labels);

        // Create the Y axis and define its properties
        let y_axis = Axis::default()
            .style(Style::default().white())
            .bounds([0.0, 100.0])
            .labels(["0", "25", "50", "75", "100"]);

        // Create the chart and link all the parts together
        let chart = Chart::new(datasets).x_axis(x_axis).y_axis(y_axis);

        chart.render(area, buf);
    }
}
