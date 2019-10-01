# Flipdot Badge Software examples

## Toolchain Setup

1. install a Rust toolchain
   * Arch Linux: install `rustup` with pacman, then run `rustup default stable`
   * Other Linux: `curl https://sh.rustup.rs | sh`
   * Windows: Download & run <https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe>
2. setup rustup toolchain
   1. `rustup target add thumbv6m-none-eabi`
   2. `rustup component add rust-src rust-analysis rls rustfmt llvm-tools-preview`
3. install cargo-binutils: `cargo install cargo-binutils`
4. install dfu-util
   1. Linux: install the package `dfu-util` with your favorite package manager
   2. MacOS: `brew install dfu-util`
   3. Windows: <http://dfu-util.sourceforge.net/releases/dfu-util-0.9-win64.zip>

## Build the project

`cargo build --release -p blink`
