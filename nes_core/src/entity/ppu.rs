use crate::util::bit::{AsU8, PartialBit};

pub type VerticalMirroring = bool;

/// 2kiB VRAM
pub type VRam = [u8; 0x800];

/// Palette RAM (3F00 ~ 3F1F)
pub type PaletteRam = [u8; 0x20];

/// OAM (256B)
pub type Oam = [u8; 0x100];
/// secondary OAM (32B)
// type SecondaryOam = [Sprite; 8];

/// sprite
#[derive(Debug)]
pub struct Sprite {
    pub x: u8,
    pub attr: u8,
    pub tile: u8,
    pub y: u8,
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Register {
    pub PPU_CTRL: PpuCtrl,
    pub PPU_MASK: PpuMask,
    pub PPU_STATUS: PpuStatus,
    pub OAM_ADDR: u8,
    // pub OAM_DATA: u8,
    // pub PPU_SCROLL: u8,
    // pub PPU_ADDR: u8,
    // pub PPU_DATA: u8,
    // pub OAM_DMA: u8,
}

#[derive(Debug, Default)]
pub struct BusLatch {
    pub result: u8,
    pub buffer: u8,
    pub strobe: bool,
}

#[derive(Debug, Default)]
pub struct PpuCtrl {
    // Generate an NMI at the start of the vertical blanking interval
    pub nmi: bool,
    //PPU master/slave select
    pub slave: bool,
    // Sprite size
    pub spr_sz: bool,
    // Background pattern table address
    pub bg_tbl: bool,
    // Sprite pattern table address for 8x8 sprites
    pub spr_tbl: bool,
    // VRAM address increment per CPU read/write of PPUDATA
    pub incr: bool,
    //  Base nametable address (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    pub nt: u8,
}

impl PpuCtrl {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_u8(&mut self, value: u8) {
        self.nmi = value.bit_flag(7);
        self.slave = value.bit_flag(6);
        self.spr_sz = value.bit_flag(5);
        self.bg_tbl = value.bit_flag(4);
        self.spr_tbl = value.bit_flag(3);
        self.incr = value.bit_flag(2);
        self.nt = value.partial_bit(0..2);
    }
}

#[derive(Debug, Default)]
pub struct PpuMask {
    // Emphasize BGR
    pub blue: bool,
    pub green: bool,
    pub red: bool,
    // Show sprites
    pub spr: bool,
    // Show background
    pub bg: bool,
    // Show sprites in leftmost 8 pixels of screen
    pub spr_left: bool,
    // Show background in leftmost 8 pixels of screen
    pub bg_left: bool,
    // Grayscale
    pub gray: bool,
}

impl PpuMask {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_u8(&mut self, value: u8) {
        self.gray = value.bit_flag(7);
        self.bg_left = value.bit_flag(6);
        self.spr_left = value.bit_flag(5);
        self.bg = value.bit_flag(4);
        self.spr = value.bit_flag(3);
        self.red = value.bit_flag(2);
        self.green = value.bit_flag(1);
        self.blue = value.bit_flag(0);
    }
}

#[derive(Debug, Default)]
pub struct PpuStatus {
    // Vertical blank has started
    pub vblank: bool,
    // Sprite 0 Hit
    pub spr_hit: bool,
    // Sprite overflow
    pub spr_ovf: bool,
    // Least significant bits previously written into a PPU register
    pub lsb_hist: u8,
}

impl PpuStatus {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_u8(&self) -> u8 {
        (self.vblank.as_u8() << 7)
            | (self.spr_hit.as_u8() << 6)
            | (self.spr_ovf.as_u8() << 5)
            | self.lsb_hist
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn set_u8(&mut self, value: u8) {
        self.vblank = value.bit_flag(7);
        self.spr_hit = value.bit_flag(6);
        self.spr_ovf = value.bit_flag(5);
        self.lsb_hist = value.partial_bit(0..5);
    }
}
