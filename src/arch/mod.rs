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
