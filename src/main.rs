use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{
        Block, Paragraph, Widget,
        canvas::{Canvas, Circle},
    },
};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    ball: Circle,
    exit: bool,
}

impl App {
    const fn new() -> Self {
        Self {
            exit: false,
            ball: Circle {
                x: 80.00,
                y: 80.00,
                radius: 40.00,
                color: Color::LightBlue,
            },
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        //frame.render_widget(self, frame.area());
        frame.render_widget(self.draw_ball(), frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Char('q') {
            self.exit()
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw_ball(&self) -> impl Widget + '_ {
        Canvas::default().paint(|ctx| {
            ctx.draw(&self.ball);
        })
    }
}

/* impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Rehoboam divergent circle ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);
        Paragraph::new("Rehoboam")
            .centered()
            .block(block)
            .render(area, buf);
    }
}*/

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
