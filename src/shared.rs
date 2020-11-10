use derive_new::new;

#[derive(Clone, Copy, Debug, new)]
pub struct Block {
    color: Color,
    point: Point,
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

#[derive(Clone, Copy, Debug, new)]
pub struct Point {
    x: i32,
    y: i32,
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
        Self {
            color: self.color,
            point: self.point.move_(dir),
        }
    }
}

impl Point {
    fn move_(&self, dir: MoveDirection) -> Self {
        match dir {
            MoveDirection::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            MoveDirection::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            MoveDirection::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}
