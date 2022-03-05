#![no_std]
#![no_main]

use stivale_boot::v2::{StivaleHeader,StivaleStruct};

static STACK: [u8; 4096] = [0; 4096];

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[4095] as *const u8);

#[no_mangle]
extern "C" fn _start(_header_addr: *const StivaleStruct) -> ! {
    loop {}
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
