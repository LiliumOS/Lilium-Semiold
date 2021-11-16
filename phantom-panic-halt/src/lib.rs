#![no_std]

use core::panic::PanicInfo;

extern "C" {
    fn _halt() -> !;
}

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    unsafe { _halt() }
}
