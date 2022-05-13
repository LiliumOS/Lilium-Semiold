use core::{arch::asm, cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicUsize};

pub mod atomic {
    pub use core::sync::atomic::*;

    use core::{
        cell::UnsafeCell,
        mem::{ManuallyDrop, MaybeUninit},
    };

    pub unsafe trait Atomic: Sized {
        type Base: HasAtomic;

        fn load(&self, ord: Ordering) -> Self::Base;

        fn store(&self, val: Self::Base, ord: Ordering);

        fn swap(&self, val: Self::Base, ord: Ordering) -> Self::Base;

        fn compare_exchange(
            &self,
            cmp: Self::Base,
            new: Self::Base,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<Self::Base, Self::Base>;

        fn compare_exchange_weak(
            &self,
            cmp: Self::Base,
            new: Self::Base,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<Self::Base, Self::Base>;
    }

    pub unsafe trait HasAtomic: Copy + Sized {
        type Atomic: Atomic;
        type Storage: Copy + Sized;

        fn into_base(self) -> <Self::Atomic as Atomic>::Base;

        unsafe fn from_base(base: <Self::Atomic as Atomic>::Base) -> Self;
    }

    /// Trait for types that an atomic operation exists for a type that is at least the size of `Self`
    /// This trait is implemented for the smallest type that can exactly represent atomic operations.
    ///
    /// The operations `into_least` and `from_least` shall be equivalent to the same transmute where both are well-defined.
    pub unsafe trait HasAtomicLeast: Copy + Sized {
        type AtomicLeast: Atomic;
        type Storage: Copy + Sized;

        /// Converts from `Self` into the least base type.
        fn into_least(self) -> <Self::AtomicLeast as Atomic>::Base;

        /// Converts from the least type into `Self`.
        ///
        /// Must at least roundtrip values from [`HasAtomicLeast::into_least`]
        ///
        /// # Safety
        /// The value of `least` must be a valid value of `Self`. For example, if `Self` is `bool`, then `least` must be either `0` or `1`
        unsafe fn from_least(least: <Self::AtomicLeast as Atomic>::Base) -> Self;
    }

    macro_rules! impl_has_atomic_atomic{
        {
            $([
                $cfg:meta,
                $base:ty,
                $atomic:ty
            ]),* $(,)?
        } => {
            $(
            #[cfg($cfg)]
            unsafe impl HasAtomic for $base{
                type Atomic = $atomic;
                type Storage = [u8;core::mem::size_of::<$atomic>()];

                #[inline(always)]
                fn into_base(self) -> Self{
                    self
                }

                #[inline(always)]
                unsafe fn from_base(base: Self) -> Self{
                    base
                }
            }

            #[cfg($cfg)]
            unsafe impl HasAtomicLeast for $base{
                type AtomicLeast = $atomic;
                type Storage = [u8;core::mem::size_of::<$atomic>()];

                #[inline(always)]
                fn into_least(self) -> Self {
                    self
                }

                #[inline(always)]
                unsafe fn from_least(least: Self) -> Self {
                    least
                }
            }

            #[cfg($cfg)]
            unsafe impl Atomic for $atomic{
                type Base = $base;

                #[inline(always)]
                fn load(&self, ord: Ordering) -> $base{
                    self.load(ord)
                }

                #[inline(always)]
                fn store(&self, val: $base, order: Ordering){
                    self.store(val, order)
                }

                #[inline(always)]
                fn swap(&self,val: $base, order: Ordering) -> $base{
                    self.swap(val,order)
                }

                #[inline(always)]
                fn compare_exchange(&self, current: $base, new: $base, success: Ordering, failure: Ordering) -> Result<$base,$base>{
                    self.compare_exchange(current,new,success,failure)
                }

                #[inline(always)]
                fn compare_exchange_weak(&self, current: $base, new: $base, success: Ordering, failure: Ordering) -> Result<$base,$base>{
                    self.compare_exchange_weak(current,new,success,failure)
                }
            }
            )*
        }
    }

    impl_has_atomic_atomic! {
        [target_has_atomic = "8", bool, AtomicBool],
        [target_has_atomic = "8", u8, AtomicU8],
        [target_has_atomic = "8", i8, AtomicI8],
        [target_has_atomic = "16", u16, AtomicU16],
        [target_has_atomic = "16", i16, AtomicI16],
        [target_has_atomic = "32", u32, AtomicU32],
        [target_has_atomic = "32", i32, AtomicI32],
        [target_has_atomic = "64", u64, AtomicU64],
        [target_has_atomic = "64", i64, AtomicI64],
        [target_has_atomic = "ptr", usize, AtomicUsize],
        [target_has_atomic = "ptr", isize, AtomicIsize]
    }

    macro_rules! impl_has_atomic_least_one {
        [$base_cfg:meta, $base:ty, {}] => {};
        [$base_cfg:meta, $base:ty, {
            $next_cfg:meta => $least_base:ty | $least_atomic:ty,
            $($rest_cfg:meta => $rest_least_base:ty | $rest_least_atomic:ty),* $(,)?
        }] => {
            #[cfg(not($base_cfg))]
            #[cfg($next_cfg)]
            unsafe impl HasAtomicLeast for $base{
                type AtomicLeast = $least_atomic;
                type Storage = <$least_base as HasAtomic>::Storage;

                #[inline(always)]
                fn into_least(self) -> $least_base{
                    self as $least_base
                }

                #[inline(always)]
                unsafe fn from_least(least: $least_base) -> Self{
                    least as $base
                }
            }

            impl_has_atomic_least_one![all($base_cfg,$next_cfg), $base, {$($rest_cfg => $rest_least_base | $rest_least_atomic ,)* }];
        }
    }

    macro_rules! impl_has_atomic_least {
        {
            $([$base_cfg:meta, $base:ty, {$($next_cfg:meta => $least_base:ty | $least_atomic:ty),* $(,)?}]),* $(,)?
        } => {
            $(
                impl_has_atomic_least_one![$base_cfg, $base, {$($next_cfg => $least_base | $least_atomic,)*}];
            )*
        }
    }

    macro_rules! impl_has_atomic_least_bool{
        [$base_cfg:meta, $base:ty, {}] => {};
        [$base_cfg:meta, {
            $next_cfg:meta => $least_base:ty | $least_atomic:ty,
            $($rest_cfg:meta => $rest_least_base:ty | $rest_least_atomic:ty),* $(,)?
        }] => {
            #[cfg(not($base_cfg))]
            #[cfg($next_cfg)]
            unsafe impl HasAtomicLeast for bool{
                type AtomicLeast = $least_atomic;
                type Storage = <$least_base as HasAtomic>::Storage;

                fn into_least(self) -> $least_base{
                    self as $least_base
                }

                fn from_least(least: $least_base) -> Self{
                    least != 0
                }
            }
        }
    }

    impl_has_atomic_least! {
        [target_has_atomic = "8", u8, {target_has_atomic = "16" => u16 | AtomicU16,target_has_atomic = "32" => u32 | AtomicU32,target_has_atomic = "64" => u64 | AtomicU64}],
        [target_has_atomic = "8", i8, {target_has_atomic = "16" => u16 | AtomicU16,target_has_atomic = "32" => u32 | AtomicU32,target_has_atomic = "64" => u64 | AtomicU64}],
        [target_has_atomic = "16", u16, {target_has_atomic = "32" => u32 | AtomicU32,target_has_atomic = "64" => u64 | AtomicU64}],
        [target_has_atomic = "16", i16, {target_has_atomic = "32" => u32 | AtomicU32,target_has_atomic = "64" => u64 | AtomicU64}],
        [target_has_atomic = "32", u32, {target_has_atomic = "64" => u64 | AtomicU64}],
        [target_has_atomic = "32", i32, {target_has_atomic = "64" => u64 | AtomicU64}],
    }

    impl_has_atomic_least_bool![target_has_atomic = "8", {target_has_atomic = "16" => u16 | AtomicU16,target_has_atomic = "32" => u32 | AtomicU32,target_has_atomic = "64" => u64 | AtomicU64, target_has_atomic = "ptr" => usize | AtomicUsize}];

    #[cfg(target_has_atomic = "ptr")]
    unsafe impl<T> HasAtomic for *mut T {
        type Atomic = AtomicPtr<T>;
        type Storage = [u8; core::mem::size_of::<*mut ()>()];

        #[inline(always)]
        fn into_base(self) -> Self {
            self
        }

        #[inline(always)]
        unsafe fn from_base(base: Self) -> Self {
            base
        }
    }

    #[cfg(target_has_atomic = "ptr")]
    unsafe impl<T> Atomic for AtomicPtr<T> {
        type Base = *mut T;

        #[inline(always)]
        fn load(&self, ord: Ordering) -> Self::Base {
            self.load(ord)
        }

        #[inline(always)]
        fn store(&self, val: Self::Base, ord: Ordering) {
            self.store(val, ord)
        }

        #[inline(always)]
        fn swap(&self, val: Self::Base, ord: Ordering) -> Self::Base {
            self.swap(val, ord)
        }

        #[inline(always)]
        fn compare_exchange(
            &self,
            cmp: Self::Base,
            new: Self::Base,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<Self::Base, Self::Base> {
            self.compare_exchange(cmp, new, succ_ord, fail_ord)
        }

        #[inline(always)]
        fn compare_exchange_weak(
            &self,
            cmp: Self::Base,
            new: Self::Base,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<Self::Base, Self::Base> {
            self.compare_exchange_weak(cmp, new, succ_ord, fail_ord)
        }
    }

    union AtomicStorage<Base: Copy, Storage: Copy, Align> {
        inner: Base,
        storage: MaybeUninit<Storage>,
        align: ManuallyDrop<[Align; 0]>,
    }

    impl<Base: Copy, Storage: Copy, Align> AtomicStorage<Base, Storage, Align> {
        #[inline(always)]
        pub const fn new_zeroed_storage(inner: Base) -> Self {
            let mut ret = AtomicStorage {
                storage: MaybeUninit::zeroed(),
            };
            ret.inner = inner;

            ret
        }

        #[inline(always)]
        pub const fn into_inner(self) -> Base {
            unsafe { self.inner }
        }

        #[inline(always)]
        pub fn get_mut(&mut self) -> &mut Base {
            unsafe { &mut self.inner }
        }
    }

    pub struct AtomicCell<T: HasAtomic> {
        inner: UnsafeCell<AtomicStorage<T, T::Storage, T::Atomic>>,
    }

    impl<T: HasAtomic> AtomicCell<T> {
        #[inline]
        pub const fn new(x: T) -> Self {
            Self {
                inner: UnsafeCell::new(AtomicStorage::new_zeroed_storage(x)),
            }
        }

        #[inline]
        pub fn into_inner(self) -> T {
            self.inner.into_inner().into_inner()
        }

        #[inline]
        pub fn get_mut(&mut self) -> &mut T {
            self.inner.get_mut().get_mut()
        }

        #[inline]
        fn as_atomic(&self) -> &T::Atomic {
            unsafe { &*(self.inner.get() as *mut T::Atomic) }
        }

        #[inline]
        pub fn load(&self, ord: Ordering) -> T {
            unsafe { T::from_base(self.as_atomic().load(ord)) }
        }

        #[inline]
        pub fn store(&self, val: T, ord: Ordering) {
            self.as_atomic().store(val.into_base(), ord)
        }

        #[inline]
        pub fn swap(&self, val: T, ord: Ordering) -> T {
            unsafe { T::from_base(self.as_atomic().swap(val.into_base(), ord)) }
        }

        #[inline]
        pub fn compare_exchange(
            &self,
            cmp: T,
            new: T,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<T, T> {
            match self.as_atomic().compare_exchange(
                cmp.into_base(),
                new.into_base(),
                succ_ord,
                fail_ord,
            ) {
                Ok(val) => Ok(unsafe { T::from_base(val) }),
                Err(val) => Err(unsafe { T::from_base(val) }),
            }
        }

        #[inline]
        pub fn compare_exchange_weak(
            &self,
            cmp: T,
            new: T,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<T, T> {
            match self.as_atomic().compare_exchange_weak(
                cmp.into_base(),
                new.into_base(),
                succ_ord,
                fail_ord,
            ) {
                Ok(val) => Ok(unsafe { T::from_base(val) }),
                Err(val) => Err(unsafe { T::from_base(val) }),
            }
        }
    }

    impl<T: HasAtomic + Default> Default for AtomicCell<T> {
        #[inline]
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: HasAtomic> From<T> for AtomicCell<T> {
        #[inline]
        fn from(val: T) -> Self {
            Self::new(val)
        }
    }

    unsafe impl<T: HasAtomic> Sync for AtomicCell<T> {}

    pub struct AtomicLeastCell<T: HasAtomicLeast> {
        inner: UnsafeCell<AtomicStorage<T, T::Storage, T::AtomicLeast>>,
    }

    unsafe impl<T: HasAtomicLeast> Sync for AtomicLeastCell<T> {}

    impl<T: HasAtomicLeast + Default> Default for AtomicLeastCell<T> {
        #[inline]
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: HasAtomicLeast> From<T> for AtomicLeastCell<T> {
        #[inline]
        fn from(val: T) -> Self {
            Self::new(val)
        }
    }

    impl<T: HasAtomicLeast> AtomicLeastCell<T> {
        #[inline]
        pub const fn new(x: T) -> Self {
            Self {
                inner: UnsafeCell::new(AtomicStorage::new_zeroed_storage(x)),
            }
        }

        #[inline]
        pub fn into_inner(self) -> T {
            self.inner.into_inner().into_inner()
        }

        #[inline]
        pub fn get_mut(&mut self) -> &mut T {
            self.inner.get_mut().get_mut()
        }

        #[inline]
        fn as_atomic(&self) -> &T::AtomicLeast {
            unsafe { &*(self.inner.get() as *mut T::AtomicLeast) }
        }

        #[inline]
        pub fn load(&self, ord: Ordering) -> T {
            // Safety: We're a valid value
            unsafe { T::from_least(self.as_atomic().load(ord)) }
        }

        #[inline]
        pub fn store(&self, val: T, ord: Ordering) {
            self.as_atomic().store(val.into_least(), ord)
        }

        #[inline]
        pub fn swap(&self, val: T, ord: Ordering) -> T {
            unsafe { T::from_least(self.as_atomic().swap(val.into_least(), ord)) }
        }

        #[inline]
        pub fn compare_exchange(
            &self,
            cmp: T,
            new: T,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<T, T> {
            match self.as_atomic().compare_exchange(
                cmp.into_least(),
                new.into_least(),
                succ_ord,
                fail_ord,
            ) {
                Ok(val) => Ok(unsafe { T::from_least(val) }),
                Err(val) => Err(unsafe { T::from_least(val) }),
            }
        }

        #[inline]
        pub fn compare_exchange_weak(
            &self,
            cmp: T,
            new: T,
            succ_ord: Ordering,
            fail_ord: Ordering,
        ) -> Result<T, T> {
            match self.as_atomic().compare_exchange_weak(
                cmp.into_least(),
                new.into_least(),
                succ_ord,
                fail_ord,
            ) {
                Ok(val) => Ok(unsafe { T::from_least(val) }),
                Err(val) => Err(unsafe { T::from_least(val) }),
            }
        }
    }

    #[macro_export]
    macro_rules! impl_has_atomic_for_enum {
        ($enum:ty, $base:ty, $base_cfg:meta) => {
            #[cfg($base_cfg)]
            unsafe impl $crate::sync::atomic::HasAtomic for $enum {
                type Atomic = <$base as $crate::sync::atomic::HasAtomic>::Atomic;
                type Storage = <$base as $crate::sync::atomic::HasAtomic>::Storage;

                fn into_base(self) -> $base {
                    self as $base
                }

                unsafe fn from_base(b: $base) -> Self {
                    core::mem::transmute(b)
                }
            }

            unsafe impl $crate::sync::atomic::HasAtomicLeast for $enum {
                type AtomicLeast = <$base as $crate::sync::atomic::HasAtomicLeast>::AtomicLeast;
                type Storage = <$base as $crate::sync::atomic::HasAtomicLeast>::Storage;

                fn into_least(self) -> <Self::AtomicLeast as $crate::sync::atomic::Atomic>::Base {
                    <$base>::into_least(self as $base)
                }

                unsafe fn from_least(
                    least: <Self::AtomicLeast as $crate::sync::atomic::Atomic>::Base,
                ) -> Self {
                    core::mem::transmute(<$base>::from_least(least))
                }
            }
        };
    }

    impl_has_atomic_for_enum!(char, u32, target_has_atomic = "32");

    pub struct AtomicFlag(AtomicLeastCell<bool>);

    impl AtomicFlag {
        pub const fn new() -> Self {
            Self(AtomicLeastCell::new(false))
        }

        pub const fn new_with_val(val: bool) -> Self {
            Self(AtomicLeastCell::new(val))
        }

        pub fn test_and_set(&self, ord: Ordering) -> bool {
            self.0.swap(true, ord)
        }

        pub fn clear(&self, ord: Ordering) {
            self.0.store(false, ord);
        }

        pub fn set(&self, ord: Ordering) {
            self.0.store(true, ord);
        }
    }
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
            while (self.flag.fetch_or(ONCE_LOCKED, atomic::Ordering::Acquire) & ONCE_LOCKED) != 0 {
                unsafe {
                    asm!("pause");
                }
            }
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
        if (self.flag.load(atomic::Ordering::Acquire) & ONCE_INIT) != 0 {
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
        if (self.flag.load(atomic::Ordering::Acquire) & ONCE_INIT) != 0 {
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
        unsafe { &*(self.cell.get().cast::<T>()) }
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
        Ok(unsafe { &*(self.cell.get().cast::<T>()) })
    }
}
