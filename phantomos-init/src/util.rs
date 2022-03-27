use crate::MEMORY_MAP;
use core::fmt::Write;
use std::io::{Read, Seek};

pub trait ReadSeek: Read + Seek {}

impl<T: Read + Seek> ReadSeek for T {}

pub trait ToVirtual: Sized {
    fn to_virtual(self) -> Option<Self>;
}

impl<T> ToVirtual for *const T {
    fn to_virtual(self) -> Option<Self> {
        let mem_map_iter = MEMORY_MAP.get().unwrap().iter();
        let address = self as u64;
        writeln!(crate::term(), "{:#018X}", address).unwrap();
        Some(address as Self)
    }
}

#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out {0}, {1}", in(reg) port, in(reg_byte) val);
}

#[inline(always)]
pub unsafe fn outw(port: u16, val: u16) {
    core::arch::asm!("out {0}, {1}", in(reg) port, in(reg) val);
}

#[inline(always)]
pub unsafe fn outl(port: u16, val: u32) {
    core::arch::asm!("out {0}, {1}", in(reg) port, in(reg) val);
}

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let mut result;
    core::arch::asm!("out {0}, {1}", in(reg) port, out(reg_byte) result);
    result
}

#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let mut result;
    core::arch::asm!("out {0}, {1}", in(reg) port, out(reg) result);
    result
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let mut result;
    core::arch::asm!("out {0}, {1}", in(reg) port, out(reg) result);
    result
}
