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
        0
    }
    pub fn blocks(&self) -> Vec<Block> {
        vec![]
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
}
