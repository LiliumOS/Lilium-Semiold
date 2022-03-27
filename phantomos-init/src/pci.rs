use alloc::vec::Vec;
use crate::util::outl;

pub struct PciAddress {
    bus: u8,
    device: u8,
    function: u8,
}

pub struct PciFunction {
    address: PciAddress,
}

bitflags! {
    struct PciFunctionStatus: u16 {
        const DTC_PARITY_ERR     = 0b1000000000000000;
        const SIG_SYSTEM_ERR     = 0b0100000000000000;
        const REC_MASTER_ABRT    = 0b0010000000000000;
        const REC_TARGET_ABRT    = 0b0001000000000000;
        const SIG_TARGET_ABRT    = 0b0000100000000000;
        const DEVSEL_TIMING_MASK = 0b0000011000000000;
        const MASTER_PARITY_ERR  = 0b0000000100000000;
        const FAST_BACK_TO_BACK  = 0b0000000010000000;
        const SUPPORTS_66MHZ     = 0b0000000000100000;
        const CAPS_LIST          = 0b0000000000010000;
        const INT_STATUS         = 0b0000000000001000;
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[repr(u8)]
pub enum PciClass {
    Unclassified(PciUnclassifiedSubclass),
    MassStorageController(PciMassStorageControllerSubclass),
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum PciUnclassifiedSubclass {
    NonVgaCompatible,
    VgaCompatible,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum PciMassStorageSubclass {
    ScsiBusController,
    IdeController(IdeControllerProgIf),
    FloppyDiskController,
    IpiBusController,
    RaidController,
    AtaController(AtaControllerProgIf),
    SataController(SataControllerProgIf),
    SasController(SasControllerProgIf),
    NvmController(NvmControllerProgIf),
    Other = 0x80,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum IdeControllerProgIf {
    IsaCompat,
    PciNative = 5,
    IsaCompatPciNative = 0xA,
    PciNativeIsaCompat = 0xF,
    IsaCompatBusMaster = 0x80,
    PciNativeBusMaster = 0x85,
    IsaCompatPciNativeBusMaster = 0x8A,
    PciNativeIsaCompatBusMaster = 0x8F,
}

pub enum IdeControllerDefaultMode {
    IsaCompat,
    PciNative,
}

impl IdeControllerProgIf {
    #[inline(always)]
    pub fn default_mode(&self) -> IdeControllerDefaultMode {
        match self {
            Self::IsaCompat | Self::IsaCompatPciNative | Self::IsaCompatBusMaster | Self::IsaCompatPciNativeBusMaster => IdeControllerDefaultMode::IsaCompat,
            Self::PciNative | Self::PciNativeIsaCompat | Self::PciNativeBusMaster | Self::PciNativeIsaCompatBusMaster => IdeControllerDefaultMode::PciNative,
        }
    }

    #[inline(always)]
    pub fn supports_isa_compat(&self) -> bool {
        !matches!(self, Self::PciNative | Self::PciNativeBusMaster)
    }

    #[inline(always)]
    pub fn supports_pci_native(&self) -> bool {
        !matches!(self, Self::IsaCompat | Self::IsaCompatBusMaster)
    }

    #[inline(always)]
    pub fn supports_bus_mastering(&self) -> bool {
        (self & 0x80) != 0
    }
}

#[repr(packed)]
pub struct PciConfig {
    device_id: u16,
    vendor_id: u16,
    status: PciFunctionStatus,
    class: PciClass,
}

fn read_pci_config(addr: PciAddress) -> Option<PciConfig> {
    todo!()
}

pub fn enumerate() -> Vec<PciFunction> {
    let mut result = Vec::new();
    result
}
