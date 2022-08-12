use core::ops::Range;

pub trait Zero: Sized {
    fn is_zero(&self) -> bool;

    #[inline(always)]
    fn as_bool(&self) -> bool {
        !self.is_zero()
    }
}

impl Zero for u8 {
    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl Zero for u16 {
    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

pub trait PartialBit<T: Zero>: Sized {
    fn partial_bit(self, range: Range<u8>) -> T;

    #[inline(always)]
    fn bit(self, n: u8) -> T {
        self.partial_bit(n..n + 1)
    }

    #[inline(always)]
    fn bit_flag(self, n: u8) -> bool {
        self.bit(n).as_bool()
    }
}

impl PartialBit<u8> for u8 {
    #[inline(always)]
    fn partial_bit(self, range: Range<u8>) -> u8 {
        (self >> range.start) & ((1 << (range.end - range.start)) - 1)
    }
}

impl PartialBit<u16> for u16 {
    #[inline(always)]
    fn partial_bit(self, range: Range<u8>) -> u16 {
        (self >> range.start) & ((1 << (range.end - range.start)) - 1)
    }
}

pub trait AsU8 {
    fn as_u8(&self) -> u8;
}

impl AsU8 for bool {
    #[inline(always)]
    fn as_u8(&self) -> u8 {
        if *self {
            1
        } else {
            0
        }
    }
}

pub trait AsU16 {
    fn as_u16(&self) -> u16;
}

impl AsU16 for bool {
    #[inline(always)]
    fn as_u16(&self) -> u16 {
        if *self {
            1
        } else {
            0
        }
    }
}

#[inline(always)]
pub fn get_little_endian(a: u8, b: u8) -> u16 {
    (a as u16) | ((b as u16) << 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _partial_bit() {
        assert_eq!(7, 0b1111_0000u8.partial_bit(5..8));
        assert_eq!(4, 0b1111_0000u8.partial_bit(2..5));
        assert_eq!(0, 0b1111_0000u8.partial_bit(0..2));
    }

    #[test]
    fn _bit() {
        assert_eq!(1, 0b1000_0000u8.bit(7));
        assert_eq!(0, 0b1000_0000u8.bit(6));
    }

    #[test]
    fn _to_bool() {
        assert_eq!(true, 1u8.as_bool());
    }

    #[test]
    fn _to_u8() {
        assert_eq!(1, true.as_u8());
    }
}
