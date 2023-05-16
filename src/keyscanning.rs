#![allow(dead_code)]
use arduino_hal::{
    hal::port::Dynamic,
    port::{
        mode::{Input, Output},
        Pin,
    },
};

pub struct Col {
    output: Pin<Output, Dynamic>,
}

impl Col {
    pub fn new(output: Pin<Output, Dynamic>) -> Self {
        Col { output }
    }
    pub fn set_high(&mut self) {
        self.output.set_high()
    }
    pub fn set_low(&mut self) {
        self.output.set_low()
    }
}

pub struct Row {
    input: Pin<Input, Dynamic>,
}

impl Row {
    pub fn new(input: Pin<Input, Dynamic>) -> Self {
        Row { input }
    }
    pub fn is_high(&self) -> bool {
        self.input.is_high()
    }
    pub fn is_low(&self) -> bool {
        self.input.is_low()
    }
    pub fn drain(&mut self) {
        self.input.with_pin_as_output(|p| p.set_low());
    }
}

pub struct KeyMatrix<const R: usize, const C: usize> {
    rows: [Row; R],
    cols: [Col; C],
    matrix: [[u16; C]; R],
    callback: fn(row: usize, col: usize, state: bool),
    debounce: u16,
}

impl<const R: usize, const C: usize> KeyMatrix<R, C> {
    pub fn new(
        rows: [Row; R],
        cols: [Col; C],
        callback: fn(row: usize, col: usize, state: bool),
    ) -> Self {
        KeyMatrix {
            rows,
            cols,
            matrix: [[0; C]; R],
            callback,
            debounce: 5,
        }
    }
    pub fn set_debounce(&mut self, debounce: u16) {
        self.debounce = debounce;
    }
    fn execute_callback(&self, row: usize, col: usize, state: bool) {
        (self.callback)(row, col, state);
    }
    fn debounce(&mut self, row: usize, col: usize) -> bool {
        if self.matrix[row][col] >= self.debounce {
            return true;
        }
        self.matrix[row][col] += 1;
        false
    }
    pub fn poll(&mut self) {
        for c in 0..(C - 1) {
            self.cols[c].set_high();
            for r in 0..(R - 1) {
                let prevstate = self.matrix[r][c] >= self.debounce;
                let mut state: bool = false;
                if self.rows[r].is_high() {
                    state = self.debounce(r, c);
                } else {
                    self.matrix[r][c] = 0;
                }
                if state != prevstate {
                    self.execute_callback(r, c, state);
                }
                self.rows[r].drain();
            }
            self.cols[c].set_low();
        }
    }
}
