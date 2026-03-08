use crossterm::event::KeyEvent;
use ratatui::{Frame, prelude::*};
use std::io;

pub mod left_panel;
pub mod right_panel;
pub mod status_bar;
pub mod title_bar;

pub use left_panel::LeftPanel;
pub use right_panel::RightPanel;
pub use status_bar::StatusBar;
pub use title_bar::TitleBar;

pub enum FocusedColor {
    Focused,
    Unfocused,
}

impl FocusedColor {
    pub fn to_color(&self) -> Color {
        match self {
            FocusedColor::Focused => Color::LightCyan,
            FocusedColor::Unfocused => Color::Gray,
        }
    }
}

pub trait Component {
    fn render(&mut self, frame: &mut Frame, area: Rect);

    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()>;

    #[allow(unused)]
    fn update(&mut self) -> io::Result<()> {
        Ok(())
    }
}
