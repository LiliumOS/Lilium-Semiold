use std::sync::atomic::AtomicCell;

#[repr(usize)]
pub enum HandleType {
    Unused = 0,
    ThreadHandle = 1,
    ProcessHandle = 2,
    IOHandle = 3,
}

/// # Safety
/// TODO
pub unsafe trait HandleKind {
    const TYPE: HandleType;
}

#[repr(C)]
pub struct Handle {
    pub ty: HandleType,
    pub rc: AtomicCell<usize>,
}
