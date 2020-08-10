#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga_buffer::{Writer, Color, ColorCode};

mod vga_buffer;

extern crate rlibc;

static HELLO_WORLD: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

