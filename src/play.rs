use crate::game::*;
use getch_rs::{Getch, Key};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub fn normal() -> ! {
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
            if !is_collision(&game.field, &new_pos, &game.block) {
                game.pos = new_pos;
            } else if landing(&mut game).is_err() {
                game_over(&game);
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
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Up) => {
                let mut game = game.lock().unwrap();
                hard_drop(&mut game);
                if landing(&mut game).is_err() {
                    game_over(&game);
                }
                draw(&game);
            }
            Ok(Key::Char('z')) => {
                let mut game = game.lock().unwrap();
                rotate_left(&mut game);
                draw(&game);
            }
            Ok(Key::Char('x')) => {
                let mut game = game.lock().unwrap();
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Char(' ')) => {
                let mut game = game.lock().unwrap();
                hold(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                quit();
            }
            _ => (),
        }
    }
}

pub fn auto() -> ! {
    let _ = thread::spawn(|| {
        let mut game = Game::new();
        // 画面クリア・カーソル非表示
        println!("\x1b[2J\x1b[H\x1b[?25l");
        draw(&game);
        loop {
            thread::sleep(time::Duration::from_millis(100));
            let mut rng = rand::thread_rng();
            if rng.gen_range(0..5) == 0 {
                hold(&mut game);
            }
            for _ in 0..rng.gen_range(0..=3) {
                rotate_right(&mut game);
            }
            let dx: isize = rng.gen_range(-4..=5);
            let new_pos = Position {
                x: game.pos.x.checked_add_signed(dx).unwrap(),
                y: game.pos.y,
            };
            move_block(&mut game, new_pos);
            hard_drop(&mut game);
            if landing(&mut game).is_err() {
                game_over(&game);
            }
            draw(&game);
        }
    });

    let g = Getch::new();
    loop {
        if let Ok(Key::Char('q')) = g.getch() {
            quit();
        }
    }
}
