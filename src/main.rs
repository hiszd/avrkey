#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

mod keyscanning;

use arduino_hal::{
    pac::TC1,
    port::{mode::Output, Pin},
    Peripherals,
};
use atmega_usbd::UsbBus;
use avr_device::interrupt::{self, CriticalSection, Mutex};
use heapless::{String, Vec};
use keyscanning::{Col, Row};
use panic_halt as _;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    UsbError,
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

// use avr_device::interrupt::Mutex;
use core::cell::Cell;

use crate::keyscanning::KeyMatrix;

#[allow(dead_code)]
struct InterruptState {
    blinker: Pin<Output>,
    tmr: TC1,
}

static mut USB_BUS: Option<usb_device::prelude::UsbDevice<UsbBus>> = None;
static mut SERIAL: Option<SerialPort<UsbBus>> = None;

static DEBUGMSG: Mutex<Cell<String<10>>> = Mutex::new(Cell::new(String::new()));

#[allow(dead_code)]
fn println(msg: &[u8]) -> bool {
    unsafe {
        if let Some(ser) = SERIAL.as_mut() {
            match ser.write(msg) {
                Ok(count) => count == msg.len(),
                Err(UsbError::WouldBlock) => false, // No data could be written (buffers full)
                Err(_err) => false,                 // An error occurred
            }
        } else {
            false
        }
    }
}

#[allow(dead_code)]
fn printmsg(cs: CriticalSection) {
    let msg_ref = DEBUGMSG.borrow(cs);
    let binding = msg_ref.take();
    let msg = binding.as_bytes();
    unsafe {
        if let Some(ser) = SERIAL.as_mut() {
            match ser.write(msg) {
                Ok(count) => count == msg.len(),
                Err(UsbError::WouldBlock) => false, // No data could be written (buffers full)
                Err(_err) => false,                 // An error occurred
            };
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let pll = dp.PLL;
    let usb = dp.USB_DEVICE;

    avr_device::interrupt::free(|cs| {
        // Interrupts are disabled here
        let msg_ref = DEBUGMSG.borrow(cs);
        msg_ref.set("start".into());
    });

    // Configure PLL interface
    // prescale 16MHz crystal -> 8MHz ** actually I got rid of the scaling
    // pll.pllcsr.write(|w| unsafe { w.bits(0_u8) });
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());

    // Check PLL lock
    while pll.pllcsr.read().plock().bit_is_clear() {}

    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        &*USB_BUS.insert(UsbBus::new(usb))
    };

    unsafe {
        SERIAL = Some(SerialPort::new(usb_bus));
    }

    unsafe {
        USB_BUS = Some(
            UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27dd))
                .product("Serial port")
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    //setup timer interrupt
    let tmr1: TC1 = dp.TC1;
    tmr1.tccr1b.write(|w| w.cs1().prescale_8());
    tmr1.tcnt1.write(|w| w.bits(0_u16));

    unsafe {
        USB_BUS.as_mut().unwrap().force_reset().ok();
    }

    let rows: Vec<Row, 5> = Vec::from_iter(
        [
            Row::new(pins.a3.into_output().downgrade()),
            Row::new(pins.a2.into_output().downgrade()),
            Row::new(pins.a1.into_output().downgrade()),
            Row::new(pins.a0.into_output().downgrade()),
            Row::new(pins.d13.into_output().downgrade()),
        ]
        .into_iter(),
    );

    let cols: Vec<Col, 16> = Vec::from_iter(
        [
            Col::new(pins.d5.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d7.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d9.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d8.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d6.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d12.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d4.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.led_tx.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d1.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d0.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d2.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d3.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.d11.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.miso.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.mosi.into_floating_input().downgrade().forget_imode()),
            Col::new(pins.sck.into_floating_input().downgrade().forget_imode()),
        ]
        .into_iter(),
    );

    fn callback(row: usize, col: usize, state: bool) {
        // let blank: String<20> = String::from("                    \n");
        let rowstr: String<2> = String::from(row as u32);
        let colstr: String<2> = String::from(col as u32);
        let mut str: String<25> = String::from("row: ");
        str.push_str(rowstr.as_str()).unwrap();
        str.push_str(", col: ").unwrap();
        str.push_str(colstr.as_str()).unwrap();
        str.push_str(match state {
            true => " true",
            false => " false",
        })
        .unwrap();
        str.push_str("\n").unwrap();
        // println(&blank.into_bytes());
        println(&str.into_bytes());
    }

    let mut matrix: KeyMatrix<5, 16> = KeyMatrix::new(rows, cols, callback);
    matrix.set_debounce(150);

    let mut countinit: usize = 0;

    // pins.a3.into_output_high();
    // let mut in1 = pins.d5.into_floating_input();
    // let mut counting: usize = 0;
    // let mut ispress: bool = false;

    loop {
        unsafe {
            if poll_usb()
                || USB_BUS.as_mut().unwrap().state() != UsbDeviceState::Configured
                || !SERIAL.as_mut().unwrap().dtr()
                || !println(&[0x00])
            {
                continue;
            }
        }

        matrix.poll();

        // let waspress = ispress;
        // if counting >= 10 && in1.is_high() {
        //     ispress = true;
        // } else if counting < 10 && in1.is_high() {
        //     counting += 1;
        // } else if in1.is_low() {
        //     counting = 0;
        //     ispress = false;
        // }
        // if ispress != waspress {
        //     if ispress {
        //         println(b"bobby\n");
        //     } else {
        //         println(b"sorry\r");
        //     }
        // }
        //
        // in1.with_pin_as_output(|p| p.set_low());

        if countinit <= 11 {
            println(b"heyonce ");
            countinit += 1;
        }

        interrupt::free(|cs| {
            if DEBUGMSG.borrow(cs).take() != "" {
                printmsg(cs);
            }
        });
    }
}

fn poll_usb() -> bool {
    unsafe {
        if let (Some(usb_dev), Some(hid)) = (USB_BUS.as_mut(), SERIAL.as_mut()) {
            usb_dev.poll(&mut [hid])
        } else {
            false
        }
    }
}
