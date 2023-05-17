#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

mod hid_descriptor;
mod key_codes;
mod key_mapping;
mod keyscanning;

use arduino_hal::{
    pac::TC1,
    port::{mode::Output, Pin},
    Peripherals,
};
use atmega_usbd::UsbBus;
use avr_device::interrupt::{self, CriticalSection, Mutex};
use heapless::String;
use keyscanning::{Col, Row};
use panic_halt as _;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    UsbError,
};
use usbd_hid::hid_class::HidClassSettings;
use usbd_hid::{
    descriptor::KeyboardReport,
    hid_class::{HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig},
};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};
use usbd_serial::SerialPort;

// use avr_device::interrupt::Mutex;
use core::cell::Cell;

use crate::keyscanning::StateMatrix;

#[allow(dead_code)]
struct InterruptState {
    blinker: Pin<Output>,
    tmr: TC1,
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;
static mut SERIAL: Option<SerialPort<UsbBus>> = None;

static DEBUGMSG: Mutex<Cell<String<10>>> = Mutex::new(Cell::new(String::new()));

// #[allow(dead_code)]
// fn println(msg: &[u8]) -> bool {
//     unsafe {
//         if let Some(ser) = SERIAL.as_mut() {
//             match ser.write(msg) {
//                 Ok(count) => count == msg.len(),
//                 Err(UsbError::WouldBlock) => false, // No data could be written (buffers full)
//                 Err(_err) => false,                 // An error occurred
//             }
//         } else {
//             false
//         }
//     }
// }
//
// #[allow(dead_code)]
// fn printmsg(cs: CriticalSection) {
//     let msg_ref = DEBUGMSG.borrow(cs);
//     let binding = msg_ref.take();
//     let msg = binding.as_bytes();
//     unsafe {
//         if let Some(ser) = SERIAL.as_mut() {
//             match ser.write(msg) {
//                 Ok(count) => count == msg.len(),
//                 Err(UsbError::WouldBlock) => false, // No data could be written (buffers full)
//                 Err(_err) => false,                 // An error occurred
//             };
//         }
//     }
// }

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

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(UsbBus::new(usb));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    // unsafe {
    //     SERIAL = Some(SerialPort::new(bus_allocator));
    // }

    // The latest keyboard report for responding to USB interrupts.
    let mut key_report: KeyboardReport = KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0u8; 6],
    };

    let mut hid_endpoint = HIDClass::new_with_settings(
        bus_allocator,
        KeyboardReport::desc(),
        // Keyboard Polling Rate
        1,
        HidClassSettings {
            subclass: HidSubClass::NoSubClass,
            protocol: HidProtocol::Keyboard,
            config: ProtocolModeConfig::ForceReport,
            locale: HidCountryCode::US,
        },
    );
    let mut keyboard_usb_device = UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27db))
        .manufacturer("HisZd")
        .product("avrkey")
        .supports_remote_wakeup(true)
        .build();

    // let mut usb_hid = unsafe {
    //     USB_HID = Some(hid_endpoint);
    //     USB_HID.as_ref().unwrap()
    // };
    // let mut serial_bus = UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
    //     .product("Serial port")
    //     .manufacturer("HisZd")
    //     .serial_number("00001")
    //     .max_packet_size_0(64_u8)
    //     .device_class(usbd_serial::USB_CLASS_CDC)
    //     .build();
    //
    // serial_bus.force_reset().ok();

    let rows: [Row; 5] = [
        Row::new(pins.a3.into_output().downgrade()),
        Row::new(pins.a2.into_output().downgrade()),
        Row::new(pins.a1.into_output().downgrade()),
        Row::new(pins.a0.into_output().downgrade()),
        Row::new(pins.d13.into_output().downgrade()),
    ];

    let cols: [Col; 16] = [
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
    ];

    fn info(info: &str) {
        // println(info.as_bytes());
    }

    fn callback(row: usize, col: usize, state: bool) {
        // let blank: String<20> = String::from("                    \n");
        // let rowstr: String<2> = String::from(row as u32);
        // let colstr: String<2> = String::from(col as u32);
        // let mut str: String<25> = String::from("row: ");
        // str.push_str(rowstr.as_str()).unwrap();
        // str.push_str(", col: ").unwrap();
        // str.push_str(colstr.as_str()).unwrap();
        // str.push_str(match state {
        //     true => " true",
        //     false => " false",
        // })
        // .unwrap();
        // str.push_str("\n").unwrap();
        // // println(&blank.into_bytes());
        // println(&str.into_bytes());
    }

    let mut matrix: StateMatrix<5, 16> = StateMatrix::new(rows, cols, callback, info);
    matrix.set_debounce(4);

    // let mut countinit: usize = 0;

    loop {
        // unsafe {
        //     if poll_usb() || USB_BUS.as_mut().unwrap().state() != UsbDeviceState::Configured
        //     // || !SERIAL.as_mut().unwrap().dtr()
        //     // || !println(&[0x00])
        //     {
        //         continue;
        //     }
        // }
        keyboard_usb_device.poll(&mut [&mut hid_endpoint]);

        // key_report = matrix.poll().unwrap().into();
        unsafe {
            let usb_hid = USB_HID.as_ref().unwrap_unchecked();
            usb_hid.push_input(&key_report).unwrap_unchecked();
            // macOS doesn't like it when you don't pull this, apparently.
            // TODO: maybe even parse something here
            usb_hid.pull_raw_output(&mut [0; 64]).ok();
            // Wake the host if a key is pressed and the device supports
            // remote wakeup.
            // if !report_is_empty(&key_report)
            //     && keyboard_usb_device.state() == UsbDeviceState::Suspend
            //     && keyboard_usb_device.remote_wakeup_enabled()
            // {
            //     keyboard_usb_device.
            // }
        }

        // if countinit <= 11 {
        //     println(b"heyonce ");
        //     countinit += 1;
        // }

        // interrupt::free(|cs| {
        //     if DEBUGMSG.borrow(cs).take() != "" {
        //         printmsg(cs);
        //     }
        // });
    }
}

fn poll_usb() -> bool {
    unsafe {
        if let (Some(usb_dev), Some(hid)) = (USB_BUS.as_mut(), USB_HID.as_mut()) {
            usb_dev.poll(&mut [hid])
        } else {
            false
        }
    }
}

fn report_is_empty(report: &KeyboardReport) -> bool {
    report.modifier != 0
        || report
            .keycodes
            .iter()
            .any(|key| *key != key_codes::KeyCode::Emp as u8)
}
