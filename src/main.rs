#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern crate ch58x_hal as hal;

extern crate embedded_hal;

const REGISTER_R32_PA_DIR: *mut u64 = 0x400010A0 as *mut u64;
const REGISTER_R32_PB_DIR: *mut u64 = 0x400010C0 as *mut u64;
const REGISTER_R32_PA_OUT: *mut u64 = 0x400010A8 as *mut u64;
const REGISTER_R32_PB_OUT: *mut u64 = 0x400010C8 as *mut u64;

#[ch32v_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_80mhz();

    //let p = hal::init(config);

    // switch
    //let mut button = Input::new(p.PB22, Pull::Up);

    /*let pins = [
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
    ];*/

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
    ]; // todo fill with real values

    let mut register_mask: u64 = 0;
    for pin in register_map {
        register_mask |= pin;
    }

    let pixels = [
        [
            false, false, true, true, false, false, true, true, true, false, true, true, false,
            false, true, true, true, false, true, true, true, false, false, true, false, true,
            false, true, true, true, false, true, true, true, false, true, true, true, false, true,
            true, true, false, false,
        ],
        [
            false, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false, false, false, true, false, false, false, false, true, true, true,
            false, true, false, true, false, true, false, false, false, false, true, false, false,
            true, false, false, false, false,
        ],
        [
            false, false, true, true, false, false, true, true, true, false, true, false, true,
            false, true, false, false, false, true, true, false, false, false, true, false, true,
            false, true, true, true, false, true, false, false, false, false, true, false, false,
            true, false, false, false, false,
        ],
        [
            false, false, true, false, true, false, true, false, true, false, true, false, true,
            false, true, false, true, false, true, false, false, false, false, true, false, true,
            false, true, false, true, false, true, false, true, false, false, true, false, false,
            true, false, false, false, false,
        ],
        [
            false, false, true, true, false, false, true, false, true, false, true, true, false,
            false, true, true, true, false, true, true, true, false, false, true, false, true,
            false, true, false, true, false, true, true, true, false, true, true, true, false,
            true, true, true, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, true, true, false, true, true, true, false, true, true, true,
            false, true, true, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, false, false, false, true, false, true, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, true, false, false, true, false, true, false, true, true, true,
            false, true, true, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, false, false, false, true, false, true, false, false, false, true,
            false, false, false, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false,
        ],
        [
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, false, false, false, true, true, true, false, true, true, true,
            false, true, true, true, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
        ],
    ];

    // matrix controller
    let mut matrix_x: usize;
    let mut matrix_y: usize;

    let mut pin_counter: usize;
    let mut current_pin: u64;

    let mut direction_registers: u64;
    let mut output_registers: u64;

    let mut previous_dir: u64;
    let mut previous_output: u64;

    loop {
        // read prev dir a / b
        unsafe {
            // read current state of direction and output register, so this code doesn't affect other peripherals on PA/PB
            previous_dir = core::ptr::read_volatile(REGISTER_R32_PB_DIR);
            previous_dir |= core::ptr::read_volatile(REGISTER_R32_PA_DIR) << 32;
            previous_output = core::ptr::read_volatile(REGISTER_R32_PB_OUT);
            previous_output |= core::ptr::read_volatile(REGISTER_R32_PA_OUT) << 32;
        }
        for col_idx in 0..22 {
            direction_registers = previous_dir & !register_mask;
            output_registers = previous_output & !register_mask;

            direction_registers |= register_map[col_idx + 1]; // maybe optimize: remove duplicate +1 (see line below)
            output_registers |= register_map[col_idx + 1];

            pin_counter = 0;
            for px_idx in 0..22 {
                matrix_x = (col_idx << 1) | (px_idx & 1);
                matrix_y = px_idx >> 1;

                current_pin = register_map[pin_counter];
                if (direction_registers & current_pin) > 0 {
                    pin_counter += 1;
                    current_pin = register_map[pin_counter];
                }

                if pixels[matrix_y][matrix_x] {
                    direction_registers |= current_pin; // setting pin to output
                    output_registers &= !current_pin; // setting pin to low
                }

                pin_counter += 1;
            }
            // write registers
            unsafe {
                core::ptr::write_volatile(REGISTER_R32_PB_DIR, direction_registers & 0xffffffff);
                core::ptr::write_volatile(REGISTER_R32_PA_DIR, direction_registers >> 32);
                core::ptr::write_volatile(REGISTER_R32_PB_OUT, output_registers & 0xffffffff);
                core::ptr::write_volatile(REGISTER_R32_PA_OUT, output_registers >> 32);
            }
        }
        /*
        // Do this before returning a function doing the stuff
        unsafe {
            core::ptr::write_volatile(REGISTER_R32_PB_DIR, previous_dir & 0xffffffff);
            core::ptr::write_volatile(REGISTER_R32_PA_DIR, previous_dir >> 32);
            core::ptr::write_volatile(REGISTER_R32_PB_OUT, previous_output & 0xffffffff);
            core::ptr::write_volatile(REGISTER_R32_PA_OUT, previous_output >> 32);
        }
        */
    }
}
