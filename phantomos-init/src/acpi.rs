use bytemuck::{Pod, Zeroable};
use core::fmt::{self, Debug};

#[derive(Clone, Copy, Hash, Pod, Zeroable)]
#[repr(C, packed)]
pub struct RsdpDescriptor {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    ext_checksum: u8,
    reserved: [u8; 3],
}

#[derive(Clone, Copy, Debug, Hash, Pod, Zeroable)]
#[repr(C)]
pub struct AcpiSdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

pub unsafe trait AcpiSdt: Pod {
    fn id() -> &'static [u8; 4];
}

#[derive(Clone, Copy, Debug, Hash, Pod, Zeroable)]
#[repr(C, packed)]
pub struct Bgrt {
    header: AcpiSdtHeader,
    version: u16,
    status: u8,
    image_type: u8,
    image_addr: u64,
    image_x_offset: u32,
    image_y_offset: u32,
}

unsafe impl AcpiSdt for Bgrt {
    fn id() -> &'static [u8; 4] {
        b"BGRT"
    }
}

#[derive(Clone, Copy, Hash, Zeroable)]
#[repr(transparent)]
pub struct AcpiSdtPointer(*const AcpiSdtHeader);

impl AcpiSdtPointer {
    fn downcast<T: AcpiSdt>(self) -> Option<&'static T> {
        let header = unsafe { &*self.0 };
        if &header.signature == T::id() {
            Some(unsafe { &*(self.0 as *const T) })
        } else {
            None
        }
    }
}

impl Debug for AcpiSdtPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = unsafe { &*self.0 };
        match &header.signature {
            b"BGRT" => self.downcast::<Bgrt>(),
            x => todo!("signature {}", unsafe { core::str::from_utf8_unchecked(x) }),
        }
        .fmt(f)
    }
}

#[derive(Debug)]
pub struct Xsdt {
    header: &'static AcpiSdtHeader,
    sdt_list: &'static [AcpiSdtPointer],
}

impl Xsdt {
    // # Safety
    // If the pointer passed is not to a valid XSDT structure, everything will go wrong. Very wrong.
    pub unsafe fn load(loc: *const u8) -> Self {
        let header: &'static AcpiSdtHeader =
            bytemuck::cast_ref(&*(loc as *const [u8; core::mem::size_of::<AcpiSdtHeader>()]));
        let sdt_list = core::slice::from_raw_parts(
            loc.add(core::mem::size_of::<AcpiSdtHeader>()) as *const AcpiSdtPointer,
            (header.length as usize - core::mem::size_of::<AcpiSdtHeader>()) / 8,
        );
        Xsdt { header, sdt_list }
    }
}

impl RsdpDescriptor {
    pub fn validate(&self) {
        // TODO: Check signature field and checksums, panic if something's wrong
    }

    pub fn oem_id(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.oem_id) }
    }

    pub fn xsdt(&self) -> Xsdt {
        assert!(self.revision >= 2);
        unsafe { Xsdt::load(self.xsdt_address as *const u8) }
    }
}
