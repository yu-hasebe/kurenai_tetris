use crate::shared::{Block, MoveDirection, RotateDirection};
use crate::tetromino::{Tetromino, TetrominoDirection};

pub struct I {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for I {
    fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
    fn move_(&mut self, dir: MoveDirection) {
        self.set_axis(self.axis().move_(dir));
    }
    fn rotate(&mut self, dir: RotateDirection) {
        self.set_dir(self.dir().rotate(dir));
    }
    fn dry_move(&self, dir: MoveDirection) -> Vec<Block> {
        Self::new(*self.dir(), self.axis().move_(dir)).blocks()
    }
    fn dry_rotate(&self, dir: RotateDirection) -> Vec<Block> {
        Self::new(self.dir().rotate(dir), *self.axis()).blocks()
    }
    fn blocks(&self) -> Vec<Block> {
        Vec::new()
    }
}

impl I {
    fn dir(&self) -> &TetrominoDirection {
        &self.dir
    }
    fn axis(&self) -> &Block {
        &self.axis
    }
    fn set_dir(&mut self, dir: TetrominoDirection) {
        self.dir = dir;
    }
    fn set_axis(&mut self, axis: Block) {
        self.axis = axis;
    }
}
