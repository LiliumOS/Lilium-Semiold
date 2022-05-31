use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

extern "C" {
    fn kalloc(size: usize, align: usize, vaddr_hint: Option<NonNull<u8>>) -> *mut u8;
    fn kfree(ptr: *mut u8, size: usize, align: usize);
}

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
