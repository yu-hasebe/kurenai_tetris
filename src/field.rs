use crate::shared::{Block, Color};

use derive_new::new;

#[derive(Clone, Debug, new)]
pub struct Field(Vec<Vec<Option<Color>>>);

impl Field {
    pub fn is_vacant(&self, blocks: &Vec<Block>) -> bool {
        true
    }
    pub fn fix_blocks(&mut self, blocks: Vec<Block>) {}
    pub fn clear_blocks(&mut self) -> i32 {
        0
    }
    pub fn blocks(&self) -> Vec<Block> {
        vec![]
    }
}
