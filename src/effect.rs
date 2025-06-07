use crate::color;
use tachyonfx::fx::{effect_fn_buf, parallel, prolong_start, sequence, sleep, sweep_in};
use tachyonfx::{fx, Duration, Effect, SimpleRng};

pub fn startup_effect() -> Effect {
    let mut effects = vec![];
    let initial_delay = Duration::from_millis(300);
    let mut accrued_delay = initial_delay.as_millis();
    effects.push(fx::delay(, effect));
}
