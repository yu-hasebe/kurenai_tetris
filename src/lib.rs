mod field;
mod shared;
mod state;
mod tetromino;

use crate::field::Field;
use crate::state::State;
use crate::tetromino::{i::I, MoveDirection, RotateDirection, Tetromino};

use kurenai::game_loop;
use kurenai::game_service::GameService;
use kurenai::key_event::KeyEvent;
use kurenai::{canvas, image};

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

struct TetrisGameService {
    state: RefCell<State>,
    field: RefCell<Field>,
    tetromino: RefCell<Box<Tetromino>>,
    image: Rc<web_sys::HtmlImageElement>,
}

impl GameService for TetrisGameService {
    fn key_event(&self, key_event: &KeyEvent) {
        if let Dropped = &self.state {
            return;
        }

        let mut tetromino = self.tetromino.borrow_mut();
        let field = self.field.borrow();

        if key_event.arrow_left() {
            let blocks = tetromino.dry_move(MoveDirection::Left);
            if field.is_vacant(&blocks) {
                tetromino.move_(MoveDirection::Left);
            }
        } else if key_event.arrow_right() {
            let blocks = tetromino.dry_move(MoveDirection::Right);
            if field.is_vacant(&blocks) {
                tetromino.move_(MoveDirection::Right);
            }
        } else if key_event.arrow_down() {
            let blocks = tetromino.dry_move(MoveDirection::Down);
            if field.is_vacant(&blocks) {
                tetromino.move_(MoveDirection::Down);
            }
        } else if key_event.key_z() {
            let blocks = tetromino.dry_rotate(RotateDirection::Left);
            if field.is_vacant(&blocks) {
                tetromino.rotate(RotateDirection::Left);
            }
        } else if key_event.key_x() {
            let blocks = tetromino.dry_rotate(RotateDirection::Right);
            if field.is_vacant(&blocks) {
                tetromino.rotate(RotateDirection::Right);
            }
        }
    }

    fn update(&self) {
        let mut state = self.state.borrow_mut();
        let mut field = self.field.borrow_mut();
        let mut tetromino = self.tetromino.borrow_mut();

        match state.clone() {
            Dropping => {
                let blocks = tetromino.dry_move(MoveDirection::Down);
                if field.is_vacant(&blocks) {
                    tetromino.move_(MoveDirection::Down);
                } else {
                    *state = State::Dropped;
                }
            }
            Dropped => {
                let blocks = tetromino.blocks();
                field.fix_blocks(blocks);
                field.clear_blocks();

                *tetromino = Box::new(I::new());
                *state = State::Dropping;
            }
        }
    }

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
        let state = State::Dropping;
        let field = Field::new(vec![Vec::new(); 1]);
        let tetromino = I::new();
        Self {
            state: RefCell::new(state),
            field: RefCell::new(field),
            tetromino: RefCell::new(Box::new(tetromino)),
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
