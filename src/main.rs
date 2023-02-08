use getch_rs::{Getch, Key};
use std::{thread, time};

struct Position {
    x: usize,
    y: usize,
}

// テトリミノの種類
#[derive(Clone, Copy)]
enum MinoKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

// テトリミノの形状
const MINOS: [[[usize; 4]; 4]; 7] = [
    [
        // I
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
    ],
    [
        // O
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // S
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // Z
        [0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // J
        [0, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // L
        [0, 0, 0, 0],
        [0, 0, 1, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // T
        [0, 0, 0, 0],
        [0, 1, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
];

fn is_collision(field: &[[usize; 12]], pos: &Position, kind: MinoKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y + pos.y][x + pos.x] == 1 && MINOS[kind as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let field = [
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
    ];

    let mut pos = Position { x: 4, y: 0 };
    let g = Getch::new();

    // 画面クリア・カーソル非表示
    println!("\x1b[2J\x1b[H\x1b[?25l");

    loop {
        let new_pos = Position {
            x: pos.x,
            y: pos.y + 1,
        };
        if !is_collision(&field, &new_pos, MinoKind::I) {
            pos = new_pos;
        }

        let mut field_buf = field;
        for y in 0..4 {
            for x in 0..4 {
                field_buf[y + pos.y][x + pos.x] |= MINOS[MinoKind::I as usize][y][x];
            }
        }

        println!("\x1b[H"); // カーソルを先頭に移動
        for y in 0..22 {
            for x in 0..12 {
                if field_buf[y][x] == 0 {
                    print!(" .");
                } else {
                    print!("[]");
                }
            }
            println!();
        }

        thread::sleep(time::Duration::from_millis(1000));

        match g.getch() {
            Ok(Key::Down) => {
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Left) => {
                let new_pos = Position {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Right) => {
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, MinoKind::I) {
                    pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }

    // カーソルを表示
    println!("\x1b[?25h");
}
