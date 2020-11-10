use derive_new::new;

#[derive(Clone, Copy, Debug, new)]
pub struct Block {
    color: Color,
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
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
pub enum MoveDirection {
    Left,
    Right,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub enum RotateDirection {
    Left,
    Right,
}

impl Block {
    pub fn move_(&self, dir: MoveDirection) -> Self {
        let color = self.color;
        let (x, y) = match dir {
            MoveDirection::Left => (self.x - 1, self.y),
            MoveDirection::Right => (self.x + 1, self.y),
            MoveDirection::Down => (self.x, self.y - 1),
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
