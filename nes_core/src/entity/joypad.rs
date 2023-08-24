use crate::util::bit::AsU8;

#[allow(non_snake_case)]
#[derive(Debug, Default)]
pub struct JoyPadBtnState {
    pub A: bool,
    pub B: bool,
    pub SELECT: bool,
    pub START: bool,
    pub UP: bool,
    pub DOWN: bool,
    pub LEFT: bool,
    pub RIGHT: bool,
}

impl JoyPadBtnState {
    #[cfg_attr(not(debug_assertions), inline(always))]
    pub fn get_u8(&self) -> u8 {
        (self.RIGHT.as_u8() << 7)
            | (self.LEFT.as_u8() << 6)
            | (self.DOWN.as_u8() << 5)
            | (self.UP.as_u8() << 4)
            | (self.START.as_u8() << 3)
            | (self.SELECT.as_u8() << 2)
            | (self.B.as_u8() << 1)
            | self.A.as_u8()
    }
}
