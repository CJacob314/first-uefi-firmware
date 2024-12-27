# Project info
I made this toy project to learn a bit about making UEFI firmware with the [`uefi`](https://github.com/rust-osdev/uefi-rs) Rust crate.

# What happens
This project will print, "Welcome to my first UEFI project!", stall ~2 seconds, and then attempt to display whatever BMP image data was in `$PROJECT_ROOT/boot_image.bmp` at compile-time.

# How to Run
Running *should* be as easy as adding a BMP image file named `boot_image.bmp` to the project root and running the `qemu.sh` script. Of course, it probably won't be that easy.

You'll need to have `cargo`, `qemu`, and `OVMF` installed, and you *may* need to modify the system paths for the `.fd` files in `qemu.sh` before everything works (they should be somewhere in `/usr/share/OVMF`, though).
