#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Loading kernel...");
    println!("...");
    println!("Loaded Cunny kernel.");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
