mod adapter_impl;

use std::{cell::RefCell, rc::Rc};

use adapter_impl::{audio::AudioCtx, cartridge::CartridgeCtx, video::VideoCtx};
use js_sys::Uint8Array;
use nes_core::{adapter::nes::NesAdapter, usecase::nes::NesState};
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct WindowContext {
    nes_state: Rc<RefCell<NesState>>,
}

#[wasm_bindgen]
impl WindowContext {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, nes_file: Uint8Array) -> WindowContext {
        let nes_state = Rc::new(RefCell::new(
            NesAdapter {
                cartridge: Box::new(CartridgeCtx {
                    file_bytes: nes_file.to_vec(),
                }),
                video: Box::new(VideoCtx::new(
                    window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id(canvas_id)
                        .unwrap()
                        .dyn_into::<HtmlCanvasElement>()
                        .map_err(|_| ())
                        .unwrap()
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap(),
                )),
                audio: Box::new(AudioCtx::default()),
            }
            .init(),
        ));
        Self::run_request_animation_frame_loop(nes_state.clone());

        WindowContext {
            nes_state: nes_state,
        }
    }

    fn run_request_animation_frame_loop(nes_state: Rc<RefCell<NesState>>) {
        // We use Rc<RefCell<None>> trick for recursive calling of request_animation_frame.
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::new(move || {
            nes_state.as_ref().borrow_mut().run_frame();
            Self::request_animation_frame(f.borrow().as_ref().unwrap());
        }));
        Self::request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .unwrap();
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_A(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.A = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_B(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.B = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_SELECT(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.SELECT = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_START(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.START = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_RIGHT(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.RIGHT = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_LEFT(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.LEFT = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_DOWN(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.DOWN = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keydown_UP(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.UP = true;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_A(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.A = false;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_B(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.B = false;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_SELECT(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.SELECT = false;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_START(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.START = false;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_RIGHT(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.RIGHT = false;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_LEFT(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.LEFT = false;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_DOWN(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.DOWN = false;
    }

    #[allow(non_snake_case)]
    #[wasm_bindgen]
    pub fn keyup_UP(&mut self) {
        self.nes_state.borrow_mut().joypad.state_1p.UP = false;
    }
}
