use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use adapter_impl::{audio::AudioCtx, cartridge::CartridgeCtx, video::VideoCtx};
use nes_core::adapter::nes::NesAdapter;
use sdl2::{event::Event, keyboard::Keycode};

pub mod adapter_impl;

pub fn start_nes(file_path: String) -> Result<(), String> {
    let sdl = sdl2::init().expect("Could not initialize SDL context.");
    let mut nes_state = NesAdapter {
        cartridge: Box::new(CartridgeCtx::new(file_path)),
        video: Box::new(VideoCtx::new(&sdl, 3)),
        audio: Box::new(AudioCtx::default()),
    }
    .init();

    let mut event_pump = sdl.event_pump()?;
    'window_loop: loop {
        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'window_loop;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    nes_state.joypad.state_1p.A = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    nes_state.joypad.state_1p.B = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    nes_state.joypad.state_1p.SELECT = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    nes_state.joypad.state_1p.START = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    nes_state.joypad.state_1p.UP = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    nes_state.joypad.state_1p.DOWN = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    nes_state.joypad.state_1p.LEFT = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    nes_state.joypad.state_1p.RIGHT = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    nes_state.joypad.state_1p.A = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    nes_state.joypad.state_1p.B = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    nes_state.joypad.state_1p.SELECT = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    nes_state.joypad.state_1p.START = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    nes_state.joypad.state_1p.UP = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    nes_state.joypad.state_1p.DOWN = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    nes_state.joypad.state_1p.LEFT = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    nes_state.joypad.state_1p.RIGHT = false;
                }
                _ => {}
            }
        }

        nes_state.run_frame();

        let remaining_time_nanos = 1_000_000_000 / 60 - start.elapsed().subsec_nanos() as i64;
        if remaining_time_nanos > 0 {
            sleep(Duration::new(0, remaining_time_nanos as u32));
        }
    }

    Ok(())
}
