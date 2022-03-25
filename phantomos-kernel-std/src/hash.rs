use core::{hash::Hasher, num::Wrapping};

use alloc::slice;

use crate::sync::OnceCell;

///
/// A hasher with a consistent ABI and hash algorithm.
/// the [`XLangHasher`] hashes bytes using the `FNV-1a` 64-bit hash
#[repr(C)]
#[derive(Debug)]
pub struct XLangHasher(Wrapping<u64>);

const PRIME: u64 = 1_099_511_628_211;

pub fn xlang_hash_bytes(x: &[u8]) -> u64 {
    // When running tests, we don't want to bring in lazy_static to xlang_abi, but since hash seeds will be randomized at runtime, we want to choose a random seed when running miri
    static SEED: OnceCell<u64> = OnceCell::new();

    let mut hash = *SEED.get_or_init(|| &SEED as *const _ as usize as u64);

    for b in x {
        hash ^= (*b) as u64;
        hash = hash.wrapping_mul(PRIME);
    }

    hash
}

const XLANG_HASH_SEED: u8 = 1;

impl XLangHasher {
    /// Returns a new [`XLangHasher`]
    ///
    /// Each [`XLangHasher`] is initialized with the same value, for consistency.
    #[must_use]
    pub const fn new() -> Self {
        Self(Wrapping(14_695_981_039_346_656_037))
    }

    /// Returns a new [`XLangHasher`] with a given seed
    ///
    ///
    #[must_use]
    pub const fn from_seed(seed: u64) -> Self {
        Self(Wrapping(seed))
    }
}

impl Hasher for XLangHasher {
    fn finish(&self) -> u64 {
        self.0 .0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 ^= Wrapping(xlang_hash_bytes(bytes));
        self.0 *= Wrapping(PRIME);
    }

    fn write_u8(&mut self, i: u8) {
        self.write(&[i]);
    }

    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_ne_bytes());
    }

    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_ne_bytes());
    }

    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_ne_bytes());
    }

    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_ne_bytes());
    }

    fn write_usize(&mut self, i: usize) {
        self.write(&i.to_ne_bytes());
    }

    #[allow(clippy::cast_sign_loss)]
    fn write_i8(&mut self, i: i8) {
        self.write_u8(i as u8);
    }

    fn write_i16(&mut self, i: i16) {
        self.write(&i.to_ne_bytes());
    }

    fn write_i32(&mut self, i: i32) {
        self.write(&i.to_ne_bytes());
    }

    fn write_i64(&mut self, i: i64) {
        self.write(&i.to_ne_bytes());
    }

    fn write_i128(&mut self, i: i128) {
        self.write(&i.to_ne_bytes());
    }

    fn write_isize(&mut self, i: isize) {
        self.write(&i.to_ne_bytes());
    }
}

impl Default for XLangHasher {
    fn default() -> Self {
        Self::from_seed(xlang_hash_bytes(slice::from_ref(&XLANG_HASH_SEED)))
    }
}
