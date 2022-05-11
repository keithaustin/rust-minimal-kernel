// Exclude standard library and remove Rust-level entry points
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// Handle panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// Entry point for binary
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    loop{}
}
