#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern crate ch58x_hal as hal;
use hal::gpio::{AnyPin, Flex, Input, Level, Output, OutputDrive, Pin, Pull};

extern crate embedded_hal;

const OUTPUT_DRIVE: OutputDrive = OutputDrive::_5mA;

#[ch32v_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_32mhz();

    let p = hal::init(config);

    // switch
    let mut PB22 = Input::new(p.PB22, Pull::Up);

    //&mut Output::new(p.PA8, Level::Low, OutputDrive::_5mA).degrade(),
    //&mut Output::new(p.PA9, Level::Low, OutputDrive::_5mA).degrade(),
    //&mut Output::new(p.PB16, Level::Low, OutputDrive::_5mA).degrade(),

    // Probably led pins

    let pins = [
        // xx
        &mut Flex::new(p.PB20).degrade(),
        &mut Flex::new(p.PB21).degrade(),
        &mut Flex::new(p.PB23).degrade(),
        &mut Flex::new(p.PB1).degrade(),
        &mut Flex::new(p.PB2).degrade(),
        &mut Flex::new(p.PB4).degrade(),
        &mut Flex::new(p.PB3).degrade(),
        &mut Flex::new(p.PA4).degrade(),
        &mut Flex::new(p.PB5).degrade(),
        &mut Flex::new(p.PB12).degrade(),
        &mut Flex::new(p.PB13).degrade(),
        &mut Flex::new(p.PB14).degrade(),
        &mut Flex::new(p.PB15).degrade(),
        &mut Flex::new(p.PB8).degrade(),
        &mut Flex::new(p.PB9).degrade(),
        &mut Flex::new(p.PA11).degrade(),
        &mut Flex::new(p.PA10).degrade(),
        &mut Flex::new(p.PA12).degrade(),
        &mut Flex::new(p.PB7).degrade(),
        &mut Flex::new(p.PB0).degrade(),
        &mut Flex::new(p.PB18).degrade(),
        &mut Flex::new(p.PA15).degrade(),
        // xx
        &mut Flex::new(p.PB19).degrade(),
        &mut Flex::new(p.PA1).degrade(),
    ];

    for i in 0..pins.len() {
        pins[i].set_as_input(Pull::None);
    }

    loop {
        for i in 0..pins.len() {
            for j in 0..pins.len() {
                if i != j {
                    pins[i].set_as_output(OUTPUT_DRIVE);
                    pins[j].set_as_output(OUTPUT_DRIVE);
                    //hal::delay_us(10);
                    pins[i].set_low();
                    pins[j].set_high();
                    //hal::delay_ms(5);
                    pins[i].set_high();
                    pins[j].set_low();
                    //hal::delay_ms(5);
                    pins[i].set_as_input(Pull::None);
                    pins[j].set_as_input(Pull::None);
                    //hal::delay_us(10);
                }
            }
        }
    }
}
