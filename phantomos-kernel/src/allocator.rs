use core::alloc::{GlobalAlloc, Layout};
use phantomos_init::allocator::{kalloc, kfree};

struct KernelAllocator;

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

unsafe impl Sync for KernelAllocator {}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        kalloc(layout.size(), layout.align(), None)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        kfree(ptr, layout.size(), layout.align())
    }
}
