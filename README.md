# Flipdot Badge Software examples

## Toolchain Setup

1. install rustup `curl https://sh.rustup.rs | sh`
2. setup rustup toolchain 
   1. `rustup default stable`
   2. `rustup target add thumbv6m-none-eabi`

arm-none-eabi-gcc -> for the linker


## Build the project

`cargo build --release`
