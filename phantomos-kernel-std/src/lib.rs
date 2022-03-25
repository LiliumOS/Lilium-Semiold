#![no_std]
#![feature(allocator_api)]

extern crate alloc;

pub mod collection;
pub mod hash;
pub mod io;
pub mod str;
pub mod sync;

#[cfg(target_arch = "x86_64")]
macro_rules! has_x86_feature {
    ($feature:literal) => {
        extern "C" {
            fn __has_x86_feature(feature: $crate::str::StringView) -> ::core::primitive::bool;
        }

        unsafe { __has_x86_feature($crate::str::StringView::new($feature)) }
    };
}
