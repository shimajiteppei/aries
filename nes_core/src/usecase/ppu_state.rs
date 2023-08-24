use crate::{
    entity::ppu::{BusLatch, Oam, PaletteRam, Register, VRam, VerticalMirroring},
    util::bit::PartialBit,
};

#[derive(Debug)]
pub struct PpuState {
    pub vram: VRam,
    pub palette_ram: PaletteRam,
    pub vertical_mirroring: VerticalMirroring,
    pub oam: OamState,
    pub register: Register,
    pub bus_latch: BusLatch,
    pub loopy: LoopyState,
    pub background_shift_register: BackgroundShiftRegister,
    pub frame: FrameState,
    pub pixels: [[u8; 3]; 256 * 240],
    pub addr: u16,
}

impl PpuState {
    pub fn new(vertical_mirroring: bool) -> Self {
        Self {
            vram: [0; 0x800],
            palette_ram: [0; 0x20],
            vertical_mirroring,
            oam: OamState {
                primary: [0; 0x100],
                imaginary: [0; 8].map(|_| ImaginarySprite::default()),
                secondary: [0; 8].map(|_| ImaginarySprite::default()),
            },
            register: Register::default(),
            bus_latch: BusLatch::default(),
            loopy: LoopyState::default(),
            background_shift_register: BackgroundShiftRegister::default(),
            frame: FrameState::default(),
            pixels: [[0; 3]; 256 * 240],
            addr: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImaginarySprite {
    pub x: u8,
    pub attr: u8,
    pub tile: u8,
    pub y: u8,
    pub id: u8,
    pub data_l: u8,
    pub data_h: u8,
}

#[derive(Debug)]
pub struct OamState {
    pub primary: Oam,
    pub imaginary: [ImaginarySprite; 8],
    pub secondary: [ImaginarySprite; 8],
}

#[derive(Debug, Default)]
pub struct BackgroundShiftRegister {
    /// nametable
    pub nt: u8,
    // attribute table
    pub at: u8,
    pub bg_l: u8,
    pub bg_h: u8,
    pub at_shift_l: u8,
    pub at_shift_h: u8,
    pub bg_shift_l: u16,
    pub bg_shift_h: u16,
    pub at_latch_l: bool,
    pub at_latch_h: bool,
}

#[derive(Debug, Default)]
pub struct FrameState {
    pub scanline: u16,
    pub dot: u16,
    pub is_odd: bool,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq)]
pub enum ScanlineMode {
    VISIBLE,
    POST,
    NMI,
    PRE,
}

#[derive(Debug, Default)]
pub struct LoopyAddr {
    /// 3bit
    pub f_y: u8,
    /// 2bit
    pub nt: u8,
    /// 5bit
    pub c_y: u8,
    /// 5bit
    pub c_x: u8,
}

impl LoopyAddr {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_u16(&self) -> u16 {
        ((self.f_y as u16) << 12)
            | ((self.nt as u16) << 10)
            | ((self.c_y as u16) << 5)
            | self.c_x as u16
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_u16(&mut self, value: u16) {
        self.f_y = value.partial_bit(12..15) as u8;
        self.nt = value.partial_bit(10..12) as u8;
        self.c_y = value.partial_bit(5..10) as u8;
        self.c_x = value.partial_bit(0..5) as u8;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_addr(&self) -> u16 {
        self.get_u16() & 0b11_1111_1111_1111
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_addr(&mut self, value: u16) {
        self.set_u16((self.f_y as u16 & 0b100) << 12 | value & 0b11_1111_1111_1111)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_high(&mut self, value: u8) {
        self.f_y = value.partial_bit(4..7);
        self.nt = value.partial_bit(2..4);
        self.c_y = (self.c_y & 0b111) | (value.partial_bit(0..2) << 3);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_low(&mut self, value: u8) {
        self.c_y = (self.c_y & 0b11000) | value.partial_bit(5..8);
        self.c_x = value.partial_bit(0..5);
    }
}

#[derive(Debug, Default)]
pub struct LoopyState {
    pub v_addr: LoopyAddr,
    pub t_addr: LoopyAddr,
    pub f_x: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _loopy_addr_get_set_u16() {
        let mut addr = LoopyAddr {
            f_y: 0b111,
            nt: 0,
            c_y: 0b11111,
            c_x: 0,
        };
        assert_eq!(addr.get_u16(), {
            addr.set_u16(addr.get_u16());
            addr.get_u16()
        });
    }

    #[test]
    fn _loopy_addr_get_set_addr() {
        let mut addr = LoopyAddr {
            f_y: 0b111,
            nt: 0,
            c_y: 0b11111,
            c_x: 0,
        };
        assert_eq!(addr.get_u16(), {
            addr.set_addr(addr.get_addr());
            addr.get_u16()
        });
    }

    #[test]
    fn _loopy_addr_set_lh() {
        let mut addr = LoopyAddr {
            f_y: 0b111,
            nt: 0,
            c_y: 0b11111,
            c_x: 0,
        };
        assert_eq!(addr.get_u16(), {
            let addr_u16 = addr.get_u16();
            addr.set_low(addr_u16 as u8);
            addr.set_high(((addr_u16 & 0xFF00) >> 8) as u8);
            addr.get_u16()
        });
    }
}
