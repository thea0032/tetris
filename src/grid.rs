use std::collections::HashMap;

use crossterm::style::Color;
use rand::random;
#[derive(Debug)]
struct Piece {
    positions: [(usize, usize); 4],
    color: Color,
}
impl Piece {
    pub fn default() -> Piece {
        Piece {
            color: Color::Black,
            positions: [(0, 0); 4],
        }
    }
}
pub struct Grid {
    filled: Vec<Vec<bool>>,
    color: Vec<Vec<Color>>,
    piece: Piece,
    rows: usize,
    cols: usize,
    changes: Option<HashMap<(usize, usize), Color>>,
}
pub enum Input {
    Left,
    Right,
    Down,
    Up,
    None,
}
impl Grid {
    fn generate_line(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0 + 1, offset.1),
            (offset.0 + 2, offset.1),
            (offset.0 + 3, offset.1),
        ]
    }
    fn generate_line_v(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0, offset.1 + 1),
            (offset.0, offset.1 + 2),
            (offset.0, offset.1 + 3),
        ]
    }
    fn generate_l(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0 + 1, offset.1),
            (offset.0 + 1, offset.1 + 1),
            (offset.0 + 1, offset.1 + 2),
            (offset.0, offset.1),
        ]
    }
    fn generate_l_v(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0, offset.1 + 1),
            (offset.0, offset.1 + 2),
            (offset.0 + 1, offset.1),
        ]
    }
    fn generate_t(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0 + 1, offset.1),
            (offset.0 + 2, offset.1),
            (offset.0 + 1, offset.1 + 1),
        ]
    }
    fn generate_t_v(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0, offset.1 + 1),
            (offset.0, offset.1 + 2),
            (offset.0 + 1, offset.1 + 1),
        ]
    }
    fn generate_s(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0, offset.1 + 1),
            (offset.0 + 1, offset.1 + 1),
            (offset.0 + 1, offset.1 + 2),
        ]
    }
    fn generate_s_v(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0 + 1, offset.1),
            (offset.0 + 1, offset.1 + 1),
            (offset.0 + 2, offset.1 + 1),
        ]
    }
    fn generate_square(offset: (usize, usize)) -> [(usize, usize); 4] {
        [
            (offset.0, offset.1),
            (offset.0 + 1, offset.1),
            (offset.0, offset.1 + 1),
            (offset.0 + 1, offset.1 + 1),
        ]
    }
    fn generate_piece(&self) -> Option<Piece> {
        let mut counter = 0;
        let positions: [(usize, usize); 4] = loop {
            let offset: usize = random::<usize>() % self.rows;
            let trial_run = match rand::random::<u8>() % 10 {
                0 | 1 => Grid::generate_square((0, offset)),
                2 => Grid::generate_l((0, offset)),
                3 => Grid::generate_l_v((0, offset)),
                4 => Grid::generate_line((0, offset)),
                5 => Grid::generate_line_v((0, offset)),
                6 => Grid::generate_t((0, offset)),
                7 => Grid::generate_t_v((0, offset)),
                8 => Grid::generate_s((0, offset)),
                9 => Grid::generate_s_v((0, offset)),
                _ => panic!("Out of bounds!"),
            };
            if self.validate(&trial_run) {
                break trial_run;
            }
            if counter > 1000 {
                return None;
            }
            counter += 1;
        };
        Some(Piece {
            positions,
            color: Color::Rgb {
                r: rand::random(),
                g: rand::random(),
                b: rand::random(),
            },
        })
    }
    pub fn new(w: usize, h: usize) -> Grid {
        let mut g = Grid {
            filled: vec![vec![false; h]; w],
            color: vec![vec![Color::Black; h]; w],
            piece: Piece::default(),
            rows: h,
            cols: w,
            changes: None,
        };
        g.piece = g
            .generate_piece()
            .expect("If you see this message, the grid is flawed!");
        g
    }
    pub fn tick(&mut self, input: Input) {
        self.changes = Some(HashMap::new());
        self.check_for_lines();
        match input {
            Input::Left => self.move_left(),
            Input::Right => self.move_right(),
            Input::Down => self.move_down(),
            Input::Up => self.rotate(),
            Input::None => false,
        };
        if !self.move_down() {
            for line in &self.piece.positions {
                self.filled[line.0][line.1] = true;
                self.color[line.0][line.1] = self.piece.color;
            }
            if let Some(val) = self.generate_piece() {
                self.piece = val;
            } else {
                panic!("Your tetris game has come to an end!");
            }
            //todo!()
        }
    }
    fn check_for_lines(&mut self) {
        let mut to_rmv: Vec<usize> = vec![];
        for (i, line) in self.filled.iter().enumerate() {
            if line.iter().all(|x| *x) {
                to_rmv.push(i);
            }
        }
        let len = to_rmv.len();
        for line in to_rmv.into_iter().rev() {
            self.color.remove(line);
            self.filled.remove(line);
        }
        if len > 0 {
            self.changes = None;
        }
        for _ in 0..len {
            self.color.insert(0, vec![Color::Black; self.rows]);
            self.filled.insert(0, vec![false; self.rows]);
        }
    }
    fn can_move(&self, (dir_x, dir_y): (i32, i32)) -> bool {
        for line in &self.piece.positions {
            let new_line = (try_add(line.0, dir_x), try_add(line.1, dir_y)); //subtracts 1 from the x value
            if let (Some(x), Some(y)) = new_line {
                if !self.is_valid((x, y)) {
                    return false; // We're inside the grid, but collided with an enemy!
                }
            } else {
                return false; // We go outside the grid!
            }
        }
        return true;
    }
    fn do_move(&mut self, (dir_x, dir_y): (i32, i32)) -> bool {
        if !self.can_move((dir_x, dir_y)) {
            return false;
        }
        // performs move:
        for (i, j) in &self.piece.positions {
            self.color[*i][*j] = Color::Black;
            if let Some(val) = &mut self.changes {
                val.insert((*i, *j), Color::Black);
            }
        }
        
        for line in &mut self.piece.positions {
            if dir_x > 0 {
                line.0 += dir_x as usize;
            } else {
                line.0 -= (-dir_x) as usize;
            }
            if dir_y > 0 {
                line.1 += dir_y as usize;
            } else {
                line.1 -= (-dir_y) as usize;
            }
        }
        for (i, j) in &self.piece.positions {
            self.color[*i][*j] = self.piece.color;
            if let Some(val) = &mut self.changes {
                val.insert((*i, *j), self.piece.color);
            }
        }
        return true;
    }
    fn move_left(&mut self) -> bool {
        self.do_move((0, -1))
    }
    fn move_right(&mut self) -> bool {
        self.do_move((0, 1))
    }
    fn move_down(&mut self) -> bool {
        self.do_move((1, 0))
    }
    fn rotate(&mut self) -> bool {
        let (x_start, y_start) = self.piece.positions[0];
        let mut new_pos: [(usize, usize); 4] = self.piece.positions.clone();
        for (x_pos, y_pos) in &mut new_pos {
            let new_y_pos = *x_pos as i32 + y_start as i32 - x_start as i32;
            let new_x_pos = *y_pos as i32 + x_start as i32 - y_start as i32;
            if new_x_pos < 0 || new_y_pos < 0 {
                return false;
            } else {
                *x_pos = new_x_pos as usize;
                *y_pos = new_y_pos as usize;
            }
        }
        for (i, j) in &self.piece.positions {
            self.color[*i][*j] = Color::Black;
            if let Some(val) = &mut self.changes {
                val.insert((*i, *j), Color::Black);
            }
        }
        if self.validate(&new_pos) {
            self.piece.positions = new_pos;
            for (i, j) in &self.piece.positions {
                self.color[*i][*j] = self.piece.color;
                if let Some(val) = &mut self.changes {
                    val.insert((*i, *j), self.piece.color);
                }
            }
            return true;
        }
        for (i, j) in &self.piece.positions {
            self.color[*i][*j] = self.piece.color;
            if let Some(val) = &mut self.changes {
                val.insert((*i, *j), self.piece.color);
            }
        }
        return false;
    }
    fn is_valid(&self, (x, y): (usize, usize)) -> bool {
        x < self.cols && y < self.rows && !self.filled[x][y]
    }
    fn validate(&self, val: &[(usize, usize); 4]) -> bool {
        val.iter().all(|x| self.is_valid(*x))
    }

    /// Get a reference to the grid's color.
    pub fn color(&self) -> &Vec<Vec<Color>> {
        &self.color
    }

    /// Get a reference to the grid's changes.
    pub fn changes(&self) -> &Option<HashMap<(usize, usize), Color>> {
        &self.changes
    }
}
fn try_add(lhs: usize, rhs: i32) -> Option<usize> {
    let result: i32 = (lhs as i32) + rhs;
    if result < 0 {
        None
    } else {
        Some(result as usize)
    }
}
