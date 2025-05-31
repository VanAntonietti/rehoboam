use crate::layout::draw_layout;
use ratzilla::ratatui::Frame;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, frame: &mut Frame) {
        draw_layout(frame, frame.area());
    }
}
