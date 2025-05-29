use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};

use crate::app::App;

pub fn draw_layout(frame: &mut Frame, app: &App, area: Rect) {
    let chunk = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(7),
    ])
    .split(area);
}

pub fn draw_github(frame: &mut Frame, app: &mut App, area: Rect) {}
