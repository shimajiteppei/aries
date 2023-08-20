use crate::{adapter::nes::NesAdapter, entity::cartridge::Cartridge};

use super::{apu::ApuState, cpu::CpuState, joypad::JoyPadState, ppu_state::PpuState};

pub struct NesState {
    pub cpu: CpuState,
    pub ppu: PpuState,
    pub apu: ApuState,
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
            apu: ApuState::default(),
            cartridge,
            joypad: JoyPadState::default(),
            adapter,
        }
    }
}
