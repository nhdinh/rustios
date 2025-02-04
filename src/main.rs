#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustios::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::println;
use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) {
    crate::panic_on_test(info);
}