mod models;

use crate::models::{
    block::{Block, Color},
    count::Count,
    field::Field,
    tetromino::{i::I, MoveDirection, RotateDirection, Tetromino, TetrominoDirection},
    tetromino_factory::TetrominoFactory,
};

use kurenai::game_loop;
use kurenai::game_service::GameService;
use kurenai::key_event::KeyEvent;
use kurenai::{canvas, image};

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

struct TetrisGameService {
    count: RefCell<Count>,
    field: RefCell<Field>,
    tetromino_factory: RefCell<TetrominoFactory>,
    tetromino: RefCell<Box<Tetromino>>,
    image: Rc<web_sys::HtmlImageElement>,
}

impl GameService for TetrisGameService {
    fn key_event(&self, key_event: &KeyEvent) {
        if !self.count.borrow().beat(4) {
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
        let mut count = self.count.borrow_mut();

        count.add();
        if !count.beat(16) {
            return;
        }

        let mut field = self.field.borrow_mut();
        let mut tetromino = self.tetromino.borrow_mut();

        let blocks = tetromino.dry_move(MoveDirection::Down);
        if field.is_vacant(&blocks) {
            tetromino.move_(MoveDirection::Down);
            return;
        }

        if Field::can_fix(&blocks) {
            let blocks = tetromino.blocks();
            field.fix_blocks(blocks);
            field.clear_blocks();
        } else {
            // game over
        }

        let mut tetromino_factory = self.tetromino_factory.borrow_mut();
        *tetromino = tetromino_factory.pick_tetromino();
        if !field.is_vacant(&tetromino.blocks()) {
            // game over
        }
    }

    fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.clear_rect(0.0, 0.0, 320.0, 640.0);

        let image = self.image();

        let blocks_to_draw = {
            let mut blocks_to_draw = Vec::new();

            let field = self.field.borrow();
            let tetromino = self.tetromino.borrow();

            blocks_to_draw.append(&mut field.blocks());
            blocks_to_draw.append(&mut tetromino.blocks());
            blocks_to_draw
        };

        for block in blocks_to_draw.iter() {
            block.draw(context, image);
        }
    }
}

impl TetrisGameService {
    fn new() -> Self {
        let image = {
            let bytes = include_bytes!("./assets/image.gif");
            image::create_new_html_image_element(&bytes.to_vec(), "gif")
        };
        let field = Field::new(vec![vec![None; 10]; 24]);
        let mut tetromino_factory = TetrominoFactory::new();
        let tetromino = tetromino_factory.pick_tetromino();
        Self {
            count: RefCell::new(Count::new(0)),
            field: RefCell::new(field),
            tetromino_factory: RefCell::new(tetromino_factory),
            tetromino: RefCell::new(tetromino),
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
