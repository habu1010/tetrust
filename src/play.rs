use crate::ai::eval;
use crate::game::*;
use crate::ui;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::error::Error;
use std::{thread, time};

pub fn normal() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();
    let mut ui = ui::Ui::new()?;

    let mut next_auto_drop = time::Instant::now() + time::Duration::from_millis(1000);
    loop {
        ui.draw(&game)?;

        let wait_duration = next_auto_drop
            .checked_duration_since(time::Instant::now())
            .unwrap_or(time::Duration::ZERO);

        if !event::poll(wait_duration)? {
            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            if !is_collision(&game.field, &new_pos, &game.tetromino) {
                game.pos = new_pos;
            } else if landing(&mut game).is_err() {
                let _ = ui.game_over(&game);
                return ui.shutdown();
            }
            next_auto_drop = time::Instant::now() + time::Duration::from_millis(1000);
            continue;
        }

        let result = match event::read()? {
            Event::Key(key) => match process_key_input(&mut game, key) {
                Some(result) => result,
                _ => continue,
            },
            _ => continue,
        };

        match result {
            KeyInputProcessResult::NextAutoDropInstant(instant) => {
                next_auto_drop = instant;
            }
            KeyInputProcessResult::GameOver => {
                let _ = ui.game_over(&game);
                return ui.shutdown();
            }
            KeyInputProcessResult::QuitGame => {
                return ui.shutdown();
            }
        }
    }
}

enum KeyInputProcessResult {
    NextAutoDropInstant(time::Instant),
    QuitGame,
    GameOver,
}

fn process_key_input(game: &mut Game, key: KeyEvent) -> Option<KeyInputProcessResult> {
    match key.code {
        KeyCode::Down => {
            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            move_tetromino(game, new_pos);
            return Some(KeyInputProcessResult::NextAutoDropInstant(
                time::Instant::now() + time::Duration::from_millis(1000),
            ));
        }
        KeyCode::Left => {
            let new_pos = Position {
                x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                y: game.pos.y,
            };
            move_tetromino(game, new_pos);
        }
        KeyCode::Right => {
            let new_pos = Position {
                x: game.pos.x + 1,
                y: game.pos.y,
            };
            move_tetromino(game, new_pos);
        }
        KeyCode::Up => {
            hard_drop(game);
            if landing(game).is_err() {
                return Some(KeyInputProcessResult::GameOver);
            }
        }
        KeyCode::Char('z') => {
            rotate_left(game);
        }
        KeyCode::Char('x') => {
            rotate_right(game);
        }
        KeyCode::Char(' ') => {
            hold(game);
        }
        KeyCode::Char('q') => {
            return Some(KeyInputProcessResult::QuitGame);
        }
        _ => (),
    }

    None
}

pub fn auto() -> Result<(), Box<dyn Error>> {
    let mut game = Game::new();
    let mut ui = ui::Ui::new()?;

    let wait_duration = time::Duration::from_millis(100);

    loop {
        ui.draw(&game)?;

        if !event::poll(wait_duration)? {
            let elite = eval(&game);
            game = elite;
            ui.draw(&game)?;
            thread::sleep(wait_duration);
            hard_drop(&mut game);
            if landing(&mut game).is_err() {
                let _ = ui.game_over(&game);
                return ui.shutdown();
            }
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return ui.shutdown();
            }
        }
    }
}
