#![no_std]
#![no_main]

pub mod arch;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
