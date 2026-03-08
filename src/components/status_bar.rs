use super::Component;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};
use std::io;

pub struct StatusBar {
    pub fg_color: Color,
}

impl StatusBar {
    pub fn new(fg_color: Color) -> Self {
        Self { fg_color }
    }
}

impl Component for StatusBar {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let instructions = "<Q> Quit | <Tab/Shift+Tab> Switch Panel | <Enter> Translate | <↑↓> Navigate | <D> Delete Entry | <E> Export";
        let block = Block::bordered()
            .title("Status Bar")
            .border_style(Style::default().fg(self.fg_color));

        let paragraph = Paragraph::new(instructions)
            .block(block)
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, area);
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> io::Result<()> {
        Ok(())
    }
}
