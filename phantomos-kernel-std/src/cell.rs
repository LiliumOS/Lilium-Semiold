pub use core::cell::*;

pub struct RacyCell<T: ?Sized>(UnsafeCell<T>);

unsafe impl<T: ?Sized + Sync> Sync for RacyCell<T> {}

impl<T> RacyCell<T> {
    pub const fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    pub unsafe fn take_unchecked(&self, val: T) -> T {
        core::ptr::replace(self.get(), val)
    }
}

impl<T: ?Sized> RacyCell<T> {
    pub fn get(&self) -> *mut T {
        self.0.get()
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_unchecked_mut(&self) -> &mut T {
        &mut *(self.get())
    }

    pub unsafe fn get_unchecked(&self) -> &T {
        &*(self.get())
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}
