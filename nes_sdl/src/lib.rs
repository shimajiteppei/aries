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
                Event::Quit { .. } => {
                    break 'window_loop;
                }
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => match code {
                    Keycode::Escape => break 'window_loop,
                    Keycode::X => nes_state.joypad.state_1p.A = true,
                    Keycode::Z => nes_state.joypad.state_1p.B = true,
                    Keycode::A => nes_state.joypad.state_1p.SELECT = true,
                    Keycode::S => nes_state.joypad.state_1p.START = true,
                    Keycode::Right => nes_state.joypad.state_1p.RIGHT = true,
                    Keycode::Left => nes_state.joypad.state_1p.LEFT = true,
                    Keycode::Down => nes_state.joypad.state_1p.DOWN = true,
                    Keycode::Up => nes_state.joypad.state_1p.UP = true,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => match code {
                    Keycode::X => nes_state.joypad.state_1p.A = false,
                    Keycode::Z => nes_state.joypad.state_1p.B = false,
                    Keycode::A => nes_state.joypad.state_1p.SELECT = false,
                    Keycode::S => nes_state.joypad.state_1p.START = false,
                    Keycode::Right => nes_state.joypad.state_1p.RIGHT = false,
                    Keycode::Left => nes_state.joypad.state_1p.LEFT = false,
                    Keycode::Down => nes_state.joypad.state_1p.DOWN = false,
                    Keycode::Up => nes_state.joypad.state_1p.UP = false,
                    _ => {}
                },
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
