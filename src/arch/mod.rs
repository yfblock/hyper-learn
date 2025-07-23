use core::arch::global_asm;

#[macro_use]
pub mod macros;
#[macro_use]
pub mod console;
pub mod addr;
pub mod config;
pub mod pagetable;
pub mod riscv64;

global_asm!(include_str!("common.S"));

/// clear BSS segment
fn clear_bss() {
    unsafe extern "C" {
        fn _sbss();
        fn _ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(_sbss as usize as *mut u8, _ebss as usize - _sbss as usize)
            .fill(0);
    }
}
