#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

extern crate rlibc;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

