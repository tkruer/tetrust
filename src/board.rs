
use std::io::{stdout, Write};
use rand::Rng;
use rand::seq::SliceRandom;
use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    Result,
};

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<bool>>,
}


impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            data: vec![vec![false; width]; height],
        }
    }

    // Additional methods for Board will go here
    pub fn can_move(&self, tetromino: &Tetromino, dx: isize, dy: isize) -> bool {
        for (y, row) in tetromino.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let x = (tetromino.x + x as isize + dx) as usize;
                    let y = (tetromino.y + y as isize + dy) as usize;

                    if x >= self.width || y >= self.height || self.data[y][x] {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn move_tetromino(&mut self, tetromino: &mut Tetromino, dx: isize, dy: isize) {
        if self.can_move(tetromino, dx, dy) {
            tetromino.x += dx;
            tetromino.y += dy;
        }
    }

    pub fn rotate_tetromino(&mut self, tetromino: &mut Tetromino) {
        tetromino.rotate();

        if !self.can_move(tetromino, 0, 0) {
            tetromino.rotate(); // Revert rotation if not possible
            tetromino.rotate();
            tetromino.rotate();
        }
    }

    pub fn merge_tetromino(&mut self, tetromino: &Tetromino) {
        for (y, row) in tetromino.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let x = (tetromino.x + x as isize) as usize;
                    let y = (tetromino.y + y as isize) as usize;

                    self.data[y][x] = true;
                }
            }
        }
    }

    pub fn clear_lines(&mut self) {
        let mut lines_to_clear = vec![];

        for (y, row) in self.data.iter().enumerate() {
            if row.iter().all(|&cell| cell) {
                lines_to_clear.push(y);
            }
        }

        for &line in &lines_to_clear {
            self.data.remove(line);
            self.data.insert(0, vec![false; self.width]);
        }
    }

    pub fn check_game_over(&self, tetromino: &Tetromino) -> bool {
        !self.can_move(tetromino, 0, 0)
    }

    pub fn render(
        &self,
        stdout: &mut std::io::Stdout,
        tetromino: &Tetromino,
    ) -> crossterm::Result<()> {
        execute!(stdout, cursor::Hide)?;

        for (y, row) in self.data.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, y as u16))?;
            for &cell in row {
                if cell {
                    write!(stdout, "{}", BLOCK)?;
            
                } else {
                    write!(stdout, " ")?;
                }
            }
        }

        // Render the current tetromino
        for (y, row) in tetromino.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let x = (tetromino.x + x as isize) as usize;
                    let y = (tetromino.y + y as isize) as usize;
                    execute!(
                        stdout,
                        cursor::MoveTo(x as u16, y as u16),
                        SetForegroundColor(tetromino.color),
                        Print(BLOCK),
                        ResetColor
                    )?;
                }
            }
        }

        stdout.flush()?;
        Ok(())
    }
}



