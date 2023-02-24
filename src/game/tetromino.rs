use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};

use crate::game::cell::{I, J, L, O, S, T, Z};

// テトリミノの種類
const KIND_MAX: usize = 7;
#[derive(Clone, Copy)]
pub enum Kind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl Distribution<Kind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Kind {
        match rng.gen_range(0..=6) {
            0 => Kind::I,
            1 => Kind::O,
            2 => Kind::S,
            3 => Kind::Z,
            4 => Kind::J,
            5 => Kind::L,
            _ => Kind::T,
        }
    }
}

pub type Shape = [[usize; 4]; 4];

// テトリミノの形状
pub const TETROMINOES: [Shape; KIND_MAX] = [
    [
        // I
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [I, I, I, I],
        [0, 0, 0, 0],
    ],
    [
        // O
        [0, 0, 0, 0],
        [0, O, O, 0],
        [0, O, O, 0],
        [0, 0, 0, 0],
    ],
    [
        // S
        [0, 0, 0, 0],
        [0, S, S, 0],
        [S, S, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // Z
        [0, 0, 0, 0],
        [Z, Z, 0, 0],
        [0, Z, Z, 0],
        [0, 0, 0, 0],
    ],
    [
        // J
        [0, 0, 0, 0],
        [J, 0, 0, 0],
        [J, J, J, 0],
        [0, 0, 0, 0],
    ],
    [
        // L
        [0, 0, 0, 0],
        [0, 0, L, 0],
        [L, L, L, 0],
        [0, 0, 0, 0],
    ],
    [
        // T
        [0, 0, 0, 0],
        [0, T, 0, 0],
        [T, T, T, 0],
        [0, 0, 0, 0],
    ],
];

pub fn gen_tetromino_7() -> [Shape; KIND_MAX] {
    let mut rng = thread_rng();
    let mut tetrominoes = [
        Kind::I,
        Kind::O,
        Kind::S,
        Kind::Z,
        Kind::J,
        Kind::L,
        Kind::T,
    ];
    tetrominoes.shuffle(&mut rng);
    tetrominoes.map(|t| TETROMINOES[t as usize])
}
