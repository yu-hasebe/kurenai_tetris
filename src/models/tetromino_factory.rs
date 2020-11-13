use crate::models::{
    block::{Block, Color},
    tetromino::{i::I, j::J, l::L, o::O, s::S, t::T, z::Z, Tetromino, TetrominoDirection},
};

pub struct TetrominoFactory {
    seven_bag: Vec<Box<Tetromino>>,
    rand: usize,
}

impl TetrominoFactory {
    pub fn new() -> Self {
        Self {
            seven_bag: Vec::new(),
            rand: 0,
        }
    }

    pub fn pick_tetromino(&mut self) -> Box<Tetromino> {
        if let Some(tetromino) = self.seven_bag.pop() {
            tetromino
        } else {
            self.seven_bag = self.new_seven_bag();
            self.seven_bag.pop().unwrap()
        }
    }

    fn new_seven_bag(&mut self) -> Vec<Box<Tetromino>> {
        self.fisher_yates_shuffle(&mut Self::build_seven_tetrominos())
    }

    fn fisher_yates_shuffle(
        &mut self,
        to_shuffle: &mut Vec<Box<Tetromino>>,
    ) -> Vec<Box<Tetromino>> {
        let mut ret = Vec::new();
        for i in (1..=7).rev() {
            let rand = self.linear_congruential_generate();
            let idx = rand % i;
            let removed = to_shuffle.remove(idx);
            ret.push(removed);
        }
        ret
    }

    fn linear_congruential_generate(&mut self) -> usize {
        let next = (13 * self.rand + 5) % 24;
        self.rand = next;
        next
    }
}

impl TetrominoFactory {
    fn build_seven_tetrominos() -> Vec<Box<Tetromino>> {
        vec![
            Box::new(Self::build_default_i()),
            Box::new(Self::build_default_j()),
            Box::new(Self::build_default_l()),
            Box::new(Self::build_default_s()),
            Box::new(Self::build_default_z()),
            Box::new(Self::build_default_t()),
            Box::new(Self::build_default_o()),
        ]
    }

    fn build_default_i() -> I {
        I::new(TetrominoDirection::Right, Block::new(Color::Cyan, 5, 20))
    }

    fn build_default_j() -> J {
        J::new(TetrominoDirection::Right, Block::new(Color::Blue, 4, 20))
    }

    fn build_default_l() -> L {
        L::new(TetrominoDirection::Right, Block::new(Color::Orange, 4, 20))
    }

    fn build_default_s() -> S {
        S::new(TetrominoDirection::Right, Block::new(Color::Green, 4, 20))
    }

    fn build_default_z() -> Z {
        Z::new(TetrominoDirection::Right, Block::new(Color::Red, 4, 20))
    }

    fn build_default_t() -> T {
        T::new(TetrominoDirection::Right, Block::new(Color::Purple, 4, 20))
    }

    fn build_default_o() -> O {
        O::new(TetrominoDirection::Right, Block::new(Color::Yellow, 4, 20))
    }
}
