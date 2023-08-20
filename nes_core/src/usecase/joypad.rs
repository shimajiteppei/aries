use crate::{entity::joypad::JoyPadBtnState, util::bit::AsU8};

use super::nes::NesState;

#[derive(Default)]
pub struct JoyPadState {
    pub state_1p: JoyPadBtnState,
    pub state_2p: JoyPadBtnState,
    pub shift_register: [u8; 2],
    pub strobe: bool,
}

impl NesState {
    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn read_joypad_state(&mut self, is_player2: bool) -> u8 {
        if self.joypad.strobe {
            return 0x40 | (self.get_joypad_state(is_player2) & 1);
        }

        let shift = 0x40 | (self.joypad.shift_register[is_player2.as_u8() as usize] & 1);
        self.joypad.shift_register[is_player2.as_u8() as usize] =
            0x80 | (self.joypad.shift_register[is_player2.as_u8() as usize] >> 1);
        shift
    }

    #[cfg_attr(not(debug_assertions), inline(always))]
    #[cfg_attr(debug_assertions, inline(never))]
    pub fn write_joypad_strobe(&mut self, val: bool) {
        if self.joypad.strobe && !val {
            self.joypad.shift_register[0] = self.get_joypad_state(false);
            self.joypad.shift_register[1] = self.get_joypad_state(true);
        }
        self.joypad.strobe = val;
    }

    fn get_joypad_state(&mut self, is_player2: bool) -> u8 {
        if !is_player2 {
            self.joypad.state_1p.get_u8()
        } else {
            self.joypad.state_2p.get_u8()
        }
    }
}
