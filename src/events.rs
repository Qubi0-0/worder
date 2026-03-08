use crate::app::{App, FocusedPanel};
use crate::components::Component;
use crate::translator;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;

pub fn handle_event(app: &mut App, key: KeyEvent) -> io::Result<()> {
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    // Handle export clear confirmation dialog
    if app.awaiting_clear_confirm {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                app.awaiting_clear_confirm = false;
                app.dictionary.entries.clear();
                if let Err(e) = app.dictionary.save() {
                    app.left_panel.status = format!("✗ Save error: {}", e);
                } else {
                    app.right_panel.entries.clear();
                    app.right_panel.list_state.select(None);
                    app.left_panel.status = "✓ Exported & cleared".to_string();
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                app.awaiting_clear_confirm = false;
            }
            _ => {}
        }
        return Ok(());
    }

    // Ctrl+C always quits
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        app.exit = true;
        return Ok(());
    }

    match key.code {
        // 'q' quits only when NOT typing in the left panel
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
            // Check if a word was submitted (Enter)
            if let Some(word) = app.left_panel.submitted_word.take() {
                app.left_panel.status = format!("Translating '{}' ...", word);
                match translator::translate_de_to_en(&word) {
                    Ok(translation) => {
                        app.dictionary.add_entry(word.clone(), translation.clone());
                        if let Err(e) = app.dictionary.save() {
                            app.left_panel.status = format!("✗ Save error: {}", e);
                        } else if translation == "—" {
                            app.left_panel.status =
                                format!("⚠ No translation for '{}', saved for manual entry", word);
                            app.right_panel.entries = app.dictionary.entries.clone();
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
            // Check if input was deleted
            if let Some(index) = app.right_panel.deleted_index.take() {
                app.dictionary.remove_entry(index);
                if let Err(e) = app.dictionary.save() {
                    app.left_panel.status = format!("✗ Save error: {}", e);
                } else {
                    app.right_panel.entries = app.dictionary.entries.clone();
                    app.left_panel.status = "Entry deleted".to_string();
                    if app.right_panel.entries.is_empty() {
                        app.right_panel.list_state.select(None);
                    } else if index >= app.right_panel.entries.len() {
                        app.right_panel
                            .list_state
                            .select(Some(app.right_panel.entries.len() - 1));
                    }
                }
            }
            // Check if export was requested
            if app.right_panel.export_requested {
                app.right_panel.export_requested = false;
                match app.dictionary.export_json() {
                    Ok(path) => {
                        app.left_panel.status = format!("✓ Exported to {}", path);
                        app.awaiting_clear_confirm = true;
                    }
                    Err(e) => app.left_panel.status = format!("✗ Export error: {}", e),
                }
            }
        }
        FocusedPanel::TitleBar => app.title_bar.handle_key_event(key)?,
        FocusedPanel::StatusBar => app.status_bar.handle_key_event(key)?,
    }

    Ok(())
}
