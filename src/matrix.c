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

#include "matrix.h"

#include "HAL.h"
#include "stdlib.h"
#include "string.h"

#define MATRIX_WIDTH 44
#define MATRIX_HEIGHT 11
#define PIX_BUFFER_SIZE 61  // 11x44 pixels / 8Bit = 60.5 Byte

void printBinary(int value) {
    int i;
    for (i = 31; i >= 0; i--) {
        PRINT("%d", (value >> i) & 1);
        if (i % 4 == 0)
            PRINT(" ");  // Add space every 4 digits for better readability
    }
}

uint8_t *pixelBuffer;

// various indicies
uint32_t pixelIndex;
uint32_t columnIndex;
uint32_t pinIndex;
uint32_t bitIndex;
uint32_t byteIndex;

// temporary buffer to craft the new register value
uint64_t pinBuffer;

// some flags
uint32_t isBright;
uint8_t isLocked;

// newly crafted value, value when matrixInit() was called
uint64_t pinsUsed;
uint64_t outReg;
uint64_t dirReg;

uint32_t CLEAN_R32_PA_PD_DRV;
uint32_t CLEAN_R32_PB_PD_DRV;
uint32_t CLEAN_R32_PA_OUT;
uint32_t CLEAN_R32_PB_OUT;
uint32_t CLEAN_R32_PA_DIR;
uint32_t CLEAN_R32_PB_DIR;

const uint64_t REGISTER_MAP[23] = {
    // Port A                             Port B
    //      24      16       8       0      24      16       8       0
    0b0000000000000000000000000000000000000000000010000000000000000000,  // PB19
    0b0000000000000000000000000000000000000000000100000000000000000000,  // PB20
    0b0000000000000000000000000000000000000000001000000000000000000000,  // PB21
    0b0000000000000000000000000000000000000000100000000000000000000000,  // PB23
    0b0000000000000000000000000000000000000000000000000000000000000010,  // PB1
    0b0000000000000000000000000000000000000000000000000000000000000100,  // PB2
    0b0000000000000000000000000000000000000000000000000000000000010000,  // PB4
    0b0000000000000000000000000000000000000000000000000000000000001000,  // PB3
    0b0000000000000000000000000001000000000000000000000000000000000000,  // PA4
    0b0000000000000000000000000000000000000000000000000000000000100000,  // PB5
    0b0000000000000000000000000000000000000000000000000001000000000000,  // PB12
    0b0000000000000000000000000000000000000000000000000010000000000000,  // PB13
    0b0000000000000000000000000000000000000000000000000100000000000000,  // PB14
    0b0000000000000000000000000000000000000000000000001000000000000000,  // PB15
    0b0000000000000000000000000000000000000000000000000000000100000000,  // PB8
    0b0000000000000000000000000000000000000000000000000000001000000000,  // PB9
    0b0000000000000000000010000000000000000000000000000000000000000000,  // PA11
    0b0000000000000000000001000000000000000000000000000000000000000000,  // PA10
    0b0000000000000000000100000000000000000000000000000000000000000000,  // PA12
    0b0000000000000000000000000000000000000000000000000000000010000000,  // PB7
    0b0000000000000000000000000000000000000000000000000000000000000001,  // PB0
    0b0000000000000000000000000000000000000000000001000000000000000000,  // PB18
    0b0000000000000000100000000000000000000000000000000000000000000000,  // PA15
};

const uint64_t REGISTER_MASK_PA = 0xffff63ef;
const uint64_t REGISTER_MASK_PB = 0xff430c40;  // ~(SUM(REGISTER_MAP))

/**
 * Initialize the LED matrix driver
 */
void matrixInit() {
    // allocate memory for pixel buffer
    pixelBuffer = (uint8_t *)malloc(sizeof(uint8_t) * PIX_BUFFER_SIZE);
    memset(pixelBuffer, 0, sizeof(uint8_t) * PIX_BUFFER_SIZE);

    // init indicies, and other vars
    pixelIndex = 0;
    columnIndex = 0;
    pinIndex = 0;
    bitIndex = 0;
    byteIndex = 0;
    pinBuffer = 0;
    isBright = 0;
    isLocked = 0;
}

/**
 * Set the matrix to bright (v > 0) or dark (v == 0) mode
 */
void setBrightness(uint8_t v) {
    if (v > 0) {
        R32_PA_PD_DRV |= ~REGISTER_MASK_PA;
        R32_PB_PD_DRV |= ~REGISTER_MASK_PB;
    } else {
        R32_PA_PD_DRV &= REGISTER_MASK_PA;
        R32_PB_PD_DRV &= REGISTER_MASK_PB;
    }
}

/**
 * Set a specific bit (x, y) in the pixelBuffer to the value of v
 */
void setPixel(uint8_t x, uint8_t y, uint8_t v) {
    pixelIndex = y * MATRIX_WIDTH + x;
    byteIndex = pixelIndex >> 3;
    bitIndex = pixelIndex & 7;

    // Setting the pixelIndex'th bit to v
    pixelBuffer[byteIndex] =
        ((pixelBuffer[byteIndex] & ~((uint8_t)1 << bitIndex)) |
         (v << bitIndex));
}

/**
 * Returns the pointer to the pixel buffer
 */
uint8_t *getPixelBuffer() { return pixelBuffer; }

void matrixDisplay() {
    // if (isLocked) return;

    CLEAN_R32_PA_OUT = R32_PA_OUT & REGISTER_MASK_PA;
    CLEAN_R32_PB_OUT = R32_PB_OUT & REGISTER_MASK_PB;
    CLEAN_R32_PA_DIR = R32_PA_DIR & REGISTER_MASK_PA;
    CLEAN_R32_PB_DIR = R32_PB_DIR & REGISTER_MASK_PB;

    // iterate for each column
    while (columnIndex < 22) {
        byteIndex = columnIndex + 1;

        pinBuffer = REGISTER_MAP[byteIndex];
        dirReg = pinBuffer;
        outReg = pinBuffer;

        pinIndex = 0;
        pixelIndex = 0;
        while (pixelIndex < 22) {
            // skip if pin is the column anode pin
            if (byteIndex == pinIndex) pinIndex++;

            bitIndex = ((pixelIndex >> 1) * 44) +
                       ((columnIndex << 1) | (pixelIndex & 1));
            // checking bit in array self.pixels at self.bit_idx
            if (pixelBuffer[bitIndex >> 3] & (1 << (bitIndex & 7))) {
                dirReg |= REGISTER_MAP[pinIndex];  // set as output
                // implying that self.reg_out at the place is 0
            }
            pinIndex++;
            pixelIndex++;
        }

        R32_PA_OUT = CLEAN_R32_PA_OUT | (outReg >> 32);
        R32_PB_OUT = CLEAN_R32_PB_OUT | outReg;
        R32_PA_DIR = CLEAN_R32_PA_DIR | (dirReg >> 32);
        R32_PB_DIR = CLEAN_R32_PB_DIR | dirReg;

        columnIndex++;
    }
    columnIndex = 0;
}