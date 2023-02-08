mod mino;

use getch_rs::{Getch, Key};
use mino::{MinoKind, MINOS};
use std::sync::{Arc, Mutex};
use std::{thread, time};

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 22;

type FieldSize = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

struct Position {
    x: usize,
    y: usize,
}

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

fn draw(field: &FieldSize, pos: &Position, kind: MinoKind) {
    let mut field_buf = field.clone();
    for y in 0..4 {
        for x in 0..4 {
            field_buf[y + pos.y][x + pos.x] |= MINOS[kind as usize][y][x];
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
    let field = Arc::new(Mutex::new([
        [1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]));
    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let mino = Arc::new(Mutex::new(rand::random::<MinoKind>()));

    // 画面クリア・カーソル非表示
    println!("\x1b[2J\x1b[H\x1b[?25l");

    draw(
        &field.lock().unwrap(),
        &pos.lock().unwrap(),
        *mino.lock().unwrap(),
    );

    {
        let pos = Arc::clone(&pos);
        let field = Arc::clone(&field);
        let mino = Arc::clone(&mino);
        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(1000));
            let mut pos = pos.lock().unwrap();
            let mut field = field.lock().unwrap();
            let mut mino = mino.lock().unwrap();
            let new_pos = Position {
                x: pos.x,
                y: pos.y + 1,
            };
            if !is_collision(&field, &new_pos, *mino) {
                *pos = new_pos;
            } else {
                for y in 0..4 {
                    for x in 0..4 {
                        field[y + pos.y][x + pos.x] |= MINOS[*mino as usize][y][x];
                    }
                }
                for y in 1..FIELD_HEIGHT - 1 {
                    let mut can_erase = true;
                    for x in 0..FIELD_WIDTH {
                        if field[y][x] == 0 {
                            can_erase = false;
                            break;
                        }
                    }
                    if can_erase {
                        for y2 in (2..=y).rev() {
                            field[y2] = field[y2 - 1];
                        }
                    }
                }
                *pos = Position { x: 4, y: 0 };
                *mino = rand::random();
            }
            draw(&field, &pos, *mino);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Down) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *mino);
            }
            Ok(Key::Left) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let new_pos = Position {
                    x: pos.x.checked_sub(1).unwrap_or_else(|| pos.x),
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *mino);
            }
            Ok(Key::Right) => {
                let mut pos = pos.lock().unwrap();
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    *pos = new_pos;
                }
                draw(&field, &pos, *mino);
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
