use std::{io, sync::Arc};

use app::App;
use services::{process_watcher::ProcessWatcher, socket::SocketService};

mod api;
mod app;
mod models;
mod services;
mod utils;
mod widgets;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = SocketService::new("server-tui.socket");
    let process_watcher = ProcessWatcher::new();
    let socket_messages = Arc::clone(&socket.queue);
    let process_updates = Arc::clone(&process_watcher.status);
    socket.run().await;

    process_watcher.cleanup_task();
    process_watcher.watch_process("steam".to_string());

    let terminal = ratatui::init();
    let mut app = App::new(terminal, socket_messages, process_updates)?;
    let result = app.run().await;

    ratatui::restore();
    result
}
