use core::num::{NonZeroU32, NonZeroU64};
use std::io::{self, Read, Seek, SeekFrom, Write};

use bytemuck::{Pod, Zeroable};

use crate::traits::{ReadFS, Search, StreamId};

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

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct DirectoryElement {
    objidx: Option<NonZeroU64>,
    name_index: Option<NonZeroU64>,
    flags: u64,
    name: [u8; 40],
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct DeviceId {
    id_hi: u64,
    id_lo: u64,
}

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct LegacyDeviceNumber {
    major: u32,
    minor: u32,
}

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct SecurityDescriptorRow {
    principal: u128,
    stream_id: Option<NonZeroU64>,
    flags_and_mode: u64,
    permission_name_ref: Option<NonZeroU64>,
    permission_name: [u8; 24],
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct LegacySecurityDescriptor {
    sd_uid: u32,
    sd_gid: u32,
    sd_mode: u16,
    sd_reserved: [u8; 6],
}

pub mod consts {
    pub const STREAM_STREAMS: &[u8] = b"Streams\0";
    pub const STREAM_CUSTOM_OBJECT_INFO: &[u8] = b"CustomObjectInfo\0";
    pub const STREAM_STRINGS: &[u8] = b"Strings\0";
    pub const STREAM_FILE_DATA: &[u8] = b"FileData\0";
    pub const STREAM_DIRECTORY_CONTENT: &[u8] = b"DirectoryContent\0";
    pub const STREAM_SYMLINK_TARGET: &[u8] = b"SymlinkTarget\0";
    pub const STREAM_DEVICEID: &[u8] = b"DeviceId\0";
    pub const STREAM_LEGACY_DEVICE_NUMBER: &[u8] = b"LegacyDeviceNumber\0";
    pub const STREAM_SECURITY_DESCRIPTOR: &[u8] = b"SecurityDescriptor\0";

    pub const PHANTOMFS_MAGIC: [u8; 4] = *b"\x0FSPh";

    pub const MAJOR_VERSION: u32 = 1;
    pub const MINOR_VERSION: u32 = 0;
    pub const REVISION: u32 = 0;
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Zeroable, Pod)]
    pub struct FSFeatures: u64 {

    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Zeroable, Pod)]
    pub struct FSROFeatures: u64 {

    }
}

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Zeroable, Pod)]
pub struct RootFSDescriptor {
    /// The magic number of the filesystem
    magic: [u8; 4],
    /// The major version.
    major: u32,
    /// The minor version
    minor: u32,
    /// The revision number
    revision: u32,
    /// The partition id.
    partid: u128,
    /// Filesystem features that are required to access this filesystem
    features: FSFeatures,
    /// Filesystem features that are required to write to this filesystem, but optional when the filesystem is mounted for reading only
    rofeatures: FSROFeatures,
    /// The sector that is one byte past the end of the object table
    /// The object table grows downwards from this position
    objtab: u128,
    /// The total size (in bytes) of the object table.
    objtabsize: u64,
    /// The index (entry number) of the root object in the object table
    /// If `0`, the the filesystem needs an object table created
    rootidx: u64,
    /// An index in the root object's `Strings` stream that contains the name of this partion or `None`
    partnameidx: Option<NonZeroU64>,
    /// The UTF-8 encoded string that contains the name of this partition if it up to 24 bytes in total (with zero in the remaining bytes).
    partname: [u8; 24],
    reserved112: [u8; 8],
    /// The total size of the root descriptor
    descriptor_size: u32,
    /// The CRC32 checksum of the complete descriptor
    descriptor_crc: u32,
}

pub struct PhantomFS<S> {
    stream: S,
    descriptor: Option<RootFSDescriptor>,
}

impl<S> PhantomFS<S> {
    pub const fn new(inner: S) -> Self {
        Self {
            stream: inner,
            descriptor: None,
        }
    }

    pub fn into_inner(self) -> S {
        self.stream
    }

    pub fn create_new_fs(&mut self, partid: u128) {
        let mut desc = RootFSDescriptor {
            magic: consts::PHANTOMFS_MAGIC,
            major: consts::MAJOR_VERSION,
            minor: consts::MINOR_VERSION,
            revision: consts::REVISION,
            partid,
            descriptor_size: core::mem::size_of::<RootFSDescriptor>() as u32,
            ..Zeroable::zeroed()
        };

        let mut crc = crc_any::CRCu32::crc32();
        crc.digest(bytemuck::bytes_of(&desc));
        desc.descriptor_crc = crc.get_crc();

        self.descriptor = Some(desc);
    }
}

