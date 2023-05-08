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

use arduino_hal::{hal::port::PB0, port::Pin};
use arduino_hal::{hal::port::PB7, port::mode::Output};
use arduino_hal::{pac::tc1::tccr1a::COM1C_A, prelude::*};
use avr_device::atmega32u4::tc1::tccr1b::CS1_A;
use avr_device::atmega32u4::TC1;
use core::mem;
// use panic_halt as _;

use core::panic::PanicInfo;
use ufmt::uWrite;

struct InterruptState {
    blinker: Pin<Output>,
    tmr: TC1,
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    // ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    let led = pins.d11.into_output();
    pins.d9.into_output().set_high();

    let tmr1: TC1 = dp.TC1;

    rig_timer(&tmr1, &mut serial);

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
        avr_device::asm::sleep()
    }
}

pub const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    /*
    https://github.com/Rahix/avr-hal/issues/75
    reversing the formula F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
     */
    clock_hz / target_hz / prescale - 1
}

pub fn rig_timer<W: uWrite<Error = void::Void>>(tmr1: &TC1, _serial: &mut W) {
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
    let ticks = 10 as u16;
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

    state.blinker.toggle();
    state.tmr.tcnt1.write(|w| w.bits(0b00));
    // state.tmr.ocr1a.write(|w| w.bits(0b01));
}
