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
