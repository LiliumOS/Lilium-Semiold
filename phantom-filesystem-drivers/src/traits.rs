use core::num::NonZeroU64;
use std::str::StringView;

use bytemuck::{Pod, Zeroable};

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Zeroable, Pod)]
pub struct ObjectId(pub Option<NonZeroU64>);

pub const OBJECT_NULL: ObjectId = ObjectId(None);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Zeroable, Pod)]
pub struct StreamId(pub Option<NonZeroU64>);

pub const STREAM_NONE: ObjectId = ObjectId(None);

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Zeroable, Pod)]
pub struct InodeId(pub ObjectId, pub StreamId);

pub trait Search {
    fn get_object_from(&mut self, pos: InodeId, pname: StringView) -> std::io::Result<ObjectId>;
    fn get_stream_of_object(
        &mut self,
        obj: ObjectId,
        lname: StringView,
    ) -> std::io::Result<StreamId>;
}

pub trait ReadFS {
    fn read_bytes_from(
        &mut self,
        pos: InodeId,
        offset: u64,
        bytes: &mut [u8],
    ) -> std::io::Result<usize>;

    fn read_exact_from(
        &mut self,
        pos: InodeId,
        mut offset: u64,
        mut bytes: &mut [u8],
    ) -> std::io::Result<()> {
        while !bytes.is_empty() {
            match self.read_bytes_from(pos, offset, bytes) {
                Ok(0) => return Err(std::io::Error::UnexpectedEof),
                Ok(cnt) => {
                    offset += u64::try_from(cnt).unwrap();
                    bytes = &mut bytes[cnt..];
                }
                Err(std::io::Error::Interrupted) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
