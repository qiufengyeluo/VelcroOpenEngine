#![deny(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::ops::BitXor;

//use num_traits::ToBytes;
use velcro_utils::{city_hash32, city_hash64};

/// A builder for default velcro hashers.
pub type VBuildHasher = BuildHasherDefault<VHasher>;


/// A `HashMap` using a default velcro hasher.
///
/// Use `VHashMap::default()`, not `new()` to create a new `VHashMap`.
/// To create with a reserved capacity, use `VHashMap::with_capacity_and_hasher(num, Default::default())`.
pub type VHashMap<K, V> = HashMap<K, V, VBuildHasher>;

/// A `HashSet` using a default velcro hasher.
///
/// Note: Use `VHashSet::default()`, not `new()` to create a new `VHashSet`.
/// To create with a reserved capacity, use `VHashSet::with_capacity_and_hasher(num, Default::default())`.
pub type VHashSet<V> = HashSet<V, VBuildHasher>;


const ROTATE: u32 = 5;

#[inline]
fn write32(bytes: &[u8]) -> u32 {
    return city_hash32(bytes);
}

#[inline]
fn write64(bytes: &[u8]) -> u64 {
    return city_hash64(bytes);
}

#[inline]
#[cfg(target_pointer_width = "32")]
fn write(bytes: &[u8]) -> usize {
    write32(bytes) as usize
}

#[inline]
#[cfg(target_pointer_width = "64")]
fn write(bytes: &[u8]) -> usize {
    write64(bytes) as usize
}

/// This hashing algorithm was extracted from the Rustc compiler.
/// This is an implementation based on google city hash algorithm
#[derive(Debug, Clone)]
pub struct VHasher {
    hash: usize,
}

impl Default for VHasher {
    #[inline]
    fn default() -> VHasher {
        VHasher { hash: 0 }
    }
}

impl Hasher for VHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = write(bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash = write32(&[i]) as usize;
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash = write32(&i.to_ne_bytes()) as usize;
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = write32(&i.to_ne_bytes()) as usize;
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn write_u64(&mut self, i: u64) {
        let a = write32(&(i as u32).to_ne_bytes());
        let b = write32(&((i >> 32) as u32).to_ne_bytes());
        self.hash = self.hash.rotate_left(ROTATE).bitxor(b).wrapping_mul(a) as usize;
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn write_u64(&mut self, i: u64) {
        self.hash = write64(&i.to_be_bytes()) as usize;
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash = write64(&i.to_ne_bytes()) as usize;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }
}


/// This hashing algorithm was extracted from the Rustc compiler.
/// This is an implementation based on google city hash algorithm
#[derive(Debug, Clone)]
pub struct VHasher64 {
    hash: u64,
}

impl Default for VHasher64 {
    #[inline]
    fn default() -> VHasher64 {
        VHasher64 { hash: 0 }
    }
}

impl  Hasher for VHasher64 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = write64( bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash = write64(&u64::from(i).to_ne_bytes());
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash = write64(&u64::from(i).to_ne_bytes());
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = write64(&u64::from(i).to_ne_bytes());
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash = write64(&i.to_ne_bytes());
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash = write64(&(i as u64).to_ne_bytes());
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

/// This hashing algorithm was extracted from the Rustc compiler.
/// This is an implementation based on google city hash algorithm
#[derive(Debug, Clone)]
pub struct VHasher32 {
    hash: u32,
}

impl Default for VHasher32 {
    #[inline]
    fn default() -> VHasher32 {
        VHasher32 { hash: 0 }
    }
}

impl Hasher for VHasher32 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hash = write32(bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash = write32(&u32::from(i).to_ne_bytes());
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash = write32(&u32::from(i).to_ne_bytes());
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = write32(&i.to_ne_bytes());
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        let a = write32(&(i as u32).to_ne_bytes());
        let b = write32(&((i >> 32) as u32).to_ne_bytes());
        self.hash = self.hash.rotate_left(ROTATE).bitxor(b).wrapping_mul(a);
    }

    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn write_usize(&mut self, i: usize) {
        self.write_u32(i as u32);
    }

    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        u64::from(self.hash)
    }
}

/// A convenience function for when you need a quick 64-bit hash.
#[inline]
pub fn hash64<T: Hash + ?Sized>(v: &T) -> u64 {
    let mut state = VHasher64::default();
    v.hash(&mut state);
    state.finish()
}

/// A convenience function for when you need a quick 32-bit hash.
#[inline]
pub fn hash32<T: Hash + ?Sized>(v: &T) -> u32 {
    let mut state = VHasher32::default();
    v.hash(&mut state);
    state.finish() as u32
}

/// A convenience function for when you need a quick usize hash.
#[inline]
pub fn hash<T: Hash + ?Sized>(v: &T) -> usize {
    let mut state = VHasher::default();
    v.hash(&mut state);
    state.finish() as usize
}