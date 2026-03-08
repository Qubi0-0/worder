use crate::app::{App, FocusedPanel};
use crate::components::Component;
use crate::translator;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;

pub fn handle_event(app: &mut App, key: KeyEvent) -> io::Result<()> {
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    // Ctrl+C zawsze kończy
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        app.exit = true;
        return Ok(());
    }

    match key.code {
        // 'q' kończy tylko gdy NIE wpisujemy tekst w lewym panelu
        KeyCode::Char('q') if app.focused_panel != FocusedPanel::Left => {
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
        FocusedPanel::Left => {
            app.left_panel.handle_key_event(key)?;
            // Sprawdź czy słowo zostało zatwierdzone (Enter)
            if let Some(word) = app.left_panel.submitted_word.take() {
                app.left_panel.status = format!("Tłumaczenie '{}' ...", word);
                match translator::translate_de_to_en(&word) {
                    Ok(translation) => {
                        app.dictionary.add_entry(word.clone(), translation.clone());
                        if let Err(e) = app.dictionary.save() {
                            app.left_panel.status = format!("✗ Błąd zapisu: {}", e);
                        } else {
                            app.left_panel.status = format!("✓ {} → {}", word, translation);
                            app.right_panel.entries = app.dictionary.entries.clone();
                        }
                    }
                    Err(e) => {
                        app.left_panel.status = format!("✗ {}", e);
                    }
                }
            }
        }
        FocusedPanel::Right => {
            app.right_panel.handle_key_event(key)?;
            // Sprawdź czy wpis został usunięty
            if let Some(index) = app.right_panel.deleted_index.take() {
                app.dictionary.remove_entry(index);
                if let Err(e) = app.dictionary.save() {
                    app.left_panel.status = format!("✗ Błąd zapisu: {}", e);
                } else {
                    app.right_panel.entries = app.dictionary.entries.clone();
                    app.left_panel.status = "Usunięto wpis".to_string();
                    if app.right_panel.entries.is_empty() {
                        app.right_panel.list_state.select(None);
                    } else if index >= app.right_panel.entries.len() {
                        app.right_panel
                            .list_state
                            .select(Some(app.right_panel.entries.len() - 1));
                    }
                }
            }
        }
        FocusedPanel::TitleBar => app.title_bar.handle_key_event(key)?,
        FocusedPanel::StatusBar => app.status_bar.handle_key_event(key)?,
    }

    Ok(())
}
