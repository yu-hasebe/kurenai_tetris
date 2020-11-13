use crate::models::{
    shared::{Block, Direction},
    tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct L {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for L {
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
        let mut blocks = Vec::new();
        match self.dir() {
            TetrominoDirection::Left => {
                blocks.push(self.axis().move_(Direction::Left).move_(Direction::Down));
                blocks.push(self.axis().move_(Direction::Left));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right));
            }
            TetrominoDirection::Up => {
                blocks.push(self.axis().move_(Direction::Left).move_(Direction::Up));
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
            }
            TetrominoDirection::Right => {
                blocks.push(self.axis().move_(Direction::Left));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right));
                blocks.push(self.axis().move_(Direction::Right).move_(Direction::Up));
            }
            TetrominoDirection::Down => {
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
                blocks.push(self.axis().move_(Direction::Right).move_(Direction::Down));
            }
        }
        blocks
    }
}

impl L {
    pub fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
}

impl L {
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
    use crate::models::shared::Color;

    #[test]
    fn test_move() {
        let mut l = build_l_tetromino();
        l.move_(MoveDirection::Left);
        assert_eq!(
            L::new(TetrominoDirection::Right, Block::new(Color::Orange, -1, 0)),
            l
        );
        l.move_(MoveDirection::Right);
        assert_eq!(
            L::new(TetrominoDirection::Right, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.move_(MoveDirection::Down);
        assert_eq!(
            L::new(TetrominoDirection::Right, Block::new(Color::Orange, 0, -1)),
            l
        );
    }

    #[test]
    fn test_rotate() {
        let mut l = build_l_tetromino();
        l.rotate(RotateDirection::Left);
        assert_eq!(
            L::new(TetrominoDirection::Up, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.rotate(RotateDirection::Left);
        assert_eq!(
            L::new(TetrominoDirection::Left, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.rotate(RotateDirection::Left);
        assert_eq!(
            L::new(TetrominoDirection::Down, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.rotate(RotateDirection::Left);
        assert_eq!(
            L::new(TetrominoDirection::Right, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.rotate(RotateDirection::Right);
        assert_eq!(
            L::new(TetrominoDirection::Down, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.rotate(RotateDirection::Right);
        assert_eq!(
            L::new(TetrominoDirection::Left, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.rotate(RotateDirection::Right);
        assert_eq!(
            L::new(TetrominoDirection::Up, Block::new(Color::Orange, 0, 0)),
            l
        );
        l.rotate(RotateDirection::Right);
        assert_eq!(
            L::new(TetrominoDirection::Right, Block::new(Color::Orange, 0, 0)),
            l
        );
    }

    #[test]
    fn test_dry_move() {
        let mut l = build_l_tetromino();
        let got = l.dry_move(MoveDirection::Left);
        l.move_(MoveDirection::Left);
        assert_eq!(l.blocks(), got);
        let got = l.dry_move(MoveDirection::Right);
        l.move_(MoveDirection::Right);
        assert_eq!(l.blocks(), got);
        let got = l.dry_move(MoveDirection::Down);
        l.move_(MoveDirection::Down);
        assert_eq!(l.blocks(), got);
    }

    #[test]
    fn test_dry_rotate() {
        let mut l = build_l_tetromino();
        for _ in 0..4 {
            let got = l.dry_rotate(RotateDirection::Left);
            l.rotate(RotateDirection::Left);
            assert_eq!(l.blocks(), got);
        }
        for _ in 0..4 {
            let got = l.dry_rotate(RotateDirection::Right);
            l.rotate(RotateDirection::Right);
            assert_eq!(l.blocks(), got);
        }
    }

    #[test]
    fn test_blocks() {
        let mut l = build_l_tetromino();
        assert_eq!(build_l_blocks(TetrominoDirection::Right), l.blocks());
        l.rotate(RotateDirection::Right);
        assert_eq!(build_l_blocks(TetrominoDirection::Down), l.blocks());
        l.rotate(RotateDirection::Right);
        assert_eq!(build_l_blocks(TetrominoDirection::Left), l.blocks());
        l.rotate(RotateDirection::Right);
        assert_eq!(build_l_blocks(TetrominoDirection::Up), l.blocks());
    }

    fn build_l_tetromino() -> L {
        L::new(TetrominoDirection::Right, Block::new(Color::Orange, 0, 0))
    }

    fn build_l_blocks(dir: TetrominoDirection) -> Vec<Block> {
        match dir {
            TetrominoDirection::Left => vec![
                Block::new(Color::Orange, -1, -1),
                Block::new(Color::Orange, -1, 0),
                Block::new(Color::Orange, 0, 0),
                Block::new(Color::Orange, 1, 0),
            ],
            TetrominoDirection::Up => vec![
                Block::new(Color::Orange, -1, 1),
                Block::new(Color::Orange, 0, -1),
                Block::new(Color::Orange, 0, 0),
                Block::new(Color::Orange, 0, 1),
            ],
            TetrominoDirection::Right => vec![
                Block::new(Color::Orange, -1, 0),
                Block::new(Color::Orange, 0, 0),
                Block::new(Color::Orange, 1, 0),
                Block::new(Color::Orange, 1, 1),
            ],
            TetrominoDirection::Down => vec![
                Block::new(Color::Orange, 0, -1),
                Block::new(Color::Orange, 0, 0),
                Block::new(Color::Orange, 0, 1),
                Block::new(Color::Orange, 1, -1),
            ],
        }
    }
}
