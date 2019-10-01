# Flipdot Badge Software examples

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
   * Windows: <http://dfu-util.sourceforge.net/releases/dfu-util-0.9-win64.zip>

## Build the first example

```
cd blink
# Automatically builds the project (like cargo build),
# then extracts a flashable binary from the ELF build output
cargo objcopy --bin blink --release -- -O binary blink.bin
```

## Flash the example

1. Connect the badge to your computer
2. Put the badge in DFU bootloader mode
   1. Press and hold both buttons on the badge
   2. Release the reset button
   3. Release the DFU button
3. Flash the binary (run command in blink directory)
   * Linux & MacOS: `sudo dfu-util -a0 --dfuse-address 0x08000000 -D blink.bin`
   * Windows: `dfu-util.exe -a0 --dfuse-address 0x08000000 -D blink.bin`
4. Press and release the reset button to run the program
