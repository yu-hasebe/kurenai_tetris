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

    pub fn draw(
        &self,
        context: &web_sys::CanvasRenderingContext2d,
        image: &web_sys::HtmlImageElement,
    ) {
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                self.idx_on_image(),
                0.0,
                32.0,
                32.0,
                self.x_idx_on_canvas(),
                self.y_idx_on_canvas(),
                32.0,
                32.0,
            )
            .expect(format!("Failed to draw image {:?}", image).as_str());
    }
}

impl Block {
    fn idx_on_image(&self) -> f64 {
        32.0 * match self.color() {
            Color::Cyan => 1.0,
            Color::Blue => 2.0,
            Color::Orange => 3.0,
            Color::Green => 4.0,
            Color::Red => 5.0,
            Color::Purple => 6.0,
            Color::Yellow => 7.0,
        }
    }

    fn x_idx_on_canvas(&self) -> f64 {
        (*self.x() * 32) as f64
    }

    fn y_idx_on_canvas(&self) -> f64 {
        ((19 - *self.y()) * 32) as f64
    }
}
