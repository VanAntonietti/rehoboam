mod app;
use app::{App, AppState};
use ratzilla::ratatui::Terminal;
use ratzilla::{DomBackend, WebRenderer};
use std::{cell::RefCell, io, rc::Rc};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let app = Rc::new(App::new());

    let app_state = Rc::new(RefCell::new(AppState::default()));
    let render_state = Rc::clone(&app_state);

    terminal.draw_web(move |frame| {
        let mut state = render_state.borrow_mut();
        state.update(0.016);
        app.render(frame, &mut *state);
    });
    Ok(())
}
