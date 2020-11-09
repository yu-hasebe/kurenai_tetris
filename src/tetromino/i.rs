use crate::shared::Block;
use crate::tetromino::{MoveDirection, RotateDirection, Tetromino};

pub struct I;

impl Tetromino for I {
    fn new() -> Self {
        Self
    }
    fn move_(&mut self, dir: MoveDirection) {}
    fn rotate(&mut self, dir: RotateDirection) {}
    fn dry_move(&self, dir: MoveDirection) -> Vec<Block> {
        Vec::new()
    }
    fn dry_rotate(&self, dir: RotateDirection) -> Vec<Block> {
        Vec::new()
    }
    fn blocks(&self) -> Vec<Block> {
        Vec::new()
    }
}
