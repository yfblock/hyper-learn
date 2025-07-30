use riscv::register::{
    hstatus::Hstatus,
    sstatus::{self, Sstatus},
};

pub const CONTEXT_SIZE: usize = size_of::<TrapContext>();

#[repr(C)]
#[derive(Debug, Clone)]
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
    pub ksp: usize,
    /// CSR hstatus
    pub hstatus: Hstatus,
}

impl TrapContext {
    /// Create a new trap context with default values
    pub fn new() -> Self {
        TrapContext {
            x: [0; 32],
            sstatus: sstatus::read(),
            sepc: 0,
            hgatp: 0,
            ksp: 0,
            hstatus: Hstatus::from_bits(0),
        }
    }

    impl_field!(set_sp, sp, x[2]);
    impl_field!(set_pc, pc, sepc);
    impl_field!(set_ksp, ksp, ksp);
    impl_field!(set_a0, a0, x[10]);
    impl_field!(set_a1, a1, x[11]);
}
