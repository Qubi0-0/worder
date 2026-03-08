use super::Component;
use crate::dictionary::DictionaryEntry;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use std::io;

pub struct RightPanel {
    pub fg_color: Color,
    pub entries: Vec<DictionaryEntry>,
    pub list_state: ListState,
    pub deleted_index: Option<usize>,
}

impl RightPanel {
    pub fn new(fg_color: Color) -> Self {
        Self {
            fg_color,
            entries: Vec::new(),
            list_state: ListState::default(),
            deleted_index: None,
        }
    }
}

impl Component for RightPanel {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let content = format!("{}. {} → {}", i + 1, e.german, e.translation);
                ListItem::new(content)
            })
            .collect();

        let title = format!("Słownik ({} słów)", self.entries.len());
        let list = List::new(items)
            .block(
                Block::bordered()
                    .title(title)
                    .border_style(Style::default().fg(self.fg_color)),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::REVERSED)
                    .fg(Color::Yellow),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Up => {
                    if !self.entries.is_empty() {
                        let i = match self.list_state.selected() {
                            Some(i) => i.saturating_sub(1),
                            None => 0,
                        };
                        self.list_state.select(Some(i));
                    }
                }
                KeyCode::Down => {
                    if !self.entries.is_empty() {
                        let i = match self.list_state.selected() {
                            Some(i) => {
                                if i >= self.entries.len() - 1 {
                                    i
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        self.list_state.select(Some(i));
                    }
                }
                KeyCode::Char('d') | KeyCode::Delete => {
                    if let Some(i) = self.list_state.selected() {
                        if i < self.entries.len() {
                            self.deleted_index = Some(i);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
