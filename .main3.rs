#![no_std]
#![no_main]
// #![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use arduino_hal::{entry, Peripherals};
use atmega_usbd::UsbBus;
use panic_halt as _;
use usb_device::{class_prelude::UsbBusAllocator, prelude::UsbDeviceBuilder, UsbError};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    // let pins = pins!(dp);
    let pll = dp.PLL;
    let usb = dp.USB_DEVICE;

    // Configure PLL interface
    // prescale 16MHz crystal -> 8MHz
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());

    // Check PLL lock
    // while pll.pllcsr.read().plock().bit_is_clear() {}

    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        &*USB_BUS.insert(UsbBus::new(usb))
    };

    let mut serial = SerialPort::new(usb_bus);

    let mut usb_dev =
        UsbDeviceBuilder::new(usb_bus, usb_device::prelude::UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(USB_CLASS_CDC)
            .build();

    // unsafe { interrupt::enable() };

    usb_dev.force_reset().ok();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        arduino_hal::delay_ms(3);

        let mut buf = [0u8; 64];

        match serial.read(&mut buf[..]) {
            Ok(_count) => {
                // count bytes were read to &buf[..count]
            }
            Err(UsbError::WouldBlock) => { // No data received
            }
            Err(_err) => { // An error occurred
            }
        };

        match serial.write(b"bob") {
            Ok(_count) => {
                // count bytes were written
            }
            Err(UsbError::WouldBlock) => { // No data could be written (buffers full)
            }
            Err(_err) => { // An error occurred
            }
        };
    }
}
