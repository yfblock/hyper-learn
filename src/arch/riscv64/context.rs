use riscv::register::{hstatus::Hstatus, sstatus::Sstatus};

pub const CONTEXT_SIZE: usize = size_of::<TrapContext>();

#[repr(C)]
#[derive(Debug)]
/// trap context structure containing sstatus, sepc and registers
pub struct TrapContext {
    /// General purpose registers
    pub x: [usize; 32],
    /// CSR sstatus
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
    /// Addr of Page Table
    pub hgatp: usize,
    /// kernel stack
    pub kernel_sp: usize,
    /// CSR hstatus
    pub hstatus: Hstatus,
}
