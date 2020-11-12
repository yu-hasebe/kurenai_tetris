pub mod i;
pub mod j;
pub mod l;
pub mod o;
pub mod s;
pub mod t;
pub mod z;

use crate::models::{
    shared::{Block, Direction},
    tetromino::{i::I, j::J, l::L, o::O, s::S, t::T, z::Z},
};

pub trait Tetromino {
    fn new(dir: TetrominoDirection, axis: Block) -> Self
    where
        Self: Sized;
    fn move_(&mut self, move_dir: MoveDirection);
    fn rotate(&mut self, rotate_dir: RotateDirection);
    fn dry_move(&self, move_dir: MoveDirection) -> Vec<Block>;
    fn dry_rotate(&self, rotate_dir: RotateDirection) -> Vec<Block>;
    fn blocks(&self) -> Vec<Block>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TetrominoDirection {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub enum RotateDirection {
    Left,
    Right,
}

impl TetrominoDirection {
    fn rotate(&self, rotate_dir: RotateDirection) -> Self {
        match rotate_dir {
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

#[derive(Clone, Copy, Debug)]
pub enum MoveDirection {
    Left,
    Right,
    Down,
}

impl From<MoveDirection> for Direction {
    fn from(move_dir: MoveDirection) -> Self {
        match move_dir {
            MoveDirection::Left => Direction::Left,
            MoveDirection::Right => Direction::Right,
            MoveDirection::Down => Direction::Down,
        }
    }
}
