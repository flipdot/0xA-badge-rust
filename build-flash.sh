#!/bin/bash

app="$1"

if [[ -z "$app" ]]; then
    echo "usage: $0 <app>"
    exit 1
fi

cd "$app"
echo -e "\033[1m====== Building ======\033[0m"
cargo objcopy --bin "$app" --release -- -O binary "$app.bin"

cargo_exit_status=$?
[[ $cargo_exit_status != 0 ]] && exit $cargo_exit_status

echo
echo -e "\033[1m====== Flashing ======\033[0m"
sudo dfu-util -a0 --dfuse-address 0x08000000 -D "$app.bin"

if [[ $? == 74 ]]; then
    echo
    echo -e "\033[91mFlashing failed.\033[0m Is the badge in DFU bootloader mode?"
fi
