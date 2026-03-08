use crate::components::FocusedColor;
use crate::components::*;
use crate::dictionary::Dictionary;
use crate::events;
use crate::ui;
use ratatui::DefaultTerminal;
use std::io;
use std::path::PathBuf;

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
    pub dictionary: Dictionary,

    pub title_bar: TitleBar,
    pub status_bar: StatusBar,
    pub left_panel: LeftPanel,
    pub right_panel: RightPanel,
}

impl App {
    pub fn new() -> Self {
        let dictionary = Dictionary::load(PathBuf::from("dictionary.json"));
        let mut right_panel = RightPanel::new(FocusedColor::Unfocused.to_color());
        right_panel.entries = dictionary.entries.clone();

        Self {
            exit: false,
            focused_panel: FocusedPanel::Left,
            dictionary,
            title_bar: TitleBar::new(FocusedColor::Unfocused.to_color()),
            status_bar: StatusBar::new(FocusedColor::Unfocused.to_color()),
            left_panel: LeftPanel::new(FocusedColor::Unfocused.to_color()),
            right_panel,
        }
    }

    pub fn next_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::Left => FocusedPanel::Right,
            _ => FocusedPanel::Left,
        };
    }

    pub fn prev_panel(&mut self) {
        self.focused_panel = match self.focused_panel {
            FocusedPanel::Right => FocusedPanel::Left,
            _ => FocusedPanel::Right,
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
