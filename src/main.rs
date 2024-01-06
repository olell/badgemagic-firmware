#![no_std]
#![no_main]
#![feature(core_intrinsics)]
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
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern crate ch58x_hal as hal;
extern crate embedded_hal;

mod matrix;

#[ch32v_rt::entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_32mhz();

    let p = hal::init(config);

    // switch
    let button = hal::gpio::Input::new(p.PB22, hal::gpio::Pull::Up);

    let mut matrix = matrix::MatrixController::new();
    matrix.init();
    matrix.is_bright = true;

    let pixels = [
        0, 0, 0, 0, 0, 224, 103, 224, 103, 0, 2, 126, 6, 126, 6, 64, 96, 102, 96, 96, 224, 8, 102,
        6, 6, 6, 128, 96, 102, 224, 97, 0, 8, 102, 6, 30, 6, 142, 96, 102, 96, 96, 0, 4, 126, 126,
        126, 126, 32, 224, 231, 231, 231, 7, 0, 0, 0, 0, 0, 0, 0,
    ];

    // set pixels:
    // bit = y * 44 + x
    // to 1
    // pixels[bit >> 3] |= 1 << (bit & 3);
    // to 0
    // pixels[bit >> 3] &= !(1 << (bit & 3));

    matrix.copy_pixels(pixels);

    loop {
        matrix.display();
    }
}
