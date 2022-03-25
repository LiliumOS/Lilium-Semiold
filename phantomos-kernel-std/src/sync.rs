use core::{arch::asm, mem::MaybeUninit};

pub mod atomic {
    pub use core::sync::atomic::*;
}
const ONCE_INIT: usize = 0x02;
const ONCE_LOCKED: usize = 0x01;

pub struct OnceCell<T> {
    flag: AtomicUsize,
    cell: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send + Sync> Sync for OnceCell<T> {}

impl<T> Default for OnceCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> OnceCell<T> {
    pub const fn new() -> Self {
        Self {
            flag: AtomicUsize::new(0),
            cell: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    fn begin_init(&self) -> bool {
        if (self.flag.load(atomic::Ordering::Relaxed) & ONCE_INIT) == 0 {
            while (self.flag.fetch_or(ONCE_LOCKED, atomic::Ordering::Acquire) & ONCE_LOCKED) != 0 {}
            if (self.flag.load(atomic::Ordering::Relaxed) & ONCE_INIT) == 0 {
                true
            } else {
                self.flag.store(ONCE_INIT, atomic::Ordering::Relaxed);
                false
            }
        } else {
            atomic::fence(atomic::Ordering::Acquire);
            false
        }
    }

    unsafe fn end_init(&self) {
        self.flag.store(ONCE_INIT, atomic::Ordering::Release);
    }

    unsafe fn abort_init(&self) {
        self.flag.store(0, atomic::Ordering::Release);
    }

    pub fn get(&self) -> Option<&T> {
        if (self.flag.load(Ordering::Acquire) & ONCE_INIT) != 0 {
            // SAFETY:
            // Initialization has occured, and no other initialization can be performed.
            // No writes occur after the first initialization w/o a mutable reference
            // Thus returning a `&T` is safe
            Some(unsafe { &*(self.cell.get().cast::<T>()) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if (self.flag.load(Ordering::Acquire) & ONCE_INIT) != 0 {
            // SAFETY:
            // Initialization has occured, so the value returned is valid
            // Having `&mut self` precludes all other access to the OnceCell
            Some(unsafe { &mut *(self.cell.get().cast::<T>()) })
        } else {
            None
        }
    }

    pub fn set(&self, val: T) -> Result<(), T> {
        if self.begin_init() {
            // SAFETY:
            // We haven't initialized anything yet, so no references to the inner exists
            // We hold a lock, so no data race can occur with this line
            unsafe {
                (*self.cell.get()).write(val);
            }
            // SAFETY:
            // We hold the lock, so it's safe to release it
            unsafe {
                self.end_init();
            }
            Ok(())
        } else {
            Err(val)
        }
    }

    pub fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> &T {
        if self.begin_init() {
            // SAFETY:
            // We haven't initialized anything yet, so no references to the inner exists
            // We hold a lock, so no data race can occur with this line
            unsafe {
                (*self.cell.get()).write(f());
            }

            unsafe {
                self.end_init();
            }
        }
        // SAFETY:
        // Initialization has just occured, and no other initialization can be performed.
        // No writes occur after the first initialization w/o a mutable reference
        // Thus returning a `&T` is safe
        Some(unsafe { &*(self.cell.get().cast::<T>()) })
    }

    pub fn get_or_try_init<E, F: FnOnce() -> Result<T, E>>(&self, f: F) -> Result<&T, E> {
        if self.begin_init() {
            match f() {
                Ok(t) => {
                    unsafe {
                        (*self.cell.get()).write(t);
                    }
                    unsafe {
                        self.end_init();
                    }
                }
                Err(e) => {
                    unsafe {
                        self.abort_init();
                    }
                    return Err(e);
                }
            }
        }

        // SAFETY:
        // Initialization has just occured, and no other initialization can be performed.
        // No writes occur after the first initialization w/o a mutable reference
        // Thus returning a `&T` is safe
        Some(unsafe { &*(self.cell.get().cast::<T>()) })
    }
}
