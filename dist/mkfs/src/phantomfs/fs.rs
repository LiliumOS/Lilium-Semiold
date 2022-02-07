use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

use bytemuck::Zeroable;
use crc::Crc;

use super::{
    object::Object,
    root::{self, FSInfo, FsFlags},
};

pub struct FileSystemAccess<S> {
    inner: S,
    info_block: Option<root::FSInfo>,
}

impl<S> FileSystemAccess<S> {
    pub const fn new(inner: S) -> Self {
        Self {
            inner,
            info_block: None,
        }
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}

static CRC32: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_CKSUM);

impl<S: Write + Seek> FileSystemAccess<S> {
    pub fn create_info_block(&mut self, volsize: u64) -> std::io::Result<()> {
        if self.info_block.is_none() {
            let mut info = FSInfo {
                fssig: *b"PHFSv1\0\0",
                infosize: core::mem::size_of::<FSInfo>() as u64,
                volsize: volsize as u128,
                objtable_idx: 2048,
                objtable_len: 0,
                fs_flags: FsFlags::VERSION1,
                base_chksum: 0,
                allocation_map: 0x3,
                ..FSInfo::zeroed()
            };
            info.base_chksum = CRC32.checksum(bytemuck::bytes_of(&info));
            self.info_block = Some(info);
        }

        let info = self.info_block.unwrap();
        self.inner.seek(SeekFrom::Start(1024))?;
        self.inner.write_all(bytemuck::bytes_of(&info))
    }

    pub fn create_object_table(&mut self, preallocate_count: usize) -> std::io::Result<()> {
        let preallocate_count = preallocate_count.next_power_of_two();
        if let Some(info) = &mut self.info_block {
            let size = (preallocate_count * core::mem::size_of::<Object>()) as u64;
            let array = vec![Object::zeroed(); preallocate_count];
            let scount = (size + 1023) >> 10;
            let alloc_bits = (1 << (scount)) - 1;
            info.allocation_map = 0x3 | alloc_bits << 2;
            info.objtable_len = size;
            self.inner
                .seek(SeekFrom::Start((info.objtable_idx * 1024) as u64))?;
            self.inner.write_all(bytemuck::cast_slice(&*array))
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "No Information Table available. Call create_info_block first",
            ))
        }
    }
}

impl<S: Read + Seek> FileSystemAccess<S> {
    pub fn read_info_block(&mut self) -> std::io::Result<()> {
        self.inner.seek(SeekFrom::Start(1024))?;
        let mut info = FSInfo::zeroed();
        self.inner.read_exact(bytemuck::bytes_of_mut(&mut info))?;

        if CRC32.checksum(bytemuck::bytes_of(&info)) != 0 {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "CRC Checksum failed",
            ));
        }

        if info.fssig != *b"PHFSv1\0\0" {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Not a PhantomFS volume",
            ));
        }

        if info.fs_flags != FsFlags::all() {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Unrecognized flags in information block",
            ));
        }

        if info.fs_flags.contains(FsFlags::LONG_VOL) {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Long Volumes are not supported",
            ));
        }

        self.info_block = Some(info);

        Ok(())
    }
}
