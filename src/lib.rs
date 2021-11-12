#![no_std]
#![no_main]

#[macro_export]
macro_rules! boot_fn {
    ($entry:path) => {
        
        #[no_mangle]
        pub extern "C" fn _start(boot_info: &'static mut bootloader::BootInfo) -> ! {
            let boot: fn(&'static mut BootInfo) -> ! = $entry;
            boot(boot_info);
        } 
    };
}

#[macro_export]
macro_rules! hook_panic_fn {
    ($entry:path) => {
        #[panic_handler]
        pub fn _panic(info: &core::panic::PanicInfo) -> ! {
            let f: fn(&core::panic::PanicInfo) -> ! = $entry;

            f(info)
        }
    };
}