use crate::{adapter::nes::NesAdapter, entity::cartridge::Cartridge};

use super::{cpu::CpuState, joypad::JoyPadState, ppu_state::PpuState};

pub struct NesState {
    pub cpu: CpuState,
    pub ppu: PpuState,
    pub cartridge: Cartridge,
    pub joypad: JoyPadState,
    pub adapter: NesAdapter,
}

impl NesState {
    pub fn new(adapter: NesAdapter) -> Self {
        let cartridge = Cartridge::new(adapter.cartridge.read_file());
        Self {
            cpu: CpuState::default(),
            ppu: PpuState::new(cartridge.vertical_mirroring),
            cartridge,
            joypad: JoyPadState::default(),
            adapter,
        }
    }
}
