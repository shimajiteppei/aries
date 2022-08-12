use crate::util::bit::{AsU8, PartialBit};

#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Register {
    /// Accumulator
    pub A: u8,
    /// Index register X
    pub X: u8,
    /// Index register Y
    pub Y: u8,
    /// Stack Pointer
    pub S: u8,
    /// Program Counter
    pub PC: u16,
    /// Status register
    pub P: StatusRegister,
}

/// 6502 cpu status register
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct StatusRegister {
    /// negative flag (1 when result is negative)
    pub N: bool,
    /// overflow flag (1 on signed overflow)
    pub V: bool,
    /// reserved flag (always 1)
    pub R: bool,
    /// break flag (1 when interupt was caused by a BRK)
    pub B: bool,
    /// decimal flag (1 when CPU in BCD mode)
    pub D: bool,
    /// IRQ flag (when 1, no interupts will occur (exceptions are IRQs forced by BRK and NMIs))
    pub I: bool,
    /// zero flag (1 when all bits of a result are 0)
    pub Z: bool,
    /// carry flag (1 on unsigned overflow)
    pub C: bool,
}

impl Default for StatusRegister {
    fn default() -> Self {
        Self {
            N: false,
            V: false,
            R: true,
            B: false,
            D: false,
            I: false,
            Z: false,
            C: false,
        }
    }
}

impl StatusRegister {
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn get_u8(&self) -> u8 {
        (self.N.as_u8() << 7)
            | (self.V.as_u8() << 6)
            | (self.R.as_u8() << 5)
            | (self.B.as_u8() << 4)
            | (self.D.as_u8() << 3)
            | (self.I.as_u8() << 2)
            | (self.Z.as_u8() << 1)
            | self.C.as_u8()
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn set_u8(&mut self, value: u8) {
        self.N = value.bit_flag(7);
        self.V = value.bit_flag(6);
        self.R = value.bit_flag(5);
        self.B = value.bit_flag(4);
        self.D = value.bit_flag(3);
        self.I = value.bit_flag(2);
        self.Z = value.bit_flag(1);
        self.C = value.bit_flag(0);
    }
}

/// 2kiB WRAM
pub type WRam = [u8; 0x800];

#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct Control {
    pub RST: bool,
    pub NMI: bool,
    pub IRQ: bool,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq)]
pub enum InterruptionType {
    NMI,
    RESET,
    IRQ,
    BRK,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _enum_equal_operator() {
        let e = InterruptionType::BRK;
        assert_eq!(true, e == InterruptionType::BRK);
        assert_eq!(false, e != InterruptionType::BRK);
    }
}
