# RPi4B_baremetal_minimal_blink

This project is a minimal bare-metal Raspberry Pi 4B project written in Rust that blinks an LED on pin 21.

## How to Run

To compile the code and extract the binary for running on the Raspberry Pi, follow these steps:

```
cargo build --release
```
The binary then needs to be extracted for example with:
```
rust-objcopy --strip-all -O binary ./target/aarch64-unknown-none/release/RPi4B_baremetal_minimal_blink ./kernel/kernel8.img
```
To boot the kernel the ``` kernel8.img ``` and ``` config.txt ``` need to be put onto a FAT32 formatted SD card with the following files:
```
bcm2711-rpi-4-b.dtb
fixup4.dat
start4.elf
```
which can be found at https://github.com/raspberrypi/firmware/tree/master/boot .