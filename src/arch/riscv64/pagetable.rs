use riscv::asm::sfence_vma;

/// Assuming a page size of 4KB
pub const PAGE_SIZE: usize = 4096;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PTEFlags: u64 {
        const V = bit!(0);
        const R = bit!(1);
        const W = bit!(2);
        const X = bit!(3);
        const U = bit!(4);
        const G = bit!(5);
        const A = bit!(6);
        const D = bit!(7);

        const VRWX  = Self::V.bits() | Self::R.bits() | Self::W.bits() | Self::X.bits();
        const ADUVRX = Self::A.bits() | Self::D.bits() | Self::U.bits() | Self::V.bits() | Self::R.bits() | Self::X.bits();
        const ADVRWX = Self::A.bits() | Self::D.bits() | Self::VRWX.bits();
        const ADGVRWX = Self::G.bits() | Self::ADVRWX.bits();
    }
}

/// flush the TLB entry by VirtualAddress
#[inline]
pub fn flush_vaddr(vaddr: usize) {
    unsafe {
        sfence_vma(0, vaddr);
    }
}

/// flush all tlb entry
#[inline]
pub fn flush_all() {
    unsafe {
        riscv::asm::sfence_vma_all();
    }
}
