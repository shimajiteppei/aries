use crate::{
    entity::nes_rgb::NES_RGB,
    util::bit::{AsU16, AsU8, PartialBit, Zero},
};

use super::{
    nes::NesState,
    ppu_state::{PpuState, ScanlineMode},
};

impl PpuState {
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn nt_mirror(&self, addr: u16) -> u16 {
        if self.vertical_mirroring {
            addr % 0x800
        } else {
            ((addr >> 1) & 0x400) + (addr % 0x400)
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn is_rendering(&self) -> bool {
        self.register.PPU_MASK.bg || self.register.PPU_MASK.spr
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn spr_height(&self) -> u16 {
        if self.register.PPU_CTRL.spr_sz {
            16
        } else {
            8
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn nt_addr(&self) -> u16 {
        0x2000 | (self.loopy.v_addr.get_u16() & 0xFFF)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn at_addr(&self) -> u16 {
        0x23C0
            | ((self.loopy.v_addr.nt as u16) << 10)
            | ((self.loopy.v_addr.c_y as u16 >> 2) << 3)
            | (self.loopy.v_addr.c_x as u16 >> 2)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn bg_addr(&self) -> u16 {
        (self.background_shift_register.nt as u16) * 16
            + self.loopy.v_addr.f_y as u16
            + self.register.PPU_CTRL.bg_tbl.as_u16() * 0x1000
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn h_scroll(&mut self) {
        if self.is_rendering() {
            if self.loopy.v_addr.c_x == 31 {
                self.loopy
                    .v_addr
                    .set_u16(self.loopy.v_addr.get_u16() ^ 0x41F);
            } else {
                self.loopy.v_addr.c_x += 1;
            }
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn v_scroll(&mut self) {
        if self.is_rendering() {
            if self.loopy.v_addr.f_y < 7 {
                self.loopy.v_addr.f_y += 1;
            } else {
                self.loopy.v_addr.f_y = 0;
                match self.loopy.v_addr.c_y {
                    31 => {
                        self.loopy.v_addr.c_y = 0;
                    }
                    29 => {
                        self.loopy.v_addr.c_y = 0;
                        self.loopy.v_addr.nt ^= 0b10;
                    }
                    _ => {
                        self.loopy.v_addr.c_y += 1;
                    }
                }
            }
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn h_update(&mut self) {
        if self.is_rendering() {
            self.loopy.v_addr.set_u16(
                (self.loopy.v_addr.get_u16() & !0x041F) | (self.loopy.t_addr.get_u16() & 0x041F),
            )
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn v_update(&mut self) {
        if self.is_rendering() {
            self.loopy.v_addr.set_u16(
                (self.loopy.v_addr.get_u16() & !0x7BE0) | (self.loopy.t_addr.get_u16() & 0x7BE0),
            )
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn reload_shift(&mut self) {
        self.background_shift_register.bg_shift_l = (self.background_shift_register.bg_shift_l
            & 0xFF00)
            | (self.background_shift_register.bg_l as u16);
        self.background_shift_register.bg_shift_h = (self.background_shift_register.bg_shift_h
            & 0xFF00)
            | (self.background_shift_register.bg_h as u16);
        self.background_shift_register.at_latch_l =
            (self.background_shift_register.at & 1).as_bool();
        self.background_shift_register.at_latch_h =
            (self.background_shift_register.at & 2).as_bool();
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn clear_oam(&mut self) {
        for i in 0..8 {
            self.oam.secondary[i].y = 0xFF;
            self.oam.secondary[i].tile = 0xFF;
            self.oam.secondary[i].attr = 0xFF;
            self.oam.secondary[i].x = 0xFF;
            self.oam.secondary[i].id = 64;
            self.oam.secondary[i].data_l = 0;
            self.oam.secondary[i].data_h = 0;
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn eval_sprites(&mut self) {
        let mut n = 0;
        for i in 0..64 {
            let line: i32 = if self.frame.scanline == 261 {
                -1
            } else {
                self.frame.scanline as i32
            } - self.oam.primary[i * 4] as i32;
            if line >= 0 && line < self.spr_height() as i32 {
                self.oam.secondary[n].y = self.oam.primary[i * 4];
                self.oam.secondary[n].tile = self.oam.primary[i * 4 + 1];
                self.oam.secondary[n].attr = self.oam.primary[i * 4 + 2];
                self.oam.secondary[n].x = self.oam.primary[i * 4 + 3];
                self.oam.secondary[n].id = i as u8;
                n += 1;
                if n >= 8 {
                    self.register.PPU_STATUS.spr_ovf = true;
                    break;
                }
            }
        }
    }
}

impl NesState {
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn read_ppu_bus(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cartridge.read_chr(addr),
            0x2000..=0x3EFF => self.ppu.vram[self.ppu.nt_mirror(addr) as usize],
            0x3F00..=0x3FFF => {
                let addr_palette = if addr & 0x13 == 0x10 {
                    addr & !0x10_u16
                } else {
                    addr
                };
                self.ppu.palette_ram[(addr_palette & 0x1F) as usize]
                    & if self.ppu.register.PPU_MASK.gray {
                        0x30
                    } else {
                        0xFF
                    }
            }
            _ => 0,
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn write_ppu_bus(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.cartridge.write_chr(addr, value);
            }
            0x2000..=0x3EFF => {
                self.ppu.vram[self.ppu.nt_mirror(addr) as usize] = value;
            }
            0x3F00..=0x3FFF => {
                let addr_palette = if addr & 0x13 == 0x10 {
                    addr & !0x10_u16
                } else {
                    addr
                };
                self.ppu.palette_ram[(addr_palette & 0x1F) as usize] = value;
            }
            _ => {}
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn read_ppu(&mut self, addr: u16) -> u8 {
        match addr % 8 {
            2 => {
                self.ppu.bus_latch.result =
                    (self.ppu.bus_latch.result & 0x1F) | self.ppu.register.PPU_STATUS.get_u8();
                self.ppu.register.PPU_STATUS.vblank = false;
                self.ppu.bus_latch.strobe = false;
            }
            4 => {
                self.ppu.bus_latch.result =
                    self.ppu.oam.primary[self.ppu.register.OAM_ADDR as usize];
            }
            7 => {
                if self.ppu.loopy.v_addr.get_addr() <= 0x3EFF {
                    self.ppu.bus_latch.result = self.ppu.bus_latch.buffer;
                    self.ppu.bus_latch.buffer = self.read_ppu_bus(self.ppu.loopy.v_addr.get_addr());
                } else {
                    self.ppu.bus_latch.buffer = self.read_ppu_bus(self.ppu.loopy.v_addr.get_addr());
                    self.ppu.bus_latch.result = self.ppu.bus_latch.buffer;
                }
                self.ppu.loopy.v_addr.set_addr(
                    self.ppu.loopy.v_addr.get_addr()
                        + if self.ppu.register.PPU_CTRL.incr {
                            0b10_0000
                        } else {
                            1
                        },
                );
            }
            _ => {}
        };
        self.ppu.bus_latch.result
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn write_ppu(&mut self, addr: u16, value: u8) -> u8 {
        self.ppu.bus_latch.result = value;
        match addr % 8 {
            0 => {
                self.ppu.register.PPU_CTRL.set_u8(value);
                self.ppu.loopy.t_addr.nt = self.ppu.register.PPU_CTRL.nt;
            }
            1 => {
                self.ppu.register.PPU_MASK.set_u8(value);
            }
            3 => {
                self.ppu.register.OAM_ADDR = value;
            }
            4 => {
                self.ppu.oam.primary[self.ppu.register.OAM_ADDR as usize] = value;
                self.ppu.register.OAM_ADDR = self.ppu.register.OAM_ADDR.wrapping_add(1);
            }
            5 => {
                if !self.ppu.bus_latch.strobe {
                    self.ppu.loopy.f_x = value & 7;
                    self.ppu.loopy.t_addr.c_x = value >> 3;
                } else {
                    self.ppu.loopy.t_addr.f_y = value & 7;
                    self.ppu.loopy.t_addr.c_y = value >> 3;
                }
                self.ppu.bus_latch.strobe = !self.ppu.bus_latch.strobe;
            }
            6 => {
                if !self.ppu.bus_latch.strobe {
                    self.ppu.loopy.t_addr.set_high(value & 0x3F);
                } else {
                    self.ppu.loopy.t_addr.set_low(value);
                    self.ppu
                        .loopy
                        .v_addr
                        .set_u16(self.ppu.loopy.t_addr.get_u16());
                }
                self.ppu.bus_latch.strobe = !self.ppu.bus_latch.strobe;
            }
            7 => {
                self.write_ppu_bus(self.ppu.loopy.v_addr.get_addr(), value);
                self.ppu.loopy.v_addr.set_addr(
                    self.ppu.loopy.v_addr.get_addr()
                        + if self.ppu.register.PPU_CTRL.incr {
                            32
                        } else {
                            1
                        },
                );
            }
            _ => {}
        };
        self.ppu.bus_latch.result
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn load_sprites(&mut self) {
        let mut addr: u16;
        for i in 0..8 {
            self.ppu.oam.imaginary[i] = self.ppu.oam.secondary[i].clone();
            if self.ppu.spr_height() == 16 {
                addr = (self.ppu.oam.imaginary[i].tile as u16 & 1) * 0x1000
                    + (self.ppu.oam.imaginary[i].tile as u16 & !1) * 16;
            } else {
                addr = self.ppu.register.PPU_CTRL.spr_tbl.as_u16() * 0x1000
                    + self.ppu.oam.imaginary[i].tile as u16 * 16;
            }

            let mut spr_y = self
                .ppu
                .frame
                .scanline
                .wrapping_sub(self.ppu.oam.imaginary[i].y as u16)
                % self.ppu.spr_height();
            if (self.ppu.oam.imaginary[i].attr & 0x80).as_bool() {
                spr_y ^= self.ppu.spr_height() - 1;
            }
            addr += spr_y + (spr_y & 8);

            self.ppu.oam.imaginary[i].data_l = self.read_ppu_bus(addr);
            self.ppu.oam.imaginary[i].data_h = self.read_ppu_bus(addr + 8);
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn pixel(&mut self) {
        let mut palette: u8 = 0;
        let mut obj_palette: u8 = 0;
        let mut obj_priority: u8 = 0;
        let x: u16 = self.ppu.frame.dot - 2;

        if self.ppu.frame.scanline < 240 && x < 256 {
            // bg
            if self.ppu.register.PPU_MASK.bg && (self.ppu.register.PPU_MASK.bg_left || x >= 8) {
                palette = ((self
                    .ppu
                    .background_shift_register
                    .bg_shift_h
                    .bit(15 - self.ppu.loopy.f_x)
                    << 1)
                    | (self
                        .ppu
                        .background_shift_register
                        .bg_shift_l
                        .bit(15 - self.ppu.loopy.f_x))) as u8;

                if palette.as_bool() {
                    palette |= (((self
                        .ppu
                        .background_shift_register
                        .at_shift_h
                        .bit(7 - self.ppu.loopy.f_x)
                        << 1)
                        | (self
                            .ppu
                            .background_shift_register
                            .at_shift_l
                            .bit(7 - self.ppu.loopy.f_x))) as u8)
                        << 2;
                }
            }
            // spr
            if self.ppu.register.PPU_MASK.spr && (self.ppu.register.PPU_MASK.spr_left || x >= 8) {
                for i in (0..=7).rev() {
                    if self.ppu.oam.imaginary[i].id == 64 {
                        continue;
                    }
                    let mut spr_x: u8 = (x as u8).wrapping_sub(self.ppu.oam.imaginary[i].x);
                    if spr_x >= 8 {
                        continue;
                    }
                    if (self.ppu.oam.imaginary[i].attr & 0x40).as_bool() {
                        spr_x ^= 7;
                    }

                    let mut spr_palette = (self.ppu.oam.imaginary[i].data_h.bit(7 - spr_x) << 1)
                        | self.ppu.oam.imaginary[i].data_l.bit(7 - spr_x);
                    if spr_palette == 0 {
                        continue;
                    }

                    if self.ppu.oam.imaginary[i].id == 0 && palette.as_bool() && x != 255 {
                        self.ppu.register.PPU_STATUS.spr_hit = true;
                    }
                    spr_palette |= (self.ppu.oam.imaginary[i].attr & 3) << 2;
                    obj_palette = spr_palette + 16;
                    obj_priority = self.ppu.oam.imaginary[i].attr & 0x20;
                }
            }
            // eval priority
            if obj_palette.as_bool() && (palette == 0 || obj_priority == 0) {
                palette = obj_palette;
            }

            self.ppu.pixels[self.ppu.frame.scanline as usize * 256 + x as usize] =
                NES_RGB[self.read_ppu_bus(
                    0x3F00
                        + if self.ppu.is_rendering() {
                            palette as u16
                        } else {
                            0
                        },
                ) as usize];
        }
        self.ppu.background_shift_register.bg_shift_l <<= 1;
        self.ppu.background_shift_register.bg_shift_h <<= 1;
        self.ppu.background_shift_register.at_shift_l =
            (self.ppu.background_shift_register.at_shift_l << 1)
                | self.ppu.background_shift_register.at_latch_l.as_u8();
        self.ppu.background_shift_register.at_shift_h =
            (self.ppu.background_shift_register.at_shift_h << 1)
                | self.ppu.background_shift_register.at_latch_h.as_u8();
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn scanline_cycle(&mut self, mode: ScanlineMode) {
        if mode == ScanlineMode::NMI && self.ppu.frame.dot == 1 {
            self.ppu.register.PPU_STATUS.vblank = true;
            if self.ppu.register.PPU_CTRL.nmi {
                self.cpu.control.NMI = true;
            }
        } else if mode == ScanlineMode::POST && self.ppu.frame.dot == 0 {
            self.adapter.video.draw_frame(self.ppu.pixels);
        } else if mode == ScanlineMode::VISIBLE || mode == ScanlineMode::PRE {
            match self.ppu.frame.dot {
                1 => {
                    self.ppu.clear_oam();
                    if mode == ScanlineMode::PRE {
                        self.ppu.register.PPU_STATUS.spr_hit = false;
                        self.ppu.register.PPU_STATUS.spr_ovf = false;
                    }
                }
                257 => self.ppu.eval_sprites(),
                321 => self.load_sprites(),
                _ => {}
            };
            match self.ppu.frame.dot {
                2..=255 | 322..=337 => {
                    self.pixel();
                    match self.ppu.frame.dot % 8 {
                        1 => {
                            self.ppu.addr = self.ppu.nt_addr();
                            self.ppu.reload_shift();
                        }
                        2 => {
                            self.ppu.background_shift_register.nt =
                                self.read_ppu_bus(self.ppu.addr);
                        }
                        3 => {
                            self.ppu.addr = self.ppu.at_addr();
                        }
                        4 => {
                            self.ppu.background_shift_register.at =
                                self.read_ppu_bus(self.ppu.addr);
                            if (self.ppu.loopy.v_addr.c_y & 2).as_bool() {
                                self.ppu.background_shift_register.at >>= 4;
                            }
                            if (self.ppu.loopy.v_addr.c_x & 2).as_bool() {
                                self.ppu.background_shift_register.at >>= 2;
                            }
                        }
                        5 => {
                            self.ppu.addr = self.ppu.bg_addr();
                        }
                        6 => {
                            self.ppu.background_shift_register.bg_l =
                                self.read_ppu_bus(self.ppu.addr);
                        }
                        7 => {
                            self.ppu.addr += 8;
                        }
                        0 => {
                            self.ppu.background_shift_register.bg_h =
                                self.read_ppu_bus(self.ppu.addr);
                            self.ppu.h_scroll();
                        }
                        _ => {}
                    }
                }
                256 => {
                    self.pixel();
                    self.ppu.background_shift_register.bg_h = self.read_ppu_bus(self.ppu.addr);
                    self.ppu.v_scroll();
                }
                257 => {
                    self.pixel();
                    self.ppu.reload_shift();
                    self.ppu.h_update();
                }
                280..=304 => {
                    if mode == ScanlineMode::PRE {
                        self.ppu.v_update();
                    }
                }
                1 => {
                    self.ppu.addr = self.ppu.nt_addr();
                    if mode == ScanlineMode::PRE {
                        self.ppu.register.PPU_STATUS.vblank = false;
                    }
                }
                321 | 339 => {
                    self.ppu.addr = self.ppu.nt_addr();
                }
                338 => {
                    self.ppu.background_shift_register.nt = self.read_ppu_bus(self.ppu.addr);
                }
                340 => {
                    self.ppu.background_shift_register.nt = self.read_ppu_bus(self.ppu.addr);
                    if mode == ScanlineMode::PRE && self.ppu.is_rendering() && self.ppu.frame.is_odd
                    {
                        self.ppu.frame.dot += 1;
                    }
                }
                _ => {}
            };
            // if self.ppu.frame.dot == 260 && self.is_rendering() {
            // scanline
            // }
        }
    }

    pub fn ppu_step(&mut self) {
        match self.ppu.frame.scanline {
            0..=239 => self.scanline_cycle(ScanlineMode::VISIBLE),
            240 => self.scanline_cycle(ScanlineMode::POST),
            241 => self.scanline_cycle(ScanlineMode::NMI),
            261 => self.scanline_cycle(ScanlineMode::PRE),
            _ => {}
        };
        self.ppu.frame.dot += 1;
        if self.ppu.frame.dot > 340 {
            self.ppu.frame.dot %= 341;
            self.ppu.frame.scanline += 1;
            if self.ppu.frame.scanline > 261 {
                self.ppu.frame.scanline = 0;
                self.ppu.frame.is_odd = !self.ppu.frame.is_odd;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _bitwise_not() {
        assert_eq!(!(0x10 as u16), 0b1111_1111_1110_1111);
    }
}
