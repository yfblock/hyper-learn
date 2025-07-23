pub mod console;
pub mod csrs;

use crate::arch::{
    clear_bss,
    riscv64::csrs::{HCOUNTEREN, hedeleg, hideleg, hvip},
};
use core::arch::global_asm;
use riscv::register::{sie, sstatus};
use tock_registers::interfaces::{Readable, Writeable};

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
    let hstatus: usize;
    unsafe {
        core::arch::asm!(
            "csrr {0}, hstatus",
            out(reg) hstatus,
            options(nomem, nostack, preserves_flags)
        );
    }
    log::info!("Current hstatus: {:#x}", hstatus);
    log::info!("Current sstatus: {:#x?}", sstatus::read().spp());
    log::info!(
        "Current hedeleg: {:#x?}",
        csrs::HEDELEG.read(hedeleg::ENV_CALL_FROM_U_OR_VU)
    );

    // hedeleg: delegate some synchronous exceptions
    csrs::HEDELEG.write(
        hedeleg::INST_ADDR_MISALIGN::SET
            + hedeleg::BREAKPOINT::SET
            + hedeleg::ENV_CALL_FROM_U_OR_VU::SET
            + hedeleg::INST_PAGE_FAULT::SET
            + hedeleg::LOAD_PAGE_FAULT::SET
            + hedeleg::STORE_PAGE_FAULT::SET,
    );
    csrs::HIDELEG.write(hideleg::VSSIP::SET + hideleg::VSTIP::SET + hideleg::VSEIP::SET);
    csrs::HVIP.write(hvip::VSSIP::SET + hvip::VSTIP::SET + hvip::VSEIP::SET);
    HCOUNTEREN.set(0xffff_ffff);

    unsafe {
        sie::set_sext();
        sie::set_ssoft();
        sie::set_stimer();
    }

    // if !detect::detect_h_extension() {
    //     panic!("no RISC-V hypervisor H extension on current environment")
    // }

    crate::main(hart_id);
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
}
