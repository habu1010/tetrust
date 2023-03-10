pub mod cell;
pub mod tetromino;
use crate::game::tetromino::gen_tetromino_7;
use cell::WALL as W;
use std::collections::VecDeque;
use tetromino::Tetromino;

use self::tetromino::WallKickOffsets;

pub const FIELD_WIDTH: usize = 10 + 2 + 2; // フィールド横幅+壁+番兵
pub const FIELD_HEIGHT: usize = 20 + 1 + 1 + 1; // フィールド縦幅+床+天井+番兵

pub const NEXT_TETROMINOES_SIZE: usize = 3;

pub const SCORE_TABLE: [usize; 5] = [0, 1, 5, 25, 100];

pub type FieldSize = [[cell::Kind; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Position {
        Position { x: 5, y: 1 }
    }
}

#[derive(Clone)]
pub struct Game {
    pub field: FieldSize,
    pub pos: Position,
    pub tetromino: Tetromino,
    pub hold_tetromino: Option<Tetromino>,
    pub held: bool,
    pub next_tetrominoes: VecDeque<Tetromino>,
    pub score: usize,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            field: [
                [0, W, W, W, 0, 0, 0, 0, 0, 0, W, W, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, W, W, W, W, W, W, W, W, W, W, W, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            pos: Position::init(),
            tetromino: Default::default(),
            hold_tetromino: None,
            held: false,
            next_tetrominoes: gen_tetromino_7().into(),
            score: 0,
        };
        spawn_tetromino(&mut game).ok();
        game
    }
}

pub fn is_collision(field: &FieldSize, pos: &Position, tetromino: &Tetromino) -> bool {
    let shape = tetromino.get_shape();
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                return true;
            }
            if field[y + pos.y][x + pos.x] != cell::NONE && shape[y][x] != cell::NONE {
                return true;
            }
        }
    }
    false
}

pub fn hard_drop_pos(field: &FieldSize, pos: &Position, tetromino: &Tetromino) -> Position {
    let mut pos = *pos;
    while {
        let new_pos = Position {
            x: pos.x,
            y: pos.y + 1,
        };
        !is_collision(field, &new_pos, tetromino)
    } {
        pos.y += 1;
    }
    pos
}

pub fn fix_tetromino(
    Game {
        field,
        pos,
        tetromino,
        ..
    }: &mut Game,
) {
    let shape = tetromino.get_shape();
    for y in 0..4 {
        for x in 0..4 {
            field[y + pos.y][x + pos.x] |= shape[y][x];
        }
    }
}

pub fn erase_line(field: &mut FieldSize) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
    count
}

fn wall_kick(game: &Game, tetromino: &Tetromino, offsets: &WallKickOffsets) -> Option<Position> {
    for (dx, dy) in offsets {
        let new_pos = Position {
            x: game.pos.x.checked_add_signed(*dx).unwrap_or(game.pos.x),
            y: game.pos.y.checked_add_signed(*dy).unwrap_or(game.pos.y),
        };
        if is_collision(&game.field, &new_pos, &tetromino) {
            continue;
        }
        return Some(new_pos);
    }
    None
}

pub fn rotate_left(game: &mut Game) {
    let rotated = game.tetromino.rotate_left();
    let offsets = game.tetromino.rotate_left_wall_kick_offsets();
    if let Some(new_pos) = wall_kick(game, &rotated, &offsets) {
        game.pos = new_pos;
        game.tetromino = rotated;
    }
}

pub fn rotate_right(game: &mut Game) {
    let rotated = game.tetromino.rotate_right();
    let offsets = game.tetromino.rotate_right_wall_kick_offsets();
    if let Some(new_pos) = wall_kick(game, &rotated, &offsets) {
        game.pos = new_pos;
        game.tetromino = rotated;
    }
}

pub fn move_tetromino(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.tetromino) {
        game.pos = new_pos;
    }
}

pub fn hard_drop(game: &mut Game) {
    let pos = hard_drop_pos(&game.field, &game.pos, &game.tetromino);
    move_tetromino(game, pos);
}

pub fn hold(game: &mut Game) {
    if game.held {
        return;
    }
    if let Some(mut hold) = game.hold_tetromino {
        std::mem::swap(&mut hold, &mut game.tetromino);
        game.hold_tetromino = Some(hold);
        game.pos = Position::init();
    } else {
        game.hold_tetromino = Some(game.tetromino);
        spawn_tetromino(game).ok();
    }

    game.held = true;
}

pub fn landing(game: &mut Game) -> Result<(), ()> {
    fix_tetromino(game);
    let count = erase_line(&mut game.field);
    game.score += SCORE_TABLE[count];
    spawn_tetromino(game)?;
    game.held = false;
    Ok(())
}

pub fn spawn_tetromino(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::init();
    game.tetromino = game.next_tetrominoes.pop_front().unwrap();
    if game.next_tetrominoes.len() < NEXT_TETROMINOES_SIZE {
        let mut next7: VecDeque<_> = gen_tetromino_7().into();
        game.next_tetrominoes.append(&mut next7);
    }
    if is_collision(&game.field, &game.pos, &game.tetromino) {
        Err(())
    } else {
        Ok(())
    }
}
