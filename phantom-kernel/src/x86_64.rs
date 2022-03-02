#[repr(align(16))]
pub struct InterruptFrame {
    pub fxstor: [u8; 512],
    pub rbp: *mut core::ffi::c_void,
    pub cr0: *mut core::ffi::c_void,
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
}

