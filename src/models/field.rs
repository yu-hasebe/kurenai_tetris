use crate::models::block::{Block, Color};

use derive_new::new;

#[derive(Clone, Debug, Eq, PartialEq, new)]
pub struct Field(Vec<Vec<Option<Color>>>);

impl Field {
    pub fn is_vacant(&self, blocks: &Vec<Block>) -> bool {
        blocks.iter().all(|block| match self.get(block) {
            Some(color_or_none) => color_or_none.is_none(),
            None => false,
        })
    }
    pub fn can_fix(blocks: &Vec<Block>) -> bool {
        blocks.iter().any(|block| *block.y() < 20)
    }
    pub fn fix_blocks(&mut self, blocks: Vec<Block>) {
        for block in blocks.iter() {
            self.set(block);
        }
    }
    pub fn clear_blocks(&mut self) -> i32 {
        (0..24).rev().fold(0, |score, row_idx| {
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
        self.0.push(vec![None; 10]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vacant_1() {
        let field = build_field_with_missing_lines(4);
        let blocks = build_blocks();
        assert_eq!(true, field.is_vacant(&blocks));
    }

    #[test]
    fn test_is_vacant_2() {
        let field = build_field_with_blocks(4);
        let blocks = build_blocks();
        assert_eq!(false, field.is_vacant(&blocks));
    }

    #[test]
    fn test_is_vacant_3() {
        let mut field = build_field_with_missing_lines(4);
        field.0[3][0] = Some(Color::Cyan);
        let blocks = build_blocks();
        assert_eq!(false, field.is_vacant(&blocks));
    }

    #[test]
    fn test_can_fix_1() {
        let blocks = vec![
            Block::new(Color::Cyan, 0, 19),
            Block::new(Color::Cyan, 0, 20),
            Block::new(Color::Cyan, 0, 21),
            Block::new(Color::Cyan, 0, 22),
        ];
        assert_eq!(true, Field::can_fix(&blocks));
    }

    #[test]
    fn test_can_fix_2() {
        let blocks = vec![
            Block::new(Color::Cyan, 0, 20),
            Block::new(Color::Cyan, 0, 21),
            Block::new(Color::Cyan, 0, 22),
            Block::new(Color::Cyan, 0, 23),
        ];
        assert_eq!(false, Field::can_fix(&blocks));
    }

    #[test]
    fn test_fix_blocks() {
        let mut field = build_field_with_missing_lines(4);
        let blocks = build_blocks();
        assert_eq!(field.0[0][0], None);
        assert_eq!(field.0[1][0], None);
        assert_eq!(field.0[2][0], None);
        assert_eq!(field.0[3][0], None);
        field.fix_blocks(blocks);
        assert_eq!(field.0[0][0], Some(Color::Cyan));
        assert_eq!(field.0[1][0], Some(Color::Cyan));
        assert_eq!(field.0[2][0], Some(Color::Cyan));
        assert_eq!(field.0[3][0], Some(Color::Cyan));
    }

    #[test]
    fn test_clear_blocks_1() {
        let mut field = build_field_with_missing_lines(4);
        let field_clone = field.clone();
        assert_eq!(0, field.clear_blocks());
        assert_eq!(field_clone, field);
        assert_eq!(24, field.0.len());
    }

    #[test]
    fn test_clear_blocks_2() {
        let mut field = build_field_with_missing_lines(5);
        field.0[3][0] = Some(Color::Cyan);
        assert_eq!(1, field.clear_blocks());
        assert_eq!(missing_line(), field.0[3]);
        assert_eq!(missing_line(), field.0[2]);
        assert_eq!(missing_line(), field.0[1]);
        assert_eq!(missing_line(), field.0[0]);
        assert_eq!(24, field.0.len());
    }

    #[test]
    fn test_clear_blocks_3() {
        let mut field = build_field_with_missing_lines(5);
        field.0[3][0] = Some(Color::Cyan);
        field.0[2][0] = Some(Color::Cyan);
        assert_eq!(2, field.clear_blocks());
        assert_eq!(vec![None; 10], field.0[3]);
        assert_eq!(missing_line(), field.0[2]);
        assert_eq!(missing_line(), field.0[1]);
        assert_eq!(missing_line(), field.0[0]);
        assert_eq!(24, field.0.len());
    }

    #[test]
    fn test_clear_blocks_4() {
        let mut field = build_field_with_missing_lines(5);
        field.0[3][0] = Some(Color::Cyan);
        field.0[2][0] = Some(Color::Cyan);
        field.0[1][0] = Some(Color::Cyan);
        assert_eq!(3, field.clear_blocks());
        assert_eq!(vec![None; 10], field.0[3]);
        assert_eq!(vec![None; 10], field.0[2]);
        assert_eq!(missing_line(), field.0[1]);
        assert_eq!(missing_line(), field.0[0]);
        assert_eq!(24, field.0.len());
    }

    #[test]
    fn test_clear_blocks_5() {
        let mut field = build_field_with_missing_lines(5);
        field.0[3][0] = Some(Color::Cyan);
        field.0[2][0] = Some(Color::Cyan);
        field.0[1][0] = Some(Color::Cyan);
        field.0[0][0] = Some(Color::Cyan);
        assert_eq!(4, field.clear_blocks());
        assert_eq!(vec![None; 10], field.0[3]);
        assert_eq!(vec![None; 10], field.0[2]);
        assert_eq!(vec![None; 10], field.0[1]);
        assert_eq!(missing_line(), field.0[0]);
        assert_eq!(24, field.0.len());
    }

    #[test]
    fn test_blocks_1() {
        let field = build_field_with_blocks(4);
        assert_eq!(build_blocks(), field.blocks());
    }

    #[test]
    fn test_blocks_2() {
        let field = Field(vec![vec![None; 10]; 24]);
        assert_eq!(Vec::<Block>::new(), field.blocks());
    }

    fn build_field_with_blocks(num: i32) -> Field {
        let mut field = vec![vec![None; 10]; 24];
        for row_idx in 0..num {
            field[row_idx as usize][0] = Some(Color::Cyan);
        }
        Field(field)
    }

    fn build_field_with_missing_lines(num: i32) -> Field {
        let mut field = vec![vec![None; 10]; 24];
        for row_idx in 0..num {
            field[row_idx as usize] = missing_line();
        }
        Field(field)
    }

    fn missing_line() -> Vec<Option<Color>> {
        let mut line = vec![Some(Color::Cyan); 10];
        line[0] = None;
        line
    }

    fn build_blocks() -> Vec<Block> {
        vec![
            Block::new(Color::Cyan, 0, 0),
            Block::new(Color::Cyan, 0, 1),
            Block::new(Color::Cyan, 0, 2),
            Block::new(Color::Cyan, 0, 3),
        ]
    }
}
