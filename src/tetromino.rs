pub mod i;
pub mod j;
pub mod l;
pub mod o;
pub mod s;
pub mod t;
pub mod z;

use crate::shared::{Block, MoveDirection, RotateDirection};
use crate::tetromino::{i::I, j::J, l::L, o::O, s::S, t::T, z::Z};

pub trait Tetromino {
    fn new(dir: TetrominoDirection, axis: Block) -> Self
    where
        Self: Sized;
    fn move_(&mut self, dir: MoveDirection);
    fn rotate(&mut self, dir: RotateDirection);
    fn dry_move(&self, dir: MoveDirection) -> Vec<Block>;
    fn dry_rotate(&self, dir: RotateDirection) -> Vec<Block>;
    fn blocks(&self) -> Vec<Block>;
}

#[derive(Clone, Copy, Debug)]
pub enum TetrominoDirection {
    Left,
    Up,
    Right,
    Down,
}

impl TetrominoDirection {
    fn rotate(&self, dir: RotateDirection) -> Self {
        match dir {
            RotateDirection::Left => match self {
                TetrominoDirection::Left => TetrominoDirection::Down,
                TetrominoDirection::Down => TetrominoDirection::Right,
                TetrominoDirection::Right => TetrominoDirection::Up,
                TetrominoDirection::Up => TetrominoDirection::Left,
            },
            RotateDirection::Right => match self {
                TetrominoDirection::Left => TetrominoDirection::Up,
                TetrominoDirection::Down => TetrominoDirection::Left,
                TetrominoDirection::Right => TetrominoDirection::Down,
                TetrominoDirection::Up => TetrominoDirection::Right,
            },
        }
    }
}
