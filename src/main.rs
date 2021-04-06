mod frame;
mod input;
mod grid;

use std::{cmp::Ordering, collections::BTreeMap, io::{Write, stdout}, sync::mpsc::channel, time::Duration};
use std::time::Instant;

use crossterm::{QueueableCommand, Result, event::KeyEvent, style::{Color, ContentStyle, PrintStyledContent, StyledContent}};
use event::{Event, KeyCode, KeyModifiers, read};
use frame::Frame;
use grid::Input as Input2;
use input::Input;
use crossterm::*;
use terminal::ClearType;

fn main() -> Result<()> {
    let mut g = grid::Grid::new(40, 10);
    loop {
        //let then = Instant::now() + Duration::from_millis(250);
        let event = if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(val) = read()? {
                val.code
            } else {
                KeyCode::Null
            }
        } else {
            KeyCode::Null
        };
        g.tick(match event {
           KeyCode::Down=> Input2::Down,
           KeyCode::Left=> Input2::Left,
           KeyCode::Right=> Input2::Right,
            KeyCode::Char('c') => panic!("Program exited successfully!"),
            _ => Input2::None
        });
        let mut f = Frame::fill(40, 10, ' ', Color::Black, Color::Black);
        for (i, line) in g.color().iter().enumerate() {
            for (j, block) in line.iter().enumerate() {
                f.add(" ", i as u16, j as u16, Color::Black, *block);
            }
        }
        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout(), crossterm::terminal::Clear(ClearType::All))?;
        frame::display_in_lines(f)?;
    }
    //stop.send(()).expect("Send failed!");
    //let _ = v.join();
    //Ok(())
}
/*
    The multi-layer design: 
    Input -> Handler -> Interior game logic -> Frame
*/