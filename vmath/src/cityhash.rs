#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ptr::read_unaligned;

// Some primes between 2^63 and 2^64 for various uses.
pub const k0:u64 = 0xc3a5_c85c_97cb_3127;
pub const k1:u64 = 0xb492_b66f_be98_f273;
pub const k2:u64 = 0x9ae1_6a3b_2f90_404f;

// Magic numbers for 32-bit hashing.  Copied from Murmur3.
pub const c1:u32 = 0xcc9e_2d51;
pub const c2:u32 = 0x1b87_3593;


#[derive(Debug, Clone, Copy)]
pub struct U128(u64, u64);

// This is the point clippy :p
#[allow(clippy::cast_possible_truncation)]
impl From<u128> for U128 {
    fn from(value: u128) -> Self {
        U128(value as u64, (value >> 64) as u64)
    }
}

impl From<U128> for u128 {
    fn from(value: U128) -> Self {
        u128::from(value.1) << 64 | u128::from(value.0)
    }
}

#[inline]
#[must_use]
pub fn fmix(h:u32) -> u32 {
    h ^= h >> 16;
    h *= 0x85ebca6b;
    h ^= h >> 13;
    h *= 0xc2b2ae35;
    h ^= h >> 16;
    return h;
}


#[inline]
#[must_use]
pub fn fetch32(data: &[u8]) -> u32 {
    let p = unsafe { read_unaligned(data.as_ptr().cast::<u32>())};
    if cfg!(not(target_endian = "little")) {
        return p.swap_bytes();
    }
    p
}

#[inline]
#[must_use]
pub fn fetch64(data: &[u8]) -> u64 {
    let p = unsafe { read_unaligned(data.as_ptr().cast::<u64>())};
    if cfg!(not(target_endian = "little")) {
        return p.swap_bytes();
    }
    p
}

#[inline]
#[must_use]
pub fn shift_mix(val:u64) -> u64 {
    val ^ (val >> 47)
}

#[inline]
#[must_use]
pub fn rotate32(val: u32, shift: u32) -> u32 {
    if (shift == 0) {
        return val;
    }

    ((val >> shift) | (val << (32 - shift)));
}

#[inline]
#[must_use]
pub fn rotate64(val: u64, shift: u64) -> u64 {
    if (shift == 0) {
        return val;
    }

    ((val >> shift) | (val << (64 - shift)));
}

#[inline]
#[must_use]
pub fn mur(a:u32, h:uint32) -> u32 {
    a *= c1;
    a = rotate32(a, 17);
    a *= c2;
    h ^= a;
    h = rotate32(h, 19);
    return h * 5 + 0xe6546b64;
}



#[inline]
#[must_use]
pub fn hash32_len_13_to_24(s: &[u8]) -> u32 {
   let a = fetch32(&data[data.len() >> 1 - 4..]);
   let b = fetch32(&data[4..]);
   let c = fetch32(&data[data.len() - 8..]);
   let d = fetch32(&data[data.len() >> 1..]);
   let e = fetch32(data);
   let f = fetch32(&data[data.len() - 4..]);
   let h = data.len();

   return fmix(mur(f, mur(e, mur(d, mur(c, mur(b, mur(a, h)))))));
}