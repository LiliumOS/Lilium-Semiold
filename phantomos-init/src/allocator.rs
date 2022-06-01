use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::{null_mut, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};

const ARENA_SIZE: usize = 2 * 1024 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
struct PhantomAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize, // we allocate from the top, counting down
}

#[global_allocator]
static ALLOCATOR: PhantomAllocator = PhantomAllocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

unsafe impl Sync for PhantomAllocator {}

unsafe impl GlobalAlloc for PhantomAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        let result = self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            });
        if result.is_err() {
            return null_mut();
        };
        // SAFETY: `allocated` is guaranteed to be greater than 0 and less than ARENA_SIZE
        unsafe { (self.arena.get() as *mut u8).add(allocated) }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

pub unsafe extern "C" fn kalloc(
    size: usize,
    align: usize,
    _vaddr_hint: Option<NonNull<()>>,
) -> *mut u8 {
    // SAFETY: contract shall be upheld by calling function
    unsafe { ALLOCATOR.alloc(Layout::from_size_align_unchecked(size, align)) }
}

pub unsafe extern "C" fn kfree(ptr: *mut u8, size: usize, align: usize) {
    // SAFETY: contract shall be upheld by calling function
    unsafe { ALLOCATOR.dealloc(ptr, Layout::from_size_align_unchecked(size, align)) }
}
