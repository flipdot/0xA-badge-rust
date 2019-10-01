# Flipdot Badge Software examples

## Source

This repository lives in the flipdot organization on GitHub. Clone it with

```
git clone https://github.com/flipdot/0xA-badge-rust
```

## Toolchain Setup

1. Install a Rust toolchain
   * Arch Linux: install `rustup` with pacman, then run `rustup default stable`
   * Other Linux: `curl https://sh.rustup.rs | sh`
   * Windows: Download & run <https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe>
2. Setup rustup toolchain
   1. `rustup target add thumbv6m-none-eabi`
   2. `rustup component add rust-src rust-analysis rls rustfmt llvm-tools-preview`
3. Install cargo-binutils: `cargo install cargo-binutils`
4. Install dfu-util
   * Linux: install the package `dfu-util` with your favorite package manager
   * MacOS: `brew install dfu-util`
   * Windows
     1. Download from <http://dfu-util.sourceforge.net/releases/dfu-util-0.9-win64.zip>
     2. Put `dfu-util.exe` and `libusb-1.0.dll` into this directory.

## Build, flash and run the first example

1. Connect the badge to your computer
2. Put the badge in DFU bootloader mode
   1. Press and hold both buttons on the badge
   2. Release the reset button
   3. Release the DFU button
3. Build and flash
   * Linux & MacOS: `./build-flash.sh blink-1`
   * Windows: `build-flash.bat blink-1`
4. Press and release the reset button to run the example on the badge
