#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern crate ch58x_hal as hal;
use hal::gpio::{Level, Output, OutputDrive};

#[ch32v_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_32mhz();
    config.enable_dcdc = false;
    config.low_power = true;

    let p = hal::init(config);

    let mut PA7 = Output::new(p.PA7, Level::Low, OutputDrive::_5mA);
    let mut PA8 = Output::new(p.PA8, Level::Low, OutputDrive::_5mA);
    let mut PA9 = Output::new(p.PA9, Level::Low, OutputDrive::_5mA);
    let mut PB9 = Output::new(p.PB9, Level::Low, OutputDrive::_5mA);
    let mut PB8 = Output::new(p.PB8, Level::Low, OutputDrive::_5mA);
    let mut PB16 = Output::new(p.PB16, Level::Low, OutputDrive::_5mA);
    let mut PB15 = Output::new(p.PB15, Level::Low, OutputDrive::_5mA);
    let mut PB14 = Output::new(p.PB14, Level::Low, OutputDrive::_5mA);
    let mut PB13 = Output::new(p.PB13, Level::Low, OutputDrive::_5mA);
    let mut PB12 = Output::new(p.PB12, Level::Low, OutputDrive::_5mA);
    let mut PB7 = Output::new(p.PB7, Level::Low, OutputDrive::_5mA);
    let mut PB5 = Output::new(p.PB5, Level::Low, OutputDrive::_5mA);
    let mut PB4 = Output::new(p.PB4, Level::Low, OutputDrive::_5mA);
    let mut PB3 = Output::new(p.PB3, Level::Low, OutputDrive::_5mA);
    let mut PB2 = Output::new(p.PB2, Level::Low, OutputDrive::_5mA);
    let mut PB1 = Output::new(p.PB1, Level::Low, OutputDrive::_5mA);
    let mut PB0 = Output::new(p.PB0, Level::Low, OutputDrive::_5mA);
    let mut PB23 = Output::new(p.PB23, Level::Low, OutputDrive::_5mA);
    let mut PB22 = Output::new(p.PB22, Level::Low, OutputDrive::_5mA);
    let mut PB21 = Output::new(p.PB21, Level::Low, OutputDrive::_5mA);
    let mut PB20 = Output::new(p.PB20, Level::Low, OutputDrive::_5mA);
    let mut PB19 = Output::new(p.PB19, Level::Low, OutputDrive::_5mA);
    let mut PB18 = Output::new(p.PB18, Level::Low, OutputDrive::_5mA);
    let mut PA4 = Output::new(p.PA4, Level::Low, OutputDrive::_5mA);
    let mut PA5 = Output::new(p.PA5, Level::Low, OutputDrive::_5mA);
    let mut PA6 = Output::new(p.PA6, Level::Low, OutputDrive::_5mA);
    let mut PA0 = Output::new(p.PA0, Level::Low, OutputDrive::_5mA);
    let mut PA1 = Output::new(p.PA1, Level::Low, OutputDrive::_5mA);
    let mut PA2 = Output::new(p.PA2, Level::Low, OutputDrive::_5mA);
    let mut PA3 = Output::new(p.PA3, Level::Low, OutputDrive::_5mA);
    let mut PA15 = Output::new(p.PA15, Level::Low, OutputDrive::_5mA);
    let mut PA14 = Output::new(p.PA14, Level::Low, OutputDrive::_5mA);
    let mut PA13 = Output::new(p.PA13, Level::Low, OutputDrive::_5mA);
    let mut PA12 = Output::new(p.PA12, Level::Low, OutputDrive::_5mA);
    let mut PA11 = Output::new(p.PA11, Level::Low, OutputDrive::_5mA);
    let mut PA10 = Output::new(p.PA10, Level::Low, OutputDrive::_5mA);

    loop {
        unsafe {
            PB7.toggle();

            hal::delay_ms(1000);
        }
    }
}
