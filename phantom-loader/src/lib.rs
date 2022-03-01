#![no_std]

extern crate phantom_panic_halt;

static mut CUR_ROW: usize = 0;

pub fn kputs(str: &str) {
    let __vram_start = unsafe { (0xb8000 as *mut [u8; 128]).as_mut().unwrap_unchecked() };
    let mut cur_col = 0;
    let mut cur_row = unsafe { CUR_ROW };
    for c in str.chars() {
        match c {
            '\n' => {
                cur_col = 0;
                cur_row += 1;
            }
            _ => {
                __vram_start[cur_row * 160 + cur_col * 2] = c as u8;
                __vram_start[cur_row * 160 + cur_col * 2] = 0x0F;
                cur_col += 1;
            }
        }
    }
    cur_row += 1;
    unsafe { CUR_ROW = cur_row; }
}

#[export_name = "start_kernel"]
#[link_section = ".text.init"]
pub unsafe extern "C" fn start_kernel() {
    kputs("AAAA");
    loop {}
}

pub mod search;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
