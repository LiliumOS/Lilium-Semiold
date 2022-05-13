#[cfg(target_arch = "x86_64")]
pub mod system {
    use super::KernelState;
    #[repr(C)]
    pub struct SystemKernelState {
        ss: u64,
        rsp: u64,
    }

    pub fn get_kernel_state() -> &'static KernelState {
        let mut ret: &'static KernelState;
        unsafe {
            core::arch::asm!("lea {}, gs:0", out(reg) ret);
        }
        ret
    }
}

#[repr(C)]
pub struct KernelState {
    pub sys: system::SystemKernelState,
    pub uthread: Cell<HandlePtr<ThreadHandle>>,
}

use core::cell::Cell;

pub use system::get_kernel_state;

use crate::{addr_space::HandlePtr, thread::ThreadHandle};
