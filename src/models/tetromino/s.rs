use crate::models::{
    block::{Block, Direction},
    tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct S {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for S {
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
        match self.dir() {
            TetrominoDirection::Left => vec![
                self.axis().move_(Direction::Left),
                *self.axis(),
                self.axis().move_(Direction::Up),
                self.axis().move_(Direction::Right).move_(Direction::Up),
            ],
            TetrominoDirection::Up => vec![
                *self.axis(),
                self.axis().move_(Direction::Up),
                self.axis().move_(Direction::Right).move_(Direction::Down),
                self.axis().move_(Direction::Right),
            ],
            TetrominoDirection::Right => vec![
                self.axis().move_(Direction::Left).move_(Direction::Down),
                self.axis().move_(Direction::Down),
                *self.axis(),
                self.axis().move_(Direction::Right),
            ],
            TetrominoDirection::Down => vec![
                self.axis().move_(Direction::Left),
                self.axis().move_(Direction::Left).move_(Direction::Up),
                self.axis().move_(Direction::Down),
                *self.axis(),
            ],
        }
    }
}

impl S {
    pub fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
}

impl S {
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
        let mut s = build_s_tetromino();
        s.move_(MoveDirection::Left);
        assert_eq!(
            S::new(TetrominoDirection::Right, Block::new(Color::Green, -1, 0)),
            s
        );
        s.move_(MoveDirection::Right);
        assert_eq!(
            S::new(TetrominoDirection::Right, Block::new(Color::Green, 0, 0)),
            s
        );
        s.move_(MoveDirection::Down);
        assert_eq!(
            S::new(TetrominoDirection::Right, Block::new(Color::Green, 0, -1)),
            s
        );
    }

    #[test]
    fn test_rotate() {
        let mut s = build_s_tetromino();
        s.rotate(RotateDirection::Left);
        assert_eq!(
            S::new(TetrominoDirection::Up, Block::new(Color::Green, 0, 0)),
            s
        );
        s.rotate(RotateDirection::Left);
        assert_eq!(
            S::new(TetrominoDirection::Left, Block::new(Color::Green, 0, 0)),
            s
        );
        s.rotate(RotateDirection::Left);
        assert_eq!(
            S::new(TetrominoDirection::Down, Block::new(Color::Green, 0, 0)),
            s
        );
        s.rotate(RotateDirection::Left);
        assert_eq!(
            S::new(TetrominoDirection::Right, Block::new(Color::Green, 0, 0)),
            s
        );
        s.rotate(RotateDirection::Right);
        assert_eq!(
            S::new(TetrominoDirection::Down, Block::new(Color::Green, 0, 0)),
            s
        );
        s.rotate(RotateDirection::Right);
        assert_eq!(
            S::new(TetrominoDirection::Left, Block::new(Color::Green, 0, 0)),
            s
        );
        s.rotate(RotateDirection::Right);
        assert_eq!(
            S::new(TetrominoDirection::Up, Block::new(Color::Green, 0, 0)),
            s
        );
        s.rotate(RotateDirection::Right);
        assert_eq!(
            S::new(TetrominoDirection::Right, Block::new(Color::Green, 0, 0)),
            s
        );
    }

    #[test]
    fn test_dry_move() {
        let mut s = build_s_tetromino();
        let got = s.dry_move(MoveDirection::Left);
        s.move_(MoveDirection::Left);
        assert_eq!(s.blocks(), got);
        let got = s.dry_move(MoveDirection::Right);
        s.move_(MoveDirection::Right);
        assert_eq!(s.blocks(), got);
        let got = s.dry_move(MoveDirection::Down);
        s.move_(MoveDirection::Down);
        assert_eq!(s.blocks(), got);
    }

    #[test]
    fn test_dry_rotate() {
        let mut s = build_s_tetromino();
        for _ in 0..4 {
            let got = s.dry_rotate(RotateDirection::Left);
            s.rotate(RotateDirection::Left);
            assert_eq!(s.blocks(), got);
        }
        for _ in 0..4 {
            let got = s.dry_rotate(RotateDirection::Right);
            s.rotate(RotateDirection::Right);
            assert_eq!(s.blocks(), got);
        }
    }

    #[test]
    fn test_blocks() {
        let mut s = build_s_tetromino();
        assert_eq!(build_s_blocks(TetrominoDirection::Right), s.blocks());
        s.rotate(RotateDirection::Right);
        assert_eq!(build_s_blocks(TetrominoDirection::Down), s.blocks());
        s.rotate(RotateDirection::Right);
        assert_eq!(build_s_blocks(TetrominoDirection::Left), s.blocks());
        s.rotate(RotateDirection::Right);
        assert_eq!(build_s_blocks(TetrominoDirection::Up), s.blocks());
    }

    fn build_s_tetromino() -> S {
        S::new(TetrominoDirection::Right, Block::new(Color::Green, 0, 0))
    }

    fn build_s_blocks(dir: TetrominoDirection) -> Vec<Block> {
        match dir {
            TetrominoDirection::Left => vec![
                Block::new(Color::Green, -1, 0),
                Block::new(Color::Green, 0, 0),
                Block::new(Color::Green, 0, 1),
                Block::new(Color::Green, 1, 1),
            ],
            TetrominoDirection::Up => vec![
                Block::new(Color::Green, 0, 0),
                Block::new(Color::Green, 0, 1),
                Block::new(Color::Green, 1, -1),
                Block::new(Color::Green, 1, 0),
            ],
            TetrominoDirection::Right => vec![
                Block::new(Color::Green, -1, -1),
                Block::new(Color::Green, 0, -1),
                Block::new(Color::Green, 0, 0),
                Block::new(Color::Green, 1, 0),
            ],
            TetrominoDirection::Down => vec![
                Block::new(Color::Green, -1, 0),
                Block::new(Color::Green, -1, 1),
                Block::new(Color::Green, 0, -1),
                Block::new(Color::Green, 0, 0),
            ],
        }
    }
}
