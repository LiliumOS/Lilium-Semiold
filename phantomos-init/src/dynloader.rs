use core::arch::global_asm;

extern "C" {
    static _DYNAMIC: core::ffi::c_void;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct DynEntry(usize, *const core::ffi::c_void);

unsafe impl Sync for DynEntry {}

#[no_mangle]
#[used]
static DYNAMIC_PTRS: [DynEntry; 4096 / core::mem::size_of::<DynEntry>()] = {
    let mut x =
        [DynEntry(0xffffffff80000000, core::ptr::null()); 4096 / core::mem::size_of::<DynEntry>()];
    x[0] = DynEntry(0, unsafe { core::ptr::addr_of!(_DYNAMIC) });

    x
};

#[cfg(target_arch = "x86_64")]
global_asm! {
   r"
.hidden DYNAMIC_PTRS

.global _plt_lookup_sym
.hidden _plt_lookup_sym
_plt_lookup_sym:
    push rbp
    mov rbp, rsp
    push rdi
    push rsi
    push rdx
    push rcx
    push r8
    push r9
    push rax
    push r10
    mov rdi, [rsp-16]
    mov rsi, [rsp-8]
    call ldresolve
    mov r11, rax
    pop r10
    pop rax
    pop r9
    pop r8
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    lea rsp, [rsp+16]
    jmp [r11]
    "
}

use crate::elf::*;

#[no_mangle]
#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn ldresolve(relno: u64, mut dynamic: *mut Elf64Dyn) -> *mut core::ffi::c_void {
    let mut symtab = 0 as *const Elf64Sym;
    let mut strtab = 0 as *const u8;
    let mut reltab = 0 as *const Elf64Rela;

    while (*dynamic).d_tag != 0 {
        if (*dynamic).d_tag == 23 {
            reltab = (*dynamic).d_un as *const Elf64Rela;
        } else if (*dynamic).d_tag == 6 {
            symtab = (*dynamic).d_un as *const Elf64Sym;
        } else if (*dynamic).d_tag == 5 {
            strtab = (*dynamic).d_un as *const u8;
        }
        dynamic = dynamic.add(1);
    }
    let rel = reltab.add(relno as usize);

    let sym = symtab.add(((*rel).r_info >> 32) as usize);

    let name = strtab.add((*sym).st_name as usize);

    let mut hash = 0usize;

    let mut n = name;
    while (*n) != 0 {
        hash = (hash.wrapping_shl(4)).wrapping_add((*n) as usize);
        n = n.offset(1);

        let g = hash & 0xf0000000;

        if g != 0 {
            hash ^= g >> 24;
            hash &= !g;
        }
    }

    let mut val = 0 as *mut core::ffi::c_void;

    let mut i = 0;

    'a: while DYNAMIC_PTRS[i].1 != (0 as *const _) {
        let mut dynamic = DYNAMIC_PTRS[i].0 as *const Elf64Dyn;
        let mut symtab = 0 as *const Elf64Sym;
        let mut strtab = 0 as *const u8;
        let mut htab = 0 as *const u32;

        while (*dynamic).d_tag != 0 {
            if (*dynamic).d_tag == 6 {
                symtab = (*dynamic).d_un as *const Elf64Sym;
            } else if (*dynamic).d_tag == 5 {
                strtab = (*dynamic).d_un as *const u8;
            } else if (*dynamic).d_tag == 4 {
                htab = (*dynamic).d_un as *const u32;
            }
            dynamic = dynamic.add(1);
        }

        let nbucket = (*htab) as usize;

        let idx = *htab.add((hash % nbucket).wrapping_add(2));
        let mut cptr = htab.add(nbucket.wrapping_add(idx as usize).wrapping_add(2));
        'b: while (*cptr) != 0 {
            let idx = *cptr;
            let sym = symtab.add(idx as usize);
            let mut sname = strtab.add((*sym).st_name as usize);
            let mut n = name;
            while (*n) != 0 {
                if (*n) != (*sname) {
                    cptr = cptr.add(1);
                    continue 'b;
                }
                n = n.add(1);
                sname = sname.add(1);
            }
            let addr = DYNAMIC_PTRS[i]
                .0
                .wrapping_add((*sym).st_value as usize)
                .wrapping_add((*rel).r_added as u64 as usize);

            val = addr as *mut core::ffi::c_void;
            break 'a;
        }

        i = i.wrapping_add(1);
    }

    if val == (0 as *mut core::ffi::c_void) {
        panic!("Could not find symbol");
    }

    let ent = (*rel).r_offset as *mut *mut core::ffi::c_void;
    *ent = val;

    val
}
