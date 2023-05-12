#![allow(dead_code)]
use arduino_hal::{
    hal::port::Dynamic,
    port::{
        mode::{Input, Output},
        Pin,
    },
};
use heapless::Vec;

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
        Col {
            input,
            // output: todo!(),
        }
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
    // TODO convert to output to drain to GND
    // pub fn drain(&mut self) {
    //     self.output = Some(self.input.into_output().downgrade());
    // }
}

pub struct KeyMatrix<const R: usize, const C: usize> {
    rows: Vec<Row, R>,
    cols: Vec<Col, C>,
    matrix: Vec<Vec<u16, C>, R>,
    callback: fn(row: usize, col: usize, state: bool),
    debounce: u16,
}

impl<const R: usize, const C: usize> KeyMatrix<R, C> {
    pub fn new(
        rows: Vec<Row, R>,
        cols: Vec<Col, C>,
        callback: fn(row: usize, col: usize, state: bool),
    ) -> Self {
        KeyMatrix {
            rows,
            cols,
            matrix: Vec::from_iter(
                [
                    Vec::from_iter([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into_iter()),
                    Vec::from_iter([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into_iter()),
                    Vec::from_iter([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into_iter()),
                    Vec::from_iter([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into_iter()),
                    Vec::from_iter([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into_iter()),
                ]
                .into_iter(),
            ),
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
        for r in 0..(R - 1) {
            self.rows[r].set_high();
            for c in 0..(C - 1) {
                let prevstate = self.matrix[r][c] >= self.debounce;
                let mut state: bool = false;
                if self.cols[c].is_high() {
                    state = self.debounce(r, c);
                } else {
                    self.matrix[r][c] = 0;
                }
                if state != prevstate {
                    self.execute_callback(r, c, state);
                }
                self.cols[c].input.with_pin_as_output(|p| p.set_low());
            }
            self.rows[r].set_low();
        }
    }
}
