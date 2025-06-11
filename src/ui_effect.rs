use ratzilla::ratatui::layout::Rect;
use ratzilla::ratatui::layout::Size;

pub struct UiState {
    pub screen: Size,
}

impl UiState {
    pub fn new() -> Self {
        let area = Rect::new(0, 0, 95, 14);
        Self {
            screen: Size::default(),
        }
    }
}
