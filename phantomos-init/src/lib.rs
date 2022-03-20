#![no_std]

use core::arch::global_asm;
use core::fmt::Write;

use stivale_boot::v2::{StivaleHeader, StivaleStruct, StivaleTerminalHeaderTag};

static STACK: [u8; 4096] = [0; 4096];
static TERMINAL_HEADER_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new().flags(0);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[4095] as *const u8)
    .tags(&TERMINAL_HEADER_TAG as *const StivaleTerminalHeaderTag as *const ());

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod dynloader;
pub mod elf;
pub mod writer;

use writer::TerminalWriter;

#[cfg(target_arch = "x86_64")]
global_asm!(
    r"

.protected main

.global _start
_start:
    // todo: Setup IDTR
    lea rsi, [DYNAMIC_PTRS+rip]
    lea rdx, [_phantomos_init_start+rip]
    mov [rsi], rdx
    lea rdx, [_DYNAMIC+rip]
    mov [rsi+8], rdx
    lea rsi, [_GLOBAL_OFFSET_TABLE_+rip]
    mov [rsi], rdx
    mov [rsi+8],rdx
    lea rdx, [_plt_lookup_sym+rip]
    mov [rsi+16],rdx
    call main
    "
);

#[allow(clippy::empty_loop)]
#[no_mangle]
unsafe extern "C" fn main(stivale_data: *const StivaleStruct) -> ! {
    let stivale_data = &*stivale_data;
    let mut terminal = TerminalWriter::new(stivale_data.terminal().unwrap());
    write!(terminal, "Initializing PhantomOS...").unwrap();
    loop {}
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
