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

#[derive(Copy, Clone)]
pub struct KeyMatrix<const RSIZE: usize, const CSIZE: usize> {
    matrix: [[u16; CSIZE]; RSIZE],
}

impl<const RSIZE: usize, const CSIZE: usize> Default for KeyMatrix<RSIZE, CSIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const RSIZE: usize, const CSIZE: usize> KeyMatrix<RSIZE, CSIZE> {
    pub fn new() -> Self {
        KeyMatrix {
            matrix: [[0_u16; CSIZE]; RSIZE],
        }
    }
}

pub struct StateMatrix<const RSIZE: usize, const CSIZE: usize> {
    rows: [Row; RSIZE],
    cols: [Col; CSIZE],
    state: KeyMatrix<RSIZE, CSIZE>,
    callback: fn(row: usize, col: usize, state: bool),
    info: fn(info: &str),
    debounce: u16,
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
            debounce: 5,
            wait_cycles: 5,
            cycles: 0,
            cur_strobe: 0,
        };
        new.rows[new.cur_strobe].set_high();
        new.clear();
        new
    }
    pub fn set_debounce(&mut self, debounce: u16) {
        self.debounce = debounce;
    }
    fn execute_callback(&self, row: usize, col: usize, state: bool) {
        (self.callback)(row, col, state);
    }
    fn execute_info(&self, info: &str) {
        (self.info)(info);
    }
    fn debounce(&mut self, row: usize, col: usize) -> bool {
        self.state.matrix[row][col] += 1;
        if self.state.matrix[row][col] >= self.debounce {
            return true;
        }
        false
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
        for c in 0..(CSIZE - 1) {
            let prevstate = self.state.matrix[self.cur_strobe][c] >= self.debounce;
            let mut state: bool = false;
            if self.cols[c].is_high() {
                state = self.debounce(self.cur_strobe, c);
            } else {
                self.state.matrix[self.cur_strobe][c] = 0;
            }
            if state != prevstate {
                self.execute_callback(self.cur_strobe + 1, c + 1, state);
            }
        }
        Some(self.state)
    }
}

impl<const RSIZE: usize, const CSIZE: usize> From<KeyMatrix<RSIZE, CSIZE>>
    for usbd_hid::descriptor::KeyboardReport
{
    fn from(matrix: KeyMatrix<RSIZE, CSIZE>) -> Self {
        let mut keycodes = [0u8; 6];
        let mut keycode_index = 0;
        let mut modifier = 0;

        let mut push_keycode = |key| {
            if keycode_index < keycodes.len() {
                keycodes[keycode_index] = key;
                keycode_index += 1;
            }
        };

        // First scan for any function keys being pressed
        // let mut layer_mapping = key_mapping::NORMAL_LAYER_MAPPING;
        // for (matrix_column, mapping_column) in matrix.matrix.iter().zip(layer_mapping) {
        //     for (key_pressed, mapping_row) in matrix_column.iter().zip(mapping_column) {
        //         if mapping_row == KeyCode::Fn && *key_pressed {
        //             layer_mapping = key_mapping::FN_LAYER_MAPPING;
        //         }
        //     }
        // }

        // Second scan to generate the correct keycodes given the activated key map
        let layer_mapping = key_mapping::NORMAL_LAYER_MAPPING;
        for (matrix_column, mapping_column) in matrix.matrix.iter().zip(layer_mapping) {
            for (key_pressed, mapping_row) in matrix_column.iter().zip(mapping_column) {
                if key_pressed >= &4 {
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
