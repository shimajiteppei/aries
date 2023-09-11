mod adapter_impl;
mod util;

use std::{cell::RefCell, rc::Rc};

use adapter_impl::{audio::AudioCtx, cartridge::CartridgeCtx, video::VideoCtx};
use js_sys::Uint8Array;
use nes_core::{adapter::nes::NesAdapter, usecase::nes::NesState};
use util::{run_request_animation_frame_loop, FrameSuccesor};
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub fn start_nes(canvas_id: &str, nes_file: Uint8Array) {
    WindowContext::default().init_nes(canvas_id, nes_file);
}

#[derive(Default)]
struct WindowContext {
    nes_state: Option<Rc<RefCell<NesState>>>,
    frame_closure: Option<Rc<RefCell<Option<Closure<dyn FnMut()>>>>>,
}

impl FrameSuccesor for NesState {
    fn run_one_frame(&mut self) {
        self.run_frame();
    }
}

impl WindowContext {
    fn init_nes(&mut self, canvas_id: &str, nes_file: Uint8Array) {
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
        self.nes_state = Some(nes_state.clone());
        Self::add_keyboard_listener(nes_state.clone());
        self.frame_closure = Some(run_request_animation_frame_loop(nes_state));
    }

    fn add_keyboard_listener(nes_state_ref: Rc<RefCell<NesState>>) {
        let nes_state_ref1 = nes_state_ref.clone();
        let keydown_listener =
            Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |event: KeyboardEvent| {
                let code = event.key_code();
                let mut nes_state = nes_state_ref1.borrow_mut();
                match code {
                    // X
                    88 => nes_state.joypad.state_1p.A = true,
                    // Z
                    90 => nes_state.joypad.state_1p.B = true,
                    // A
                    65 => nes_state.joypad.state_1p.SELECT = true,
                    // S
                    83 => nes_state.joypad.state_1p.START = true,
                    // Right
                    39 => nes_state.joypad.state_1p.RIGHT = true,
                    // Left
                    37 => nes_state.joypad.state_1p.LEFT = true,
                    // Down
                    40 => nes_state.joypad.state_1p.DOWN = true,
                    // Up
                    38 => nes_state.joypad.state_1p.UP = true,
                    _ => {}
                }
            }));
        window()
            .unwrap()
            .add_event_listener_with_callback("keydown", keydown_listener.as_ref().unchecked_ref())
            .unwrap();
        keydown_listener.forget();

        let nes_state_ref2 = nes_state_ref.clone();
        let keyup_listener =
            Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |event: KeyboardEvent| {
                let code = event.key_code();
                let mut nes_state = nes_state_ref2.borrow_mut();
                match code {
                    // X
                    88 => nes_state.joypad.state_1p.A = false,
                    // Z
                    90 => nes_state.joypad.state_1p.B = false,
                    // A
                    65 => nes_state.joypad.state_1p.SELECT = false,
                    // S
                    83 => nes_state.joypad.state_1p.START = false,
                    // Right
                    39 => nes_state.joypad.state_1p.RIGHT = false,
                    // Left
                    37 => nes_state.joypad.state_1p.LEFT = false,
                    // Down
                    40 => nes_state.joypad.state_1p.DOWN = false,
                    // Up
                    38 => nes_state.joypad.state_1p.UP = false,
                    _ => {}
                }
            }));
        window()
            .unwrap()
            .add_event_listener_with_callback("keyup", keyup_listener.as_ref().unchecked_ref())
            .unwrap();
        keyup_listener.forget();
    }
}
