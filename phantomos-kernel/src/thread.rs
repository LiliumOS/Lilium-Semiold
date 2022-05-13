use core::sync::atomic::Ordering;
use std::{
    cell::RacyCell,
    collection::RingBuffer,
    sync::atomic::{AtomicCell, AtomicFlag, AtomicLeastCell},
};

use crate::{
    addr_space::{HandlePtr, PhysAddr},
    handle::Handle,
    security::SecurityDescriptor,
    state,
};

#[repr(u16)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ThreadStatus {
    Unstarted,
    Running,
    Blocked,
    Suspended,
    Stopped,
    Dead,
    Killed,
}

std::impl_has_atomic_for_enum!(ThreadStatus, u16, target_has_atomic = "16");

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum BlockError {
    Interrupted,
    Timeout,
}

#[repr(C)]
pub struct ThreadHandle {
    handle: Handle,
    status: AtomicLeastCell<ThreadStatus>,
    interrupted_flag: AtomicFlag,
    token_flag: AtomicFlag,
    pending_signals: RingBuffer<u32>,
    waiting_address: AtomicCell<PhysAddr>,
    exit_status: AtomicLeastCell<u32>,
    priority: AtomicLeastCell<u32>,
    security_ctx: RacyCell<HandlePtr<SecurityDescriptor>>,
}

impl ThreadHandle {
    pub fn interrupt(&self) {
        self.interrupted_flag.clear(Ordering::Release);
        // TODO: Reschedule self
    }

    fn is_current_thread(&self) -> bool {
        let state = state::get_kernel_state();

        core::ptr::eq(state.uthread.get().into_kernel_addr(), self)
    }
}
