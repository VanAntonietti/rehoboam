use crate::modules::github::github_draw;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};

pub fn draw_layout(frame: &mut Frame, area: Rect) {
    let chunk = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(7),
    ])
    .split(area);
    github_draw(frame, chunk[0]);
    github_draw(frame, chunk[1]);
    github_draw(frame, chunk[2]);
}
