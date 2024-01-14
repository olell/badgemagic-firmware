#define DEBUG  // enable debug (serial print statements)

#include "CH58x_common.h"
#include "HAL.h"

/**
 * Initialises UART 1
 */
void DebugInit(void) {
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
    DebugInit();
    HAL_Init();

    PRINT("Hello, world!\n");

    while (1) {
    }

    return 0;
}