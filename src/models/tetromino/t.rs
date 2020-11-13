use crate::models::{
    block::{Block, Direction},
    tetromino::{MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct T {
    dir: TetrominoDirection,
    axis: Block,
}

impl Tetromino for T {
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
                self.axis().move_(Direction::Down),
                *self.axis(),
                self.axis().move_(Direction::Right),
            ],
            TetrominoDirection::Up => vec![
                self.axis().move_(Direction::Left),
                self.axis().move_(Direction::Down),
                *self.axis(),
                self.axis().move_(Direction::Up),
            ],
            TetrominoDirection::Right => vec![
                self.axis().move_(Direction::Left),
                *self.axis(),
                self.axis().move_(Direction::Up),
                self.axis().move_(Direction::Right),
            ],
            TetrominoDirection::Down => vec![
                self.axis().move_(Direction::Down),
                *self.axis(),
                self.axis().move_(Direction::Up),
                self.axis().move_(Direction::Right),
            ],
        }
    }
}

impl T {
    pub fn new(dir: TetrominoDirection, axis: Block) -> Self {
        Self { dir, axis }
    }
}

impl T {
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
        let mut t = build_t_tetromino();
        t.move_(MoveDirection::Left);
        assert_eq!(
            T::new(TetrominoDirection::Right, Block::new(Color::Purple, -1, 0)),
            t
        );
        t.move_(MoveDirection::Right);
        assert_eq!(
            T::new(TetrominoDirection::Right, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.move_(MoveDirection::Down);
        assert_eq!(
            T::new(TetrominoDirection::Right, Block::new(Color::Purple, 0, -1)),
            t
        );
    }

    #[test]
    fn test_rotate() {
        let mut t = build_t_tetromino();
        t.rotate(RotateDirection::Left);
        assert_eq!(
            T::new(TetrominoDirection::Up, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.rotate(RotateDirection::Left);
        assert_eq!(
            T::new(TetrominoDirection::Left, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.rotate(RotateDirection::Left);
        assert_eq!(
            T::new(TetrominoDirection::Down, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.rotate(RotateDirection::Left);
        assert_eq!(
            T::new(TetrominoDirection::Right, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.rotate(RotateDirection::Right);
        assert_eq!(
            T::new(TetrominoDirection::Down, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.rotate(RotateDirection::Right);
        assert_eq!(
            T::new(TetrominoDirection::Left, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.rotate(RotateDirection::Right);
        assert_eq!(
            T::new(TetrominoDirection::Up, Block::new(Color::Purple, 0, 0)),
            t
        );
        t.rotate(RotateDirection::Right);
        assert_eq!(
            T::new(TetrominoDirection::Right, Block::new(Color::Purple, 0, 0)),
            t
        );
    }

    #[test]
    fn test_dry_move() {
        let mut t = build_t_tetromino();
        let got = t.dry_move(MoveDirection::Left);
        t.move_(MoveDirection::Left);
        assert_eq!(t.blocks(), got);
        let got = t.dry_move(MoveDirection::Right);
        t.move_(MoveDirection::Right);
        assert_eq!(t.blocks(), got);
        let got = t.dry_move(MoveDirection::Down);
        t.move_(MoveDirection::Down);
        assert_eq!(t.blocks(), got);
    }

    #[test]
    fn test_dry_rotate() {
        let mut t = build_t_tetromino();
        for _ in 0..4 {
            let got = t.dry_rotate(RotateDirection::Left);
            t.rotate(RotateDirection::Left);
            assert_eq!(t.blocks(), got);
        }
        for _ in 0..4 {
            let got = t.dry_rotate(RotateDirection::Right);
            t.rotate(RotateDirection::Right);
            assert_eq!(t.blocks(), got);
        }
    }

    #[test]
    fn test_blocks() {
        let mut t = build_t_tetromino();
        assert_eq!(build_t_blocks(TetrominoDirection::Right), t.blocks());
        t.rotate(RotateDirection::Right);
        assert_eq!(build_t_blocks(TetrominoDirection::Down), t.blocks());
        t.rotate(RotateDirection::Right);
        assert_eq!(build_t_blocks(TetrominoDirection::Left), t.blocks());
        t.rotate(RotateDirection::Right);
        assert_eq!(build_t_blocks(TetrominoDirection::Up), t.blocks());
    }

    fn build_t_tetromino() -> T {
        T::new(TetrominoDirection::Right, Block::new(Color::Purple, 0, 0))
    }

    fn build_t_blocks(dir: TetrominoDirection) -> Vec<Block> {
        match dir {
            TetrominoDirection::Left => vec![
                Block::new(Color::Purple, -1, 0),
                Block::new(Color::Purple, 0, -1),
                Block::new(Color::Purple, 0, 0),
                Block::new(Color::Purple, 1, 0),
            ],
            TetrominoDirection::Up => vec![
                Block::new(Color::Purple, -1, 0),
                Block::new(Color::Purple, 0, -1),
                Block::new(Color::Purple, 0, 0),
                Block::new(Color::Purple, 0, 1),
            ],
            TetrominoDirection::Right => vec![
                Block::new(Color::Purple, -1, 0),
                Block::new(Color::Purple, 0, 0),
                Block::new(Color::Purple, 0, 1),
                Block::new(Color::Purple, 1, 0),
            ],
            TetrominoDirection::Down => vec![
                Block::new(Color::Purple, 0, -1),
                Block::new(Color::Purple, 0, 0),
                Block::new(Color::Purple, 0, 1),
                Block::new(Color::Purple, 1, 0),
            ],
        }
    }
}
