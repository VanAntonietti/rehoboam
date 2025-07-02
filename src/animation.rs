use crate::app::{App, AppState};
use ratzilla::ratatui::Terminal;
use ratzilla::{web_sys, DomBackend, WebRenderer};
use std::{cell::RefCell, io, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn run() -> Result<(), JsValue> {
    setup_and_start().map_err(|e| JsValue::from_str(&format!("Animation error: {}", e)))
}

fn setup_and_start() -> Result<(), io::Error> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let app = Rc::new(App::new());
    let state = Rc::new(RefCell::new(AppState::default()));

    start_animation_loop(state.clone());
    terminal.draw_web(move |frame| {
        let mut state_ref = state.borrow_mut();
        app.render(frame, &mut state_ref);
    });
    Ok(())
}

fn start_animation_loop(state: Rc<RefCell<AppState>>) {
    let mut last_time = None::<f64>;

    let animation_loop = Rc::new(RefCell::new(None));
    let animation_loop_clone = animation_loop.clone();

    *animation_loop_clone.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        let delta = match last_time {
            Some(last) => (timestamp - last) / 1000.0,
            None => 0.016,
        };
        last_time = Some(timestamp);

        state.borrow_mut().update(delta as f32);

        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = document.body() {
                // Reading offsetHeight forces a layout/repaint
                let _ = body.offset_height();
            }
        }

        request_animation_frame(animation_loop.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(animation_loop_clone.borrow().as_ref().unwrap());
}

fn request_animation_frame(closure: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .expect("window should exist")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("requestAnimationFrame should work");
}
