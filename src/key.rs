use crate::{key_codes::KeyCode, keyscanning::StateType};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct KeyBase {
    pub cycles: usize,
    pub cycles_off: usize,
    pub raw_state: bool,
    pub state: StateType,
    pub prevstate: StateType,
    pub keycode: [KeyCode; 2],
    pub debounce_cycles: usize,
    pub hold_cycles: usize,
    pub idle_cycles: usize,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Key {
    pub key: KeyBase,
}

pub trait Default {
    fn new(KC1: KeyCode) -> Self;
    fn tap(&self) -> ([KeyCode; 2], u8);
    fn hold(&self) -> ([KeyCode; 2], u8);
    fn idle(&self) -> ([KeyCode; 2], u8);
}

impl Default for Key {
    fn new(KC1: KeyCode) -> Self {
        Key {
            key: KeyBase::new(KC1, KeyCode::________),
        }
    }
    fn tap(&self) -> ([KeyCode; 2], u8) {
        let curcode = self.key.keycode[0];
        let mut modi: u8 = 0;
        if let Some(bitmask) = curcode.modifier_bitmask() {
            modi |= bitmask;
            ([KeyCode::________, KeyCode::________], modi)
        } else {
            ([self.key.keycode[0], KeyCode::________], modi)
        }
    }
    fn hold(&self) -> ([KeyCode; 2], u8) {
        let curcode = self.key.keycode[0];
        let mut modi: u8 = 0;
        if let Some(bitmask) = curcode.modifier_bitmask() {
            modi |= bitmask;
            ([KeyCode::________, KeyCode::________], modi)
        } else {
            ([self.key.keycode[0], KeyCode::________], modi)
        }
    }
    fn idle(&self) -> ([KeyCode; 2], u8) {
        let curcode = self.key.keycode[0];
        let mut modi: u8 = 0;
        if let Some(bitmask) = curcode.modifier_bitmask() {
            modi |= bitmask;
            ([KeyCode::________, KeyCode::________], modi)
        } else {
            ([self.key.keycode[0], KeyCode::________], modi)
        }
    }
}

impl KeyBase {
    pub fn new(KC1: KeyCode, KC2: KeyCode) -> Self {
        KeyBase {
            cycles: 0,
            cycles_off: 0,
            raw_state: false,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: [KC1, KC2],
            // TODO create functions to set these after object creation
            debounce_cycles: 2,
            hold_cycles: 10,
            idle_cycles: 100,
        }
    }
    /// Perform state change as a result of the scan
    pub fn scan(&mut self, is_high: bool) -> bool {
        // if they KeyCode is empty then don't bother processing
        if self.keycode == [KeyCode::________, KeyCode::________] {
            return false;
        }
        //     ____________________________
        //    |                            |
        //    |       Cycle Counters       |
        //    |                            |
        //    |____________________________|

        // set the raw state to the state of the pin
        self.raw_state = is_high;
        if is_high {
            // increment cycles while pin is high
            if self.cycles < usize::MAX {
                self.cycles += 1;
            }
            self.cycles_off = 0;
        } else {
            // increment cycles_off while pin is low
            if self.cycles_off < usize::MAX {
                self.cycles_off += 1;
            }
            // reset cycles since pin is low
            self.cycles = 0;
        }

        //     ____________________________
        //    |                            |
        //    |        State Change        |
        //    |                            |
        //    |____________________________|

        // if we have gotten more cycles in than the debounce_cycles
        if self.cycles >= self.debounce_cycles {
            // if the current state is Tap  and we have more cycles than hold_cycles
            if self.state == StateType::Tap && self.cycles >= self.hold_cycles {
                self.prevstate = self.state;
                self.state = StateType::Hold;
            } else if self.state == StateType::Off {
                // if the current state is Off
                self.prevstate = self.state;
                self.state = StateType::Tap;
            }
            return true;
        }
        false
    }
    // fn keyfunc(&mut self) -> KeyCode {
    //     self.keycode[0]
    // }
}
