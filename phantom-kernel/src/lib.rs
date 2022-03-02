#![no_std]

extern crate phantom_panic_halt;

pub mod x86_64;

#[no_mangle]
pub unsafe extern "C" fn _start() {}
