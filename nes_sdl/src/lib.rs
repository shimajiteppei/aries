use std::time::Duration;

use adapter_impl::{cartridge::CartridgeCtx, video::VideoCtx, audio::AudioCtx, joypad::JoyPadCtx};
use nes_core::adapter::nes::NesAdapter;
use sdl2::{keyboard::Keycode, event::Event};

pub mod adapter_impl;

pub fn start_nes(file_path: String) -> Result<(), String> {
    let sdl = sdl2::init().expect("Could not initialize SDL context.");
    let nes = NesAdapter {
        cartridge: Box::new(CartridgeCtx::new(file_path)),
        video: Box::new(VideoCtx::new(&sdl, 3)),
        audio: Box::new(AudioCtx::default()),
        joypad: Box::new(JoyPadCtx::default()),
    };
    let mut nes_state = nes.init();

    let mut event_pump = sdl.event_pump()?;
    'window_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'window_loop;
                }
                _ => {}
            }
        }

        nes_state.run_frame();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
