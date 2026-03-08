use super::Component;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
use std::io;

pub struct TitleBar {
    pub fg_color: Color,
}

impl TitleBar {
    pub fn new(fg_color: Color) -> Self {
        Self { fg_color }
    }
}

impl Component for TitleBar {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title("Title Bar")
            .border_style(Style::default().fg(self.fg_color));
        frame.render_widget(block, area);
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> io::Result<()> {
        Ok(())
    }
}
