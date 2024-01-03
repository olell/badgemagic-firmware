# Badgemagic Firmware

This repository contains an attempt to alternative FOSS firmware for the badgemagic led sign.

⚠️ \
**Note that the project is still quite incomplete and at an early stage. Flashing this code onto your badge will make it almost useless. It is not possible to revert to the original firmware!**\
⚠️

![pcb image top](hardware/pcb_top_stitched.png)
![pcb image bottom](hardware/pcb_bottom_stiched.png)

## Features / TODO

- [x] Build and flash firmware
- [x] Controlling LED matrix
- [ ] Reverse engineer schematic
- [ ] Reading side buttons
- [ ] USB (?)
- [ ] Battery voltage reading
- [ ] Bluetooth LE communication

## Hardware

For documentation / reverse engineering of the hardware (mostly based on the PCB marked with `B1144_221028`) see the hardware directory.

## Build / Flash

### Toolchain Setup

Install rust via rustup, if you have an existing rust installation, uninstall it. Then run
`rustup target add riscv32imac-unknown-none-elf`

### WCHISP Setup

To flash the badge you need the [WCHISP](https://github.com/ch32-rs/wchisp) tool,
you can find prebuild binaries in their repository or set it up from source:

```
# install libusb for your platform
# macOS
brew install libusb
# Ubuntu
sudo apt install libusb-1.0-0-dev

# install wchisp
cargo install wchisp --git https://github.com/ch32-rs/wchisp
```

Please note, that you might have to add the wchisp tool to your $PATH variable. Cargo will tell you the path.

### Test WCHISP

To check if the WCHISP tool works, do the following steps:

1. Disconnect the battery from your badge
2. Press the two side-buttons
3. Attach an USB wire

Now you can run `wchisp info` to get information about the chip, it should look like this:

```
[INFO] Chip: CH582[0x8216] (Code Flash: 448KiB, Data EEPROM: 32KiB)
[INFO] Chip UID: ...
[INFO] BTVER(bootloader ver): ...
[INFO] Current config registers: ...
```

### Run / Flash Firmware

This will overwrite your badges firmware (irreversable!!) To flash the new firmware, first bring your badge into download mode:

1. Disconnect Battery
2. Press the two side-buttons
3. Attach an USB wire

Then execute the following command:

```
cargo run --release
```

# LICENSE

MIT / Apache License, see LICENSE-MIT and LICENSE-APACHE
