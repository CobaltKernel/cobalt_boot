#![no_std]
#![no_main]

use bootloader::BootInfo;
use cobalt_boot::boot_fn;
use cobalt_boot::hook_panic_fn;

boot_fn!(kmain);
hook_panic_fn!(panic);

pub fn kmain(_: &'static mut BootInfo) -> ! {
    loop {}
}

pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}