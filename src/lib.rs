mod field;
mod shared;
mod tetromino;

use crate::field::Field;
use crate::shared::Block;
use crate::tetromino::{i::I, EnumTetromino, Tetromino};

use kurenai::game_loop;
use kurenai::game_service::GameService;
use kurenai::key_event::KeyEvent;
use kurenai::{canvas, image};

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

struct TetrisGameService {
    field: RefCell<Field>,
    tetromino: RefCell<EnumTetromino>,
    image: Rc<web_sys::HtmlImageElement>,
}

impl GameService for TetrisGameService {
    fn key_event(&self, key_event: &KeyEvent) {}

    fn update(&self) {}

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        let image = self.image();
        context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image, 0.0, 0.0, 32.0, 32.0, 0.0, 0.0, 32.0, 32.0,
            )
            .expect(format!("Failed to draw image {:?}", image).as_str());
    }
}

impl TetrisGameService {
    fn new() -> Self {
        let image = {
            let bytes = include_bytes!("./image.gif");
            image::create_new_html_image_element(&bytes.to_vec(), "gif")
        };
        let field = Field::new(vec![Vec::new(); 1]);
        let tetromino = I::new();
        Self {
            field: RefCell::new(field),
            tetromino: RefCell::new(EnumTetromino::I(tetromino)),
            image: Rc::new(image),
        }
    }

    fn image(&self) -> &web_sys::HtmlImageElement {
        self.image.deref()
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&JsValue::from_str("Hello world!"));

    let tetris_game_service = TetrisGameService::new();
    let canvas_rendering_context = canvas::get_canvas_rendering_context_2d("main-canvas");
    game_loop::run(tetris_game_service, canvas_rendering_context);

    Ok(())
}
