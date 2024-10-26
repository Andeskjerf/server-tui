use std::io;

use api::systemctl as systemctl_api;
use app::App;

mod api;
mod app;
mod widgets;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let mut app = App::new(terminal)?;
    let result = app.run();

    // let services = systemctl_api::get_units("");
    // for s in services.unwrap() {
    //     println!("{s}")
    // }
    // Ok(())

    ratatui::restore();
    result
}
