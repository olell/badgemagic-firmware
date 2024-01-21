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
#include "CH58x_common.h"
#include "HAL.h"
#include "matrix.h"

static volatile uint32_t millisCounter = 0;
static volatile uint64_t millis = 0;

static void delay_ms(uint64_t ms) {
    uint64_t dest = millis + ms;
    while (millis > dest)  // if dest overflowed
        ;
    while (millis < dest)
        ;
}

/**
 * Initialises UART 1
 */
void SerialInit() {
    GPIOA_SetBits(GPIO_Pin_9);
    GPIOA_ModeCfg(GPIO_Pin_8, GPIO_ModeIN_PU);
    GPIOA_ModeCfg(GPIO_Pin_9, GPIO_ModeOut_PP_5mA);
    UART1_DefInit();
}

/**
 * Main function, is executed after reset
 */
int main() {
    SetSysClock(CLK_SOURCE_PLL_60MHz);
    SerialInit();

    HAL_Init();

    PRINT("Hello, world!\n");

    matrixInit();
    TMR0_TimerInit(FREQ_SYS / 11000);
    TMR0_ITCfg(ENABLE, TMR0_3_IT_CYC_END);
    PFIC_EnableIRQ(TMR0_IRQn);
    uint8_t* pixels = getPixelBuffer();
    uint16_t index = 0;

    while (1) {
        pixels[index >> 3] ^= (uint8_t)(1 << (index & 7));
        index = (index + 1) % 484;
        delay_ms(10);
    }

    return 0;
}

__INTERRUPT
__HIGH_CODE
void TMR0_IRQHandler(void) {
    if (TMR0_GetITFlag(TMR0_3_IT_CYC_END)) {
        TMR0_ClearITFlag(TMR0_3_IT_CYC_END);
        millisCounter++;            // 11000 times per second
        if (millisCounter == 11) {  // 1000 times per second
            millis++;
            millisCounter = 0;
        }
        matrixDisplay();
    }
}