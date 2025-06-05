#!/bin/bash
export RUSTFLAGS='--cfg getrandom_backend="rdrand"'
RUSTFLAGS='--cfg getrandom_backend="rdrand"' cargo build --release --target x86_64-unknown-uefi

# exit

clear
qemu-system-x86_64 -serial stdio -bios /usr/share/edk2/x64/OVMF.4m.fd -kernel target/x86_64-unknown-uefi/release/snake_bin_uefi.efi &
sleep 1
# remmina -c vnc://localhost:5900
vncviewer localhost:5900
killall -9 qemu-system-x86_64