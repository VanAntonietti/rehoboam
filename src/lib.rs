pub mod animation;
pub mod app;
pub mod color;
pub mod color_cycle;
pub mod effect;
pub mod layout;
pub mod modules;
pub mod ui_effect;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    animation::run()
}
