use nes_core::adapter::nes::NesAdapter;
use nes_sdl::adapter_impl::audio::AudioCtx;
use nes_sdl::adapter_impl::cartridge::CartridgeCtx;
use nes_sdl::adapter_impl::joypad::JoyPadCtx;
use nes_sdl::adapter_impl::video::VideoCtx;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl = sdl2::init().expect("Could not initalize SDL context.");

    let nes = NesAdapter {
        cartridge: Box::new(CartridgeCtx::new(String::from("assets/helloworld.nes"))),
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
