use std::io::{self, Read, Seek, SeekFrom};

#[allow(dead_code)]
pub struct RawMemReader {
    address: *const u8,
    start: *const u8,
    size: usize,
}

impl RawMemReader {
    pub fn new(start: *const u8, size: usize) -> Self {
        Self {
            address: start,
            start,
            size,
        }
    }
}

impl Read for RawMemReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl Seek for RawMemReader {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<usize> {
        todo!()
    }
}
