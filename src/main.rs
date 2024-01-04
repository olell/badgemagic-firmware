#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern crate ch58x_hal as hal;

extern crate embedded_hal;

use core::intrinsics;

const REGISTER_R32_PA_DIR: *mut u64 = 0x400010A0 as *mut u64;
const REGISTER_R32_PB_DIR: *mut u64 = 0x400010C0 as *mut u64;
const REGISTER_R32_PA_OUT: *mut u64 = 0x400010A8 as *mut u64;
const REGISTER_R32_PB_OUT: *mut u64 = 0x400010C8 as *mut u64;
const REGISTER_R32_PA_DRV: *mut u64 = 0x400010B4 as *mut u64;
const REGISTER_R32_PB_DRV: *mut u64 = 0x400010D4 as *mut u64;

#[ch32v_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_80mhz();

    let p = hal::init(config);

    // switch
    let mut button = hal::gpio::Input::new(p.PB22, hal::gpio::Pull::Up);

    let mut pixels = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, true, false, false, true, true, false,
        true, true, true, false, true, true, true, false, true, false, true, false, false, false,
        true, true, true, false, true, false, true, false, true, true, true, false, false, false,
        false, false, false, false, false, false, true, false, false, true, true, false, false,
        true, true, false, true, false, true, false, true, false, false, false, true, true, false,
        false, false, false, false, true, false, false, true, true, true, false, true, true, false,
        false, false, false, false, false, false, false, false, false, false, false, false, true,
        true, false, false, true, true, false, true, true, true, false, true, false, false, false,
        true, false, true, false, false, false, false, true, false, false, true, true, true, false,
        true, false, false, false, false, false, false, false, true, false, true, false, true,
        false, false, true, true, true, true, true, true, false, true, false, true, false, true,
        true, true, false, true, false, true, false, false, false, false, true, false, false, true,
        false, true, false, true, true, true, false, false, false, false, false, false, false,
        false, false, false, false, false, true, true, true, true, true, true, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, true, false, false, true, true, false, false,
        true, true, false, false, false, false, false, false, false, false, true, true, true,
        false, true, false, false, false, true, true, true, false, true, false, true, false, true,
        true, true, false, true, true, true, false, false, false, false, false, false, false, true,
        true, false, false, true, true, false, false, false, false, false, false, false, false,
        true, false, true, false, true, false, false, false, true, false, true, false, true, true,
        true, false, true, true, false, false, false, true, false, false, false, true, false, true,
        false, false, true, true, false, false, true, true, false, false, false, false, false,
        false, false, false, true, true, true, false, true, false, false, false, true, true, true,
        false, true, true, true, false, true, false, false, false, false, true, false, false,
        false, false, false, false, false, false, true, true, false, false, true, true, false,
        false, false, false, false, false, false, false, true, false, false, false, true, true,
        true, false, true, false, true, false, true, false, true, false, true, true, true, false,
        false, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false,
    ];

    let register_map: [u64; 23] = [
        // Port A                             Port B
        //      24       16        8        0       24       16        8        0
        0b00000000_00000000_00000000_00000000_00000000_00001000_00000000_00000000, // PB19    0
        0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000, // PB20    1
        0b00000000_00000000_00000000_00000000_00000000_00100000_00000000_00000000, // PB21    2
        0b00000000_00000000_00000000_00000000_00000000_10000000_00000000_00000000, // PB23    3
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010, // PB1     4
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100, // PB2     5
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000, // PB4     6
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000, // PB3
        0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000, // PA4
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100000, // PB5
        0b00000000_00000000_00000000_00000000_00000000_00000000_00010000_00000000, // PB12
        0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000000, // PB13
        0b00000000_00000000_00000000_00000000_00000000_00000000_01000000_00000000, // PB14
        0b00000000_00000000_00000000_00000000_00000000_00000000_10000000_00000000, // PB15
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000, // PB8
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000, // PB9
        0b00000000_00000000_00001000_00000000_00000000_00000000_00000000_00000000, // PA11
        0b00000000_00000000_00000100_00000000_00000000_00000000_00000000_00000000, // PA10
        0b00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000000, // PA12
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000, // PB7
        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001, // PB0
        0b00000000_00000000_00000000_00000000_00000000_00000100_00000000_00000000, // PB18
        0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000, // PA15
    ];

    let mut register_mask: u64 = 0;
    for pin in register_map {
        register_mask |= pin;
    }

    register_mask = !register_mask;

    // matrix controller
    let mut pin_counter: usize;
    let mut current_pin: u64;

    let mut direction_registers: u64;
    let mut output_registers: u64;
    let mut drv_registers: u64;

    let mut previous_dir: u64;
    let mut previous_output: u64;
    let mut previous_drv: u64;

    let mut col_idx = 0;
    let mut px_idx = 0;

    let mut iteration: u64 = 0;

    let mut y: isize = 0;
    let mut smy: isize = 0;
    let mut bgy: isize = 0;
    let mut x: isize = 0;
    let mut smx: isize = 0;
    let mut bgx: isize = 0;

    let mut c: usize = 0;

    let mut new_pixels: [bool; 484] = [false; 484];

    unsafe {
        // read current state of direction and output register, so this code doesn't affect other peripherals on PA/PB
        previous_dir = intrinsics::volatile_load(REGISTER_R32_PB_DIR);
        previous_dir = intrinsics::volatile_load(REGISTER_R32_PA_DIR) << 32;
        previous_dir &= register_mask;

        previous_output = intrinsics::volatile_load(REGISTER_R32_PB_OUT);
        previous_output |= intrinsics::volatile_load(REGISTER_R32_PA_OUT) << 32;
        previous_output &= register_mask;

        previous_drv = intrinsics::volatile_load(REGISTER_R32_PB_DRV);
        previous_drv |= intrinsics::volatile_load(REGISTER_R32_PA_DRV) << 32;
        previous_drv &= register_mask;
    }
    loop {
        // read prev dir a / b

        col_idx = 0;
        while col_idx < 22 {
            direction_registers = previous_dir;
            output_registers = previous_output;
            drv_registers = previous_drv;

            current_pin = register_map[col_idx + 1];
            direction_registers |= current_pin;
            output_registers |= current_pin;
            drv_registers |= current_pin;

            pin_counter = 0;
            px_idx = 0;
            while px_idx < 22 {
                if col_idx + 1 == pin_counter {
                    pin_counter += 1;
                }

                // pixels ((x position)  * width) + (y position)
                if pixels[((px_idx >> 1) * 44) + ((col_idx << 1) | (px_idx & 1))] {
                    current_pin = register_map[pin_counter];
                    direction_registers |= current_pin; // setting pin to output
                }

                pin_counter += 1;
                px_idx += 1;
            }
            // write registers
            unsafe {
                intrinsics::volatile_store(REGISTER_R32_PB_DRV, drv_registers & 0xffffffff);
                intrinsics::volatile_store(REGISTER_R32_PA_DRV, drv_registers >> 32);
                intrinsics::volatile_store(REGISTER_R32_PB_DIR, direction_registers & 0xffffffff);
                intrinsics::volatile_store(REGISTER_R32_PA_DIR, direction_registers >> 32);
                intrinsics::volatile_store(REGISTER_R32_PB_OUT, output_registers & 0xffffffff);
                intrinsics::volatile_store(REGISTER_R32_PA_OUT, output_registers >> 32);
            }
            col_idx += 1;
        }

        iteration += 1;
        if iteration > 200 {
            iteration = 0;
            y = 0;
            while y < 11 {
                x = 0;
                while x < 44 {
                    c = 0;
                    //if x > 0 && x < 43 && y > 0 && y < 10 {
                    smx = x - 1;
                    if smx < 0 {
                        smx = 43;
                    }
                    bgx = x + 1;
                    if bgx > 43 {
                        bgx = 0;
                    }
                    smy = y - 1;
                    if smy < 0 {
                        smy = 10;
                    }
                    bgy = y + 1;
                    if bgy > 10 {
                        bgy = 0;
                    }

                    c += pixels[(smy * 44 + smx) as usize] as usize;
                    c += pixels[(smy * 44 + x) as usize] as usize;
                    c += pixels[(smy * 44 + bgx) as usize] as usize;

                    c += pixels[(bgy * 44 + smx) as usize] as usize;
                    c += pixels[(bgy * 44 + x) as usize] as usize;
                    c += pixels[(bgy * 44 + bgx) as usize] as usize;

                    c += pixels[(y * 44 + smx) as usize] as usize;
                    c += pixels[(y * 44 + bgx) as usize] as usize;
                    //}

                    new_pixels[(y * 44 + x) as usize] = match c {
                        2 => pixels[(y * 44 + x) as usize],
                        3 => true,
                        _ => false,
                    };
                    x += 1;
                }
                y += 1;
            }

            y = 0;
            while y < 11 {
                x = 0;
                while x < 44 {
                    pixels[(y * 44 + x) as usize] = new_pixels[(y * 44 + x) as usize];
                    x += 1;
                }
                y += 1;
            }
        }
    }
}
