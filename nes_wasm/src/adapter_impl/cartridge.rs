use nes_core::adapter::cartridge::CartridgeAdapter;

pub struct CartridgeCtx {
    pub file_bytes: Vec<u8>,
}

impl CartridgeAdapter for CartridgeCtx {
    fn read_file(&self) -> Vec<u8> {
        self.file_bytes.clone()
    }
}
