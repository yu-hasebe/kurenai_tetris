use crate::models::{
    shared::{Block, Color},
    tetromino::{i::I, j::J, l::L, o::O, s::S, t::T, z::Z, Tetromino, TetrominoDirection},
};

pub struct TetrominoFactory {
    seven_bag: Vec<Box<Tetromino>>,
}

impl TetrominoFactory {
    pub fn new() -> Self {
        Self {
            seven_bag: Self::new_seven_bag(),
        }
    }

    pub fn pick_tetromino(&mut self) -> Box<Tetromino> {
        if let Some(tetromino) = self.seven_bag.pop() {
            tetromino
        } else {
            self.seven_bag = Self::new_seven_bag();
            self.seven_bag.pop().unwrap()
        }
    }
}

impl TetrominoFactory {
    fn new_seven_bag() -> Vec<Box<Tetromino>> {
        vec![
            Box::new(Self::create_i()),
            Box::new(Self::create_j()),
            Box::new(Self::create_l()),
            Box::new(Self::create_s()),
            Box::new(Self::create_z()),
        ]
    }

    fn create_i() -> I {
        I::new(TetrominoDirection::Right, Block::new(Color::Cyan, 5, 20))
    }

    fn create_j() -> J {
        J::new(TetrominoDirection::Right, Block::new(Color::Blue, 4, 20))
    }

    fn create_l() -> L {
        L::new(TetrominoDirection::Right, Block::new(Color::Orange, 4, 20))
    }

    fn create_s() -> S {
        S::new(TetrominoDirection::Right, Block::new(Color::Green, 4, 20))
    }

    fn create_z() -> Z {
        Z::new(TetrominoDirection::Right, Block::new(Color::Red, 4, 20))
    }

    // fn create_t() -> T {
    //     T::new(TetrominoDirection::Right, Block::new(Color::Purple, 4, 20))
    // }
    //
    // fn create_o() -> O {
    //     O::new(TetrominoDirection::Right, Block::new(Color::Yellow, 4, 20))
    // }
}
