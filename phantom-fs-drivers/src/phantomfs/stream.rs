use core::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};

#[repr(C, align(128))]
#[derive(Zeroable, Pod, Clone, Copy, Hash, PartialEq, Eq)]
pub struct StreamListing {
    pub name: [u8; 32],
    pub name_ref: Option<NonZeroU64>,
    pub flags: u64,
    pub content_ref: u128,
    pub size: u64,
    pub reserved: [u64; 3],
    pub inline_data: [u8; 32],
}

pub const STREAMS_NAME: &'static str = "Streams";
pub const STRINGS_NAME: &'static str = "Strings";
