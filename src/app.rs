use ratzilla::ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
    Frame,
};

pub struct AppState {
    pub sections: [SectionState; 5],
}

#[derive(Clone)]
pub struct SectionState {
    pub title: String,
    pub content: String,
    pub animation_progress: f32,
    pub is_focused: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sections: [
                SectionState {
                    title: "About Me".to_string(),
                    content: "Software Enfineer".to_string(),
                    animation_progress: 0.0,
                    is_focused: false,
                },
                SectionState {
                    title: "Skills".to_string(),
                    content: "Rust".to_string(),
                    animation_progress: 0.0,
                    is_focused: false,
                },
                SectionState {
                    title: "Current Project".to_string(),
                    content: "Portfolio Terminal".to_string(),
                    animation_progress: 0.0,
                    is_focused: false,
                },
                SectionState {
                    title: "Experience".to_string(),
                    content: "Learning".to_string(),
                    animation_progress: 0.0,
                    is_focused: false,
                },
                SectionState {
                    title: "Contact".to_string(),
                    content: "Email".to_string(),
                    animation_progress: 0.0,
                    is_focused: false,
                },
            ],
        }
    }
}

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, frame: &mut Frame, state: &mut AppState) {
        draw_layout(frame, frame.area(), state);
    }
}

fn draw_layout(frame: &mut Frame, area: Rect, state: &mut AppState) {
    let chunk = Layout::vertical([Constraint::Percentage(75), Constraint::Fill(1)]).split(area);
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Fill(1),
        ])
        .split(chunk[1]);
    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(75), Constraint::Fill(1)])
        .split(chunk[0]);
    render_section(frame, top[0], &mut state.sections[0]);
    render_section(frame, top[1], &mut state.sections[1]);
    render_section(frame, bottom[0], &mut state.sections[2]);
    render_section(frame, bottom[1], &mut state.sections[3]);
    render_section(frame, bottom[2], &mut state.sections[4]);
}

struct SectionWidget;

impl StatefulWidget for SectionWidget {
    type State = SectionState;

    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .title(state.title.clone())
            .border_type(BorderType::Rounded)
            .border_style(if state.is_focused {
                ratatui::style::Style::default().fg(Color::Yellow)
            } else {
                ratatui::style::Style::default().fg(Color::White)
            });
        let paragraph = Paragraph::new(state.content.clone())
            .block(block)
            .fg(Color::White)
            .bg(Color::Black);

        paragraph.render(area, buf);
    }
}

fn render_section(frame: &mut Frame, area: Rect, section_state: &mut SectionState) {
    frame.render_stateful_widget(SectionWidget, area, section_state);
}

impl AppState {
    pub fn update(&mut self, delta_time: f32) {
        for section in &mut self.sections {
            if section.animation_progress < 1.0 {
                section.animation_progress += delta_time * 2.0;
                section.animation_progress = section.animation_progress.min(1.0);
            }
        }
    }
}
