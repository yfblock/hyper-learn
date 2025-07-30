#![no_std]
#![no_main]

use crate::{arch::pagetable::PAGE_SIZE, config::KERNEL_STACK_SIZE};

pub mod arch;
pub mod config;
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

    let frame_ptr = mem::frame::frame_alloc(KERNEL_STACK_SIZE / PAGE_SIZE);
    guest_test::enter_guest_test(frame_ptr.raw() + KERNEL_STACK_SIZE);
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod guest_test {
    use core::arch::naked_asm;

    use riscv::register::{hstatus, sstatus};

    use crate::arch::riscv64::context::{CONTEXT_SIZE, TrapContext};

    #[unsafe(naked)]
    pub unsafe extern "C" fn guest_main() {
        naked_asm! {
            "li a0, 0x1234",
            "li a1, 0x5678",
            "ecall",
            "j .",
        }
    }

    pub fn enter_guest_test(stack_top: usize) {
        log::info!("test enter guest code");
        let file = include_bytes!("../testcases/uboot/u-boot.bin");
        let dtb = include_bytes!("../testcases/uboot/u-boot.dtb");
        unsafe {
            core::ptr::copy_nonoverlapping(file.as_ptr(), 0x90200000 as *mut u8, file.len());

            let ctx = ((stack_top - CONTEXT_SIZE) as *mut TrapContext)
                .as_mut()
                .unwrap();
            *ctx = TrapContext::new();
            ctx.sstatus.set_spp(sstatus::SPP::Supervisor);
            ctx.hstatus.set_spv(true);
            // ctx.sepc = guest_main as usize;
            ctx.set_pc(0x90200000);
            ctx.set_a0(0);
            ctx.set_a1(dtb.as_ptr() as usize);

            ctx.x[2] = stack_top;
            sstatus::set_spp(sstatus::SPP::Supervisor);
            hstatus::set_spv();

            core::arch::asm!(
                "mv sp, {stack_top}",
                "addi sp, sp, -{context_size}",
                "j  vmenter",
                stack_top = in(reg) stack_top,
                context_size = const CONTEXT_SIZE,
                options(noreturn)
            );
        }
    }
}
