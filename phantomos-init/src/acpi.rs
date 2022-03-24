use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Hash, Pod, Zeroable)]
#[repr(C, packed)]
pub struct RsdpDescriptor {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    ext_checksum: u8,
    reserved: [u8; 3],
}

impl RsdpDescriptor {
    pub fn validate(&self) {
        // TODO: Check signature field and checksums, panic if something's wrong
    }

    pub fn oemid(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.oemid) }
    }
}
