// main.rs

use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::{thread, time};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use rand::prelude::*;

mod tetris;
use tetris::{Board, Tetromino};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;

    let mut board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);
    let mut rng = rand::thread_rng();

    let mut current_tetromino = Tetromino::random(&mut rng);
    let mut next_tetromino = Tetromino::random(&mut rng);

    let mut last_fall_time = Instant::now();
    let mut input_timeout = Duration::from_millis(100);
    let mut fall_speed = Duration::from_millis(500);

    loop {
        if poll(input_timeout)? {
            match read()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Left => board.move_tetromino(&mut current_tetromino, -1, 0),
                    KeyCode::Right => board.move_tetromino(&mut current_tetromino, 1, 0),
                    KeyCode::Down => board.move_tetromino(&mut current_tetromino, 0, 1),
                    KeyCode::Up => board.rotate_tetromino(&mut current_tetromino),
                    _ => {}
                },
                _ => {}
            }
        }

        if last_fall_time.elapsed() > fall_speed {
            if board.can_move(&current_tetromino, 0, 1) {
                current_tetromino.y += 1;
            } else {
                board.merge_tetromino(&current_tetromino);
                board.clear_lines();

                current_tetromino = next_tetromino.clone();
                next_tetromino = Tetromino::random(&mut rng);

                if board.check_game_over(&current_tetromino) {
                    break;
                }
            }
            last_fall_time = Instant::now();
        }

        board.render(&mut stdout, &current_tetromino)?;
    }

    disable_raw_mode()?;
    Ok(())
}