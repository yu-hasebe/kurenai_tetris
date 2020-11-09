pub mod i;
pub mod j;
pub mod l;
pub mod o;
pub mod s;
pub mod t;
pub mod z;

use crate::shared::Block;
use crate::tetromino::{i::I, j::J, l::L, o::O, s::S, t::T, z::Z};

pub enum EnumTetromino {
    I(I),
    J(J),
    L(L),
    S(S),
    Z(Z),
    T(T),
    O(O),
}

pub trait Tetromino {
    fn new() -> Self;
    fn move_(&mut self, dir: MoveDirection);
    fn rotate(&mut self, dir: RotateDirection);
    fn dry_move(&self, dir: MoveDirection) -> Vec<Block>;
    fn dry_rotate(&self, dir: RotateDirection) -> Vec<Block>;
    fn blocks(&self) -> Vec<Block>;
}

#[derive(Clone, Copy, Debug)]
pub enum MoveDirection {
    Left,
    Right,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub enum RotateDirection {
    Left,
    Right,
}
