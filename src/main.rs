use std::{env, io, sync::{Arc, Mutex}};

use crate::traits::runnable::Runnable;
use app::App;
use services::{event_bus::EventBus, process_watcher::ProcessWatcher, socket::SocketService};
// use tokio::sync::Mutex;

mod api;
mod app;
mod models;
mod services;
mod traits;
mod utils;
mod widgets;

#[tokio::main]
async fn main() -> io::Result<()> {
    let event_bus = Arc::new(Mutex::new(EventBus::new()));

    let services: Vec<Box<dyn Runnable>> = vec![
        Box::new(SocketService::new(Arc::clone(&event_bus), "server-tui.sock").await),
        Box::new(ProcessWatcher::new(
            Arc::clone(&event_bus),
            env::args().skip(1).collect::<Vec<String>>(),
        )),
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
