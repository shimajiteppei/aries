use crate::entity::apu::Register;

use super::nes::NesState;

#[derive(Debug, Default)]
pub struct ApuState {
    pub register: Register,
}

impl NesState {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn read_apu(&self, addr: u16) -> u8 {
        match addr {
            0x4000..=0x4003 => self.apu.register.pulse1[(addr - 0x4000) as usize],
            0x4004..=0x4007 => self.apu.register.pulse2[(addr - 0x4004) as usize],
            0x4008..=0x400B => self.apu.register.triangle[(addr - 0x4008) as usize],
            0x400C..=0x400F => self.apu.register.noise[(addr - 0x400C) as usize],
            0x4010..=0x4013 => self.apu.register.dmc[(addr - 0x4010) as usize],
            0x4015 => self.apu.register.status,
            0x4017 => self.apu.register.frame_counter,
            _ => 0,
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn write_apu(&mut self, addr: u16, val: u8) -> u8 {
        match addr {
            0x4000..=0x4003 => {
                self.apu.register.pulse1[(addr - 0x4000) as usize] = val;
            }
            0x4004..=0x4007 => {
                self.apu.register.pulse2[(addr - 0x4004) as usize] = val;
            }
            0x4008..=0x400B => {
                self.apu.register.triangle[(addr - 0x4008) as usize] = val;
            }
            0x400C..=0x400F => {
                self.apu.register.noise[(addr - 0x400C) as usize] = val;
            }
            0x4010..=0x4013 => {
                self.apu.register.dmc[(addr - 0x4010) as usize] = val;
            }
            0x4015 => {
                self.apu.register.status = val;
            }
            0x4017 => {
                self.apu.register.frame_counter = val;
            }
            _ => {}
        }
        val
    }
}
