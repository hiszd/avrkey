#![allow(dead_code)]
use arduino_hal::{
    hal::port::Dynamic,
    port::{
        mode::{Input, Output},
        Pin,
    },
};
use heapless::Vec;

use crate::println;

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
    // output: Option<Pin<Output, Dynamic>>,
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
    // TODO convert to output to drain to GND
    // pub fn drain(&mut self) {
    //     self.output = Some(self.input.into_output().downgrade());
    // }
}

pub struct KeyMatrix<const R: usize, const C: usize> {
    rows: Vec<Row, R>,
    cols: Vec<Col, C>,
    matrix: Option<Vec<Vec<bool, C>, R>>,
}

impl<const R: usize, const C: usize> KeyMatrix<R, C> {
    fn new(rows: Vec<Row, R>, cols: Vec<Col, C>) -> Self {
        KeyMatrix {
            rows,
            cols,
            matrix: Some(Vec::new()),
        }
    }
    fn poll(&mut self) {
        for r in 1..R {
            let row = r.to_ne_bytes();
            self.rows[r].set_high();
            arduino_hal::delay_us(2);
            for c in 1..C {
                let col = c.to_ne_bytes();
                if self.cols[c].is_high() {
                    println(row.as_slice());
                    println(col.as_slice());
                    if let Some(matrix) = self.matrix.as_mut() {
                        matrix[r][c] = true;
                    }
                } else if let Some(matrix) = self.matrix.as_mut() {
                    matrix[r][c] = false;
                }
            }
            arduino_hal::delay_us(2);
            self.rows[r].set_low();
        }
    }
}
