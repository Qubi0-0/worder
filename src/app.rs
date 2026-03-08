use crate::components::FocusedColor;
use crate::components::*;
use crate::events;
use crate::ui;
use ratatui::DefaultTerminal;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedPanel {
    TitleBar,
    StatusBar,
    Left,
    Right,
}

pub struct App {
    pub exit: bool,
    pub focused_panel: FocusedPanel,

    pub title_bar: TitleBar,
    pub status_bar: StatusBar,
    pub left_panel: LeftPanel,
    pub right_panel: RightPanel,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            focused_panel: FocusedPanel::Left,
            title_bar: TitleBar::new(FocusedColor::Unfocused.to_color()),
            status_bar: StatusBar::new(FocusedColor::Unfocused.to_color()),
            left_panel: LeftPanel::new(FocusedColor::Unfocused.to_color()),
            right_panel: RightPanel::new(FocusedColor::Unfocused.to_color()),
        }
    }

    pub fn next_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::TitleBar => FocusedPanel::Left,
            FocusedPanel::Left => FocusedPanel::Right,
            FocusedPanel::Right => FocusedPanel::StatusBar,
            FocusedPanel::StatusBar => FocusedPanel::TitleBar,
        };
    }

    pub fn prev_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::TitleBar => FocusedPanel::StatusBar,
            FocusedPanel::StatusBar => FocusedPanel::Right,
            FocusedPanel::Right => FocusedPanel::Left,
            FocusedPanel::Left => FocusedPanel::TitleBar,
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui::draw(self, frame))?;

            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                events::handle_event(self, key)?;
            }
        }
        Ok(())
    }
}
