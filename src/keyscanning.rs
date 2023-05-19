#![allow(dead_code)]

use arduino_hal::{
    hal::port::Dynamic,
    port::{
        mode::{Input, Output},
        Pin,
    },
};
use usbd_hid::descriptor::KeyboardReport;

use crate::{key_codes::KeyCode, key_mapping};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum StateType {
    Tap = 0,
    Hold = 1,
    Idle = 2,
    Off = 3,
}

pub struct Row {
    output: Pin<Output, Dynamic>,
}

impl Row {
    pub fn new(output: Pin<Output, Dynamic>) -> Self {
        Row { output }
    }
    pub fn set_high(&mut self) {
        self.output.set_high()
    }
    pub fn set_low(&mut self) {
        self.output.set_low()
    }
}

pub struct Col {
    input: Pin<Input, Dynamic>,
}

impl Col {
    pub fn new(input: Pin<Input, Dynamic>) -> Self {
        Col { input }
    }
    pub fn is_high(&self) -> bool {
        self.input.is_high()
    }
    pub fn is_low(&self) -> bool {
        self.input.is_low()
    }
    pub fn drain(&mut self) {
        self.input.with_pin_as_output(|p| {
            p.set_low();
        });
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Key<const DEBOUNCE_CYC: usize, const HOLD_CYC: usize, const IDLE_CYC: usize> {
    cycles: usize,
    cycles_off: usize,
    raw_state: bool,
    state: StateType,
    prevstate: StateType,
    keycode: KeyCode,
    debounce_cycles: usize,
    hold_cycles: usize,
    idle_cycles: usize,
}

impl<const DEBOUNCE_CYC: usize, const HOLD_CYC: usize, const IDLE_CYC: usize>
    Key<DEBOUNCE_CYC, HOLD_CYC, IDLE_CYC>
{
    fn new() -> Self {
        Key::<DEBOUNCE_CYC, HOLD_CYC, IDLE_CYC> {
            cycles: 0,
            cycles_off: 0,
            raw_state: false,
            state: StateType::Off,
            prevstate: StateType::Off,
            keycode: KeyCode::________,
            debounce_cycles: DEBOUNCE_CYC,
            hold_cycles: HOLD_CYC,
            idle_cycles: IDLE_CYC,
        }
    }
    /// Perform state change as a result of the scan
    fn scan(&mut self, is_high: bool) -> bool {
        // set the raw state to the state of the pin
        self.raw_state = is_high;
        if is_high {
            if !(self.cycles >= usize::MAX) {
                // increment cycles while pin is high
                self.cycles += 1;
            }
            self.cycles_off = 0;
        } else {
            if !(self.cycles_off >= usize::MAX) {
                self.cycles_off += 1;
            }
            // reset cycles since pin is low
            self.cycles = 0;
        }
        if self.cycles >= self.debounce_cycles {
            if self.state == StateType::Tap && self.cycles >= self.hold_cycles {
                self.prevstate = self.state;
                self.state = StateType::Hold;
            } else {
                self.prevstate = self.state;
                self.state = StateType::Tap;
            }
            return true;
        }
        false
    }
    fn keyfunc(&mut self) -> KeyCode {
        return self.keycode;
    }
}

#[derive(Copy, Clone)]
pub struct KeyMatrix<const RSIZE: usize, const CSIZE: usize> {
    matrix: [[Key<2, 10, 100>; CSIZE]; RSIZE],
}

impl<const RSIZE: usize, const CSIZE: usize> Default for KeyMatrix<RSIZE, CSIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const RSIZE: usize, const CSIZE: usize> KeyMatrix<RSIZE, CSIZE> {
    pub fn new() -> Self {
        KeyMatrix {
            matrix: [[Key::new(); CSIZE]; RSIZE],
        }
    }
}

pub struct StateMatrix<const RSIZE: usize, const CSIZE: usize> {
    rows: [Row; RSIZE],
    cols: [Col; CSIZE],
    state: KeyMatrix<RSIZE, CSIZE>,
    callback: fn(row: usize, col: usize, state: bool),
    info: fn(info: &str),
    wait_cycles: u16,
    cycles: u16,
    cur_strobe: usize,
}

impl<const RSIZE: usize, const CSIZE: usize> StateMatrix<RSIZE, CSIZE> {
    pub fn new(
        rows: [Row; RSIZE],
        cols: [Col; CSIZE],
        callback: fn(row: usize, col: usize, state: bool),
        info: fn(info: &str),
    ) -> Self {
        let mut new = StateMatrix {
            rows,
            cols,
            state: KeyMatrix::default(),
            callback,
            info,
            wait_cycles: 2,
            cycles: 0,
            cur_strobe: 0,
        };
        new.rows[new.cur_strobe].set_high();
        new.clear();
        new
    }
    pub fn set_debounce(&mut self, debounce: u16) {
        // set the debounce for each key
        for r in 0..RSIZE {
            for c in 0..CSIZE {
                self.state.matrix[r][c].debounce_cycles = debounce as usize;
            }
        }
    }
    fn execute_callback(&self, row: usize, col: usize, state: bool) {
        (self.callback)(row, col, state);
    }
    fn execute_info(&self, info: &str) {
        (self.info)(info);
    }
    fn clear(&mut self) {
        for r in self.rows.iter_mut() {
            r.set_low();
        }
    }
    fn next_strobe(&mut self) {
        // Unset current strobe
        self.rows[self.cur_strobe].set_low();

        // Drain stray potential from sense lines
        for c in self.cols.iter_mut() {
            c.drain();
        }

        // Check overflow condition
        if self.cur_strobe >= RSIZE - 1 {
            self.cur_strobe = 0;
        } else {
            // Increment current strobe
            self.cur_strobe += 1;
        }

        // Set new strobe as high
        self.rows[self.cur_strobe].set_high();
        // let mut str: String<10> = "strobing ".into();
        // let strobe: String<10> = String::from(self.cur_strobe as u32);
        // str.push_str(&strobe).unwrap();
        // self.execute_info(&str)
    }
    pub fn poll(&mut self) -> Option<KeyMatrix<RSIZE, CSIZE>> {
        if self.cycles < self.wait_cycles {
            self.cycles += 1;
            return None;
        }
        self.next_strobe();
        self.cycles = 0;
        let r = self.cur_strobe;
        for c in 0..CSIZE {
            let state = self.state.matrix[r][c].scan(self.cols[c].is_high());
            if self.state.matrix[r][c].state != self.state.matrix[r][c].prevstate {
                self.execute_callback(r + 1, c + 1, state);
            }
        }
        Some(self.state)
    }
}

impl<const RSIZE: usize, const CSIZE: usize> From<StateMatrix<RSIZE, CSIZE>>
    for usbd_hid::descriptor::KeyboardReport
{
    fn from(matrix: StateMatrix<RSIZE, CSIZE>) -> Self {
        let mut keycodes = [0u8; 6];
        let mut keycode_index = 0;
        let mut modifier = 0;

        let mut push_keycode = |key| {
            if keycode_index < keycodes.len() {
                keycodes[keycode_index] = key;
                keycode_index += 1;
            }
        };

        // Scan to generate the correct keycodes given the activated key map
        let layer_mapping = key_mapping::NORMAL_LAYER_MAPPING;
        for (matrix_column, mapping_column) in matrix.state.matrix.iter().zip(layer_mapping) {
            for (key_pressed, mapping_row) in matrix_column.iter().zip(mapping_column) {
                if key_pressed.state == StateType::Tap || key_pressed.state == StateType::Hold {
                    if let Some(bitmask) = mapping_row.modifier_bitmask() {
                        modifier |= bitmask;
                    } else {
                        push_keycode(mapping_row as u8);
                    }
                }
            }
        }

        KeyboardReport {
            modifier,
            reserved: 0,
            leds: 0,
            keycodes,
        }
    }
}
