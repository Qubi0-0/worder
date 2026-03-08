use super::Component;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use std::io;

pub struct LeftPanel {
    pub input: String,
    pub status: String,
    pub submitted_word: Option<String>,
    pub focused: bool,
    pub fg_color: Color,
}

impl LeftPanel {
    pub fn new(fg_color: Color) -> Self {
        Self {
            input: String::new(),
            status: String::from("Type a German word and press Enter"),
            submitted_word: None,
            focused: false,
            fg_color,
        }
    }
}

impl Component for LeftPanel {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let outer_block = Block::bordered()
            .title("New Word")
            .border_style(Style::default().fg(self.fg_color));
        let inner = outer_block.inner(area);
        frame.render_widget(outer_block, area);

        let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(inner);

        // Text input
        let input_block = Block::bordered()
            .title("Word (DE)")
            .border_style(Style::default().fg(Color::White));
        let input_paragraph = Paragraph::new(self.input.as_str()).block(input_block);
        frame.render_widget(input_paragraph, chunks[0]);

        // Cursor
        if self.focused {
            frame.set_cursor_position(Position::new(
                chunks[0].x + self.input.len() as u16 + 1,
                chunks[0].y + 1,
            ));
        }

        // Status
        let status_block = Block::bordered()
            .title("Status")
            .border_style(Style::default().fg(Color::DarkGray));
        let status_paragraph = Paragraph::new(self.status.as_str())
            .block(status_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(status_paragraph, chunks[1]);
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    if !self.input.is_empty() {
                        self.submitted_word = Some(self.input.drain(..).collect());
                    }
                }
                KeyCode::Esc => {
                    self.input.clear();
                }
                _ => {}
            }
        }
        Ok(())
    }
}
