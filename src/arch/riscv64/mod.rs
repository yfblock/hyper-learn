pub mod console;

use core::arch::global_asm;

use crate::arch::clear_bss;

global_asm!(include_str!("boot.S"));

#[unsafe(no_mangle)]
extern "C" fn rust_main(hart_id: usize, dtb_ptr: usize) {
    // Handle external interrupts here
    clear_bss();
    println!("Booting RISC-V hypervisor...");
    super::console::log_init();
    log::info!(
        "Hello, RISC-V world! Hart ID: {}, DTB Pointer: {:#x}",
        hart_id,
        dtb_ptr
    );

    if sbi_rt::probe_extension(sbi_rt::Hsm).is_unavailable() {
        panic!("no HSM extension exist on current SBI environment");
    }
    // Test Code
    let hgatp: usize;
    unsafe {
        core::arch::asm!(
            "csrr {0}, hgatp",
            out(reg) hgatp,
            options(nomem, nostack, preserves_flags)
        );
    }
    log::info!("Current hgatp: {:#x}", hgatp);
    // if !detect::detect_h_extension() {
    //     panic!("no RISC-V hypervisor H extension on current environment")
    // }

    crate::main(hart_id);
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
}