impl<S: Read + Seek> PhantomFS<S> {
    pub fn read_descriptor(&mut self) -> std::io::Result<()> {
        self.stream.seek(SeekFrom::Start(1024))?;
        let desc = self.descriptor.get_or_insert_with(Zeroable::zeroed);
        self.stream.read_exact(bytemuck::bytes_of_mut(desc))?;

        if desc.magic != consts::PHANTOMFS_MAGIC {
            return Err(io::Error::InvalidData(Some(alloc::format!(
                "Invalid Magic {:x?}",
                desc.magic
            ))));
        }

        Ok(())
    }

    pub fn get_or_read_descriptor(&mut self) -> std::io::Result<&mut RootFSDescriptor> {
        match &mut self.descriptor {
            Some(desc) => Ok(unsafe { &mut *(desc as *mut RootFSDescriptor) }),
            None => {
                self.read_descriptor()?;
                Ok(self.descriptor.as_mut().unwrap())
            }
        }
    }
}

impl<S: Read + Seek> Search for PhantomFS<S> {
    fn get_object_from(
        &mut self,
        _pos: crate::traits::InodeId,
        _pname: std::str::StringView,
    ) -> std::io::Result<crate::traits::ObjectId> {
        todo!()
    }

    fn get_stream_of_object(
        &mut self,
        _obj: crate::traits::ObjectId,
        _lname: std::str::StringView,
    ) -> std::io::Result<crate::traits::StreamId> {
        let _desc = self.get_or_read_descriptor()?;

        Ok(StreamId(None))
    }
}

impl<S: Read + Seek> ReadFS for PhantomFS<S> {
    fn read_bytes_from(
        &mut self,
        node: crate::traits::InodeId,
        _offset: u64,
        _bytes: &mut [u8],
    ) -> std::io::Result<usize> {
        let desc = self.get_or_read_descriptor()?;
        let _objtab = desc.objtab;
        let obj: u64 = if let Some(obj) = node.0 .0 {
            obj.get()
        } else {
            desc.rootidx
        };

        if obj > (desc.objtabsize / u64::try_from(core::mem::size_of::<PhantomFSObject>()).unwrap())
        {
            return Err(std::io::Error::NotFound);
        }

        let objtab = desc.objtab;

        #[allow(clippy::drop_ref)]
        {
            drop(desc); // ensure we don't use it again
        }

        self.stream.seek(SeekFrom::StartFar(objtab))?;
        let objoffset: u64 = obj * u64::try_from(core::mem::size_of::<PhantomFSObject>()).unwrap();

        let pos = -(objoffset as i64);
        self.stream.seek(SeekFrom::Current(pos))?;

        let mut obj: PhantomFSObject = Zeroable::zeroed();

        self.stream.read_exact(bytemuck::bytes_of_mut(&mut obj))?;

        let streamdisp = obj.streams_ref;

        let indirect = obj.streams_indirection;

        let streamslen = obj.streams_size;

        let streampos = node.1 .0.ok_or(std::io::Error::NotFound)?.get();

        if streampos >= (streamslen / u64::try_from(core::mem::size_of::<StreamListing>()).unwrap())
        {
            return Err(std::io::Error::NotFound);
        }

        if indirect == 0 {
            return Err(std::io::Error::NotFound);
        }

        if indirect == 1 {
            self.stream.seek(SeekFrom::StartFar(streamdisp))?;
            let disp =
                (streampos * u64::try_from(core::mem::size_of::<StreamListing>()).unwrap()) as i64;
            self.stream.seek(SeekFrom::Current(disp))?;
            let mut stream = StreamListing::zeroed();
            self.stream
                .read_exact(bytemuck::bytes_of_mut(&mut stream))?;
        } else {
        }

        Err(std::io::Error::Interrupted)
    }
}

impl<S: Write + Seek> PhantomFS<S> {
    pub fn write_descriptor(&mut self) -> std::io::Result<()> {
        if let Some(desc) = &self.descriptor {
            self.stream.seek(SeekFrom::Start(1024))?;
            self.stream.write_all(bytemuck::bytes_of(desc))?;
        }
        Ok(())
    }
}
