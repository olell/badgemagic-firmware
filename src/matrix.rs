#![no_std]

use core::intrinsics;

const REGISTER_R32_PA_DIR: *mut u64 = 0x400010A0 as *mut u64;
const REGISTER_R32_PB_DIR: *mut u64 = 0x400010C0 as *mut u64;
const REGISTER_R32_PA_OUT: *mut u64 = 0x400010A8 as *mut u64;
const REGISTER_R32_PB_OUT: *mut u64 = 0x400010C8 as *mut u64;
const REGISTER_R32_PA_DRV: *mut u64 = 0x400010B4 as *mut u64;
const REGISTER_R32_PB_DRV: *mut u64 = 0x400010D4 as *mut u64;

const REGISTER_MAP: [u64; 23] = [
    // Port A                             Port B
    //      24       16        8        0       24       16        8        0
    0b00000000_00000000_00000000_00000000_00000000_00001000_00000000_00000000, // PB19
    0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000, // PB20
    0b00000000_00000000_00000000_00000000_00000000_00100000_00000000_00000000, // PB21
    0b00000000_00000000_00000000_00000000_00000000_10000000_00000000_00000000, // PB23
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010, // PB1
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100, // PB2
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000, // PB4
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

// Every bit of REGISTER_MAP to mask not-used pins
const REGISTER_MASK: u64 = 0xffff63efff430c40;

pub struct MatrixController {
    // variables to craft the new register value in
    reg_dir: u64,
    reg_out: u64,
    reg_drv: u64,

    // to restore the state
    clean_reg_dir: u64,
    clean_reg_out: u64,
    clean_reg_drv: u64,

    // temporary buffer to craft the new register value
    pin_buffer: u64,

    // iterator vars
    pin_idx: usize,
    col_idx: usize,
    pix_idx: usize,

    // if true, the display doesn't perform any action
    is_locked: bool,

    // pixel buffer idx = (y * 44 + x)
    pub pixels: [u8; 484],
}

impl MatrixController {
    pub fn new() -> Self {
        Self {
            reg_dir: 0,
            reg_out: 0,
            reg_drv: 0,
            clean_reg_dir: 0,
            clean_reg_out: 0,
            clean_reg_drv: 0,
            pin_buffer: 0,
            pin_idx: 0,
            col_idx: 0,
            pix_idx: 0,
            is_locked: false,
            pixels: [0; 484],
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.clean_reg_dir = intrinsics::volatile_load(REGISTER_R32_PB_DIR);
            self.clean_reg_dir = intrinsics::volatile_load(REGISTER_R32_PA_DIR) << 32;
            self.clean_reg_dir &= REGISTER_MASK;

            self.clean_reg_out = intrinsics::volatile_load(REGISTER_R32_PB_OUT);
            self.clean_reg_out |= intrinsics::volatile_load(REGISTER_R32_PA_OUT) << 32;
            self.clean_reg_out &= REGISTER_MASK;

            self.clean_reg_drv = intrinsics::volatile_load(REGISTER_R32_PB_DRV);
            self.clean_reg_drv |= intrinsics::volatile_load(REGISTER_R32_PA_DRV) << 32;
            self.clean_reg_drv &= REGISTER_MASK;
        }
    }

    pub fn copy_pixels(&mut self, new_pixels: [u8; 484]) {
        self.lock();

        self.pix_idx = 0;
        while self.pix_idx < 484 {
            self.pixels[self.pix_idx] = new_pixels[self.pix_idx];
            self.pix_idx += 1;
        }

        self.col_idx = 0;
        self.pix_idx = 0;
        self.pin_idx = 0;

        self.unlock();
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    pub fn display(&mut self) {
        // Do not act if locked
        if self.is_locked {
            return;
        }

        // reset states to clean values
        self.reg_dir = self.clean_reg_dir;
        self.reg_out = self.clean_reg_out;
        self.reg_drv = self.clean_reg_drv;

        // column common-anode pin
        self.pin_buffer = REGISTER_MAP[self.col_idx + 1]; // first pin in register_map isn't an column anode
        self.reg_dir |= self.pin_buffer; // set as output
        self.reg_out |= self.pin_buffer; // set high level
        self.reg_drv |= self.pin_buffer; // set as 20mA output

        self.pin_idx = 0;
        self.pix_idx = 0;
        while self.pix_idx < 22 {
            // skip pin if its the columns common-anode pin
            if self.col_idx + 1 == self.pin_idx {
                self.pin_idx += 1;
            }

            // check if the pixel is on
            if self.pixels[((self.pix_idx >> 1) * 44) + ((self.col_idx << 1) | (self.pix_idx & 1))]
                > 0
            // test if "as bool" is faster
            {
                self.pin_buffer = REGISTER_MAP[self.pin_idx];
                self.reg_dir |= self.pin_buffer; // set as output
                self.reg_drv |= self.pin_buffer; // set as 20mA output
                                                 // implying that self.reg_out at the place is 0
            }

            self.pin_idx += 1;
            self.pix_idx += 1;
        }

        // write freshly crafted registers
        unsafe {
            intrinsics::volatile_store(REGISTER_R32_PB_DIR, self.reg_dir & 0xffffffff);
            intrinsics::volatile_store(REGISTER_R32_PA_DIR, self.reg_dir >> 32);
            intrinsics::volatile_store(REGISTER_R32_PB_DRV, self.reg_drv & 0xffffffff);
            intrinsics::volatile_store(REGISTER_R32_PA_DRV, self.reg_drv >> 32);
            intrinsics::volatile_store(REGISTER_R32_PB_OUT, self.reg_out & 0xffffffff);
            intrinsics::volatile_store(REGISTER_R32_PA_OUT, self.reg_out >> 32);
        }

        // increase column index, wrap around at 22
        self.col_idx += 1;
        if self.col_idx == 22 {
            self.col_idx = 0;
        }
    }
}
