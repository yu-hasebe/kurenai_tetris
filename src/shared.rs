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
