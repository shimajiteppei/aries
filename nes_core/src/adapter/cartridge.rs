pub trait CartridgeAdapter {
    fn read_file(&self) -> Vec<u8>;
}
