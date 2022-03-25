use core::num::{NonZeroU32, NonZeroU64};

use bytemuck::{Pod, Zeroable};

bitflags::bitflags! {
    #[derive(Default,Zeroable,Pod)]
    #[repr(transparent)]
    pub struct PhantomFSObjectFlags : u32{

    }
}

bitflags::bitflags! {
    #[derive(Default,Zeroable,Pod)]
    #[repr(transparent)]
    pub struct PhantomFSStreamFlags : u64 {
        const REQUIRED       = 0x0000000000000001;
        const WRITE_REQUIRED = 0x0000000000000002;
        const ENUM_REQUIRED  = 0x0000000000000004;
    }
}

fake_enum::fake_enum! {
    #[repr(u16)]
    #[derive(Hash,Zeroable,Pod)]
    pub enum struct PhantomFSObjectType{
        Regular = 0,
        Directory = 1,
        Symlink = 2,
        Fifo = 3,
        Socket = 4,
        BlockDeivce = 5,
        CharDevice = 6,
        CustomType = 65535
    }
}

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct PhantomFSObject {
    strong_ref: u32,
    weak_ref: Option<NonZeroU32>,
    streams_size: u64,
    streams_ref: u128,
    streams_indirection: u8,
    reserved33: [u8; 5],
    ty: PhantomFSObjectType,
    flags: PhantomFSObjectFlags,
    reserved44: [u8; 20],
}

#[repr(C, align(128))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct StreamListing {
    name: [u8; 32],
    name_ref: Option<NonZeroU64>,
    flags: PhantomFSStreamFlags,
    size: u64,
    reserved: [u64; 3],
    inline_data: [u8; 48],
}
