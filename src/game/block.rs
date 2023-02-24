use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};

use crate::game::cell::{I, J, L, O, S, T, Z};

// テトリミノの種類
const BLOCK_KIND_MAX: usize = 7;
#[derive(Clone, Copy)]
pub enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=6) {
            0 => BlockKind::I,
            1 => BlockKind::O,
            2 => BlockKind::S,
            3 => BlockKind::Z,
            4 => BlockKind::J,
            5 => BlockKind::L,
            _ => BlockKind::T,
        }
    }
}

pub type BlockShape = [[usize; 4]; 4];

// テトリミノの形状
pub const BLOCKS: [BlockShape; BLOCK_KIND_MAX] = [
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

pub fn gen_block_7() -> [BlockShape; BLOCK_KIND_MAX] {
    let mut rng = thread_rng();
    let mut blocks = [
        BlockKind::I,
        BlockKind::O,
        BlockKind::S,
        BlockKind::Z,
        BlockKind::J,
        BlockKind::L,
        BlockKind::T,
    ];
    blocks.shuffle(&mut rng);
    blocks.map(|block| BLOCKS[block as usize])
}
