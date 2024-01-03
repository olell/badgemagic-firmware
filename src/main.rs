#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern crate ch58x_hal as hal;
use hal::gpio::{Flex, Input, OutputDrive, Pull};

extern crate embedded_hal;

const OUTPUT_DRIVE: OutputDrive = OutputDrive::_5mA;

#[ch32v_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_32mhz();

    let p = hal::init(config);

    // switch
    let mut button = Input::new(p.PB22, Pull::Up);

    let pixels: [[bool; 44]; 11] = [
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
        [
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            false, false, true, false, false, true, false, false, true, false, false, true, false,
            false, false, false, true, false, false, true, false, false, true, false, false, false,
            false, true, false, false, true,
        ],
    ];

    let pins = [
        // not a row
        &mut Flex::new(p.PB19).degrade(),
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
    ];

    let mut col = 1;
    let mut x = 0;
    let mut y = 0;

    let mut realx = 0;

    loop {
        x = (col - 1) * 2;
        pins[col].set_as_output(OUTPUT_DRIVE);
        pins[col].set_high();
        y = 0;
        for px in 0..pins.len() {
            if px != col {
                realx = x + (y % 2);
                if (realx % 3) == 0 && (y % 3) == 0 {
                    pins[px].set_as_output(OUTPUT_DRIVE);
                    pins[px].set_low();
                    pins[px].set_as_input(Pull::None);
                }
                y += 1;
            }
        }
        pins[col].set_as_input(Pull::None);

        col = col + 1;
        if col >= pins.len() {
            col = 1;
        }
    }
}
