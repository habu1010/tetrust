use crate::block::block_kind;
use crate::game::*;

pub fn eval(game: &Game) -> Game {
    let mut elite = (game.clone(), 0, FIELD_HEIGHT);

    for rotate_count in 0..=3 {
        let mut game = game.clone();
        for _ in 0..=rotate_count {
            rotate_right(&mut game);
        }
        let game_rotated = &game;

        for dx in -4..=5 {
            let mut game = game_rotated.clone();
            let new_pos = Position {
                x: game.pos.x.checked_add_signed(dx).unwrap_or(0),
                y: game.pos.y,
            };
            move_block(&mut game, new_pos);
            hard_drop(&mut game);
            fix_block(&mut game);

            let line_count = erase_line_count(&game.field);
            let height_max = field_height_max(&game.field);

            if line_count >= elite.1 && height_max <= elite.2 {
                let mut game = game_rotated.clone();
                move_block(&mut game, new_pos);
                elite.0 = game;
                elite.1 = line_count;
                elite.2 = height_max;
            }
        }
    }
    elite.0
}

pub fn erase_line_count(field: &FieldSize) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let line = &field[y][2..FIELD_WIDTH - 2];
        if line.iter().all(|&f| f != block_kind::NONE) {
            count += 1;
        }
    }
    count
}

pub fn field_height_max(field: &FieldSize) -> usize {
    for y in 1..FIELD_HEIGHT - 2 {
        let line = &field[y][2..FIELD_WIDTH - 2];
        if line.iter().any(|&f| f != block_kind::NONE) {
            return FIELD_HEIGHT - y - 2;
        }
    }
    unreachable!();
}
