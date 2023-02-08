use crate::mino::{MinoKind, MinoShape, MINOS};

pub const FIELD_WIDTH: usize = 12;
pub const FIELD_HEIGHT: usize = 22;

pub type FieldSize = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Position {
        Position { x: 4, y: 0 }
    }
}

pub struct Game {
    pub field: FieldSize,
    pub pos: Position,
    pub mino: MinoShape,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: [
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
            ],
            pos: Position::init(),
            mino: MINOS[rand::random::<MinoKind>() as usize],
        }
    }
}

pub fn is_collision(field: &FieldSize, pos: &Position, mino: &MinoShape) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] == 1 && mino[y][x] == 1 {
                return true;
            }
        }
    }
    false
}

#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, mino }: &Game) {
    let mut field_buf = *field;
    for y in 0..4 {
        for x in 0..4 {
            field_buf[y + pos.y][x + pos.x] |= mino[y][x];
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

pub fn fix_mino(Game { field, pos, mino }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            field[y + pos.y][x + pos.x] |= mino[y][x];
        }
    }
}

pub fn erase_line(field: &mut FieldSize) {
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
}

pub fn move_mino(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.mino) {
        game.pos = new_pos;
    }
}

pub fn spawn_mino(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::init();
    game.mino = MINOS[rand::random::<MinoKind>() as usize];
    if is_collision(&game.field, &game.pos, &game.mino) {
        Err(())
    } else {
        Ok(())
    }
}

pub fn game_over(game: &Game) -> ! {
    draw(game);
    println!("GAME OVER");
    quit();
}

pub fn quit() -> ! {
    // カーソルを表示
    println!("\x1b[?25h");
    std::process::exit(0);
}
