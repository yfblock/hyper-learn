use riscv::interrupt::Trap;
use tock_registers::interfaces::Readable;

use crate::arch::riscv64::csrs::{HSTATUS, hstatus};

#[unsafe(no_mangle)]
pub extern "C" fn handle_trap() {
    // This function will be defined in Rust code
    // to handle the trap logic.
    log::info!("Trap handler invoked");
    let sstatus = riscv::register::sstatus::read();
    let scause = riscv::register::scause::read();
    log::info!("sstatus spp: {:#x?}", sstatus.spp());
    log::info!("scause: {:#x?}", scause);
    unsafe {
        log::info!("hstatus: {:#x?}", HSTATUS.read(hstatus::SPV));
    }
    match scause.cause() {
        Trap::Interrupt(interrupt) => {
            log::info!("Interrupt: {:?}", interrupt);
        }
        Trap::Exception(exception) => {
            log::info!("Exception: {:?}", exception);
        }
    }
    loop {}
}
