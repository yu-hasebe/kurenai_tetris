use crate::models::{
    block::{Block, Direction},
    tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct O {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for O {
    fn move_(&mut self, move_dir: MoveDirection) {
        let dir = Direction::from(move_dir);
        self.set_axis(self.axis().move_(dir));
    }
    fn rotate(&mut self, rotate_dir: RotateDirection) {
        let dir = self.dir().rotate(rotate_dir);
        self.set_dir(dir);
    }
    fn dry_move(&self, move_dir: MoveDirection) -> Vec<Block> {
        let dir = Direction::from(move_dir);
        Self::new(*self.dir(), self.axis().move_(dir)).blocks()
    }
    fn dry_rotate(&self, rotate_dir: RotateDirection) -> Vec<Block> {
        let dir = self.dir().rotate(rotate_dir);
        Self::new(dir, *self.axis()).blocks()
    }
    fn blocks(&self) -> Vec<Block> {
        vec![
            *self.axis(),
            self.axis().move_(Direction::Up),
            self.axis().move_(Direction::Right),
            self.axis().move_(Direction::Right).move_(Direction::Up),
        ]
    }
}

impl O {
    pub fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
}

impl O {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::block::Color;

    #[test]
    fn test_move() {
        let mut o = build_o_tetromino();
        o.move_(MoveDirection::Left);
        assert_eq!(
            O::new(TetrominoDirection::Right, Block::new(Color::Yellow, -1, 0)),
            o
        );
        o.move_(MoveDirection::Right);
        assert_eq!(
            O::new(TetrominoDirection::Right, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.move_(MoveDirection::Down);
        assert_eq!(
            O::new(TetrominoDirection::Right, Block::new(Color::Yellow, 0, -1)),
            o
        );
    }

    #[test]
    fn test_rotate() {
        let mut o = build_o_tetromino();
        o.rotate(RotateDirection::Left);
        assert_eq!(
            O::new(TetrominoDirection::Up, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.rotate(RotateDirection::Left);
        assert_eq!(
            O::new(TetrominoDirection::Left, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.rotate(RotateDirection::Left);
        assert_eq!(
            O::new(TetrominoDirection::Down, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.rotate(RotateDirection::Left);
        assert_eq!(
            O::new(TetrominoDirection::Right, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.rotate(RotateDirection::Right);
        assert_eq!(
            O::new(TetrominoDirection::Down, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.rotate(RotateDirection::Right);
        assert_eq!(
            O::new(TetrominoDirection::Left, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.rotate(RotateDirection::Right);
        assert_eq!(
            O::new(TetrominoDirection::Up, Block::new(Color::Yellow, 0, 0)),
            o
        );
        o.rotate(RotateDirection::Right);
        assert_eq!(
            O::new(TetrominoDirection::Right, Block::new(Color::Yellow, 0, 0)),
            o
        );
    }

    #[test]
    fn test_dry_move() {
        let mut o = build_o_tetromino();
        let got = o.dry_move(MoveDirection::Left);
        o.move_(MoveDirection::Left);
        assert_eq!(o.blocks(), got);
        let got = o.dry_move(MoveDirection::Right);
        o.move_(MoveDirection::Right);
        assert_eq!(o.blocks(), got);
        let got = o.dry_move(MoveDirection::Down);
        o.move_(MoveDirection::Down);
        assert_eq!(o.blocks(), got);
    }

    #[test]
    fn test_dry_rotate() {
        let mut o = build_o_tetromino();
        for _ in 0..4 {
            let got = o.dry_rotate(RotateDirection::Left);
            o.rotate(RotateDirection::Left);
            assert_eq!(o.blocks(), got);
        }
        for _ in 0..4 {
            let got = o.dry_rotate(RotateDirection::Right);
            o.rotate(RotateDirection::Right);
            assert_eq!(o.blocks(), got);
        }
    }

    #[test]
    fn test_blocks() {
        let mut o = build_o_tetromino();
        assert_eq!(build_o_blocks(), o.blocks());
        o.rotate(RotateDirection::Right);
        assert_eq!(build_o_blocks(), o.blocks());
        o.rotate(RotateDirection::Right);
        assert_eq!(build_o_blocks(), o.blocks());
        o.rotate(RotateDirection::Right);
        assert_eq!(build_o_blocks(), o.blocks());
    }

    fn build_o_tetromino() -> O {
        O::new(TetrominoDirection::Right, Block::new(Color::Yellow, 0, 0))
    }

    fn build_o_blocks() -> Vec<Block> {
        vec![
            Block::new(Color::Yellow, 0, 0),
            Block::new(Color::Yellow, 0, 1),
            Block::new(Color::Yellow, 1, 0),
            Block::new(Color::Yellow, 1, 1),
        ]
    }
}
