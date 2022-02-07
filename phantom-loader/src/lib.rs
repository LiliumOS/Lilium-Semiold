#![no_std]

extern crate phantom_panic_halt;

#[export_name = "start_kernel"]
#[link_section = ".text.init"]
pub unsafe extern "C" fn start_kernel() {
    loop {}
}

pub mod search;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
