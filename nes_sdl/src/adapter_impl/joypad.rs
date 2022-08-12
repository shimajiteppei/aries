use nes_core::adapter::joypad::JoyPadAdapter;

#[derive(Default)]
pub struct JoyPadCtx {}

impl JoyPadAdapter for JoyPadCtx {
    fn get_state(&self, _is_player2: bool) -> u8 {
        todo!()
    }
}
