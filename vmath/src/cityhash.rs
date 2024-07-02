#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::mem;
use std::ptr::read_unaligned;

// Some primes between 2^63 and 2^64 for various uses.
pub const k0:u64 = 0xc3a5_c85c_97cb_3127;
pub const k1:u64 = 0xb492_b66f_be98_f273;
pub const k2:u64 = 0x9ae1_6a3b_2f90_404f;
pub const kul:u32 = 0xe654_6b64;

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
pub fn fmix(mut h:u32) -> u32 {
    h ^= h >> 16;
    h  = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h  = h.wrapping_mul(0xc2b2ae35);
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
    if shift == 0 {
        return val;
    }

    return (val >> shift) | (val << (32 - shift));
}

#[inline]
#[must_use]
pub fn rotate64(val: u64, shift: u64) -> u64 {
    if shift == 0 {
        return val;
    }

    return ((val >> shift) | (val << (64 - shift)));
}

#[inline]
#[must_use]
pub fn mur(mut a: u32,mut h: u32) -> u32 {
    a = a.wrapping_mul(c1);
    a = rotate32(a, 17);
    a = a.wrapping_mul(c2);
    h ^= a;
    h = rotate32(h, 19);
    return h.wrapping_mul(5).wrapping_add(0xe6546b64);
}

#[inline]
#[must_use]
pub fn hash32_len_5_to_12(s: &[u8], len:usize) -> u32 {
    let mut a = len as u32;
    let mut b = len as u32 * 5;
    let mut c: u32 = 9;
    let d: u32 = b;
    a += fetch32(&s[0..]);
    b += fetch32(&s[len-4..]);
    c += fetch32(&s[((len >> 1) & 4)..]);
    return fmix(mur(c, mur(b, mur(a, d))));
}

#[inline]
#[must_use]
pub fn hash32_len_13_to_24(s: &[u8], len: usize) -> u32 {
   let a = fetch32(&s[(len>>1 as u64) - 4..]);
   let b = fetch32(&s[4..]);
   let c = fetch32(&s[len - 8..]);
   let d = fetch32(&s[len >> 1..]);
   let e = fetch32(&s[0..]);
   let f = fetch32(&s[len - 4..]);
   let h = len as u32;

   return fmix(mur(f, mur(e, mur(d, mur(c, mur(b, mur(a, h)))))));
}

#[inline]
#[must_use]
pub fn hash32_len_0_to_4(s: &[u8], len: usize) -> u32 {
    let mut b: u32 = 0;
    let mut c: u32 = 9;
    for i in 0..len {
        let v: u8 = s[i];
        b = b.wrapping_mul(c1) + v as u32;
        c ^= b;
    }
    return fmix(mur(b, mur(len as u32, c)));
}

#[inline]
#[must_use]
fn bswap32(x: u32) -> u32 {
    return ((x >> 24) & 0xFF) | ((x >> 8) & 0xFF00) |
        ((x << 8) & 0xFF0000) | ((x << 24) & 0xFF000000)
}

pub fn CityHash32(mut s: &[u8], len: usize) -> u32 { 


    if len <= 24 {
        if len <= 12 {
            if len <= 4 {
                return hash32_len_0_to_4(s, len);
            }

            return hash32_len_5_to_12(s, len);
        }
        return hash32_len_13_to_24(s, len);
    }

    // len > 32
    let mut h = len as u32;
    let mut g = (len as u32).wrapping_mul(c1);
    let mut f = g;

    let mut a0: u32 = rotate32(fetch32(&s[len - 4..]).wrapping_mul(c1), 17).wrapping_mul(c2);
    let mut a1: u32 = rotate32(fetch32(&s[len - 8..]).wrapping_mul(c1), 17).wrapping_mul(c2);
    let mut a2: u32 = rotate32(fetch32(&s[len - 16..]).wrapping_mul(c1), 17).wrapping_mul(c2);
    let mut a3: u32 = rotate32(fetch32(&s[len - 12..]).wrapping_mul(c1), 17).wrapping_mul(c2);
    let mut a4: u32 = rotate32(fetch32(&s[len - 20..]).wrapping_mul(c1), 17).wrapping_mul(c2);

    h ^= a0;
    h = rotate32(h, 19);
    h = h.wrapping_mul(5).wrapping_add(kul);
    h ^= a2;
    h = rotate32(h, 19);
    h = h.wrapping_mul(5).wrapping_add(kul);

    g ^= a1;
    g = rotate32(g, 19);
    g = g.wrapping_mul(5).wrapping_add(kul);
    g ^= a3;
    g = rotate32(g, 19);
    g = g.wrapping_mul(5).wrapping_add(kul);

    f ^= a4;
    f = rotate32(f, 19);
    f = f.wrapping_mul(5).wrapping_add(kul);

    let mut iters = ((len - 1) / 20) as u64;
    while iters > 0 {
        let a0 = rotate32(fetch32(&s[..]).wrapping_mul(c1), 17).wrapping_mul(c2);
        let a1 = fetch32(&s[4..]);
        let a2 = rotate32(fetch32(&s[8..]).wrapping_mul(c1), 17).wrapping_mul(c2);
        let a3 = rotate32(fetch32(&s[12..]).wrapping_mul(c1), 17).wrapping_mul(c2);
        let a4 = fetch32(&s[16..]);

        h ^= a0;
        h = rotate32(h, 18);
        h = h.wrapping_mul(5).wrapping_add(kul);

        f += a1;
        f = rotate32(f, 19);
        f = f.wrapping_mul(c1);

        g += a2;
        g = rotate32(g, 18);
        g = (g * 5).wrapping_add(kul);

        h ^= a3 + a1;
        h = rotate32(h, 19);
        h = (h * 5).wrapping_add(kul);

        g ^= a4;
        g = bswap32(g) * 5;
        h += a4 * 5;
        h = bswap32(h);
        f += a0;
        
        //#define PERMUTE3(a, b, c) do { std::swap(a, b); std::swap(a, c); } while (0)
        //等价于 PERMUTE3(f, h, g);
        mem::swap(&mut h, &mut f);  
        mem::swap(&mut g, &mut f);
        s = &s[20..];
        iters -= 1;
    }

    g = rotate32(g, 11) * c1;
    g = rotate32(g, 17) * c1;

    f = rotate32(f, 11) * c1;
    f = rotate32(f, 17) * c1;

    h = rotate32(h + g, 19);
    h = h * 5 + 0xe6546b64;
    h = rotate32(h, 17) * c1;
    h = rotate32(h + f, 19);
    h = h * 5 + 0xe6546b64;
    h = rotate32(h, 17) * c1;

    return h;
}