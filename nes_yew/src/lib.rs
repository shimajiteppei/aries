use adapter_impl::{audio::AudioCtx, cartridge::CartridgeCtx, video::VideoCtx};
use nes_core::{adapter::nes::NesAdapter, usecase::nes::NesState};
use std::{cell::RefCell, rc::Rc};
use util::{console_log, run_request_animation_frame_loop, FrameSuccesor};
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{prelude::*, props};

pub mod adapter_impl;
pub mod util;

pub fn start_nes(file_path: String) -> Result<(), String> {
    let nes_props = props! {
        NesProps {
            file_path: file_path,
        }
    };
    yew::Renderer::<App>::with_props(nes_props).render();
    Ok(())
}

#[function_component]
fn App(nes_props: &NesProps) -> Html {
    let nes_props = nes_props.clone();
    html! {
        <div style="display:flex;flex-direction:column">
            <div style="display:flex;justify-content:center">
                <NesComponent ..nes_props />
            </div>
            <p style="display:flex;justify-content:center">
                { "S: START / A: SELECT / Z: A / X: B / 方向ボタン: 十字キー" }
            </p>
        </div>
    }
}

struct NesComponent {
    nes_state: Option<Rc<RefCell<NesState>>>,
    frame_closure: Option<Rc<RefCell<Option<Closure<dyn FnMut()>>>>>,
    keyboard_event_listener_closure: Vec<Closure<dyn Fn(KeyboardEvent)>>,
}

enum NesMessage {
    KeyUp(u32),
    KeyDown(u32),
    FileFetched(CartridgeCtx),
}

#[derive(Properties, PartialEq, Clone)]
struct NesProps {
    pub file_path: String,
}

impl FrameSuccesor for NesState {
    fn run_one_frame(&mut self) {
        self.run_frame();
    }
}

const CANVAS_ID: &str = "nes-canvas";

impl Component for NesComponent {
    type Message = NesMessage;
    type Properties = NesProps;

    fn create(ctx: &Context<Self>) -> Self {
        // add keyboard listener
        let link = ctx.link().clone();
        let keydown_listener =
            Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |event: KeyboardEvent| {
                link.callback(|event: KeyboardEvent| NesMessage::KeyDown(event.key_code()))
                    .emit(event);
            }));
        window()
            .unwrap()
            .add_event_listener_with_callback("keydown", keydown_listener.as_ref().unchecked_ref())
            .unwrap();

        let link = ctx.link().clone();
        let keyup_listener =
            Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |event: KeyboardEvent| {
                link.callback(|event: KeyboardEvent| NesMessage::KeyUp(event.key_code()))
                    .emit(event);
            }));
        window()
            .unwrap()
            .add_event_listener_with_callback("keyup", keyup_listener.as_ref().unchecked_ref())
            .unwrap();

        Self {
            nes_state: None,
            frame_closure: None,
            keyboard_event_listener_closure: vec![keydown_listener, keyup_listener],
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas id={CANVAS_ID} width="256" height="240" style="height:90vh" />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // download nes file
            let link = ctx.link().clone();
            let mut cartridge_ctx = CartridgeCtx::new(ctx.props().file_path.clone());
            spawn_local(async move {
                cartridge_ctx.fetch_file().await;
                link.callback(|cartridge_ctx: CartridgeCtx| NesMessage::FileFetched(cartridge_ctx))
                    .emit(cartridge_ctx);
            });
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Some(nes_state) = self.nes_state.clone() {
            let mut nes_state = nes_state.borrow_mut();
            match msg {
                NesMessage::KeyDown(code) => {
                    match code {
                        // Escape
                        27 => self.stop_frame_loop(),
                        // Z
                        90 => nes_state.joypad.state_1p.A = true,
                        // X
                        88 => nes_state.joypad.state_1p.B = true,
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
                }
                NesMessage::KeyUp(code) => {
                    match code {
                        // Z
                        90 => nes_state.joypad.state_1p.A = false,
                        // X
                        88 => nes_state.joypad.state_1p.B = false,
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
                }
                _ => {}
            }
        } else {
            match msg {
                NesMessage::FileFetched(cartridge_ctx) => {
                    let nes_state = Rc::new(RefCell::new(
                        NesAdapter {
                            cartridge: Box::new(cartridge_ctx),
                            video: Box::new(VideoCtx::new(
                                window()
                                    .unwrap()
                                    .document()
                                    .unwrap()
                                    .get_element_by_id(CANVAS_ID)
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
                    self.frame_closure = Some(run_request_animation_frame_loop(nes_state));
                }
                _ => {}
            }
        }
        true
    }
}

impl NesComponent {
    pub fn stop_frame_loop(&mut self) {
        drop(self.frame_closure.take());
    }
}
