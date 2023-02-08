mod game;
mod mino;

use game::{FieldSize, Game, Position, FIELD_HEIGHT, FIELD_WIDTH};
use getch_rs::{Getch, Key};
use mino::{MinoKind, MINOS};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn is_collision(field: &FieldSize, pos: &Position, kind: MinoKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] == 1 && MINOS[kind as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn draw(Game { field, pos, kind }: &Game) {
    let mut field_buf = field.clone();
    for y in 0..4 {
        for x in 0..4 {
            field_buf[y + pos.y][x + pos.x] |= MINOS[*kind as usize][y][x];
        }
    }

    println!("\x1b[H");
    // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 0 {
                print!(" .");
            } else {
                print!("[]");
            }
        }
        println!();
    }
}

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア・カーソル非表示
    println!("\x1b[2J\x1b[H\x1b[?25l");

    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(1000));
            let mut game = game.lock().unwrap();
            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            if !is_collision(&game.field, &new_pos, game.kind) {
                game.pos = new_pos;
            } else {
                let gx = game.pos.x;
                let gy = game.pos.y;
                for y in 0..4 {
                    for x in 0..4 {
                        game.field[y + gy][x + gx] |= MINOS[game.kind as usize][y][x];
                    }
                }
                for y in 1..FIELD_HEIGHT - 1 {
                    let mut can_erase = true;
                    for x in 0..FIELD_WIDTH {
                        if game.field[y][x] == 0 {
                            can_erase = false;
                            break;
                        }
                    }
                    if can_erase {
                        for y2 in (2..=y).rev() {
                            game.field[y2] = game.field[y2 - 1];
                        }
                    }
                }
                game.pos = Position::init();
                game.kind = rand::random();
            }
            draw(&game);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                if !is_collision(&game.field, &new_pos, game.kind) {
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or_else(|| game.pos.x),
                    y: game.pos.y,
                };
                if !is_collision(&game.field, &new_pos, game.kind) {
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                if !is_collision(&game.field, &new_pos, game.kind) {
                    game.pos = new_pos;
                }
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                // カーソルを表示
                println!("\x1b[?25h");
                return;
            }
            _ => (),
        }
    }
}
