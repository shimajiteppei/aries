use crate::{
    entity::cpu::{Control, InterruptionType, Register, WRam},
    util::bit::{get_little_endian, AsU8, Zero},
};

use super::nes::NesState;

const TOTAL_CYCLES: u16 = 29781;

#[derive(Debug)]
pub struct CpuState {
    pub register: Register,
    pub wram: WRam,
    pub control: Control,
    pub remaining_cycles: i32,
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            register: Register::default(),
            wram: [0xFF; 0x800],
            control: Control::default(),
            remaining_cycles: 0,
        }
    }
}

impl CpuState {
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_irq(&mut self) {
        self.control.IRQ = true;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn update_CV(&mut self, x: u8, y: u8, res: i16) {
        self.register.P.C = res > 0xFF;
        self.register.P.V = (!(x ^ y) & (x as i16 ^ res) as u8 & 0x80).as_bool();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn update_NZ(&mut self, res: u8) {
        self.register.P.N = (res & 0x80).as_bool();
        self.register.P.Z = !res.as_bool()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn cross_page(&mut self, addr: u16, delta: u8) -> bool {
        (addr.wrapping_add(delta as u16) & 0xFF00) != (addr & 0xFF00)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn cross_page_i8(&mut self, addr: u16, delta: i8) -> bool {
        ((addr as i32).wrapping_add(delta as i32) & 0xFF00) as u16 != (addr & 0xFF00)
    }
}

impl NesState {
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn read_cpu_bus(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu.wram[(addr % 0x800) as usize],
            0x2000..=0x3FFF => self.read_ppu(addr),
            0x4000..=0x4013 | 0x4015 => 0xFF,
            0x4017 => self.read_joypad_state(true),
            0x4014 => 0,
            0x4016 => self.read_joypad_state(false),
            0x4018..=0xFFFF => self.cartridge.read_prg(addr),
        }
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn write_cpu_bus(&mut self, addr: u16, value: u8) -> u8 {
        match addr {
            0x0000..=0x1FFF => {
                self.cpu.wram[(addr % 0x800) as usize] = value;
                value
            }
            0x2000..=0x3FFF => self.write_ppu(addr, value),
            0x4000..=0x4013 | 0x4015 => 0xFF,
            0x4017 => todo!(),
            0x4014 => {
                self.dma_oam(value);
                0
            }
            0x4016 => {
                self.write_joypad_strobe((value & 1).as_bool());
                0
            }
            0x4018..=0xFFFF => self.cartridge.write_prg(addr, value),
        }
    }

    fn tick(&mut self) {
        self.ppu_step();
        self.ppu_step();
        self.ppu_step();
        self.cpu.remaining_cycles -= 1;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn read_cpu(&mut self, addr: u16) -> u8 {
        self.tick();
        self.read_cpu_bus(addr)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn write_cpu(&mut self, addr: u16, value: u8) -> u8 {
        self.tick();
        self.write_cpu_bus(addr, value)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn read16_little_endian(&mut self, addr1: u16, addr2: u16) -> u16 {
        get_little_endian(self.read_cpu(addr1), self.read_cpu(addr2))
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn read16(&mut self, addr: u16) -> u16 {
        self.read16_little_endian(addr, addr + 1)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn push(&mut self, val: u8) -> u8 {
        let value = self.write_cpu(0x100 + self.cpu.register.S as u16, val);
        self.cpu.register.S -= 1;
        value
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn pop(&mut self) -> u8 {
        self.cpu.register.S += 1;
        self.read_cpu(0x100 + self.cpu.register.S as u16)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn dma_oam(&mut self, bank: u8) {
        for i in 0..256 {
            let value = self.read_cpu((bank as u16) * 0x100 + (i as u16));
            self.write_cpu(0x2014, value);
        }
    }

    /// Addressing Modes
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn imm(&mut self) -> u16 {
        let pc = self.cpu.register.PC;
        self.cpu.register.PC += 1;
        pc
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn imm16(&mut self) -> u16 {
        let pc = self.cpu.register.PC;
        self.cpu.register.PC += 2;
        pc
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn abs(&mut self) -> u16 {
        let addr = self.imm16();
        self.read16(addr)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn _abx(&mut self) -> u16 {
        self.tick();
        self.abs() + self.cpu.register.X as u16
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn abx(&mut self) -> u16 {
        let addr = self.abs();
        if self.cpu.cross_page(addr, self.cpu.register.X) {
            self.tick();
        }
        addr.wrapping_add(self.cpu.register.X as u16)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn aby(&mut self) -> u16 {
        let addr = self.abs();
        if self.cpu.cross_page(addr, self.cpu.register.Y) {
            self.tick();
        }
        addr.wrapping_add(self.cpu.register.Y as u16)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn zp(&mut self) -> u16 {
        let addr = self.imm();
        self.read_cpu(addr) as u16
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn zpx(&mut self) -> u16 {
        self.tick();
        (self.zp() + self.cpu.register.X as u16) & 0xFF
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn zpy(&mut self) -> u16 {
        self.tick();
        (self.zp() + self.cpu.register.Y as u16) & 0xFF
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn izx(&mut self) -> u16 {
        let addr = self.zpx();
        self.read16_little_endian(addr, (addr + 1) & 0xFF)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn _izy(&mut self) -> u16 {
        let addr = self.zp();
        self.read16_little_endian(addr, (addr + 1) & 0xFF)
            .wrapping_add(self.cpu.register.Y as u16)
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn izy(&mut self) -> u16 {
        let addr = self._izy();
        if self.cpu.cross_page(
            addr.wrapping_sub(self.cpu.register.Y as u16),
            self.cpu.register.Y,
        ) {
            self.tick();
        }
        addr
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn st(&mut self, val_fn: fn(&mut Self) -> u8, addr_mode_fn: fn(&mut Self) -> u16) {
        let addr = addr_mode_fn(self);
        let val = val_fn(self);
        self.write_cpu(addr, val);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn st_A_izy(&mut self) {
        self.tick();
        let addr = self._izy();
        self.write_cpu(addr, self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn st_A_abx(&mut self) {
        self.tick();
        let addr = self.abs() + self.cpu.register.X as u16;
        self.write_cpu(addr, self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn st_A_aby(&mut self) {
        self.tick();
        let addr = self.abs() + self.cpu.register.Y as u16;
        self.write_cpu(addr, self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn G(&mut self, addr_fn: fn(&mut Self) -> u16) -> (u16, u8) {
        let addr = addr_fn(self);
        let val = self.read_cpu(addr);
        (addr, val)
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_A(&mut self) -> u8 {
        self.cpu.register.A
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_A(&mut self, val: u8) {
        self.cpu.register.A = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_X(&mut self) -> u8 {
        self.cpu.register.X
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_AX(&mut self) -> u8 {
        self.cpu.register.A & self.cpu.register.X
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_X(&mut self, val: u8) {
        self.cpu.register.X = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_Y(&mut self) -> u8 {
        self.cpu.register.Y
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_Y(&mut self, val: u8) {
        self.cpu.register.Y = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_S(&mut self) -> u8 {
        self.cpu.register.S
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_S(&mut self, val: u8) {
        self.cpu.register.S = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_P_N(&mut self) -> bool {
        self.cpu.register.P.N
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_P_N(&mut self, val: bool) {
        self.cpu.register.P.N = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_P_V(&mut self) -> bool {
        self.cpu.register.P.V
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_P_V(&mut self, val: bool) {
        self.cpu.register.P.V = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_P_B(&mut self) -> bool {
        self.cpu.register.P.B
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_P_B(&mut self, val: bool) {
        self.cpu.register.P.B = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_P_D(&mut self) -> bool {
        self.cpu.register.P.D
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_P_D(&mut self, val: bool) {
        self.cpu.register.P.D = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_P_I(&mut self) -> bool {
        self.cpu.register.P.I
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_P_I(&mut self, val: bool) {
        self.cpu.register.P.I = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_P_Z(&mut self) -> bool {
        self.cpu.register.P.Z
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_P_Z(&mut self, val: bool) {
        self.cpu.register.P.Z = val;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_P_C(&mut self) -> bool {
        self.cpu.register.P.C
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_P_C(&mut self, val: bool) {
        self.cpu.register.P.C = val;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ld(&mut self, addr_fn: fn(&mut Self) -> u16, register_setter: fn(&mut Self, u8)) {
        let (_, val) = self.G(addr_fn);
        register_setter(self, val);
        self.cpu.update_NZ(val);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn _cmp(&mut self, val: u8, register_getter: fn(&mut Self) -> u8) {
        let reg = register_getter(self);
        self.cpu.update_NZ(reg.wrapping_sub(val));
        self.cpu.register.P.C = reg >= val;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn cmp(&mut self, addr_fn: fn(&mut Self) -> u16, register_getter: fn(&mut Self) -> u8) {
        let (_, val) = self.G(addr_fn);
        self._cmp(val, register_getter);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn _ADC(&mut self, val: u8) {
        let res = self.cpu.register.A as i16 + val as i16 + self.cpu.register.P.C as i16;
        self.cpu.update_CV(self.cpu.register.A, val, res);
        let res_u8 = res as u8;
        self.cpu.register.A = res_u8;
        self.cpu.update_NZ(res_u8);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ADC(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (_, val) = self.G(addr_fn);
        self._ADC(val);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn SBC(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (_, val) = self.G(addr_fn);
        self._ADC(val ^ 0xFF);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn BIT(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (_, val) = self.G(addr_fn);
        self.cpu.register.P.Z = !(self.cpu.register.A & val).as_bool();
        self.cpu.register.P.N = (val & 0x80).as_bool();
        self.cpu.register.P.V = (val & 0x40).as_bool();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn AND(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (_, val) = self.G(addr_fn);
        self.cpu.register.A &= val;
        self.cpu.update_NZ(self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn XOR(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (_, val) = self.G(addr_fn);
        self.cpu.register.A ^= val;
        self.cpu.update_NZ(self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn OR(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (_, val) = self.G(addr_fn);
        self.cpu.register.A |= val;
        self.cpu.update_NZ(self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ASL(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (addr, val) = self.G(addr_fn);
        self.cpu.register.P.C = (val & 0x80).as_bool();
        self.tick();
        let res = self.write_cpu(addr, val << 1);
        self.cpu.update_NZ(res);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn LSR(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (addr, val) = self.G(addr_fn);
        self.cpu.register.P.C = (val & 0x01).as_bool();
        self.tick();
        let res = self.write_cpu(addr, val >> 1);
        self.cpu.update_NZ(res);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ROL(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (addr, val) = self.G(addr_fn);
        let c = self.cpu.register.P.C.as_u8();
        self.cpu.register.P.C = (val & 0x80).as_bool();
        self.tick();
        let res = self.write_cpu(addr, (val << 1) | c);
        self.cpu.update_NZ(res);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ROR(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (addr, val) = self.G(addr_fn);
        let c = self.cpu.register.P.C.as_u8() << 7;
        self.cpu.register.P.C = (val & 0x01).as_bool();
        self.tick();
        let res = self.write_cpu(addr, (val >> 1) | c);
        self.cpu.update_NZ(res);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn DEC(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (addr, val) = self.G(addr_fn);
        self.tick();
        let res = self.write_cpu(addr, val.wrapping_sub(1));
        self.cpu.update_NZ(res);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn INC(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (addr, val) = self.G(addr_fn);
        self.tick();
        let res = self.write_cpu(addr, val.wrapping_add(1));
        self.cpu.update_NZ(res);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn dec(&mut self, register_getter: fn(&mut Self) -> u8, register_setter: fn(&mut Self, u8)) {
        let res = register_getter(self).wrapping_sub(1);
        register_setter(self, res);
        self.cpu.update_NZ(res);
        self.tick();
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn inc(&mut self, register_getter: fn(&mut Self) -> u8, register_setter: fn(&mut Self, u8)) {
        let res = register_getter(self).wrapping_add(1);
        register_setter(self, res);
        self.cpu.update_NZ(res);
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ASL_A(&mut self) {
        self.cpu.register.P.C = (self.cpu.register.A & 0x80).as_bool();
        self.cpu.register.A <<= 1;
        self.cpu.update_NZ(self.cpu.register.A);
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn LSR_A(&mut self) {
        self.cpu.register.P.C = (self.cpu.register.A & 0x01).as_bool();
        self.cpu.register.A >>= 1;
        self.cpu.update_NZ(self.cpu.register.A);
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ROL_A(&mut self) {
        let c = self.cpu.register.P.C.as_u8();
        self.cpu.register.P.C = (self.cpu.register.A & 0x80).as_bool();
        self.cpu.register.A = (self.cpu.register.A << 1) | c;
        self.cpu.update_NZ(self.cpu.register.A);
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ROR_A(&mut self) {
        let c = self.cpu.register.P.C.as_u8() << 7;
        self.cpu.register.P.C = (self.cpu.register.A & 0x01).as_bool();
        self.cpu.register.A = (self.cpu.register.A >> 1) | c;
        self.cpu.update_NZ(self.cpu.register.A);
        self.tick();
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn tr(&mut self, register_getter: fn(&mut Self) -> u8, register_setter: fn(&mut Self, u8)) {
        let res = register_getter(self);
        register_setter(self, res);
        self.cpu.update_NZ(res);
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn tr_X_S(&mut self) {
        self.cpu.register.S = self.cpu.register.X;
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn PLP(&mut self) {
        self.tick();
        self.tick();
        let val = self.pop();
        self.cpu.register.P.set_u8(val & 0b1110_1111);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn PHP(&mut self) {
        self.tick();
        let val = self.cpu.register.P.get_u8() | 0b0011_0000;
        self.push(val);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn PLA(&mut self) {
        self.tick();
        self.tick();
        let val = self.pop();
        self.cpu.register.A = val;
        self.cpu.update_NZ(self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn PHA(&mut self) {
        self.tick();
        self.push(self.cpu.register.A);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn br(&mut self, status_flag_getter: fn(&mut Self) -> bool, val: bool) {
        let addr = self.imm();
        let imm = self.read_cpu(addr) as i8;
        if status_flag_getter(self) == val {
            if self.cpu.cross_page_i8(self.cpu.register.PC, imm) {
                self.tick();
            }
            self.tick();
            if imm >= 0 {
                self.cpu.register.PC += imm as u16;
            } else {
                self.cpu.register.PC -= (-imm) as u16;
            }
        }
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn JMP_IND(&mut self) {
        let addr = self.imm16();
        let imm = self.read16(addr);
        self.cpu.register.PC = self.read16_little_endian(imm, (imm & 0xFF00) | ((imm + 1) & 0xFF));
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn JMP(&mut self) {
        let addr = self.imm16();
        let imm = self.read16(addr);
        self.cpu.register.PC = imm;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn JSR(&mut self) {
        let pc = self.cpu.register.PC + 1;
        self.push((pc >> 8) as u8);
        self.push(pc as u8);
        let addr = self.imm16();
        let imm = self.read16(addr);
        self.cpu.register.PC = imm;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn RTS(&mut self) {
        self.tick();
        self.tick();
        let addr_l = self.pop() as u16;
        let addr_h = self.pop() as u16;
        self.cpu.register.PC = ((addr_h << 8) | addr_l) + 1;
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn RTI(&mut self) {
        self.PLP();
        let addr_l = self.pop() as u16;
        let addr_h = self.pop() as u16;
        self.cpu.register.PC = (addr_h << 8) | addr_l;
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn flag(&mut self, status_flag_setter: fn(&mut Self, bool), val: bool) {
        status_flag_setter(self, val);
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn INT(&mut self, t: InterruptionType) {
        self.tick();
        if t != InterruptionType::BRK {
            self.tick();
        }
        if t != InterruptionType::RESET {
            let pc = self.cpu.register.PC;
            self.push((pc >> 8) as u8);
            self.push(pc as u8);
            let p = self.cpu.register.P.get_u8();
            self.push(p | ((t == InterruptionType::BRK).as_u8() << 4));
        } else {
            self.cpu.register.S = self.cpu.register.S.wrapping_sub(3);
            self.tick();
            self.tick();
            self.tick();
        }
        self.cpu.register.P.I = true;
        let addr = match t {
            InterruptionType::NMI => 0xFFFA,
            InterruptionType::RESET => 0xFFFC,
            InterruptionType::IRQ => 0xFFFE,
            InterruptionType::BRK => 0xFFFE,
        };
        self.cpu.register.PC = self.read16(addr);
        if t == InterruptionType::NMI {
            self.cpu.control.NMI = false;
        }
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn NOP(&mut self) {
        self.tick();
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn nop(&mut self, addr_fn: fn(&mut Self) -> u16) {
        self.G(addr_fn);
        self.tick();
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn SLO(&mut self, addr_fn: fn(&mut Self) -> u16) {
        // ASL + OR
        let (addr, val) = self.G(addr_fn);
        self.cpu.register.P.C = (val & 0x80).as_bool();
        let res = val << 1;
        self.tick();
        self.write_cpu(addr, res);
        self.cpu.register.A |= res;
        self.cpu.update_NZ(self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn RLA(&mut self, addr_fn: fn(&mut Self) -> u16) {
        // ROL + AND
        let (addr, val) = self.G(addr_fn);
        let c = self.cpu.register.P.C.as_u8();
        self.cpu.register.P.C = (val & 0x80).as_bool();
        let res = (val << 1) | c;
        self.tick();
        self.write_cpu(addr, res);
        self.cpu.register.A &= res;
        self.cpu.update_NZ(self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn SRE(&mut self, addr_fn: fn(&mut Self) -> u16) {
        // LSR + OR
        let (addr, val) = self.G(addr_fn);
        self.cpu.register.P.C = (val & 0x01).as_bool();
        let res = val >> 1;
        self.tick();
        self.write_cpu(addr, res);
        self.cpu.register.A ^= res;
        self.cpu.update_NZ(self.cpu.register.A);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn RRA(&mut self, addr_fn: fn(&mut Self) -> u16) {
        // ROR + ADC
        let (addr, val) = self.G(addr_fn);
        let c = self.cpu.register.P.C.as_u8() << 7;
        self.cpu.register.P.C = (val & 0x01).as_bool();
        let res = (val >> 1) | c;
        self.tick();
        self.write_cpu(addr, res);
        let res_a = self.cpu.register.A as i16 + res as i16 + self.cpu.register.P.C as i16;
        self.cpu.register.A = res_a as u8;
        self.cpu.update_CV(self.cpu.register.A, res, res_a);
        self.cpu.update_NZ(res_a as u8);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn SAX(&mut self, addr_fn: fn(&mut Self) -> u16) {
        self.st(Self::get_AX, addr_fn);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn LAX(&mut self, addr_fn: fn(&mut Self) -> u16) {
        let (_, val) = self.G(addr_fn);
        self.set_A(val);
        self.set_X(val);
        self.cpu.update_NZ(val);
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn DCP(&mut self, addr_fn: fn(&mut Self) -> u16) {
        // DEC + CMP
        let (addr, val) = self.G(addr_fn);
        let res = val.wrapping_sub(1);
        self.tick();
        self.write_cpu(addr, res);
        self.cpu.update_NZ(self.cpu.register.A.wrapping_sub(res));
        self.cpu.register.P.C = self.cpu.register.A >= res;
    }

    #[allow(non_snake_case)]
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn ISC(&mut self, addr_fn: fn(&mut Self) -> u16) {
        // INC + SBC
        let (addr, val) = self.G(addr_fn);
        let res = val.wrapping_add(1);
        self.tick();
        self.write_cpu(addr, res);
        self.cpu.update_NZ(res);
        let res = res ^ 0xFF;
        let res_a = self.cpu.register.A as i16 + res as i16 + self.cpu.register.P.C as i16;
        self.cpu.register.A = res_a as u8;
        self.cpu.update_CV(self.cpu.register.A, res, res_a);
        self.cpu.update_NZ(res_a as u8);
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    fn exec(&mut self) {
        let pc = self.cpu.register.PC;
        let val = self.read_cpu(pc);
        self.cpu.register.PC += 1;
        // println!(
        //     "PC:{:04X} OP:{:02X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} S:{:02X}",
        //     pc,
        //     val,
        //     self.cpu.register.A,
        //     self.cpu.register.X,
        //     self.cpu.register.Y,
        //     self.cpu.register.P.get_u8(),
        //     self.cpu.register.S
        // );
        match val {
            0x00 => self.INT(InterruptionType::BRK),
            0x01 => self.OR(Self::izx),
            0x03 => self.SLO(Self::izx),
            0x04 => self.nop(Self::zp),
            0x05 => self.OR(Self::zp),
            0x06 => self.ASL(Self::zp),
            0x07 => self.SLO(Self::zp),
            0x08 => self.PHP(),
            0x09 => self.OR(Self::imm),
            0x0A => self.ASL_A(),
            0x0C => self.nop(Self::abs),
            0x0D => self.OR(Self::abs),
            0x0E => self.ASL(Self::abs),
            0x0F => self.SLO(Self::abs),
            0x10 => self.br(Self::get_P_N, false),
            0x11 => self.OR(Self::izy),
            0x13 => self.SLO(Self::izy),
            0x14 => self.nop(Self::zpx),
            0x15 => self.OR(Self::zpx),
            0x17 => self.SLO(Self::zpx),
            0x16 => self.ASL(Self::zpx),
            0x18 => self.flag(Self::set_P_C, false),
            0x19 => self.OR(Self::aby),
            0x1A => self.NOP(),
            0x1B => self.SLO(Self::aby),
            0x1C => self.nop(Self::abx),
            0x1D => self.OR(Self::abx),
            0x1E => self.ASL(Self::_abx),
            0x1F => self.SLO(Self::abx),
            0x20 => self.JSR(),
            0x21 => self.AND(Self::izx),
            0x23 => self.RLA(Self::izx),
            0x24 => self.BIT(Self::zp),
            0x25 => self.AND(Self::zp),
            0x26 => self.ROL(Self::zp),
            0x27 => self.RLA(Self::zp),
            0x28 => self.PLP(),
            0x29 => self.AND(Self::imm),
            0x2A => self.ROL_A(),
            0x2C => self.BIT(Self::abs),
            0x2D => self.AND(Self::abs),
            0x2E => self.ROL(Self::abs),
            0x2F => self.RLA(Self::abs),
            0x30 => self.br(Self::get_P_N, true),
            0x31 => self.AND(Self::izy),
            0x33 => self.RLA(Self::izy),
            0x34 => self.nop(Self::zpx),
            0x35 => self.AND(Self::zpx),
            0x36 => self.ROL(Self::zpx),
            0x37 => self.RLA(Self::zpx),
            0x38 => self.flag(Self::set_P_C, true),
            0x39 => self.AND(Self::aby),
            0x3A => self.NOP(),
            0x3B => self.RLA(Self::aby),
            0x3C => self.nop(Self::abx),
            0x3D => self.AND(Self::abx),
            0x3E => self.ROL(Self::_abx),
            0x3F => self.RLA(Self::abx),
            0x40 => self.RTI(),
            0x41 => self.XOR(Self::izx),
            0x43 => self.SRE(Self::izx),
            0x44 => self.nop(Self::zp),
            0x45 => self.XOR(Self::zp),
            0x46 => self.LSR(Self::zp),
            0x47 => self.SRE(Self::zp),
            0x48 => self.PHA(),
            0x49 => self.XOR(Self::imm),
            0x4A => self.LSR_A(),
            0x4C => self.JMP(),
            0x4D => self.XOR(Self::abs),
            0x4E => self.LSR(Self::abs),
            0x4F => self.SRE(Self::abs),
            0x50 => self.br(Self::get_P_V, false),
            0x51 => self.XOR(Self::izy),
            0x53 => self.SRE(Self::izy),
            0x54 => self.nop(Self::zpx),
            0x55 => self.XOR(Self::zpx),
            0x56 => self.LSR(Self::zpx),
            0x57 => self.SRE(Self::zpx),
            0x58 => self.flag(Self::set_P_I, false),
            0x59 => self.XOR(Self::aby),
            0x5A => self.NOP(),
            0x5B => self.SRE(Self::aby),
            0x5C => self.nop(Self::abx),
            0x5D => self.XOR(Self::abx),
            0x5E => self.LSR(Self::_abx),
            0x5F => self.SRE(Self::abx),
            0x60 => self.RTS(),
            0x61 => self.ADC(Self::izx),
            0x63 => self.RRA(Self::izx),
            0x64 => self.nop(Self::zp),
            0x65 => self.ADC(Self::zp),
            0x66 => self.ROR(Self::zp),
            0x67 => self.RRA(Self::zp),
            0x68 => self.PLA(),
            0x69 => self.ADC(Self::imm),
            0x6A => self.ROR_A(),
            0x6C => self.JMP_IND(),
            0x6D => self.ADC(Self::abs),
            0x6E => self.ROR(Self::abs),
            0x6F => self.RRA(Self::abs),
            0x70 => self.br(Self::get_P_V, true),
            0x71 => self.ADC(Self::izy),
            0x73 => self.RRA(Self::izy),
            0x74 => self.nop(Self::zpx),
            0x75 => self.ADC(Self::zpx),
            0x76 => self.ROR(Self::zpx),
            0x77 => self.RRA(Self::zpx),
            0x78 => self.flag(Self::set_P_I, true),
            0x79 => self.ADC(Self::aby),
            0x7A => self.NOP(),
            0x7B => self.RRA(Self::aby),
            0x7C => self.nop(Self::abx),
            0x7D => self.ADC(Self::abx),
            0x7E => self.ROR(Self::_abx),
            0x7F => self.RRA(Self::abx),
            0x80 => self.nop(Self::imm),
            0x81 => self.st(Self::get_A, Self::izx),
            0x82 => self.nop(Self::imm),
            0x83 => self.SAX(Self::izx),
            0x84 => self.st(Self::get_Y, Self::zp),
            0x85 => self.st(Self::get_A, Self::zp),
            0x86 => self.st(Self::get_X, Self::zp),
            0x87 => self.SAX(Self::zp),
            0x88 => self.dec(Self::get_Y, Self::set_Y),
            0x89 => self.nop(Self::imm),
            0x8A => self.tr(Self::get_X, Self::set_A),
            0x8C => self.st(Self::get_Y, Self::abs),
            0x8D => self.st(Self::get_A, Self::abs),
            0x8E => self.st(Self::get_X, Self::abs),
            0x8F => self.SAX(Self::abs),
            0x90 => self.br(Self::get_P_C, false),
            0x91 => self.st_A_izy(),
            0x94 => self.st(Self::get_Y, Self::zpx),
            0x95 => self.st(Self::get_A, Self::zpx),
            0x96 => self.st(Self::get_X, Self::zpy),
            0x97 => self.SAX(Self::zpy),
            0x98 => self.tr(Self::get_Y, Self::set_A),
            0x99 => self.st_A_aby(),
            0x9A => self.tr_X_S(),
            0x9D => self.st_A_abx(),
            0xA0 => self.ld(Self::imm, Self::set_Y),
            0xA1 => self.ld(Self::izx, Self::set_A),
            0xA2 => self.ld(Self::imm, Self::set_X),
            0xA3 => self.LAX(Self::izx),
            0xA4 => self.ld(Self::zp, Self::set_Y),
            0xA5 => self.ld(Self::zp, Self::set_A),
            0xA6 => self.ld(Self::zp, Self::set_X),
            0xA7 => self.LAX(Self::zp),
            0xA8 => self.tr(Self::get_A, Self::set_Y),
            0xA9 => self.ld(Self::imm, Self::set_A),
            0xAA => self.tr(Self::get_A, Self::set_X),
            0xAB => self.LAX(Self::imm),
            0xAC => self.ld(Self::abs, Self::set_Y),
            0xAD => self.ld(Self::abs, Self::set_A),
            0xAE => self.ld(Self::abs, Self::set_X),
            0xAF => self.LAX(Self::abs),
            0xB0 => self.br(Self::get_P_C, true),
            0xB1 => self.ld(Self::izy, Self::set_A),
            0xB3 => self.LAX(Self::izy),
            0xB4 => self.ld(Self::zpx, Self::set_Y),
            0xB5 => self.ld(Self::zpx, Self::set_A),
            0xB6 => self.ld(Self::zpy, Self::set_X),
            0xB7 => self.LAX(Self::zpy),
            0xB8 => self.flag(Self::set_P_V, false),
            0xB9 => self.ld(Self::aby, Self::set_A),
            0xBA => self.tr(Self::get_S, Self::set_X),
            0xBC => self.ld(Self::abx, Self::set_Y),
            0xBD => self.ld(Self::abx, Self::set_A),
            0xBE => self.ld(Self::aby, Self::set_X),
            0xBF => self.LAX(Self::aby),
            0xC0 => self.cmp(Self::imm, Self::get_Y),
            0xC1 => self.cmp(Self::izx, Self::get_A),
            0xC2 => self.nop(Self::imm),
            0xC3 => self.DCP(Self::izx),
            0xC4 => self.cmp(Self::zp, Self::get_Y),
            0xC5 => self.cmp(Self::zp, Self::get_A),
            0xC6 => self.DEC(Self::zp),
            0xC7 => self.DCP(Self::zp),
            0xC8 => self.inc(Self::get_Y, Self::set_Y),
            0xC9 => self.cmp(Self::imm, Self::get_A),
            0xCA => self.dec(Self::get_X, Self::set_X),
            0xCC => self.cmp(Self::abs, Self::get_Y),
            0xCD => self.cmp(Self::abs, Self::get_A),
            0xCE => self.DEC(Self::abs),
            0xCF => self.DCP(Self::abs),
            0xD0 => self.br(Self::get_P_Z, false),
            0xD1 => self.cmp(Self::izy, Self::get_A),
            0xD3 => self.DCP(Self::izy),
            0xD4 => self.nop(Self::zpx),
            0xD5 => self.cmp(Self::zpx, Self::get_A),
            0xD6 => self.DEC(Self::zpx),
            0xD7 => self.DCP(Self::zpx),
            0xD8 => self.flag(Self::set_P_D, false),
            0xD9 => self.cmp(Self::aby, Self::get_A),
            0xDA => self.NOP(),
            0xDB => self.DCP(Self::aby),
            0xDC => self.nop(Self::abx),
            0xDD => self.cmp(Self::abx, Self::get_A),
            0xDE => self.DEC(Self::_abx),
            0xDF => self.DCP(Self::abx),
            0xE0 => self.cmp(Self::imm, Self::get_X),
            0xE1 => self.SBC(Self::izx),
            0xE2 => self.nop(Self::imm),
            0xE3 => self.ISC(Self::izx),
            0xE4 => self.cmp(Self::zp, Self::get_X),
            0xE5 => self.SBC(Self::zp),
            0xE6 => self.INC(Self::zp),
            0xE7 => self.ISC(Self::zp),
            0xE8 => self.inc(Self::get_X, Self::set_X),
            0xE9 => self.SBC(Self::imm),
            0xEA => self.NOP(),
            0xEB => self.SBC(Self::imm),
            0xEC => self.cmp(Self::abs, Self::get_X),
            0xED => self.SBC(Self::abs),
            0xEE => self.INC(Self::abs),
            0xEF => self.ISC(Self::abs),
            0xF0 => self.br(Self::get_P_Z, true),
            0xF1 => self.SBC(Self::izy),
            0xF3 => self.ISC(Self::izy),
            0xF4 => self.nop(Self::zpx),
            0xF5 => self.SBC(Self::zpx),
            0xF6 => self.INC(Self::zpx),
            0xF7 => self.ISC(Self::zpx),
            0xF8 => self.flag(Self::set_P_D, true),
            0xF9 => self.SBC(Self::aby),
            0xFA => self.NOP(),
            0xFB => self.ISC(Self::aby),
            0xFC => self.nop(Self::abx),
            0xFD => self.SBC(Self::abx),
            0xFE => self.INC(Self::_abx),
            0xFF => self.ISC(Self::abx),
            _ => panic!("{:?}", (val, &self.cpu.register)),
        }
    }

    pub fn power(&mut self) {
        // self.cpu.register.P.B = true;
        self.INT(InterruptionType::RESET);
    }

    pub fn run_frame(&mut self) {
        self.cpu.remaining_cycles += TOTAL_CYCLES as i32;

        while self.cpu.remaining_cycles > 0 {
            if self.cpu.control.NMI {
                self.INT(InterruptionType::NMI)
            } else if self.cpu.control.IRQ && !self.cpu.register.P.I {
                self.INT(InterruptionType::IRQ)
            }
            self.exec();
        }
        // apu::runframe
    }
}
