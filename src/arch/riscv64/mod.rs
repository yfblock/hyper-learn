pub mod console;

use core::arch::global_asm;

use crate::arch::clear_bss;

global_asm!(include_str!("boot.S"));

#[unsafe(no_mangle)]
extern "C" fn rust_main(hart_id: usize, dtb_ptr: usize) {
    // Handle external interrupts here
    clear_bss();
    super::console::log_init();
    log::info!(
        "Hello, RISC-V world! Hart ID: {}, DTB Pointer: {:#x}",
        hart_id,
        dtb_ptr
    );

    loop {}
}
