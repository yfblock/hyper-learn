//! CSR Number list
//!
//! <https://five-embeddev.com/quickref/csrs.html>

use core::marker::PhantomData;

use tock_registers::{
    RegisterLongName,
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields![
    usize,
    pub hedeleg [
        INST_ADDR_MISALIGN 0,
        INST_ACCESSS_FAULT 1,
        ILLEGAL_INST 2,
        BREAKPOINT 3,
        LOAD_ADDR_MISALIGNED 4,
        LOAD_ACCESS_FAULT 5,
        STORE_ADDR_MISALIGNED 6,
        STORE_ACCESS_FAULT 7,
        ENV_CALL_FROM_U_OR_VU 8,
        ENV_CALL_FROM_HS 9,
        ENV_CALL_FROM_VS 10,
        ENV_CALL_FROM_M 11,
        INST_PAGE_FAULT 12,
        LOAD_PAGE_FAULT 13,
        STORE_PAGE_FAULT 15,
        INST_GUEST_PAGE_FAULT 20,
        LOAD_GUEST_PAGE_FAULT 21,
        VIRTUAL_INST 22,
        STORE_GUEST_PAGE_FAULT 23,
    ],
];

register_bitfields![
    usize,
    /// <https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/hypervisor.html#hypervisor-status-register-hstatus>
    pub hstatus [
        VSBE    OFFSET(5) NUMBITS(1) [],
        GVA     OFFSET(6) NUMBITS(1) [],
        SPV     OFFSET(7) NUMBITS(1) [],
        SPVP    OFFSET(8) NUMBITS(1) [],
        HU      OFFSET(9) NUMBITS(1) [],
        VGENIN  OFFSET(12) NUMBITS(6) [],
        VTVM    OFFSET(20) NUMBITS(1) [],
        VTW     OFFSET(21) NUMBITS(1) [],
        VTSR    OFFSET(22) NUMBITS(1) [],
        VSXL    OFFSET(32) NUMBITS(2) [],
    ],
    /// <https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/hypervisor.html#hypervisor-trap-delegation-registers-hedeleg-and-hideleg>
    pub hideleg [
        VSSIP   OFFSET(2) NUMBITS(1) [],
        VSTIP   OFFSET(6) NUMBITS(1) [],
        VSEIP   OFFSET(10) NUMBITS(1) [],
    ],
    /// <https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/hypervisor.html#sec:hinterruptregs>
    pub hvip [
        VSSIP   OFFSET(2) NUMBITS(1) [],
        VSTIP   OFFSET(6) NUMBITS(1) [],
        VSEIP   OFFSET(10) NUMBITS(1) [],
    ],
];

#[derive(Copy, Clone)]
pub struct ReadWriteCsr<R: RegisterLongName, const V: usize> {
    associated_register: PhantomData<R>,
}

impl<R: RegisterLongName, const V: usize> ReadWriteCsr<R, V> {
    pub const fn new() -> Self {
        ReadWriteCsr {
            associated_register: PhantomData,
        }
    }
}

impl<R: RegisterLongName, const V: usize> Readable for ReadWriteCsr<R, V> {
    type T = usize;
    type R = R;

    #[inline]
    fn get(&self) -> usize {
        use core::arch::asm;
        let r: usize;
        unsafe {
            asm!("csrr {rd}, {csr}", rd = out(reg) r, csr = const V);
        }
        r
    }
}

impl<R: RegisterLongName, const V: usize> Writeable for ReadWriteCsr<R, V> {
    type T = usize;
    type R = R;

    #[inline]
    fn set(&self, value: usize) {
        use core::arch::asm;
        unsafe {
            asm!("csrw {csr}, {value}", csr = const V, value = in(reg) value);
        }
    }
}

pub const HEDELEG: ReadWriteCsr<hedeleg::Register, 0x602> = ReadWriteCsr::new();
pub const HIDELEG: ReadWriteCsr<hideleg::Register, 0x603> = ReadWriteCsr::new();
pub const HVIP: ReadWriteCsr<hvip::Register, 0x645> = ReadWriteCsr::new();
pub const HCOUNTEREN: ReadWriteCsr<(), 0x606> = ReadWriteCsr::new();
pub const VSATP: ReadWriteCsr<(), 0x280> = ReadWriteCsr::new();
pub const HSTATUS: ReadWriteCsr<hstatus::Register, 0x600> = ReadWriteCsr::new();
