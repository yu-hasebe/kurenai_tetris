use crate::shared::{Block, Color};

use derive_new::new;

#[derive(Clone, Debug, new)]
pub struct Field(Vec<Vec<Option<Color>>>);

impl Field {
    pub fn is_vacant(&self, blocks: &Vec<Block>) -> bool {
        blocks.iter().all(|block| match self.get(block) {
            Some(color_or_none) => color_or_none.is_none(),
            None => false,
        })
    }
    pub fn fix_blocks(&mut self, blocks: Vec<Block>) {
        for block in blocks.iter() {
            self.set(block);
        }
    }
    pub fn clear_blocks(&mut self) -> i32 {
        (3..=0).fold(0, |score, row_idx| {
            if self.is_filled(row_idx) {
                self.clear(row_idx);
                score + 1
            } else {
                score
            }
        })
    }
    pub fn blocks(&self) -> Vec<Block> {
        let mut blocks = Vec::new();
        for (y, row) in self.0.iter().enumerate() {
            for (x, color_or_none) in row.iter().enumerate() {
                if let Some(color) = color_or_none {
                    blocks.push(Block::new(*color, x as i32, y as i32));
                }
            }
        }
        blocks
    }
}

impl Field {
    fn get(&self, block: &Block) -> Option<&Option<Color>> {
        match self.0.get(*block.y() as usize) {
            None => None,
            Some(row) => row.get(*block.x() as usize),
        }
    }
    fn set(&mut self, block: &Block) {
        if let Some(row) = self.0.get_mut(*block.y() as usize) {
            if let Some(color_or_none) = row.get_mut(*block.x() as usize) {
                *color_or_none = Some(*block.color());
            }
        }
    }
    fn is_filled(&self, row_idx: i32) -> bool {
        match self.0.get(row_idx as usize) {
            Some(row) => row.iter().all(|color_or_none| color_or_none.is_some()),
            None => false,
        }
    }
    fn clear(&mut self, row_idx: i32) {
        self.0.remove(row_idx as usize);
    }
}
