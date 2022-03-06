#![no_std]
#![no_main]

use stivale_boot::v2::{StivaleHeader, StivaleStruct, StivaleTerminalHeaderTag};

static STACK: [u8; 4096] = [0; 4096];
static TAGS: &() = unsafe { &*make_tag_list() };

const fn make_tag_list() -> *const () {
    let mut next = core::ptr::null();
    {
        let next_temp = StivaleTerminalHeaderTag::new().next(next).flags(0);
        next = (&next_temp as *const StivaleTerminalHeaderTag).cast();
        core::mem::forget(next_temp);
    }
    next
}

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[4095] as *const u8)
    .tags(TAGS as *const ());

#[no_mangle]
extern "C" fn _start(stivale_data: *const StivaleStruct) -> ! {
    
    loop {}
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
