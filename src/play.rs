use crate::ai::eval;
use crate::game::*;
use crate::timer::Timer;
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub fn normal() -> ! {
    let game = Arc::new(Mutex::new(Game::new()));
    let drop_timer = Arc::new(Timer::new());

    // 画面クリア・カーソル非表示
    println!("\x1b[2J\x1b[H\x1b[?25l");

    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let drop_timer = Arc::clone(&drop_timer);

        let _ = thread::spawn(move || loop {
            drop_timer.wait(time::Duration::from_millis(1000));
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
                drop_timer.reset();
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
        loop {
            draw(&game);
            thread::sleep(time::Duration::from_millis(100));
            let elite = eval(&game);
            game = elite;
            draw(&game);
            thread::sleep(time::Duration::from_millis(100));
            hard_drop(&mut game);
            if landing(&mut game).is_err() {
                game_over(&game);
            }
        }
    });

    let g = Getch::new();
    loop {
        if let Ok(Key::Char('q')) = g.getch() {
            quit();
        }
    }
}
