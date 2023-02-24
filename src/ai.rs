use crate::game::*;

pub fn eval(game: &Game) -> Game {
    let mut elite = (game.clone(), 0f64);

    for do_hold in [true, false] {
        let mut game = game.clone();
        if do_hold {
            hold(&mut game);
        }
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
                let height_diff = diff_in_height(&game.field);
                let dead_space_count = dead_space_count(&game.field);

                // 正規化 & 重み付け
                let line_count = normalization(line_count as f64, 0.0, 4.0);
                let height_max = 1.0 - normalization(height_max as f64, 0.0, 20.0);
                let height_diff = 1.0 - normalization(height_diff as f64, 0.0, 200.0);
                let dead_space_count = 1.0 - normalization(dead_space_count as f64, 0.0, 200.0);
                let line_count = line_count * 100.0;
                let height_max = height_max * 1.0;
                let height_diff = height_diff * 10.0;
                let dead_space_count = dead_space_count * 30.0;

                let score = line_count + height_max + height_diff + dead_space_count;
                if elite.1 < score {
                    let mut game = game_rotated.clone();
                    move_block(&mut game, new_pos);
                    elite.0 = game;
                    elite.1 = score;
                }
            }
        }
    }
    elite.0
}

pub fn normalization(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}

pub fn erase_line_count(field: &FieldSize) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let line = &field[y][2..FIELD_WIDTH - 2];
        if line.iter().all(|&f| f != cell::NONE) {
            count += 1;
        }
    }
    count
}

pub fn field_height_max(field: &FieldSize) -> usize {
    for y in 1..FIELD_HEIGHT - 2 {
        let line = &field[y][2..FIELD_WIDTH - 2];
        if line.iter().any(|&f| f != cell::NONE) {
            return FIELD_HEIGHT - y - 2;
        }
    }
    unreachable!();
}

pub fn diff_in_height(field: &FieldSize) -> usize {
    let mut top = [0; FIELD_WIDTH - 4];
    for x in 2..FIELD_WIDTH - 2 {
        for y in 1..FIELD_HEIGHT - 2 {
            if field[y][x] != cell::NONE {
                top[x - 2] = FIELD_HEIGHT - y - 2;
                break;
            }
        }
    }

    let adjacent_pair_iter = top.iter().zip(top.iter().skip(1));
    adjacent_pair_iter.fold(0, |sum, i| sum + i.0.abs_diff(*i.1))
}

pub fn dead_space_count(field: &FieldSize) -> usize {
    let mut count = 0;
    for y in (1..FIELD_HEIGHT - 2).rev() {
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == cell::NONE {
                for y2 in (2..y).rev() {
                    if field[y2][x] != cell::NONE {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}
