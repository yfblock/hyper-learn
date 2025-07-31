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

    let ksp_frame_ptr = mem::frame::frame_alloc(KERNEL_STACK_SIZE / PAGE_SIZE);
    ksp_frame_ptr.slice_mut_with_len(KERNEL_STACK_SIZE).fill(0);
    guest_test::enter_guest_test(
        ksp_frame_ptr.raw() + KERNEL_STACK_SIZE,
        mem::frame::frame_alloc(4),
    );
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    log::error!("{:#x?}", info.message());
    loop {}
}

mod guest_test {
    use include_bytes_aligned::include_bytes_aligned;
    use riscv::register::sstatus;

    use crate::arch::{
        addr::PhysAddr,
        pagetable::PAGE_SIZE,
        riscv64::context::{CONTEXT_SIZE, TrapContext},
    };

    pub fn enter_guest_test(ksp_top: usize, global_page: PhysAddr) {
        log::info!("test enter guest code");

        global_page
            .slice_mut_with_len::<usize>(0x200)
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = ((i * 0x4000_0000) >> 2) | 0x1f);
        log::info!(
            "global: {:#x}",
            global_page.slice_with_len::<usize>(0x200)[2]
        );

        // let file = include_bytes!("../testcases/uboot/u-boot.bin.1");
        let dtb = include_bytes_aligned!(16, "../testcases/uboot/u-boot.dtb");
        let file = include_bytes!("../testcases/rcore-tutorial/rCore-Tutorial-v3.bin");
        unsafe {
            core::ptr::copy_nonoverlapping(file.as_ptr(), 0x90200000 as *mut u8, file.len());

            let ctx = ((ksp_top - CONTEXT_SIZE) as *mut TrapContext)
                .as_mut()
                .unwrap();
            *ctx = TrapContext::new();

            ctx.hgatp
                .set_mode(riscv::register::hgatp::HgatpValues::Sv39x4);
            ctx.hgatp.set_vmid(0);
            ctx.hgatp.set_ppn(global_page.raw() / PAGE_SIZE);
            // ctx.hgatp.set_ppn(0);

            ctx.sstatus.set_spp(sstatus::SPP::Supervisor);
            ctx.hstatus.set_spv(true);
            ctx.set_ksp(ksp_top);

            ctx.set_pc(0x90200000);
            ctx.set_a0(0);
            ctx.set_a1(dtb.as_ptr() as usize);
            ctx.first_enter(ksp_top)
        }
    }
}
