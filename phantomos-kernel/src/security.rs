use crate::handle::Handle;

#[repr(C)]
pub struct SecurityDescriptor {
    header: Handle,
}
