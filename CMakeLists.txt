set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_VERSION 1)
set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")
cmake_minimum_required(VERSION 3.20)

set(TOOLPATH riscv-none-elf-)

if(WIN32)
  message(STATUS "Now is windows!")
  set(CMAKE_C_COMPILER ${TOOLPATH}gcc.exe)
  set(CMAKE_CXX_COMPILER ${TOOLPATH}g++.exe)
  set(CMAKE_ASM_COMPILER ${TOOLPATH}gcc.exe)
  set(CMAKE_AR ${TOOLPATH}ar.exe)
  set(CMAKE_OBJCOPY ${TOOLPATH}objcopy.exe)
  set(CMAKE_OBJDUMP ${TOOLPATH}objdump.exe)
  set(SIZE ${TOOLPATH}size.exe)
elseif(UNIX)
  message(STATUS "Now is UNIX-like OS!")
  set(CMAKE_C_COMPILER ${TOOLPATH}gcc)
  set(CMAKE_CXX_COMPILER ${TOOLPATH}g++)
  set(CMAKE_ASM_COMPILER ${TOOLPATH}gcc)
  set(CMAKE_AR ${TOOLPATH}ar)
  set(CMAKE_OBJCOPY ${TOOLPATH}objcopy)
  set(CMAKE_OBJDUMP ${TOOLPATH}objdump)
  set(SIZE ${TOOLPATH}size)
else()
  message(STATUS "Unsupported system!")
endif()

set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

set(TOP_PROJECT_NAME "ch58x-ninja")

project(${TOP_PROJECT_NAME} C CXX ASM)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_C_STANDARD 99)

add_compile_options(-march=rv32imac_zicsr -mabi=ilp32
                    -mcmodel=medany -msmall-data-limit=8 -mno-save-restore)

add_compile_options(-fmessage-length=0 -fsigned-char -ffunction-sections
                    -fdata-sections -fno-common)

#set(CMAKE_BUILD_TYPE "Debug")
SET(CMAKE_BUILD_TYPE "Release")

if(CMAKE_BUILD_TYPE AND (CMAKE_BUILD_TYPE STREQUAL "Debug"))
  set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -Wall -O1")
  set(CMAKE_CXX_FLAGS_DEBUG "-DDEBUG -O1 -g")
  set(CMAKE_ASM_FLAGS_DEBUG "-DDEBUG -O1 -g")
  message("Debug mode:${CMAKE_C_FLAGS_DEBUG}")
elseif(CMAKE_BUILD_TYPE AND (CMAKE_BUILD_TYPE STREQUAL "Release"))
  set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -Wall -O1")
  set(CMAKE_CXX_FLAGS_RELEASE "-DNDEBUG -O1") # -flto
  set(CMAKE_ASM_FLAGS_RELEASE "-DNDEBUG -O1")
  message("Release mode:${CMAKE_C_FLAGS_RELEASE}")
else()
  message("else:${CMAKE_BUILD_TYPE}")
  message("else:${CMAKE_C_FLAGS_RELEASE}")
endif()

include_directories(
  CH58X-CMake/Dependency/BLE_LIB
  CH58X-CMake/Dependency/USB_LIB
  CH58X-CMake/Dependency/RVMSIS
  CH58X-CMake/Dependency/StdPeriphDriver/inc
  CH58X-CMake/Dependency/printf
  CH58X-CMake/Dependency/HAL/include
  src/inc
)

#add_definitions(-DDEBUG=1)

file(
  GLOB_RECURSE
  SOURCES
  "CH58X-CMake/Dependency/Startup/*.S"
  "CH58X-CMake/Dependency/USB_LIB/*.c"
  "CH58X-CMake/Dependency/RVMSIS/*.c"
  "CH58X-CMake/Dependency/StdPeriphDriver/*.c"
  "CH58X-CMake/Dependency/printf/printf.c"
  "CH58X-CMake/Dependency/HAL/*.c"
  "src/*.c"
  "src/*.cpp"
)

set(LINKER_SCRIPT ${CMAKE_SOURCE_DIR}/CH58X-CMake/Dependency/Ld/Link.ld)
add_link_options(
  -march=rv32imac
  -mabi=ilp32
  -nostartfiles
  -Xlinker
  --gc-sections
  -Wl,--print-memory-usage
  -Wl,-Map,${PROJECT_NAME}.map
  --specs=nano.specs
  --specs=nosys.specs)
add_link_options(-T ${LINKER_SCRIPT})

add_executable(${PROJECT_NAME}.elf ${SOURCES} ${LINKER_SCRIPT})


target_link_libraries(
  ${PROJECT_NAME}.elf
  ${CMAKE_SOURCE_DIR}/CH58X-CMake/Dependency/StdPeriphDriver/libISP583.a
  ${CMAKE_SOURCE_DIR}/CH58X-CMake/Dependency/BLE_LIB/LIBCH58xBLE.a
  ${CMAKE_SOURCE_DIR}/CH58X-CMake/Dependency/USB_LIB/libRV3UFI.a)

if(CMAKE_BUILD_TYPE AND (CMAKE_BUILD_TYPE STREQUAL "Debug"))
  set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/build/debug) # ELF
  set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/build/debug)
  set(PROJECT_BINARY_DIR ${PROJECT_SOURCE_DIR}/build/debug) # Hex/build
elseif(CMAKE_BUILD_TYPE AND (CMAKE_BUILD_TYPE STREQUAL "Release"))
  set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/build/release) # ELF
  set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/build/release)
  set(PROJECT_BINARY_DIR ${PROJECT_SOURCE_DIR}/build/release) # Hex/Bin
endif()

set(HEX_FILE ${PROJECT_BINARY_DIR}/${PROJECT_NAME}.hex)
set(BIN_FILE ${PROJECT_BINARY_DIR}/${PROJECT_NAME}.bin)
add_custom_command(
  TARGET ${PROJECT_NAME}.elf
  POST_BUILD
  COMMAND ${CMAKE_OBJCOPY} -Oihex $<TARGET_FILE:${PROJECT_NAME}.elf> ${HEX_FILE}
  COMMAND ${CMAKE_OBJCOPY} -Obinary $<TARGET_FILE:${PROJECT_NAME}.elf>
          ${BIN_FILE})
