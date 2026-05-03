#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use infinity_core::println;
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Infinity Core - Boot Sequence Initiated");
    println!("Loading NanoCore subsystem...");
    
    infinity_core::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { infinity_core::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        infinity_core::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    infinity_core::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    println!("Unified Memory Plane initialized.");
    
    use alloc::boxed::Box;
    let x = Box::new(41);
    println!("Heap allocation test: Box::new({})", x);

    println!("Initialization complete. Infinity NanoCore halted.");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
