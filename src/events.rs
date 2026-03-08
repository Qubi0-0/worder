use crate::app::{App, FocusedPanel};
use crate::components::Component;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::io;

pub fn handle_event(app: &mut App, key: KeyEvent) -> io::Result<()> {
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    match key.code {
        KeyCode::Char('q') => {
            app.exit = true;
            return Ok(());
        }
        KeyCode::Tab => {
            app.next_panel();
            return Ok(());
        }
        KeyCode::BackTab => {
            app.prev_panel();
            return Ok(());
        }
        _ => {}
    }
    match app.focused_panel {
        FocusedPanel::TitleBar => app.title_bar.handle_key_event(key)?,
        FocusedPanel::StatusBar => app.status_bar.handle_key_event(key)?,
        FocusedPanel::Left => app.left_panel.handle_key_event(key)?,
        FocusedPanel::Right => app.right_panel.handle_key_event(key)?,
    }

    Ok(())
}
