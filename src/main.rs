use app::App;
use ratatui::Terminal;
use ratzilla::{DomBackend, WebRenderer};
use rehoboam::{app, effect, layout, modules};
use std::{io, rc::Rc};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let app = Rc::new(App::new());

    terminal.draw_web(move |frame| {
        app.render(frame);
    });
    Ok(())
}
