#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
/*
 This is derived from Rahix' comment to
https://github.com/Rahix/avr-hal/issues/75
and then modernized to account for API drift since 2020

*/

extern crate arduino_hal;
extern crate avr_device;
extern crate ufmt;

use arduino_hal::{hal::port::PB7, port::mode::Output};
use arduino_hal::{
    hal::{port::PB0, Atmega},
    pac::USART1,
    port::Pin,
};
use arduino_hal::{pac::tc1::tccr1a::COM1C_A, prelude::*};
use atmega_usbd::UsbBus;
use avr_device::atmega32u4::TC1;
use avr_device::{atmega32u4::tc1::tccr1b::CS1_A, interrupt};
use core::mem;
use core::{cell::RefCell, fmt::Write};
use panic_halt as _;
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use ufmt::{uWrite, uwriteln};

// type Console = arduino_hal::pac::USB_DEVICE;
// static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
//     interrupt::Mutex::new(RefCell::new(None));
//
// macro_rules! print {
//     ($($t:tt)*) => {
//         interrupt::free(
//             |cs| {
//                 if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
//                     let _ = ufmt::uwrite!(console, $($t)*);
//                 }
//             },
//         )
//     };
// }
//
// macro_rules! println {
//     ($($t:tt)*) => {
//         interrupt::free(
//             |cs| {
//                 if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
//                     let _ = ufmt::uwriteln!(console, $($t)*);
//                 }
//             },
//         )
//     };
// }

struct InterruptState {
    blinker: Pin<Output>,
    tmr: TC1,
    // serl: &mut arduino_hal::Usart<
    //     USART1,
    //     Pin<arduino_hal::port::mode::Input, arduino_hal::hal::port::PD2>,
    //     Pin<Output, arduino_hal::hal::port::PD3>,
    // >,
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
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
    while pll.pllcsr.read().plock().bit_is_clear() {}

    let usb_bus = unsafe {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        &*USB_BUS.insert(UsbBus::new(usb))
    };

    let mut serial = SerialPort::new(usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .product("Serial port")
        .device_class(USB_CLASS_CDC)
        .build();

    // writeln!(&mut serial, "Hello There!\r");
    // uwriteln!(&mut serial, "Hello from Arduino!\r");

    let led = pins.d11.into_output();
    pins.d9.into_output().set_high();

    let tmr1: TC1 = dp.TC1;

    // println!("Hello World!");

    rig_timer(&tmr1);

    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        INTERRUPT_STATE = mem::MaybeUninit::new(InterruptState {
            blinker: led.downgrade(),
            tmr: tmr1,
        });
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    // Enable interrupts globally, not a replacement for the specific interrupt enable
    unsafe {
        // SAFETY: Not inside a critical section and any non-atomic operations have been completed
        // at this point.
        avr_device::interrupt::enable();
    }

    // ufmt::uwriteln!(
    //     &mut serial,
    //     "configured timer output compare register = {}",
    //     tmr1.ocr1a.read().bits()
    // )
    // .void_unwrap();

    loop {
        loop {
            if !usb_dev.poll(&mut [&mut serial]) {
                continue;
            }

            let mut buf = [0u8; 64];

            serial.read(&mut buf[..]).unwrap();

            serial.write(&[0x3a, 0x29]).unwrap();
        }
        // ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
        // avr_device::asm::sleep()
    }
}

pub const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    /*
    https://github.com/Rahix/avr-hal/issues/75
    reversing the formula F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
     */
    clock_hz / target_hz / prescale - 1
}

pub fn rig_timer(tmr1: &TC1) {
    /*
     https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
     section 15.11
    */
    use arduino_hal::clock::Clock;

    const CLOCK_FREQUENCY_HZ: u32 = arduino_hal::DefaultClock::FREQ;
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_256;
    let clock_divisor: u32 = match CLOCK_SOURCE {
        CS1_A::DIRECT => 1,
        CS1_A::PRESCALE_8 => 8,
        CS1_A::PRESCALE_64 => 64,
        CS1_A::PRESCALE_256 => 256,
        CS1_A::PRESCALE_1024 => 1024,
        CS1_A::NO_CLOCK | CS1_A::EXT_FALLING | CS1_A::EXT_RISING => {
            // uwriteln!(serial, "uhoh, code tried to set the clock source to something other than a static prescaler {}", CLOCK_SOURCE as usize)
            // .void_unwrap();
            1
        }
    };

    // let ticks = calc_overflow(CLOCK_FREQUENCY_HZ, 16000000, clock_divisor) as u16;
    // let ticks = 10_u16;
    // ufmt::uwriteln!(
    // serial,
    // "configuring timer output compare register = {}",
    // ticks
    // )
    // .void_unwrap();

    tmr1.tcnt1.write(|w| w.bits(0b00));
    tmr1.tccr1a.write(|w| w.wgm1().bits(0b00));
    tmr1.tccr1b
        .write(|w| w.cs1().prescale_256().wgm1().bits(0b01));
    tmr1.ocr1a.write(|w| w.bits(257));
    tmr1.timsk1.write(|w| w.ocie1a().set_bit()); //enable this specific interrupt
}

#[avr_device::interrupt(atmega32u4)]
fn TIMER1_COMPA() {
    let state = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
        // initialized so this ISR will never run when LED is uninitialized.
        &mut *INTERRUPT_STATE.as_mut_ptr()
    };

    // ufmt::uwriteln!(&mut state.serl, "Hello from Arduino!\r").void_unwrap();

    state.blinker.toggle();
    state.tmr.tcnt1.write(|w| w.bits(0b00));
    // state.tmr.ocr1a.write(|w| w.bits(0b01));
}
