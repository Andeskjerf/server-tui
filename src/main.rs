use std::{io, sync::Arc};

use app::App;
use services::socket::SocketService;

mod api;
mod app;
mod models;
mod services;
mod utils;
mod widgets;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = SocketService::new("server-tui.socket");
    let queue = Arc::clone(&socket.queue);
    socket.run().await;

    let terminal = ratatui::init();
    let mut app = App::new(terminal, queue)?;
    let result = app.run().await;

    ratatui::restore();
    result
}
