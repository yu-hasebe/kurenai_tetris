use crate::models::{
    shared::{Block, Direction},
    tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct J {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for J {
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
                blocks.push(self.axis().move_(Direction::Left));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right).move_(Direction::Down));
                blocks.push(self.axis().move_(Direction::Right));
            }
            TetrominoDirection::Up => {
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
                blocks.push(self.axis().move_(Direction::Up).move_(Direction::Right));
            }
            TetrominoDirection::Right => {
                blocks.push(self.axis().move_(Direction::Left));
                blocks.push(self.axis().move_(Direction::Left).move_(Direction::Up));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right));
            }
            TetrominoDirection::Down => {
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
                blocks.push(self.axis().move_(Direction::Up).move_(Direction::Right));
            }
        }
        blocks
    }
}

impl J {
    pub fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
}

impl J {
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
        let mut j = build_j_tetromino();
        j.move_(MoveDirection::Left);
        assert_eq!(
            J::new(TetrominoDirection::Right, Block::new(Color::Blue, -1, 0)),
            j
        );
        j.move_(MoveDirection::Right);
        assert_eq!(
            J::new(TetrominoDirection::Right, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.move_(MoveDirection::Down);
        assert_eq!(
            J::new(TetrominoDirection::Right, Block::new(Color::Blue, 0, -1)),
            j
        );
    }

    #[test]
    fn test_rotate() {
        let mut j = build_j_tetromino();
        j.rotate(RotateDirection::Left);
        assert_eq!(
            J::new(TetrominoDirection::Up, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.rotate(RotateDirection::Left);
        assert_eq!(
            J::new(TetrominoDirection::Left, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.rotate(RotateDirection::Left);
        assert_eq!(
            J::new(TetrominoDirection::Down, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.rotate(RotateDirection::Left);
        assert_eq!(
            J::new(TetrominoDirection::Right, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.rotate(RotateDirection::Right);
        assert_eq!(
            J::new(TetrominoDirection::Down, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.rotate(RotateDirection::Right);
        assert_eq!(
            J::new(TetrominoDirection::Left, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.rotate(RotateDirection::Right);
        assert_eq!(
            J::new(TetrominoDirection::Up, Block::new(Color::Blue, 0, 0)),
            j
        );
        j.rotate(RotateDirection::Right);
        assert_eq!(
            J::new(TetrominoDirection::Right, Block::new(Color::Blue, 0, 0)),
            j
        );
    }

    #[test]
    fn test_dry_move() {
        let mut j = build_j_tetromino();
        let got = j.dry_move(MoveDirection::Left);
        j.move_(MoveDirection::Left);
        assert_eq!(j.blocks(), got);
        let got = j.dry_move(MoveDirection::Right);
        j.move_(MoveDirection::Right);
        assert_eq!(j.blocks(), got);
        let got = j.dry_move(MoveDirection::Down);
        j.move_(MoveDirection::Down);
        assert_eq!(j.blocks(), got);
    }

    #[test]
    fn test_dry_rotate() {
        let mut j = build_j_tetromino();
        for _ in 0..4 {
            let got = j.dry_rotate(RotateDirection::Left);
            j.rotate(RotateDirection::Left);
            assert_eq!(j.blocks(), got);
        }
        for _ in 0..4 {
            let got = j.dry_rotate(RotateDirection::Right);
            j.rotate(RotateDirection::Right);
            assert_eq!(j.blocks(), got);
        }
    }

    #[test]
    fn test_blocks() {
        let mut j = build_j_tetromino();
        assert_eq!(build_j_blocks(TetrominoDirection::Right), j.blocks());
        j.rotate(RotateDirection::Right);
        assert_eq!(build_j_blocks(TetrominoDirection::Down), j.blocks());
        j.rotate(RotateDirection::Right);
        assert_eq!(build_j_blocks(TetrominoDirection::Left), j.blocks());
        j.rotate(RotateDirection::Right);
        assert_eq!(build_j_blocks(TetrominoDirection::Up), j.blocks());
    }

    fn build_j_tetromino() -> J {
        J::new(TetrominoDirection::Right, Block::new(Color::Blue, 0, 0))
    }

    fn build_j_blocks(dir: TetrominoDirection) -> Vec<Block> {
        match dir {
            TetrominoDirection::Left => vec![
                Block::new(Color::Blue, -1, 0),
                Block::new(Color::Blue, 0, 0),
                Block::new(Color::Blue, 1, -1),
                Block::new(Color::Blue, 1, 0),
            ],
            TetrominoDirection::Up => vec![
                Block::new(Color::Blue, 0, -1),
                Block::new(Color::Blue, 0, 0),
                Block::new(Color::Blue, 0, 1),
                Block::new(Color::Blue, 1, 1),
            ],
            TetrominoDirection::Right => vec![
                Block::new(Color::Blue, -1, 0),
                Block::new(Color::Blue, -1, 1),
                Block::new(Color::Blue, 0, 0),
                Block::new(Color::Blue, 1, 0),
            ],
            TetrominoDirection::Down => vec![
                Block::new(Color::Blue, 0, -1),
                Block::new(Color::Blue, 0, 0),
                Block::new(Color::Blue, 0, 1),
                Block::new(Color::Blue, 1, 1),
            ],
        }
    }
}
