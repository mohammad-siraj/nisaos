#![no_main]//disable the main rust level entry points
#![no_std]// disable auto linking of std libraries

use core::panic::PanicInfo;

#[no_mangle] // for not mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}


/// This function is called on panic. To handle panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}