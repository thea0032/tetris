use std::{io::{Read, stdin}, sync::mpsc::{Receiver, Sender}, time::Duration};

use crossterm::{Result, event::{Event, KeyCode, KeyEvent, read}};


pub struct Input {
    interval: Duration,
}
impl Input {
    pub fn new(interval: Duration, ) -> Self { Self { interval, } }


    pub fn get_forever(&self, send: Sender<KeyEvent>, stop: Receiver<()>) -> Result<()>{
        loop {
            let val = read()?;
            if let Ok(()) = stop.try_recv() {
                break Ok(());
            }
            match val {
                Event::Key(v) => {send.send(v).expect("Something went horribly wrong!");},
                _ => (),
            }
        }
    }
}