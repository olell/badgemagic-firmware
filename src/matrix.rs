#![no_std]
/**
 *
 * Open Source alternative Firmware for Badgemagic (11x44 pixel led nametags)
 *
 * Copyright 2024 olel
 *
 * Apache License
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *       http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 *
 * The MIT License (MIT)
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the “Software”), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */
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

    reg_dir_a: u64,
    reg_out_a: u64,
    reg_drv_a: u64,

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
    bit_idx: usize,

    // if true, the display doesn't perform any action
    is_locked: bool,

    // if true i/o s are set to 20mA mode
    pub is_bright: bool,

    // pixel buffer bit idx = (y * 44 + x)
    pub pixels: [u8; 61],
}

impl MatrixController {
    pub fn new() -> Self {
        Self {
            reg_dir: 0,
            reg_out: 0,
            reg_drv: 0,
            reg_dir_a: 0,
            reg_out_a: 0,
            reg_drv_a: 0,
            clean_reg_dir: 0,
            clean_reg_out: 0,
            clean_reg_drv: 0,
            pin_buffer: 0,
            pin_idx: 0,
            col_idx: 0,
            pix_idx: 0,
            bit_idx: 0,
            is_locked: false,
            is_bright: true,
            pixels: [0; 61],
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

    pub fn copy_pixels(&mut self, new_pixels: [u8; 61]) {
        self.lock();

        self.pix_idx = 0;
        while self.pix_idx < 61 {
            self.pixels[self.pix_idx] = new_pixels[self.pix_idx];
            self.pix_idx += 1;
        }

        self.col_idx = 0;
        self.pix_idx = 0;
        self.pin_idx = 0;

        self.unlock();
    }

    pub fn set_brightness(&mut self, state: bool) {
        self.is_bright = state;
    }

    pub fn lock(&mut self) {
        self.is_locked = true;

        self.col_idx = 0;
        self.pix_idx = 0;
        self.pin_idx = 0;
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
        if self.is_bright {
            self.reg_drv |= self.pin_buffer; // set as 20mA output
        }

        self.pin_idx = 0;
        self.pix_idx = 0;
        while self.pix_idx < 22 {
            // skip pin if its the columns common-anode pin
            if self.col_idx + 1 == self.pin_idx {
                self.pin_idx += 1;
            }

            // calculate which bit represents the pixel
            // y   = row / 2
            // x   = (col * 2) + (row % 2)
            // bit = (y * 44 + x)
            self.bit_idx = ((self.pix_idx >> 1) * 44) + ((self.col_idx << 1) | (self.pix_idx & 1));
            // checking bit in array self.pixels at self.bit_idx
            if (self.pixels[self.bit_idx >> 3] & (1 << (self.bit_idx & 7))) != 0 {
                self.pin_buffer = REGISTER_MAP[self.pin_idx];
                self.reg_dir |= self.pin_buffer; // set as output
                if self.is_bright {
                    self.reg_drv |= self.pin_buffer; // set as 20mA output
                }
                // implying that self.reg_out at the place is 0
            }

            self.pin_idx += 1;
            self.pix_idx += 1;
        }

        // write freshly crafted registers
        unsafe {
            self.reg_dir_a = self.reg_dir >> 32;
            self.reg_out_a = self.reg_out >> 32;
            self.reg_drv_a = self.reg_drv >> 32;
            intrinsics::unaligned_volatile_store(REGISTER_R32_PA_DIR, self.reg_dir_a);
            intrinsics::unaligned_volatile_store(REGISTER_R32_PB_DIR, self.reg_dir);
            intrinsics::unaligned_volatile_store(REGISTER_R32_PA_OUT, self.reg_out_a);
            intrinsics::unaligned_volatile_store(REGISTER_R32_PB_OUT, self.reg_out);
            intrinsics::unaligned_volatile_store(REGISTER_R32_PA_DRV, self.reg_drv_a);
            intrinsics::unaligned_volatile_store(REGISTER_R32_PB_DRV, self.reg_drv);
        }

        // increase column index, wrap around at 22
        self.col_idx += 1;
        if self.col_idx == 22 {
            self.col_idx = 0;
        }
    }
}
