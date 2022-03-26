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
