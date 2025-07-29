#![no_std]
#![no_main]

use crate::arch::pagetable::PAGE_SIZE;

pub mod arch;
pub mod mem;

fn main(hart_id: usize) {
    log::info!("Hypervisor started on hart {}", hart_id);

    // Initialize the memory allocator
    mem::heap::init_allocator();

    unsafe extern "C" {
        fn _end();
    }
    // Initialize the frame allocator
    mem::frame::add_frame_range((_end as usize).div_ceil(PAGE_SIZE) * PAGE_SIZE, 0x88000000);

    guest_test::enter_guest_test();
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod guest_test {
    use core::arch::naked_asm;

    use riscv::register::{hstatus, sstatus};

    #[unsafe(naked)]
    pub unsafe extern "C" fn guest_main() {
        naked_asm!(
            "
            li a0, 0x1234
            li a1, 0x5678
            ecall
            j .
        "
        );
    }

    pub fn enter_guest_test() {
        log::info!("test enter guest code");
        unsafe {
            sstatus::set_spp(sstatus::SPP::Supervisor);
            hstatus::set_spv();
        }
        unsafe {
            core::arch::asm!("
                la   a0, {guest_main}
                csrw sepc, a0
                sret
            ", guest_main = sym guest_main);
        }
    }
}
