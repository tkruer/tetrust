

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


pub struct Tetromino {
    pub shape: Vec<Vec<bool>>,
    pub x: isize,
    pub y: isize,
    pub color: Color,
}

const BLOCK: &str = "█";


const TETROMINO_J: [[bool; 3]; 3] = [
    [true, false, false],
    [true, true, true],
    [false, false, false],
];

const TETROMINO_L: [[bool; 3]; 3] = [
    [false, false, true],
    [true, true, true],
    [false, false, false],
];

const TETROMINO_O: [[bool; 3]; 3] = [
    [false, false, true],
    [true, true, true],
    [false, false, false],
];

const TETROMINO_S: [[bool; 3]; 3] = [
    [false, true, true],
    [true, true, false],
    [false, false, false],
];

const TETROMINO_I: [[bool; 3]; 3] = [
    [false, true, true],
    [true, true, false],
    [false, false, false],
];

const TETROMINO_T: [[bool; 3]; 3] = [
    [false, true, false],
    [true, true, true],
    [false, false, false],
];

const TETROMINO_Z: [[bool; 3]; 3] = [
    [true, true, false],
    [false, true, true],
    [false, false, false],
];


impl Tetromino {
    pub fn new(shape: Vec<Vec<bool>>, color: Color) -> Self {
        Tetromino {
            shape,
            x: 3, // starting x position
            y: 0, // starting y position
            color,
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let tetrominoes = [
            (TETROMINO_I, Color::Cyan),
            (TETROMINO_J, Color::Blue),
            (TETROMINO_L, Color::Rgb { r: 255, g: 165, b: 0 }),
            (TETROMINO_O, Color::Yellow),
            (TETROMINO_S, Color::Green),
            (TETROMINO_T, Color::Magenta),
            (TETROMINO_Z, Color::Red),
        ];

        let (shape, color) = tetrominoes.choose(rng).unwrap();
        Tetromino::new(
            shape.iter().map(|row| row.to_vec()).collect(),
            *color,
        )
    }

    pub fn rotate(&mut self) {
        let size = self.shape.len();
        let mut new_shape = vec![vec![false; size]; size];

        for y in 0..size {
            for x in 0..size {
                new_shape[x][size - 1 - y] = self.shape[y][x];
            }
        }

        self.shape = new_shape;
    }
}

impl Clone for Tetromino {
    fn clone(&self) -> Self {
        Tetromino {
            shape: self.shape.clone(),
            color: self.color,
            x: self.x,
            y: self.y,
        }
    }
}
