use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::{
    ExecutableCommand, cursor, execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use rand::random;
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;

    let (max_x, max_y) = crossterm::terminal::size()?;
    let center_x = max_x as f32 / 2.0;
    let center_y = max_y as f32 / 2.0;
    let radious = (max_x.min(max_y) / 4) as f32;

    let mut t = 0.0;

    loop {
        queue!(stdout, Clear(ClearType::All))?;

        for theta in (0..360).step_by(5) {
            let rad = (theta as f32).to_radians();
            let oscillation = (t * 0.1 + theta as f32 * 0.05).sin() * 2.0;
            let r = radious + oscillation;
            let x = (center_x + r * rad.cos()) as u16;
            let y = (center_y + r * rad.sin()) as u16;
            if x < max_x && y < max_y {
                queue!(stdout, cursor::MoveTo(x, y), Print("#"))?;
            }
        }
    }
}
