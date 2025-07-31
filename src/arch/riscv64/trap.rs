use riscv::register::{
    htinst,
    scause::{Exception, Trap},
};
use tock_registers::interfaces::Readable;

use crate::arch::riscv64::{
    context::TrapContext,
    csrs::{HSTATUS, hstatus},
    sbi::sbi_call,
};

#[unsafe(no_mangle)]
pub extern "C" fn handle_trap(ctx: &mut TrapContext) {
    // This function will be defined in Rust code
    // to handle the trap logic.
    // log::info!("Trap handler invoked, ctx: {:#x?}", ctx);
    let sstatus = riscv::register::sstatus::read();
    let scause = riscv::register::scause::read();
    log::info!("sstatus spp: {:#x?}", sstatus.spp());
    log::info!("scause: {:x?}", scause.cause());
    log::info!("hstatus: {:#x?}", HSTATUS.read(hstatus::SPV));
    let htinst = htinst::read();
    log::info!(
        "htinst: {:#x}   {:?}",
        htinst,
        riscv_decode::decode(htinst as u32)
    );
    match scause.cause() {
        Trap::Interrupt(interrupt) => {
            log::info!("Interrupt: {:?}", interrupt);
        }
        Trap::Exception(Exception::Breakpoint) => {
            log::info!("Exception BreakPoint");
        }
        Trap::Exception(Exception::VirtualSupervisorEnvCall) => {
            ctx.x[10] = sbi_call(ctx.x[17], ctx.x[10], ctx.x[11], ctx.x[12]);
            log::info!("Exception Virtual Supervisor Env Call");
        }
        Trap::Exception(exception) => {
            log::info!("Exception: {:?}", exception);
            panic!("Trap handler invoked, ctx: {:#x?}", ctx);
        }
    }
    ctx.set_pc(ctx.pc() + 4);
    // loop {}
}
