#!/bin/env bash
set -e

DIR="$(dirname "${BASH_SOURCE[-1]}")"

# Build
cargo build --manifest-path="$DIR/Cargo.toml" --target x86_64-unknown-uefi

# Make imitation UEFI system partition
mkdir -p "$DIR/esp/efi/boot"
cp "$DIR/target/x86_64-unknown-uefi/debug/first-uefi-app.efi" "$DIR/esp/efi/boot/bootx64.efi"

# Copy over OVMF firmware (this step might be a bit finicky on other systems!)
if [[ ! ( -f "$DIR/OVMF_CODE.4m.fd" && -f "$DIR/OVMF_VARS.4m.fd" ) ]]; then
cp /usr/share/OVMF/x64/{OVMF_CODE.4m.fd,OVMF_VARS.4m.fd} "$DIR/"
fi

# Run qemu
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file="$DIR/OVMF_CODE.4m.fd" \
    -drive if=pflash,format=raw,readonly=on,file="$DIR/OVMF_VARS.4m.fd" \
    -drive format=raw,file="fat:rw:$DIR/esp"
