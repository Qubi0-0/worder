use super::Component;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::Color, prelude::*, widgets::*};
use std::io;

pub struct LeftPanel {
    counter: i32,
    pub fg_color: Color,
}

impl LeftPanel {
    pub fn new(fg_color: Color) -> Self {
        Self {
            counter: 0,
            fg_color: fg_color,
        }
    }
}

impl Component for LeftPanel {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered()
            .title("Left Panel")
            .border_style(Style::default().fg(self.fg_color));
        // take user imput
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read Stdin");
        let text = format!("Counter: {}\nPress +/- to change", self.counter);
        let paragraph = Paragraph::new(text).block(block);

        frame.render_widget(paragraph, area);
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('+') => self.counter += 1,
                KeyCode::Char('-') => self.counter -= 1,
                _ => {}
            }
        }
        Ok(())
    }
}
