use crate::color::Catppuccin;
use crate::color_cycle::{ColorCycle, IndexResolver, PingPongColorCycle};
use ratzilla::ratatui::buffer::Cell;
use ratzilla::ratatui::layout::Position;
use ratzilla::ratatui::style::Color;
use std::fmt::Debug;
use std::time::Instant;
use tachyonfx::{fx, Duration, Effect, Interpolation, RangeSampler, SimpleRng};

pub fn startup_effect() -> Effect {
    let mut effects = vec![];
    let mut rng = SimpleRng::default();
    let initial_delay = Duration::from_millis(300);
    let mut accrued_delay = initial_delay.as_millis();
    let color = Catppuccin::new();

    "rehoboam".char_indices().for_each(|(_, _c)| {
        let delta: u32 = rng.gen_range(100..200);
        accrued_delay += delta;
    });
    accrued_delay += 300;
    effects.push(fx::delay(
        accrued_delay + 200,
        fx::parallel(&[
            fx::never_complete(led_border()),
            fx::fade_from_fg(color.crust, (800, Interpolation::SineOut)),
        ]),
    ));
    fx::parallel(&effects)
}

pub fn led_border() -> Effect {
    let catppuccin = Catppuccin::new();
    let color_1 = catppuccin.sapphire;
    let color_2 = catppuccin.lavender;
    let color_3 = catppuccin.mauve;

    let color_cycle = PingPongColorCycle::new(color_1, &[(40, color_2), (20, color_3)]);
    color_cycle_fg(color_cycle, 100, |cell| {
        let symbol = cell.symbol();
        symbol != " " && !symbol.chars().next().map(is_box_drawing).unwrap_or(false)
    })
}

fn is_box_drawing(c: char) -> bool {
    ('\u{2500}'..='\u{257F}').contains(&c)
}

pub fn color_cycle_fg<I>(
    colors: ColorCycle<I>,
    step_duration: u32,
    predicate: impl Fn(&Cell) -> bool + 'static,
) -> Effect
where
    I: IndexResolver<Color> + Clone + Debug + Send + 'static,
{
    use tachyonfx::fx::*;

    let duration = Duration::from_millis(u32::MAX);
    effect_fn(
        (colors, None),
        duration,
        move |(colors, started_at), _ctx, cell_iter| {
            if started_at.is_none() {
                *started_at = Some(Instant::now());
            }

            let elapsed = started_at.as_ref().unwrap().elapsed().as_millis().max(1);
            let raw_color_idx = elapsed as u32 / step_duration;

            let color = |pos: Position| -> Color {
                let idx = (raw_color_idx + (pos.x / 2 + pos.y * 3 / 2) as u32) as usize;
                *colors.color_at(idx)
            };

            cell_iter
                .filter(|(_, c)| predicate(c))
                .map(|(pos, cell)| (color(pos), cell))
                .for_each(|(color, cell)| {
                    cell.set_fg(color);
                });
        },
    )
}
