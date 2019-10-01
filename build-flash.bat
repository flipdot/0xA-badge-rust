@echo off

rem TODO
dfu-util.exe -a0 --dfuse-address 0x08000000 -D "$app.bin"
