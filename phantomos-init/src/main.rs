#![no_std]
#![no_main]

use stivale_boot::v2::{StivaleHeader, StivaleStruct, StivaleTerminalHeaderTag};

static STACK: [u8; 4096] = [0; 4096];
static TERMINAL_HEADER_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new().flags(0);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[4095] as *const u8)
    .tags(&TERMINAL_HEADER_TAG as *const StivaleTerminalHeaderTag as *const ());

#[allow(clippy::empty_loop)]
#[no_mangle]
extern "C" fn _start(stivale_data: *const StivaleStruct) -> ! {
    let stivale_data = unsafe { stivale_data.as_ref().unwrap_unchecked() };
    let terminal = stivale_data.terminal().unwrap();
    let term_write = terminal.term_write();
    term_write("Initializing PhantomOS...");
    loop {}
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
