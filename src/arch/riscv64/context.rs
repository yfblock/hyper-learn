use core::fmt::Debug;

use riscv::register::{
    hgatp::Hgatp,
    hstatus::Hstatus,
    sstatus::{self, Sstatus},
};

pub const CONTEXT_SIZE: usize = size_of::<TrapContext>();

const REG_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

#[repr(C)]
#[derive(Clone)]
/// trap context structure containing sstatus, sepc and registers
pub struct TrapContext {
    /// General purpose registers
    pub x: [usize; 32],
    /// CSR sstatus
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
    /// Addr of Page Table
    pub hgatp: Hgatp,
    /// kernel stack
    pub ksp: usize,
    /// CSR hstatus
    pub hstatus: Hstatus,
}

impl Debug for TrapContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut struct_debug = f.debug_struct("TrapContext");
        for (i, name) in REG_NAMES.iter().enumerate() {
            struct_debug.field(name, &self.x[i]);
        }
        struct_debug
            .field("sstatus", &self.sstatus.bits())
            .field("sepc", &self.sepc)
            .field("hgatp", &self.hgatp)
            .field("ksp", &self.ksp)
            .field("hstatus", &self.hstatus.bits())
            .finish()
    }
}

impl TrapContext {
    /// Create a new trap context with default values
    pub fn new() -> Self {
        TrapContext {
            x: [0; 32],
            sstatus: sstatus::read(),
            sepc: 0,
            hgatp: Hgatp::from_bits(0),
            ksp: 0,
            hstatus: Hstatus::from_bits(0),
        }
    }

    pub fn first_enter(&self, stack_top: usize) -> ! {
        unsafe {
            core::arch::asm!(
                "mv sp, {stack_top}",
                "csrw sscratch, sp",
                "addi sp, sp, -{context_size}",
                "j  vmenter",
                stack_top = in(reg) stack_top,
                context_size = const CONTEXT_SIZE,
                options(noreturn)
            );
        }
    }

    impl_field!(set_sp, sp, x[2]);
    impl_field!(set_pc, pc, sepc);
    impl_field!(set_ksp, ksp, ksp);
    impl_field!(set_a0, a0, x[10]);
    impl_field!(set_ret, ret, x[10]);
    impl_field!(set_a1, a1, x[11]);
}
