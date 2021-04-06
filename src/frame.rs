use std::{cmp::Ordering, collections::BTreeMap, io::{Write, stdout}};

use crossterm::{QueueableCommand, style::{Color, ContentStyle, PrintStyledContent, StyledContent}};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct FrameLocation (u16, u16);
impl PartialOrd for FrameLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.0.cmp(&other.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.1.cmp(&other.1),
            Ordering::Greater => Ordering::Greater,
        })
    }
}
impl Ord for FrameLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("It actually works!")
    }
}
/// This is a terminal frame. It can be displayed.
/// It is comprised of crossterm's styled content, and locations. 
/// When printing stuff out to a frame, the content is iterated through, based
/// on starting location. All content should be 1 line to avoid unpredictable
/// behavior. 
/// The cursor is moved to the location, where it prints out the string in question. 
/// As this is styled content, each string can only have one style. 
/// For multiple styles, split up your strings.
/// As putting multiple strings on the same location overrides a lot of stuff, 
/// using multiple frames - even for one screen at one time - is encouraged. 
/// Content will be displayed left to right - content on the right overrides
/// content to the left. 
pub struct Frame {
    content: BTreeMap<FrameLocation, StyledContent<String>>,
}
impl Frame {
    /// Creates an empty terminal frame that overwrites nothing. 
    pub fn empty() -> Frame {
        Frame {
            content: BTreeMap::new(),
        }
    }
    /// Creates a terminal frame that fills the entire screen (size of the screen
    /// indicated by rows and cols) with a character of one color, in a background
    /// of another color. Filling with spaces will create a solid color (bg). 
    pub fn fill(rows: u16, cols: u16, chr: char, fg: Color, bg: Color) -> Frame {
        let mut res:BTreeMap<FrameLocation, StyledContent<String>> = BTreeMap::new();
        for i in 0..rows {
            let style = ContentStyle::new().foreground(fg).background(bg);
            let str = vec![chr; cols.into()].into_iter().collect();
            let content = StyledContent::new(style, str);
            res.insert(FrameLocation(i, 0), content);
        }

        Frame {
            content: res
        }
    }
    /// Adds a styled string to the terminal frame. Note that if there is
    /// alread a string in the same position as this one, it will be removed. 
    pub fn add(&mut self, str: &str, x: u16, y: u16, fg: Color, bg: Color) {
        let style = ContentStyle::new().foreground(fg).background(bg);
        let content = StyledContent::new(style, str.to_string());
        self.content.insert(FrameLocation(x, y), content);
    }
    /// Adds a blank line to the terminal frame. Note that this does not override
    /// any text that already exists. 
    pub fn blank_line(&mut self, row: u16, cols: u16, bg: Color) {
        let str:String = vec![' '; cols.into()].into_iter().collect();
        let style = ContentStyle::new().background(bg);
        let content = StyledContent::new(style, str);
        self.content.insert(FrameLocation(row, 0), content);
    }
    /// Clears the terminal frame completely, removing all styled text and locations. 
    pub fn clear(&mut self) {
        self.content.clear();
    }
    /// Removes all elements from the terminal frame that match a pattern. 
    pub fn filter<P>(&mut self, mut p: P) where P: FnMut(&FrameLocation, &mut StyledContent<String>) -> bool {
        let mut to_remove:Vec<FrameLocation> = vec![];
        for (location, content) in self.content.iter_mut() {
            if p(location, content) {
                to_remove.push(*location);
            }
        }
        for line in to_remove {
            self.content.remove(&line);
        }
    }
}

pub fn display_in_lines(content:Frame) -> crossterm::Result<()>{
    let mut stdout = stdout();
    for (FrameLocation(x, y), line) in content.content {
        stdout.queue(crossterm::cursor::MoveTo(y, x))?
            .queue(PrintStyledContent(line))?;
    }
    stdout.flush()?;
    Ok(())
}
pub fn display_all_in_lines(content:Vec<Frame>) -> crossterm::Result<()>{
    content.into_iter().fold(Ok(()), 
        |x, y| 
        x.and_then(|_|
            display_in_lines(y)))
}