use bytemuck::{Pod, Zeroable};

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct Object {
    pub strong_ref: u32,
    pub weak_ref: u32,
    pub streams_size: u64,
    pub streams_ref: u128,
    pub streams_indirection: u8,
    pub reserved33: [u8; 5],
    pub ty: ObjectType,
    pub flags: ObjectFlags,
    pub reserved44: [u8; 20],
}

fake_enum::fake_enum! {
    #[repr(u16)]
    #[derive(Zeroable,Pod, Hash)]
    pub enum struct ObjectType{
        RegularFile = 0,
        Directory = 1,
        Symlink = 2,
        PosixFifo = 3,
        UnixSocket = 4,
        BlockDevice = 5,
        CharacterDevice = 6,
        Custom = 65535
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Zeroable,Pod)]
    pub struct ObjectFlags : u32{

    }
}

pub const DATA_NAME: &str = "FileData";
