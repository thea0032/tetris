mod frame;
mod grid;

use std::time::Duration;

use crossterm::*;
use crossterm::{style::Color, Result};
use event::{read, Event, KeyCode};
use frame::Frame;
use grid::Input;

fn main() -> Result<()> {
    let mut g = grid::Grid::new(40, 10);
    let f = Frame::fill(40, 10, ' ', Color::Black, Color::Black);
    frame::display_in_lines(f)?;
    crossterm::terminal::enable_raw_mode()?;
    loop {
        //let then = Instant::now() + Duration::from_millis(250);
        let event = if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(val) = read()? {
                val.code
            } else {
                KeyCode::Null
            }
        } else {
            KeyCode::Null
        };
        g.tick(match event {
            KeyCode::Down => Input::Down,
            KeyCode::Left => Input::Left,
            KeyCode::Right => Input::Right,
            KeyCode::Up => Input::Up,
            KeyCode::Char('c') => panic!("Program exited successfully!"),
            _ => Input::None,
        });
        if let Some(val) = g.changes() {
            let mut f = Frame::empty();
            for ((x_pos, y_pos), color) in val {
                f.add(" ", *x_pos as u16, *y_pos as u16, Color::Black, *color);
            }
            frame::display_in_lines(f)?;
        } else {
            let mut f = Frame::empty();
            for (i, line) in g.color().iter().enumerate() {
                for (j, block) in line.iter().enumerate() {
                    f.add(" ", i as u16, j as u16, Color::Black, *block);
                }
            }
            frame::display_in_lines(f)?;
        }
        //frame::display_in_lines(f)?;
    }
    //stop.send(()).expect("Send failed!");
    //let _ = v.join();
    //Ok(())
}
/*
    The multi-layer design:
    Input -> Handler -> Interior game logic -> Frame
*/
