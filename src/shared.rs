use derive_new::new;

#[derive(Clone, Copy, Debug, Eq, new, PartialEq)]
pub struct Block {
    color: Color,
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Cyan,
    Blue,
    Orange,
    Green,
    Red,
    Purple,
    Yellow,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Block {
    pub fn move_(&self, dir: Direction) -> Self {
        let color = self.color;
        let (x, y) = match dir {
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y - 1),
            Direction::Up => (self.x, self.y + 1),
        };
        Self { color, x, y }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn x(&self) -> &i32 {
        &self.x
    }

    pub fn y(&self) -> &i32 {
        &self.y
    }
}
