#![no_std]
#![feature(panic_info_message)]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub mod allocator;
pub mod devicetree;
pub mod dynloader;
pub mod elf;
pub mod rawmem;
pub mod util;
pub mod writer;

use core::arch::global_asm;
use core::fmt::Write;
use core::mem::MaybeUninit;
use elf::{Elf64Dyn, Elf64Rela};
use stivale_boot::v2::StivaleStruct;
use writer::TerminalWriter;

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

static mut TERMINAL: MaybeUninit<TerminalWriter> = MaybeUninit::uninit();

fn term<'a>() -> &'a mut TerminalWriter<'static> {
    // This is actually unsound wrt interrupts, but that's not a problem yet.
    // TODO: do something clever
    unsafe {
        TERMINAL.assume_init_mut()
    }
}

#[allow(clippy::empty_loop)]
#[no_mangle]
#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn main(stivale_data: *const StivaleStruct) -> ! {
    let stivale_data = &*stivale_data;
    TERMINAL.write(TerminalWriter::new(stivale_data.terminal().unwrap_or_else(|| loop {})));
    write!(
        term(),
        "Initializing PhantomOS {}...\n",
        core::env!("CARGO_PKG_VERSION")
    )
    .unwrap();

    if let Some(_) = stivale_data.kernel_slide() {
        write!(term(), "Relocating Loader Symbols...\n",).unwrap();
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
                write!(term(), "Applying JUMP_SLOT relocation {}...\n", i).unwrap();
                dynloader::ldresolve(i as u64, 0);
            }
            i += 1;
        }
    }

    loop {}
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(s) = info.message() {
        write!(term(), "panic @ {}: {:?}\n", info.location().unwrap(), s).unwrap_or(());
    } else {
        write!(term(), "panic @ {} (no message)\n", info.location().unwrap()).unwrap_or(());
    }
    loop {}
}
