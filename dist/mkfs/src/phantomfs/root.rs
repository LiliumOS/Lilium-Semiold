use bytemuck::{Pod, Zeroable};

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct FSInfo {
    pub fssig: [u8; 8],
    pub infosize: u64,
    pub volsize: u128,
    pub objtable_idx: u128,
    pub objtable_len: u64,
    pub fs_flags: FsFlags,
    pub base_chksum: u32,
    pub allocation_map: u128,
    pub reserved80: [u8; 48],
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Zeroable, Pod)]
    pub struct FsFlags : u32 {
        const LONG_VOL = 1;
        const VERSION1 = 2;
    }
}
