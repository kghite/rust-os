# rust-os
Basic OS built in Rust

## Running the bootloader

Build: `bootimage build`

Run with QEMU virtual machine: `bootimage run`

Run unit tests: `cargo test`

Run integration tests: `bootimage test`

Write to bootable USB: `dd if=target/x86_64-blog_os/debug/bootimage.bin of=/dev/usbaddress && sync`
