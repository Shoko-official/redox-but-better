#![no_std]
#![no_main]

use core::panic::PanicInfo;
use infinity_core::println;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Infinity Core - Boot Sequence Initiated");
    println!("Loading NanoCore subsystem...");
    
    // Initialize standard core systems
    infinity_core::init();

    println!("Initialization complete. Halting.");
    loop {}
}
