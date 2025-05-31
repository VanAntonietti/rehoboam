use crate::modules::github::github_draw;
use ratatui::layout;
use ratzilla::ratatui::layout::{Constraint, Layout, Rect};
use ratzilla::ratatui::Frame;

pub fn draw_layout(frame: &mut Frame, area: Rect) {
    let chunk = Layout::vertical([Constraint::Percentage(75), Constraint::Fill(1)]).split(area);
    let bottom = Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Fill(1),
        ])
        .split(chunk[1]);
    let top = Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(75), Constraint::Fill(1)])
        .split(chunk[0]);

    github_draw(frame, top[0]);
    github_draw(frame, top[1]);
    github_draw(frame, bottom[0]);
    github_draw(frame, bottom[1]);
    github_draw(frame, bottom[2]);
}

