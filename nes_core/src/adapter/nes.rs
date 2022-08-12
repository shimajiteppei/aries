use crate::usecase::nes::NesState;

use super::{
    audio::AudioAdapter, cartridge::CartridgeAdapter, joypad::JoyPadAdapter, video::VideoAdapter,
};

pub struct NesAdapter {
    pub cartridge: Box<dyn CartridgeAdapter>,
    pub video: Box<dyn VideoAdapter>,
    pub audio: Box<dyn AudioAdapter>,
    pub joypad: Box<dyn JoyPadAdapter>,
}

impl NesAdapter {
    pub fn init(self) -> NesState {
        let mut state = NesState::new(self);
        state.power();
        state
    }
}
