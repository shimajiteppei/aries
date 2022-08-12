pub trait JoyPadAdapter {
    fn get_state(&self, is_player2: bool) -> u8;
}
