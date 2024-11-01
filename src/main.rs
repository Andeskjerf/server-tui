use std::io;

use app::App;

mod api;
mod app;
mod widgets;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let mut app = App::new(terminal)?;
    let result = app.run();

    ratatui::restore();
    result
}
