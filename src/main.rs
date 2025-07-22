#![no_std]
#![no_main]

use crate::arch::pagetable::PAGE_SIZE;

pub mod arch;
pub mod mem;

fn main(hart_id: usize) {
    log::info!("Hypervisor started on hart {}", hart_id);

    // Initialize the memory allocator
    mem::heap::init_allocator();
    
    unsafe extern "C" {
        fn _end();
    }
    // Initialize the frame allocator
    mem::frame::add_frame_range((_end as usize).div_ceil(PAGE_SIZE) * PAGE_SIZE, 0x88000000);
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
