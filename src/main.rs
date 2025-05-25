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
    let radius = (max_x.min(max_y) / 4) as f32;

    let mut t = 0.0;
    let mut divergence_active = false;
    let mut divergence_start = 0.0;
    let mut divergence_angle = 0.0;

    loop {
        queue!(stdout, Clear(ClearType::All))?;

        for theta in (0..360).step_by(5) {
            let rad = (theta as f32).to_radians();
            let oscillation = (t * 0.1 + theta as f32 * 0.05).sin() * 2.0;
            let r = radius + oscillation;
            let x = (center_x + r * rad.cos()) as u16;
            let y = (center_y + r * rad.sin()) as u16;
            if x < max_x && y < max_y {
                queue!(stdout, cursor::MoveTo(x, y), Print("#"))?;
            }
        }

        if random::<f32>() < 0.02 && !divergence_active {
            divergence_active = true;
            divergence_start = t;
            divergence_angle = random::<f32>() * 360.0;
        }

        if divergence_active {
            for r in (radius as u32)..(radius as u32 + 5) {
                let rad = divergence_angle.to_radians();
                let x = (center_x + r as f32 * rad.cos()) as u16;
                let y = (center_y + r as f32 * rad.sin()) as u16;
                if x < max_x && y < max_y {
                    queue!(stdout, cursor::MoveTo(x, y), Print("*"))?;
                }
            }
            if t - divergence_start > 20.0 {
                divergence_active = false;
            }
        }

        let center_x_int = center_x as u16;
        let center_y_int = center_y as u16;
        if center_x_int < max_x && center_y_int < max_y {
            queue!(
                stdout,
                cursor::MoveTo(center_x_int, center_y_int),
                Print("O")
            )?;
        }

        let status = if divergence_active {
            "DIVERGENCE DETECTED"
        } else {
            "SYSTEM STATUS: NOMINAL"
        };
        queue!(stdout, cursor::MoveTo(0, max_y - 2), Print(status))?;

        stdout.flush()?;
        t += 1.0;
        thread::sleep(Duration::from_millis(50));

        if poll(Duration::from_millis(10))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    execute!(stdout, cursor::Show)?;
    Ok(())
}
