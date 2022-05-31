#![feature(default_alloc_error_handler)]
#![no_std]

pub mod addr_space;
pub mod allocator;
pub mod handle;
pub mod security;
pub mod state;
pub mod thread;

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
