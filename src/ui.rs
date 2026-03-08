use crate::app::{App, FocusedPanel};
use crate::components::Component;
use crate::components::FocusedColor;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint::*, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
};

pub fn draw(app: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([Length(3), Min(0), Length(3)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());

    let horizontal = Layout::horizontal([Fill(1), Fill(1)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    app.status_bar.fg_color = FocusedColor::Unfocused.to_color();
    app.title_bar.fg_color = FocusedColor::Unfocused.to_color();
    app.left_panel.fg_color = FocusedColor::Unfocused.to_color();
    app.left_panel.focused = false;
    app.right_panel.fg_color = FocusedColor::Unfocused.to_color();

    match app.focused_panel {
        FocusedPanel::TitleBar => app.title_bar.fg_color = FocusedColor::Focused.to_color(),
        FocusedPanel::StatusBar => app.status_bar.fg_color = FocusedColor::Focused.to_color(),
        FocusedPanel::Left => {
            app.left_panel.fg_color = FocusedColor::Focused.to_color();
            app.left_panel.focused = true;
        }
        FocusedPanel::Right => app.right_panel.fg_color = FocusedColor::Focused.to_color(),
    }

    if app.focused_panel != FocusedPanel::Right {
        app.right_panel.list_state.select(None);
    }

    app.title_bar.render(frame, title_area);
    app.status_bar.render(frame, status_area);
    app.left_panel.render(frame, left_area);
    app.right_panel.render(frame, right_area);

    if app.awaiting_clear_confirm {
        let popup = centered_popup(60, 5, frame.area());
        frame.render_widget(Clear, popup);
        frame.render_widget(
            Paragraph::new("\n\n[y] Yes  [n] No")
                .block(
                    Block::bordered()
                        .title(" Do you want to remove all entries? ")
                        .border_style(Style::default().fg(Color::Yellow)),
                )
                .alignment(Alignment::Center),
            popup,
        );
    }
}

fn centered_popup(width_pct: u16, height: u16, area: Rect) -> Rect {
    let side = (100 - width_pct) / 2;
    let vert = Layout::vertical([Min(0), Length(height), Min(0)]).split(area);
    Layout::horizontal([
        Percentage(side),
        Percentage(100 - side * 2),
        Percentage(side),
    ])
    .split(vert[1])[1]
}
