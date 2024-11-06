use sysinfo::System;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols,
    widgets::{Axis, Chart, Dataset, GraphType, Widget, WidgetRef},
};

pub struct HardwareUsageWidget {
    system: System,
    memory: Vec<f64>,
    cpu: Vec<f64>,

    history: f64,
}

impl HardwareUsageWidget {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            memory: vec![],
            cpu: vec![],
            history: 100.0,
        }
    }

    pub fn poll_usage(&mut self) {
        self.system.refresh_all();

        let memory_percentage =
            (self.system.used_memory() as f64 / self.system.total_memory() as f64) * 100.0;
        self.cpu.push(self.system.global_cpu_usage() as f64);
        self.memory.push(memory_percentage);

        // also remove from cpu, no need to check since both fill at the same rate
        if self.memory.len() > self.history as usize {
            self.memory.remove(0);
            self.cpu.remove(0);
        }
    }
}

impl WidgetRef for HardwareUsageWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut cpu_data: Vec<(f64, f64)> = vec![];
        let mut ram_data: Vec<(f64, f64)> = vec![];
        for i in 0..=self.history as usize {
            // break early if data hasn't been polled for that long
            if i + 1 > self.cpu.len() {
                break;
            }

            cpu_data.push((i as f64, self.cpu[i]));
            ram_data.push((i as f64, self.memory[i]));
        }

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
            labels.push((self.history / i as f64).to_string());
        }
        labels.push("0".to_owned());
        labels.reverse();

        // Create the X axis and define its properties
        let x_axis = Axis::default()
            .style(Style::default().white())
            .bounds([0.0, self.history])
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
