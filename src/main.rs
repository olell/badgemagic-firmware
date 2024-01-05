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
    let mut button = hal::gpio::Input::new(p.PB22, hal::gpio::Pull::Up);

    let mut matrix = matrix::MatrixController::new();
    matrix.init();

    let pixels = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let mut iteration: u64 = 0;
    let mut y: isize = 0;
    let mut smy: isize = 0;
    let mut bgy: isize = 0;
    let mut x: isize = 0;
    let mut smx: isize = 0;
    let mut bgx: isize = 0;
    let mut c: usize = 0;
    let mut new_pixels: [u8; 484] = [0; 484];

    matrix.copy_pixels(pixels);

    loop {
        matrix.display();

        // Game of Life test code
        iteration += 1;
        if iteration > 500 {
            if button.is_low() {
                matrix.copy_pixels(pixels);
            }
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

                    c += matrix.pixels[(smy * 44 + smx) as usize] as usize;
                    c += matrix.pixels[(smy * 44 + x) as usize] as usize;
                    c += matrix.pixels[(smy * 44 + bgx) as usize] as usize;

                    c += matrix.pixels[(bgy * 44 + smx) as usize] as usize;
                    c += matrix.pixels[(bgy * 44 + x) as usize] as usize;
                    c += matrix.pixels[(bgy * 44 + bgx) as usize] as usize;

                    c += matrix.pixels[(y * 44 + smx) as usize] as usize;
                    c += matrix.pixels[(y * 44 + bgx) as usize] as usize;
                    //}

                    new_pixels[(y * 44 + x) as usize] = match c {
                        2 => matrix.pixels[(y * 44 + x) as usize],
                        3 => 1,
                        _ => 0,
                    };
                    x += 1;
                }
                y += 1;
            }
            matrix.copy_pixels(new_pixels);
        }
    }
}
