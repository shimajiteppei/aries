use std::{fs::File, io::Read};

use nes_core::adapter::cartridge::CartridgeAdapter;

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
