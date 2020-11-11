use crate::shared::{Block, Direction};
use crate::tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection};

pub struct I {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for I {
    fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
    fn move_(&mut self, move_dir: MoveDirection) {
        let dir = Direction::from(move_dir);
        self.set_axis(self.axis().move_(dir));
    }
    fn rotate(&mut self, rotate_dir: RotateDirection) {
        self.set_dir(self.dir().rotate(rotate_dir));
        self.set_axis(self.rotate_axis(rotate_dir));
    }
    fn dry_move(&self, move_dir: MoveDirection) -> Vec<Block> {
        let dir = Direction::from(move_dir);
        Self::new(*self.dir(), self.axis().move_(dir)).blocks()
    }
    fn dry_rotate(&self, rotate_dir: RotateDirection) -> Vec<Block> {
        Self::new(self.dir().rotate(rotate_dir), self.rotate_axis(rotate_dir)).blocks()
    }
    fn blocks(&self) -> Vec<Block> {
        let mut blocks = Vec::new();
        match self.dir() {
            TetrominoDirection::Left => {
                blocks.push(self.axis().move_(Direction::Left));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right));
                blocks.push(self.axis().move_(Direction::Right).move_(Direction::Right));
            }
            TetrominoDirection::Up => {
                blocks.push(self.axis().move_(Direction::Down).move_(Direction::Down));
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
            }
            TetrominoDirection::Right => {
                blocks.push(self.axis().move_(Direction::Left).move_(Direction::Left));
                blocks.push(self.axis().move_(Direction::Left));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right));
            }
            TetrominoDirection::Down => {
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
                blocks.push(self.axis().move_(Direction::Up).move_(Direction::Up));
            }
        }
        blocks
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
    fn rotate_axis(&self, rotate_dir: RotateDirection) -> Block {
        match rotate_dir {
            RotateDirection::Left => match self.dir() {
                TetrominoDirection::Left => self.axis().move_(Direction::Right),
                TetrominoDirection::Up => self.axis().move_(Direction::Down),
                TetrominoDirection::Right => self.axis().move_(Direction::Left),
                TetrominoDirection::Down => self.axis().move_(Direction::Up),
            },
            RotateDirection::Right => match self.dir() {
                TetrominoDirection::Left => self.axis().move_(Direction::Up),
                TetrominoDirection::Up => self.axis().move_(Direction::Right),
                TetrominoDirection::Right => self.axis().move_(Direction::Down),
                TetrominoDirection::Down => self.axis().move_(Direction::Left),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::Color;

    #[test]
    fn test_move() {
        let mut i = build_i_tetromino();
        i.move_(MoveDirection::Left);
        assert_eq!(Block::new(Color::Cyan, -1, 0), i.axis);
        i.move_(MoveDirection::Right);
        assert_eq!(Block::new(Color::Cyan, 0, 0), i.axis);
        i.move_(MoveDirection::Down);
        assert_eq!(Block::new(Color::Cyan, 0, -1), i.axis);
    }

    fn build_i_tetromino() -> I {
        I::new(TetrominoDirection::Right, Block::new(Color::Cyan, 0, 0))
    }

    fn build_i_blocks(dir: TetrominoDirection) -> Vec<Block> {
        match dir {
            TetrominoDirection::Left => vec![
                Block::new(Color::Cyan, -2, -1),
                Block::new(Color::Cyan, -1, -1),
                Block::new(Color::Cyan, 0, -1),
                Block::new(Color::Cyan, 1, -1),
            ],
            TetrominoDirection::Up => vec![
                Block::new(Color::Cyan, -1, -2),
                Block::new(Color::Cyan, -1, -1),
                Block::new(Color::Cyan, -1, 0),
                Block::new(Color::Cyan, -1, 1),
            ],
            TetrominoDirection::Right => vec![
                Block::new(Color::Cyan, -2, 0),
                Block::new(Color::Cyan, -1, 0),
                Block::new(Color::Cyan, 0, 0),
                Block::new(Color::Cyan, 1, 0),
            ],
            TetrominoDirection::Down => vec![
                Block::new(Color::Cyan, 0, -2),
                Block::new(Color::Cyan, 0, -1),
                Block::new(Color::Cyan, 0, 0),
                Block::new(Color::Cyan, 0, 1),
            ],
        }
    }
}
