#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use arduino_hal::{
    pac::TC1,
    port::{mode::Output, Pin},
    Peripherals,
};
use atmega_usbd::UsbBus;
use avr_device::interrupt::{self, CriticalSection, Mutex};
use heapless::String;
use panic_halt as _;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

// use avr_device::interrupt::Mutex;
use core::{cell::Cell, mem};

#[allow(dead_code)]
struct InterruptState {
    blinker: Pin<Output>,
    tmr: TC1,
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

static DEBUGMSG: Mutex<Cell<String<10>>> = Mutex::new(Cell::new(String::new()));
// static mut DEBUGMSG: String<10> = String::new();

#[allow(dead_code)]
fn println(ser: &mut SerialPort<UsbBus>, msg: &[u8]) {
    unsafe {
        ser.write(msg).unwrap_unchecked();
    }
}

#[allow(dead_code)]
fn printmsg(cs: CriticalSection, ser: &mut SerialPort<UsbBus>) {
    unsafe {
        let msg_ref = DEBUGMSG.borrow(cs);
        ser.write(msg_ref.take().as_bytes()).unwrap_unchecked();
        msg_ref.set("".into());
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

    //setup timer interrupt
    pins.d9.into_output().set_high();
    let tmr1: TC1 = dp.TC1;

    rig_timer(&tmr1);

    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        INTERRUPT_STATE = mem::MaybeUninit::new(InterruptState {
            blinker: pins.d11.into_output().downgrade(),
            tmr: tmr1,
        });
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    // usb_dev.force_reset().ok();

    let mut bob: usize = 0;
    let mut cycles_ready: usize = 0;

    // let mut cycles: u16 = 0;
    let mut serial_cycles: u16 = 0;
    // const DIV1: u16 = 30000;
    const DIV2: u16 = 30000;

    // let mut rgbs = pins.led_rx.into_output();
    // let mut en = false;
    // pins.a3.into_output_high();
    // let inp = pins.d5.into_floating_input();

    loop {
        if serial_cycles == DIV2 {
            if !usb_dev.poll(&mut [&mut serial])
                || usb_dev.state() != UsbDeviceState::Configured
                || !serial.dtr()
            {
                continue;
            } else if cycles_ready < 4 {
                println(&mut serial, b"\n");
                cycles_ready += 1;
                continue;
            }
        } else {
            serial_cycles += 1;
        }

        // if inp.is_high() && !en {
        //     en = true;
        //     println(&mut serial, b"press");
        // }
        // if en {
        //     if cycles == DIV {
        //         rgbs.toggle();
        //     } else {
        //         cycles += 1;
        //     }
        // }

        if bob < 10 {
            println(&mut serial, b"heyonce ");
            bob += 1;
        }

        interrupt::free(|cs| {
            printmsg(cs, &mut serial);
        });
    }
}

pub fn rig_timer(tmr1: &TC1) {
    /*
     https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
     section 15.11
    */
    // use arduino_hal::clock::Clock;

    // const CLOCK_FREQUENCY_HZ: u32 = arduino_hal::DefaultClock::FREQ;
    // const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_256;
    // let clock_divisor: u32 = match CLOCK_SOURCE {
    //     CS1_A::DIRECT => 1,
    //     CS1_A::PRESCALE_8 => 8,
    //     CS1_A::PRESCALE_64 => 64,
    //     CS1_A::PRESCALE_256 => 256,
    //     CS1_A::PRESCALE_1024 => 1024,
    //     CS1_A::NO_CLOCK | CS1_A::EXT_FALLING | CS1_A::EXT_RISING => {
    //         // uwriteln!(serial, "uhoh, code tried to set the clock source to something other than a static prescaler {}", CLOCK_SOURCE as usize)
    //         // .void_unwrap();
    //         1
    //     }
    // };

    // let ticks = calc_overflow(CLOCK_FREQUENCY_HZ, 16000000, clock_divisor) as u16;
    // let ticks = 10_u16;
    // ufmt::uwriteln!(
    // serial,
    // "configuring timer output compare register = {}",
    // ticks
    // )
    // .void_unwrap();

    tmr1.tcnt1.write(|w| w.bits(0_u16));
    tmr1.tccr1a
        .write(|w| w.com1a().bits(0b10).wgm1().bits(0b00));
    tmr1.tccr1b.write(|w| w.cs1().bits(0b101).wgm1().bits(0b10));
    // tmr1.tccr1b.write(|w| unsafe { w.bits(0b00001101) });
    tmr1.ocr1a.write(|w| w.bits(1_u16));
    tmr1.timsk1
        .write(|w| w.ocie1a().bit(true).toie1().bit(true)); //enable this specific interrupt
}
/*
#[avr_device::interrupt(atmega32u4)]
fn TIMER1_OVF() {
    // let state = unsafe {
    // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
    // initialized so this ISR will never run when LED is uninitialized.
    // &mut *INTERRUPT_STATE.as_mut_ptr()
    // };

    // ufmt::uwriteln!(&mut state.serl, "Hello from Arduino!\r").void_unwrap();

    // state.blinker.toggle();
    // state.tmr.tcnt1.write(|w| w.bits(0_u16));
    avr_device::interrupt::free(|cs| {
        // Interrupts are disabled here
        let msg_ref = DEBUGMSG.borrow(cs);
        msg_ref.set("overf".into());
    });
    // avr_device::interrupt::free(|cs| {
    //     // Interrupts are disabled here
    //
    //     unsafe {
    //         // Acquire mutex to global variable.
    //         let msg_ref = DEBUGMSG.borrow(cs);
    //         // Write to the global variable.
    //         msg_ref.set(b"New thing");
    //     }
    // });
    // state.tmr.tcnt1.write(|w| w.bits(0b00));
    // state.tmr.ocr1a.write(|w| w.bits(0b01));
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
    state.tmr.tcnt1.write(|w| w.bits(0_u16));
    avr_device::interrupt::free(|cs| {
        // Interrupts are disabled here
        let msg_ref = DEBUGMSG.borrow(cs);
        msg_ref.set("inter".into());
    });
    // interrupt::free(|_| unsafe { DEBUGMSG = "inter".into() });
    // avr_device::interrupt::free(|cs| {
    //     // Interrupts are disabled here
    //
    //     unsafe {
    //         // Acquire mutex to global variable.
    //         let msg_ref = DEBUGMSG.borrow(cs);
    //         // Write to the global variable.
    //         msg_ref.set(b"New thing");
    //     }
    // });
    // state.tmr.tcnt1.write(|w| w.bits(0b00));
    // state.tmr.ocr1a.write(|w| w.bits(0b01));
} */
