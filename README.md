# Flipdot Badge Software examples

## Toolchain Setup

1. install a Rust toolchain
   * Arch Linux: `sudo pacman -S rustup && rustup default stable`
   * Other Linux: `curl https://sh.rustup.rs | sh`
   * Windows: Download & run <https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe>
2. setup rustup toolchain
   1. `rustup default stable`
   2. `rustup target add thumbv6m-none-eabi`
   3. `rustup component add rust-src rust-analysis rls rustfmt llvm-tools-preview`

## Build the project

`cargo build --release`
