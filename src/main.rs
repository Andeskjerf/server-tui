use std::{
    env,
    fs::File,
    io,
    sync::{Arc, Mutex},
};

use crate::traits::runnable::Runnable;
use app::App;
use log::LevelFilter;
use services::{
    datetime::DateTimeService, event_bus::EventBus, hw_usage::HwUsageService,
    process_watcher::ProcessWatcher, socket::SocketService,
};
use simplelog::{CombinedLogger, Config, WriteLogger};

mod api;
mod app;
mod models;
mod services;
mod traits;
mod utils;
mod widgets;

#[tokio::main]
async fn main() -> io::Result<()> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("log.log").unwrap(),
    )])
    .unwrap();

    let event_bus = Arc::new(Mutex::new(EventBus::new()));

    let to_watch = env::args().skip(1).collect::<Vec<String>>();
    let services: Vec<Box<dyn Runnable>> = vec![
        Box::new(SocketService::new(Arc::clone(&event_bus), "server-tui.sock").await),
        Box::new(ProcessWatcher::new(Arc::clone(&event_bus), to_watch)),
        Box::new(HwUsageService::new(Arc::clone(&event_bus))),
        Box::new(DateTimeService::new(Arc::clone(&event_bus))),
    ];

    for s in services {
        s.run();
    }

    let terminal = ratatui::init();
    let mut app = App::new(terminal, Arc::clone(&event_bus)).await?;
    let result = app.run().await;

    ratatui::restore();
    result
}
