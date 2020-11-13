use crate::models::{
    shared::{Block, Direction},
    tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Z {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for Z {
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
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right).move_(Direction::Down));
            }
            TetrominoDirection::Up => {
                blocks.push(self.axis().move_(Direction::Left).move_(Direction::Down));
                blocks.push(self.axis().move_(Direction::Left));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
            }
            TetrominoDirection::Right => {
                blocks.push(self.axis().move_(Direction::Left).move_(Direction::Up));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Up));
                blocks.push(self.axis().move_(Direction::Right));
            }
            TetrominoDirection::Down => {
                blocks.push(self.axis().move_(Direction::Down));
                blocks.push(*self.axis());
                blocks.push(self.axis().move_(Direction::Right));
                blocks.push(self.axis().move_(Direction::Right).move_(Direction::Up));
            }
        }
        blocks
    }
}

impl Z {
    pub fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
}

impl Z {
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
        let mut z = build_z_tetromino();
        z.move_(MoveDirection::Left);
        assert_eq!(
            Z::new(TetrominoDirection::Right, Block::new(Color::Red, -1, 0)),
            z
        );
        z.move_(MoveDirection::Right);
        assert_eq!(
            Z::new(TetrominoDirection::Right, Block::new(Color::Red, 0, 0)),
            z
        );
        z.move_(MoveDirection::Down);
        assert_eq!(
            Z::new(TetrominoDirection::Right, Block::new(Color::Red, 0, -1)),
            z
        );
    }

    #[test]
    fn test_rotate() {
        let mut z = build_z_tetromino();
        z.rotate(RotateDirection::Left);
        assert_eq!(
            Z::new(TetrominoDirection::Up, Block::new(Color::Red, 0, 0)),
            z
        );
        z.rotate(RotateDirection::Left);
        assert_eq!(
            Z::new(TetrominoDirection::Left, Block::new(Color::Red, 0, 0)),
            z
        );
        z.rotate(RotateDirection::Left);
        assert_eq!(
            Z::new(TetrominoDirection::Down, Block::new(Color::Red, 0, 0)),
            z
        );
        z.rotate(RotateDirection::Left);
        assert_eq!(
            Z::new(TetrominoDirection::Right, Block::new(Color::Red, 0, 0)),
            z
        );
        z.rotate(RotateDirection::Right);
        assert_eq!(
            Z::new(TetrominoDirection::Down, Block::new(Color::Red, 0, 0)),
            z
        );
        z.rotate(RotateDirection::Right);
        assert_eq!(
            Z::new(TetrominoDirection::Left, Block::new(Color::Red, 0, 0)),
            z
        );
        z.rotate(RotateDirection::Right);
        assert_eq!(
            Z::new(TetrominoDirection::Up, Block::new(Color::Red, 0, 0)),
            z
        );
        z.rotate(RotateDirection::Right);
        assert_eq!(
            Z::new(TetrominoDirection::Right, Block::new(Color::Red, 0, 0)),
            z
        );
    }

    #[test]
    fn test_dry_move() {
        let mut z = build_z_tetromino();
        let got = z.dry_move(MoveDirection::Left);
        z.move_(MoveDirection::Left);
        assert_eq!(z.blocks(), got);
        let got = z.dry_move(MoveDirection::Right);
        z.move_(MoveDirection::Right);
        assert_eq!(z.blocks(), got);
        let got = z.dry_move(MoveDirection::Down);
        z.move_(MoveDirection::Down);
        assert_eq!(z.blocks(), got);
    }

    #[test]
    fn test_dry_rotate() {
        let mut z = build_z_tetromino();
        for _ in 0..4 {
            let got = z.dry_rotate(RotateDirection::Left);
            z.rotate(RotateDirection::Left);
            assert_eq!(z.blocks(), got);
        }
        for _ in 0..4 {
            let got = z.dry_rotate(RotateDirection::Right);
            z.rotate(RotateDirection::Right);
            assert_eq!(z.blocks(), got);
        }
    }

    #[test]
    fn test_blocks() {
        let mut z = build_z_tetromino();
        assert_eq!(build_z_blocks(TetrominoDirection::Right), z.blocks());
        z.rotate(RotateDirection::Right);
        assert_eq!(build_z_blocks(TetrominoDirection::Down), z.blocks());
        z.rotate(RotateDirection::Right);
        assert_eq!(build_z_blocks(TetrominoDirection::Left), z.blocks());
        z.rotate(RotateDirection::Right);
        assert_eq!(build_z_blocks(TetrominoDirection::Up), z.blocks());
    }

    fn build_z_tetromino() -> Z {
        Z::new(TetrominoDirection::Right, Block::new(Color::Red, 0, 0))
    }

    fn build_z_blocks(dir: TetrominoDirection) -> Vec<Block> {
        match dir {
            TetrominoDirection::Left => vec![
                Block::new(Color::Red, -1, 0),
                Block::new(Color::Red, 0, -1),
                Block::new(Color::Red, 0, 0),
                Block::new(Color::Red, 1, -1),
            ],
            TetrominoDirection::Up => vec![
                Block::new(Color::Red, -1, -1),
                Block::new(Color::Red, -1, 0),
                Block::new(Color::Red, 0, 0),
                Block::new(Color::Red, 0, 1),
            ],
            TetrominoDirection::Right => vec![
                Block::new(Color::Red, -1, 1),
                Block::new(Color::Red, 0, 0),
                Block::new(Color::Red, 0, 1),
                Block::new(Color::Red, 1, 0),
            ],
            TetrominoDirection::Down => vec![
                Block::new(Color::Red, 0, -1),
                Block::new(Color::Red, 0, 0),
                Block::new(Color::Red, 1, 0),
                Block::new(Color::Red, 1, 1),
            ],
        }
    }
}
