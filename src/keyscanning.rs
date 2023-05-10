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
        Col { input }
    }
    pub fn is_high(&self) -> bool {
        self.input.is_high()
    }
    pub fn is_low(&self) -> bool {
        self.input.is_low()
    }
}

pub struct KeyMatrix<const R: usize, const C: usize> {
    rows: Vec<Row, R>,
    cols: Vec<Col, C>,
}

impl<const R: usize, const C: usize> KeyMatrix<R, C> {
    fn poll() {}
}
