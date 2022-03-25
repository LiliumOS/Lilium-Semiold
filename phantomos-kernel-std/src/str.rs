pub use core::str::*;
use core::{borrow::Borrow, marker::PhantomData, ops::Deref, ptr::NonNull};

///
/// An abi safe &str
#[repr(C)]
#[allow(clippy::module_name_repetitions)] // TODO: Determine if this should or should not be changed
#[derive(Clone, Copy)]
pub struct StringView<'a> {
    begin: NonNull<u8>,
    end: NonNull<u8>,
    phantom: PhantomData<&'a str>,
}

unsafe impl<'a> Send for StringView<'a> {}
unsafe impl<'a> Sync for StringView<'a> {}

impl<'a> From<&'a str> for StringView<'a> {
    fn from(v: &'a str) -> Self {
        Self::new(v)
    }
}

impl Deref for StringView<'_> {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.begin.as_ptr(),
                (self.end.as_ptr() as usize) - (self.begin.as_ptr() as usize), // This is really annoying that have to do this
            ))
        }
    }
}

impl<'a> AsRef<str> for StringView<'a> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl<'a> AsRef<[u8]> for StringView<'a> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<str> for StringView<'_> {
    fn borrow(&self) -> &str {
        self
    }
}

impl core::fmt::Debug for StringView<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <str as core::fmt::Debug>::fmt(self, f)
    }
}

impl core::fmt::Display for StringView<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <str as core::fmt::Display>::fmt(self, f)
    }
}

impl core::hash::Hash for StringView<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        <str as core::hash::Hash>::hash(self, state);
    }
}

impl PartialEq for StringView<'_> {
    fn eq(&self, other: &Self) -> bool {
        <str as PartialEq>::eq(self, other)
    }
}

impl PartialEq<str> for StringView<'_> {
    fn eq(&self, other: &str) -> bool {
        <str as PartialEq>::eq(self, other)
    }
}

impl PartialEq<&str> for StringView<'_> {
    fn eq(&self, other: &&str) -> bool {
        <str as PartialEq>::eq(self, other)
    }
}

impl PartialEq<&mut str> for StringView<'_> {
    fn eq(&self, other: &&mut str) -> bool {
        <str as PartialEq>::eq(self, other)
    }
}

impl Eq for StringView<'_> {}

impl<'a> StringView<'a> {
    /// Returns an empty [`StringView`]
    #[must_use]
    pub const fn empty() -> Self {
        StringView {
            begin: NonNull::dangling(),
            end: NonNull::dangling(),
            phantom: PhantomData,
        }
    }

    /// Returns a view over the string referred to by `v`
    #[must_use]
    pub fn new(v: &'a str) -> Self {
        let bytes = v.as_bytes();

        let begin = bytes.as_ptr();
        let end = unsafe { begin.add(bytes.len()) };

        unsafe { Self::from_raw_parts(begin, end) }
    }

    /// Obtains a [`StringView`] over the contiguous range `[begin,end)`
    ///
    /// ## Safety
    /// The behaviour is undefined if any of the following constraints are violated:
    /// * Neither begin nor end may be null pointers or deallocated pointers
    /// * For 'a, the range [begin,end) must be a range that is valid for reads
    /// * For 'a, the range [begin,end) must not be modified from any other
    /// * The text in the range [begin,end) must be valid UTF-8
    #[must_use]
    pub const unsafe fn from_raw_parts(begin: *const u8, end: *const u8) -> Self {
        Self {
            begin: NonNull::new_unchecked(begin as *mut u8),
            end: NonNull::new_unchecked(end as *mut u8),
            phantom: PhantomData,
        }
    }

    /// Determines the length of the string view
    #[must_use]
    #[allow(clippy::cast_sign_loss)] // offset_from can never be negative
    pub fn len(&self) -> usize {
        unsafe { self.end.as_ptr().offset_from(self.begin.as_ptr()) as usize }
    }

    /// Checks if this string view is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.begin == self.end
    }

    ///
    /// Converts an owned [`StringView`] into  &[`str`] with the same lifetime
    #[must_use]
    pub fn into_str(self) -> &'a str {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.begin.as_ptr(),
                (self.end.as_ptr() as usize) - (self.begin.as_ptr() as usize), // This is really annoying that have to do this
            ))
        }
    }
}

#[doc(hidden)]
pub use str as __rust_str;

#[doc(hidden)]
pub use core::ptr::addr_of as __addr_of;

///
/// Constructs a [`StringView`] in a constant context.
/// This is equivalent to `StringView::new(str)`, except it is valid in a const initializer
///
/// must be called with a string literal or a constant expression of type `&'static str`
/// ## Examples
/// ```
///# use xlang_abi::string::StringView;
///# use xlang_abi::const_sv;
/// const HELLO_WORLD: StringView = const_sv!("Hello World");
/// assert_eq!(HELLO_WORLD,StringView::new("Hello World"));
/// ```
///
#[macro_export]
macro_rules! const_sv {
    ($str:expr) => {{
        const __RET: $crate::str::StringView = {
            #[repr(C)]
            union AsArray<'a, T> {
                reff: &'a T,
                arr: &'a [T; 1],
            }

            let st: &'static $crate::str::__rust_str = $str;
            let slice = st.as_bytes();
            let begin = slice.as_ptr();
            let end = if let [.., reff] = slice {
                let [_, end @ ..] = unsafe { AsArray { reff }.arr };
                end.as_ptr()
            } else {
                slice.as_ptr()
            };

            unsafe { $crate::str::StringView::from_raw_parts(begin, end) }
        };
        __RET
    }};
}
