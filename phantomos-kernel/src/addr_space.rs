use core::hash::{Hash, Hasher};

use std::sync::atomic::{AtomicPtr, HasAtomic, HasAtomicLeast};

#[repr(transparent)]
pub struct UserPtr<T: ?Sized>(*mut T);

#[repr(transparent)]
pub struct HandlePtr<T: ?Sized>(*mut T);

impl<T: ?Sized> HandlePtr<T> {
    pub fn into_kernel_addr(self) -> *mut T {
        self.0
    }

    pub fn from_kernel_addr(kaddr: *mut T) -> Self {
        Self(kaddr)
    }
}

#[repr(transparent)]
pub struct IOPtr<T: ?Sized>(*mut T);

macro_rules! impl_traits_for_addr_space {
    ($($addr_ptr:ident),* $(,)?) => {
        $(
        impl<T: ?Sized> Copy for $addr_ptr<T> {}

        impl<T: ?Sized> Clone for $addr_ptr<T> {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }
        impl<T: ?Sized> PartialEq for $addr_ptr<T> {
            fn eq(&self, rhs: &Self) -> bool {
                self.0 == rhs.0
            }
        }
        impl<T: ?Sized> Eq for $addr_ptr<T> {}

        impl<T: ?Sized> PartialOrd for $addr_ptr<T> {
            fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(&rhs.0)
            }
        }

        impl<T: ?Sized> Ord for $addr_ptr<T> {
            fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
                self.0.cmp(&rhs.0)
            }
        }

        impl<T: ?Sized> Hash for $addr_ptr<T>{
            fn hash<H: Hasher>(&self, state: &mut H){
                self.0.hash(state)
            }
        }

        impl<T: ?Sized> core::fmt::Pointer for $addr_ptr<T>{
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result{
                self.0.fmt(f)
            }
        }

        impl<T: ?Sized> core::fmt::Debug for $addr_ptr<T>{
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result{
                f.debug_tuple(::core::stringify!($addr_ptr)).field(&self.0).finish()
            }
        }

        unsafe impl<T> HasAtomic for $addr_ptr<T> {
            type Atomic = AtomicPtr<T>;
            type Storage = [u8; core::mem::size_of::<*mut ()>()];

            fn into_base(self) -> *mut T {
                self.0
            }

            unsafe fn from_base(base: *mut T) -> Self {
                Self(base)
            }
        }

        unsafe impl<T> HasAtomicLeast for $addr_ptr<T> {
            type AtomicLeast = AtomicPtr<T>;
            type Storage = [u8; core::mem::size_of::<*mut ()>()];

            fn into_least(self) -> *mut T {
                self.0
            }

            unsafe fn from_least(base: *mut T) -> Self {
                Self(base)
            }
        }
    )*
    };
}

impl_traits_for_addr_space!(UserPtr, HandlePtr, IOPtr);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct PhysAddr(*mut ());

impl core::fmt::Pointer for PhysAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
unsafe impl HasAtomic for PhysAddr {
    type Atomic = AtomicPtr<()>;
    type Storage = [u8; core::mem::size_of::<*mut ()>()];

    fn into_base(self) -> *mut () {
        self.0
    }

    unsafe fn from_base(base: *mut ()) -> Self {
        Self(base)
    }
}

unsafe impl HasAtomicLeast for PhysAddr {
    type AtomicLeast = AtomicPtr<()>;
    type Storage = [u8; core::mem::size_of::<*mut ()>()];

    fn into_least(self) -> *mut () {
        self.0
    }

    unsafe fn from_least(base: *mut ()) -> Self {
        Self(base)
    }
}
