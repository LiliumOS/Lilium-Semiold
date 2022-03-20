#![no_std]

use core::arch::global_asm;
use core::fmt::Write;

use elf::{Elf64Dyn, Elf64Rela};
use stivale_boot::v2::StivaleStruct;

mod stivale_setup {
    use stivale_boot::v2::{StivaleHeader, StivaleTerminalHeaderTag};

    #[repr(C, align(16))]
    struct Stack([u8; 4096]);

    static STACK: Stack = Stack([0; 4096]);
    static TERMINAL_HEADER_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new().flags(0);

    #[link_section = ".stivale2hdr"]
    #[no_mangle]
    #[used]
    static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
        .stack(&STACK.0[4095] as *const u8)
        .tags(&TERMINAL_HEADER_TAG as *const StivaleTerminalHeaderTag as *const ());
}

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod allocator;
mod dynloader;
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
    mov qword ptr [rsi+8],0
    lea rdx, [_plt_lookup_sym+rip]
    mov [rsi+16],rdx
    jmp main
    "
);

#[allow(clippy::empty_loop)]
#[no_mangle]
#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn main(stivale_data: *const StivaleStruct) -> ! {
    let stivale_data = &*stivale_data;
    let mut terminal = TerminalWriter::new(stivale_data.terminal().unwrap());
    write!(
        terminal,
        "Initializing PhantomOS {}...\n",
        core::env!("CARGO_PKG_VERSION")
    )
    .unwrap();

    if let Some(_) = stivale_data.kernel_slide() {
        write!(terminal, "Relocating Loader Symbols...\n",).unwrap();
        let mut dynamic = core::ptr::addr_of!(dynloader::_DYNAMIC) as *const Elf64Dyn;

        let mut reltab = 0 as *const Elf64Rela;
        let mut relsize = 0;

        while (*dynamic).d_tag != 0 {
            if (*dynamic).d_tag == 23 {
                reltab = (*dynamic).d_un as *const Elf64Rela;
            } else if (*dynamic).d_tag == 2 {
                relsize = (*dynamic).d_un;
            }
            dynamic = dynamic.add(1);
        }

        let mut i = 0;
        while i < ((relsize as usize) / core::mem::size_of::<Elf64Rela>()) {
            let rel = reltab.add(1);
            if ((*rel).r_info) & 0xffffff == 7 {
                write!(terminal, "Applying JUMP_SLOT relocation {}...\n", i).unwrap();
                dynloader::ldresolve(i as u64, 0);
            }
            i += 1;
        }
    }

    loop {}
}

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
