use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};

use crate::game::cell::{I, J, L, O, S, T, Z};

// テトリミノの種類
const KIND_MAX: usize = 7;

#[derive(Clone, Copy, Default)]
pub enum Kind {
    #[default]
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

#[derive(Clone, Copy, Default)]
pub enum RotateState {
    #[default]
    _0, // spawn state
    _R, // state resulting from a clockwise rotation ("right") from spawn
    _2, // state resulting from 2 successive rotations in either direction from spawn
    _L, // state resulting from a counter-clockwise ("left") rotation from spawn
}

#[derive(Clone, Copy, Default)]
pub struct Tetromino {
    kind: Kind,
    rotate_state: RotateState,
}

pub type Shape = [[usize; 4]; 4];

impl Tetromino {
    pub fn get_shape(&self) -> Shape {
        let mut shape = SHAPES[self.kind as usize];
        let (rotate_count, rotate_size) = match self.kind {
            Kind::I => (self.rotate_state as usize, 4),
            Kind::O => (0, 0),
            _ => (self.rotate_state as usize, 3),
        };

        for _ in 0..rotate_count {
            let mut new_shape: Shape = Default::default();
            for y in 0..rotate_size {
                for x in 0..rotate_size {
                    new_shape[y][x] = shape[rotate_size - 1 - x][y];
                }
            }
            shape = new_shape;
        }

        shape
    }

    pub fn rotate_right(&self) -> Tetromino {
        let rotate_state = match self.rotate_state {
            RotateState::_0 => RotateState::_R,
            RotateState::_R => RotateState::_2,
            RotateState::_2 => RotateState::_L,
            RotateState::_L => RotateState::_0,
        };
        Tetromino {
            kind: self.kind,
            rotate_state,
        }
    }

    pub fn rotate_left(&self) -> Tetromino {
        let rotate_state = match self.rotate_state {
            RotateState::_0 => RotateState::_L,
            RotateState::_L => RotateState::_2,
            RotateState::_2 => RotateState::_R,
            RotateState::_R => RotateState::_0,
        };
        Tetromino {
            kind: self.kind,
            rotate_state,
        }
    }
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

// テトリミノの形状
const SHAPES: [Shape; KIND_MAX] = [
    [
        // I
        [0, 0, 0, 0],
        [I, I, I, I],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // O
        [0, O, O, 0],
        [0, O, O, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // S
        [0, S, S, 0],
        [S, S, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // Z
        [Z, Z, 0, 0],
        [0, Z, Z, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // J
        [J, 0, 0, 0],
        [J, J, J, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // L
        [0, 0, L, 0],
        [L, L, L, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // T
        [0, T, 0, 0],
        [T, T, T, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
];

pub fn gen_tetromino_7() -> [Tetromino; KIND_MAX] {
    let mut rng = thread_rng();
    let mut kinds = [
        Kind::I,
        Kind::O,
        Kind::S,
        Kind::Z,
        Kind::J,
        Kind::L,
        Kind::T,
    ];
    kinds.shuffle(&mut rng);
    kinds.map(|kind| Tetromino {
        kind,
        rotate_state: RotateState::_0,
    })
}
