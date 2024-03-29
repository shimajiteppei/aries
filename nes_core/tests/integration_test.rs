use std::{fs::File, io::Read};

use nes_core::adapter::{
    audio::AudioAdapter, cartridge::CartridgeAdapter, nes::NesAdapter, video::VideoAdapter,
};

pub struct CartridgeCtx {
    file_path: String,
}

impl CartridgeCtx {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl CartridgeAdapter for CartridgeCtx {
    fn read_file(&self) -> Vec<u8> {
        let mut file = File::open(self.file_path.clone()).expect("Cannot open file.");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .expect("Failed to read file to the last.");
        buf
    }
}

#[derive(Default)]
pub struct VideoCtx;
impl VideoAdapter for VideoCtx {
    fn draw_frame(&mut self, _pixels: [[u8; 3]; 256 * 240]) {}
}

#[derive(Default)]
pub struct AudioCtx;
impl AudioAdapter for AudioCtx {}

#[test]
fn nestest() {
    let nes = NesAdapter {
        cartridge: Box::new(CartridgeCtx::new(
            "../assets/nes-test-roms/other/nestest.nes".to_string(),
        )),
        video: Box::new(VideoCtx::default()),
        audio: Box::new(AudioCtx::default()),
    };
    let mut nes_state = nes.init();

    // ../assets/nes-test-roms/other/nestest.log
    nes_state.cpu.register.PC = 0xC000;
    loop {
        println!(
            "PC:{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} S:{:02X}",
            nes_state.cpu.register.PC,
            nes_state.cpu.register.A,
            nes_state.cpu.register.X,
            nes_state.cpu.register.Y,
            nes_state.cpu.register.P.get_u8(),
            nes_state.cpu.register.S
        );

        if nes_state.cpu.register.PC == 0xC66E
            && nes_state.cpu.register.A == 0x00
            && nes_state.cpu.register.X == 0xFF
            && nes_state.cpu.register.Y == 0x15
            && nes_state.cpu.register.P.get_u8() == 0x27
            && nes_state.cpu.register.S == 0xFD
        {
            break;
        }
        nes_state.exec();
    }
}
