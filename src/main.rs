mod app;
mod components;
mod dictionary;
mod events;
mod translator;
mod ui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let res = app.run(&mut terminal);
    ratatui::restore();
    res
}
