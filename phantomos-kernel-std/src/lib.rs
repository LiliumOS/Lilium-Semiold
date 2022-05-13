#![no_std]
#![feature(allocator_api, const_maybe_uninit_zeroed)]

extern crate alloc;

pub use alloc::format;

pub mod cell;
pub mod collection;
pub mod hash;
pub mod io;
pub mod str;
pub mod sync;

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! has_x86_feature {
    ($feature:literal) => {
        extern "C" {
            fn __has_x86_feature(feature: $crate::str::StringView) -> ::core::primitive::bool;
        }

        unsafe { __has_x86_feature($crate::str::StringView::new($feature)) }
    };
}
