#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::bits::*;

use std::mem;
use std::ptr::read_unaligned;

// Some primes between 2^63 and 2^64 for various uses.
const K0: u64 = 0xc3a5_c85c_97cb_3127;
const K1: u64 = 0xb492_b66f_be98_f273;
const K2: u64 = 0x9ae1_6a3b_2f90_404f;
const KUL: u32 = 0xe654_6b64;

const MUKUL: u64 = 0x9ddf_ea08_eb38_2d69;

// Magic numbers for 32-bit hashing.  Copied from Murmur3.
const C1: u32 = 0xcc9e_2d51;
const C2: u32 = 0x1b87_3593;

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
fn u128_low64(x: &U128) -> u64 {
    x.0
}

#[inline]
#[must_use]
fn u128_high64(x: &U128) -> u64 {
    x.1
}

#[inline]
#[must_use]
fn fmix(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

#[inline]
#[must_use]
fn fetch32(data: &[u8]) -> u32 {
    let p = unsafe { read_unaligned(data.as_ptr().cast::<u32>()) };
    if cfg!(not(target_endian = "little")) {
        return p.swap_bytes();
    }
    p
}

#[inline]
#[must_use]
fn rotate32(val: u32, shift: u32) -> u32 {
    if shift == 0 {
        return val;
    }

    (val >> shift) | (val << (32 - shift))
}

#[inline]
#[must_use]
fn mur(mut a: u32, mut h: u32) -> u32 {
    a = a.wrapping_mul(C1);
    a = rotate32(a, 17);
    a = a.wrapping_mul(C2);
    h ^= a;
    h = rotate32(h, 19);
    h.wrapping_mul(5).wrapping_add(0xe654_6b64)
}

#[inline]
#[must_use]
fn hash32_len_5_to_12(s: &[u8], len: usize) -> u32 {
    let mut a = len as u32;
    let mut b = len as u32 * 5;
    let mut c: u32 = 9;
    let d: u32 = b;
    a += fetch32(&s[0..]);
    b += fetch32(&s[len - 4..]);
    c += fetch32(&s[((len >> 1) & 4)..]);
    fmix(mur(c, mur(b, mur(a, d))))
}

#[inline]
#[must_use]
fn hash32_len_13_to_24(s: &[u8], len: usize) -> u32 {
    let a = fetch32(&s[(len >> 1 as u64) - 4..]);
    let b = fetch32(&s[4..]);
    let c = fetch32(&s[len - 8..]);
    let d = fetch32(&s[len >> 1..]);
    let e = fetch32(&s[0..]);
    let f = fetch32(&s[len - 4..]);
    let h = len as u32;

    fmix(mur(f, mur(e, mur(d, mur(c, mur(b, mur(a, h)))))))
}

#[inline]
#[must_use]
fn hash32_len_0_to_4(s: &[u8], len: usize) -> u32 {
    let mut b: u32 = 0;
    let mut c: u32 = 9;
    for i in 0..len {
        let v: u8 = s[i];
        b = b.wrapping_mul(C1) + v as u32;
        c ^= b;
    }
    fmix(mur(b, mur(len as u32, c)))
}

#[must_use]
pub fn city_hash32(mut s: &[u8]) -> u32 {
    let len = s.len();
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
    let mut g = (len as u32).wrapping_mul(C1);
    let mut f = g;

    let a0: u32 = rotate32(fetch32(&s[len - 4..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a1: u32 = rotate32(fetch32(&s[len - 8..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a2: u32 = rotate32(fetch32(&s[len - 16..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a3: u32 = rotate32(fetch32(&s[len - 12..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a4: u32 = rotate32(fetch32(&s[len - 20..]).wrapping_mul(C1), 17).wrapping_mul(C2);

    h ^= a0;
    h = rotate32(h, 19);
    h = h.wrapping_mul(5).wrapping_add(KUL);
    h ^= a2;
    h = rotate32(h, 19);
    h = h.wrapping_mul(5).wrapping_add(KUL);

    g ^= a1;
    g = rotate32(g, 19);
    g = g.wrapping_mul(5).wrapping_add(KUL);
    g ^= a3;
    g = rotate32(g, 19);
    g = g.wrapping_mul(5).wrapping_add(KUL);

    f ^= a4;
    f = rotate32(f, 19);
    f = f.wrapping_mul(5).wrapping_add(KUL);

    let mut iters = ((len - 1) / 20) as u64;
    while iters > 0 {
        let a0 = rotate32(fetch32(s).wrapping_mul(C1), 17).wrapping_mul(C2);
        let a1 = fetch32(&s[4..]);
        let a2 = rotate32(fetch32(&s[8..]).wrapping_mul(C1), 17).wrapping_mul(C2);
        let a3 = rotate32(fetch32(&s[12..]).wrapping_mul(C1), 17).wrapping_mul(C2);
        let a4 = fetch32(&s[16..]);

        h ^= a0;
        h = rotate32(h, 18);
        h = h.wrapping_mul(5).wrapping_add(KUL);

        f += a1;
        f = rotate32(f, 19);
        f = f.wrapping_mul(C1);

        g += a2;
        g = rotate32(g, 18);
        g = (g * 5).wrapping_add(KUL);

        h ^= a3 + a1;
        h = rotate32(h, 19);
        h = (h * 5).wrapping_add(KUL);

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

    g = rotate32(g, 11) * C1;
    g = rotate32(g, 17) * C1;

    f = rotate32(f, 11) * C1;
    f = rotate32(f, 17) * C1;

    h = rotate32(h + g, 19);
    h = h * 5 + 0xe654_6b64;
    h = rotate32(h, 17) * C1;
    h = rotate32(h + f, 19);
    h = h * 5 + 0xe654_6b64;
    h = rotate32(h, 17) * C1;

    h
}

#[inline]
#[must_use]
fn fetch64(data: &[u8]) -> u64 {
    let p = unsafe { read_unaligned(data.as_ptr().cast::<u64>()) };
    if cfg!(not(target_endian = "little")) {
        return p.swap_bytes();
    }
    p
}

#[inline]
#[must_use]
fn shift_mix(val: u64) -> u64 {
    val ^ (val >> 47)
}

#[inline]
#[must_use]
fn rotate64(val: u64, shift: u64) -> u64 {
    if shift == 0 {
        return val;
    }

    (val >> shift) | (val << (64 - shift))
}

#[inline]
#[must_use]
fn hash128_to_64(x: &U128) -> u64 {
    let mut a: u64 = (u128_low64(x) ^ u128_high64(x)).wrapping_mul(MUKUL);
    a ^= a >> 47;
    let mut b: u64 = (u128_high64(x) ^ a).wrapping_mul(MUKUL);
    b ^= b >> 46;
    b *= MUKUL;

    b
}

#[inline]
#[must_use]
fn hash_len_16(u: u64, v: u64, mul: u64) -> u64 {
    let mut a: u64 = (u ^ v).wrapping_mul(mul);
    a ^= a >> 47;
    let mut b = (v ^ a).wrapping_mul(mul);
    b ^= b >> 47;
    b = b.wrapping_mul(mul);

    b
}

#[inline]
#[must_use]
fn hash_len_16_2(u: u64, v: u64) -> u64 {
    hash128_to_64(&U128(u, v))
}

#[inline]
#[must_use]
fn hash_len_0_to_16(s: &[u8], len: usize) -> u64 {
    if len >= 8 {
        let mul: u64 = K2 + len as u64 * 2;
        let a: u64 = fetch64(&s[0..]).wrapping_add(K2);
        let b: u64 = fetch64(&s[len - 8..]);
        let c: u64 = rotate64(b, 37).wrapping_mul(mul).wrapping_add(a);
        let d: u64 = rotate64(a, 25).wrapping_add(b).wrapping_mul(mul);

        return hash_len_16(c, d, mul);
    }

    if len >= 4 {
        let mul = K2.wrapping_add(len as u64).wrapping_mul(2);
        let a: u64 = u64::from(fetch32(s));
        return hash_len_16(
            len as u64 + (a << 3),
            u64::from(fetch32(&s[len - 4..])),
            mul,
        );
    }

    if len > 0 {
        let a = s[0];
        let b: u8 = s[len >> 1];
        let c: u8 = s[len - 1];
        let y = u32::from(a) + (u32::from(b) << 8);
        let z: u32 = (len as u32) + (u32::from(c) << 2);
        return shift_mix(u64::from(y).wrapping_mul(K2) ^ u64::from(z).wrapping_mul(K0))
            .wrapping_mul(K2);
    }

    K2
}

#[inline]
#[must_use]
fn hash_len_17_to_32(s: &[u8], len: usize) -> u64 {
    let mut _mul: u64 = K2 + (len as u64).wrapping_mul(2);
    let mut _a: u64 = fetch64(&s[0..]).wrapping_mul(K1);
    let mut _b: u64 = fetch64(&s[8..]);
    let mut _c: u64 = fetch64(&s[len - 8..]).wrapping_mul(_mul);
    let mut _d: u64 = fetch64(&s[len - 16..]).wrapping_mul(K2);

    return hash_len_16(
        rotate64(_a.wrapping_add(_b), 43)
            .wrapping_add(rotate64(_c, 30))
            .wrapping_add(_d),
        _a.wrapping_add(rotate64(_b.wrapping_add(K2), 18).wrapping_add(_c)),
        _mul,
    );
}

#[inline]
#[must_use]
fn hash_len_33_to_64(s: &[u8], len: usize) -> u64 {
    let _mul: u64 = K2 + (len as u64).wrapping_mul(2);
    let mut _a: u64 = fetch64(s).wrapping_mul(K2);
    let mut _b: u64 = fetch64(&s[8..]);
    let _c: u64 = fetch64(&s[len - 24..]);
    let _d: u64 = fetch64(&s[len - 32..]);
    let _e: u64 = fetch64(&s[16..]).wrapping_mul(K2);
    let _f: u64 = fetch64(&s[24..]).wrapping_mul(9);
    let _g: u64 = fetch64(&s[len - 8..]);
    let _h: u64 = fetch64(&s[len - 16..]).wrapping_mul(_mul);
    let _u: u64 = rotate64(_a + _g, 43) + (rotate64(_b, 30).wrapping_add(_c)) * 9;
    let _v: u64 = (_a.wrapping_add(_g) ^ _d).wrapping_add(_f).wrapping_add(1);
    let _w: u64 = bswap64(_u.wrapping_add(_v).wrapping_mul(_mul)).wrapping_add(_h);
    let _x: u64 = rotate64(_e.wrapping_add(_f), 42).wrapping_add(_c);
    let _y: u64 = bswap64(_v.wrapping_add(_w).wrapping_mul(_mul)).wrapping_add(_h);
    let _z: u64 = _e.wrapping_add(_f).wrapping_add(_c);

    _a = bswap64(_x.wrapping_add(_z).wrapping_mul(_mul).wrapping_add(_y)).wrapping_add(_b);
    _b = shift_mix(
        _z.wrapping_add(_a)
            .wrapping_mul(_mul)
            .wrapping_add(_d)
            .wrapping_add(_h),
    )
    .wrapping_mul(_mul);

    return _b.wrapping_add(_x);
}

// Return a 16-byte hash for 48 bytes.  Quick and dirty.
// Callers do best to use "random-looking" values for a and b.
#[inline]
#[must_use]
fn weak_hash_len32_with_seeds(w: u64, x: u64, y: u64, z: u64, mut a: u64, mut b: u64) -> U128 {
    a = a.wrapping_add(w);
    b = rotate64(b.wrapping_add(a).wrapping_add(z), 21);
    let c: u64 = a;
    a = a.wrapping_add(x);
    a = a.wrapping_add(y);
    b = b.wrapping_add(rotate64(a, 44));
    return U128(a.wrapping_add(z), b.wrapping_add(c));
}

// Return a 16-byte hash for s[0] ... s[31], a, and b.  Quick and dirty.
#[inline]
#[must_use]
fn weak_hash_len32_with_seeds_bytes(s: &[u8], a: u64, b: u64) -> U128 {
    return weak_hash_len32_with_seeds(
        fetch64(&s[0..]),
        fetch64(&s[8..]),
        fetch64(&s[16..]),
        fetch64(&s[24..]),
        a,
        b,
    );
}

pub fn city_hash64(mut s: &[u8]) -> u64 {
    let len = s.len();
    if len <= 32 {
        if len <= 16 {
            return hash_len_0_to_16(&s[0..], len);
        }
        return hash_len_17_to_32(&s[0..], len);
    } else if len <= 64 {
        return hash_len_33_to_64(&s[0..], len);
    }

    // For strings over 64 bytes we hash the end first, and then as we
    // loop we keep 56 bytes of state: v, w, x, y, and z.
    let mut _x: u64 = fetch64(&s[len - 40..]);
    let mut _y: u64 = fetch64(&s[len - 16..]).wrapping_add(fetch64(&s[len - 56..]));
    let mut _z: u64 = hash_len_16_2(
        fetch64(&s[len - 48..]).wrapping_add(len as u64),
        fetch64(&s[len - 24..]),
    );
    let mut _v: U128 = weak_hash_len32_with_seeds_bytes(&s[len - 64..], len as u64, _z);
    let mut _w: U128 = weak_hash_len32_with_seeds_bytes(&s[len - 32..], _y.wrapping_add(K1), _x);
    _x = _x.wrapping_mul(K1) + fetch64(&s[0..]);

    // Decrease len to the nearest multiple of 64, and operate on 64-byte chunks.
    let mut _len: usize = len.wrapping_sub(1) & !(63 as usize);
    loop {
        _x = rotate64(
            _x.wrapping_add(_y)
                .wrapping_add(_v.0)
                .wrapping_add(fetch64(&s[8..])),
            37,
        )
        .wrapping_mul(K1);
        _y = rotate64(_y.wrapping_add(_v.1).wrapping_add(fetch64(&s[48..])), 42).wrapping_mul(K1);
        _x ^= _w.1;
        _y = _y.wrapping_add(_v.0).wrapping_add(fetch64(&s[40..]));
        _z = rotate64(_z.wrapping_add(_w.0), 33).wrapping_mul(K1);
        _v =
            weak_hash_len32_with_seeds_bytes(&s[0..], _v.1.wrapping_mul(K1), _x.wrapping_add(_w.1));
        _w = weak_hash_len32_with_seeds_bytes(
            &s[32..],
            _z.wrapping_add(_w.1),
            _y.wrapping_add(fetch64(&s[16..])),
        );

        mem::swap(&mut _z, &mut _x);
        s = &s[64..];
        _len = _len.wrapping_sub(64);
        if _len == 0 {
            break;
        }
    }

    return hash_len_16_2(
        hash_len_16_2(_v.0, _w.0)
            .wrapping_add(_y)
            .wrapping_mul(K1)
            .wrapping_add(_z),
        hash_len_16_2(_v.1, _w.1).wrapping_add(_x),
    );
}

/*fn city_hash64_with_seeds(s: &[u8], len: usize, seed: u64) -> u64 {
    return city_hash64_with_seeds_bytes(s, len, K2, seed);
}

fn city_hash64_with_seeds_bytes(s: &[u8], len: usize, seed0: u64, seed1: u64) -> u64 {
    return hash_len_16_2(city_hash64(s, len).wrapping_sub(seed0), seed1);
}*/

#[inline]
#[must_use]
fn city_murmur(mut s: &[u8], len: usize, seed: U128) -> U128 {
    let mut _a = u128_low64(&seed);
    let mut _b: u64 = u128_high64(&seed);
    let mut _c: u64 = 0;
    let mut _d: u64 = 0;

    let mut _l: i64 = len.wrapping_sub(16) as i64;
    if _l <= 0 {
        _a = shift_mix(_a.wrapping_mul(K1)).wrapping_mul(K1);
        _c = _b.wrapping_mul(K1) + hash_len_0_to_16(&s[0..], len);
        if len >= 8 {
            _d = shift_mix(_a.wrapping_add(fetch64(&s[0..])));
        } else {
            _d = shift_mix(_a.wrapping_add(_c));
        }
    } else {
        // len > 16
        _c = hash_len_16_2(fetch64(&s[len - 8..]).wrapping_sub(K1), _a);
        _d = hash_len_16_2(
            _b.wrapping_add(len as u64),
            _c.wrapping_add(fetch64(&s[len - 16..])),
        );
        _a = _a.wrapping_add(_d);
        loop {
            _a ^= shift_mix(fetch64(&s[0..]).wrapping_mul(K1)).wrapping_mul(K1);
            _a = _a.wrapping_mul(K1);
            _b ^= _a;
            _c ^= shift_mix(fetch64(&s[8..]).wrapping_mul(K1)).wrapping_mul(K1);
            _c = _c.wrapping_mul(K1);
            _d ^= _c;
            s = &s[16..];
            _l = _l.wrapping_sub(16);

            if _l <= 0 {
                break;
            }
        }
    }

    _a = hash_len_16_2(_a, _c);
    _b = hash_len_16_2(_d, _b);

    return U128(_a ^ _b, hash_len_16_2(_b, _a));
}

#[inline]
#[must_use]
fn city_hash128_with_seed(mut s: &[u8], mut len: usize, seed: U128) -> U128 {
    if len < 128 {
        return city_murmur(s, len, seed);
    }

    let mut _v: U128 = U128(0, 0);
    let mut _w: U128 = U128(0, 0);

    let mut _x: u64 = u128_low64(&seed);
    let mut _y: u64 = u128_high64(&seed);
    let mut _z: u64 = len.wrapping_mul(K1 as usize) as u64;

    _v.0 = rotate64(_y ^ K1, 49).wrapping_mul(K1) + fetch64(&s[0..]);
    _v.1 = rotate64(_v.0, 42).wrapping_mul(K1) + fetch64(&s[8..]);
    _w.0 = rotate64(_y.wrapping_add(_z), 35)
        .wrapping_mul(K1)
        .wrapping_add(_x);
    _w.1 = rotate64(_x.wrapping_add(fetch64(&s[88..])), 53).wrapping_mul(K1);

    loop {
        _x = rotate64(
            _x.wrapping_add(_y)
                .wrapping_add(_v.0)
                .wrapping_add(fetch64(&s[8..])),
            37,
        )
        .wrapping_mul(K1);
        _y = rotate64(_y.wrapping_add(_v.1).wrapping_add(fetch64(&s[48..])), 42).wrapping_mul(K1);
        _x ^= _w.1;
        _y = _y.wrapping_add(_v.0).wrapping_add(fetch64(&s[40..]));
        _z = rotate64(_z.wrapping_add(_w.0), 33).wrapping_mul(K1);
        _v =
            weak_hash_len32_with_seeds_bytes(&s[8..], _v.1.wrapping_mul(K1), _x.wrapping_add(_w.0));
        _w = weak_hash_len32_with_seeds_bytes(
            &s[32..],
            _z.wrapping_add(_w.1),
            _y.wrapping_add(fetch64(&s[16..])),
        );

        mem::swap(&mut _z, &mut _x);
        s = &s[64..];
        _x = rotate64(_x + _y + _v.0 + fetch64(&s[8..]), 37).wrapping_mul(K1);
        _y = rotate64(_y + _v.1 + fetch64(&s[48..]), 42).wrapping_mul(K1);

        _x ^= _w.1;
        _y = _y.wrapping_add(_v.0).wrapping_add(fetch64(&s[40..]));
        _z = rotate64(_z.wrapping_add(_w.0), 33).wrapping_mul(K1);

        _v =
            weak_hash_len32_with_seeds_bytes(&s[0..], _v.1.wrapping_mul(K1), _x.wrapping_add(_w.0));
        _w = weak_hash_len32_with_seeds_bytes(
            &s[32..],
            _z.wrapping_add(_w.1),
            _y.wrapping_add(fetch64(&s[16..])),
        );

        mem::swap(&mut _z, &mut _x);
        s = &s[64..];
        len -= 128;

        if len < 128 {
            break;
        }
    }

    _x = _x.wrapping_add(rotate64(_v.0.wrapping_add(_z), 49).wrapping_mul(K0));
    _y = _y.wrapping_mul(K0).wrapping_add(rotate64(_w.1, 37));
    _z = _z.wrapping_mul(K0).wrapping_add(rotate64(_w.1, 27));

    _w.0 = _w.0.wrapping_mul(9);
    _v.0 = _v.0.wrapping_mul(K0);

    // If 0 < len < 128, hash up to 4 chunks of 32 bytes each from the end of s.
    let mut tail_done: usize = 0;
    while tail_done < len {
        tail_done = tail_done.wrapping_add(32);
        _y = rotate64(_x.wrapping_add(_y), 42)
            .wrapping_mul(K0)
            .wrapping_add(_v.1);
        _w.0 = _w.0.wrapping_add(fetch64(&s[len - tail_done..]));
        _x = _x.wrapping_mul(K0).wrapping_add(_w.0);
        _z = _z.wrapping_add(fetch64(&s[len - tail_done + 16..]));
        _w.1 = _w.1.wrapping_add(_v.0);
        _v = weak_hash_len32_with_seeds_bytes(&s[len - tail_done..], _v.0.wrapping_add(_z), _v.1);
        _v.0 = _v.0.wrapping_mul(K0);
    }

    // At this point our 56 bytes of state should contain more than
    // enough information for a strong 128-bit hash.  We use two
    // different 56-byte-to-8-byte hashes to get a 16-byte final result.
    _x = hash_len_16_2(_x, _v.0);
    _y = hash_len_16_2(_y.wrapping_add(_z), _w.0);

    return U128(
        hash_len_16_2(_x.wrapping_add(_v.1), _w.1).wrapping_add(_y),
        hash_len_16_2(_x.wrapping_add(_w.1), _y.wrapping_add(_v.1)),
    );
}

pub fn city_hash128(s: &[u8], len: usize) -> U128 {
    if len >= 16 {
        return city_hash128_with_seed(
            &s[16..],
            len - 16,
            U128(fetch64(&s[0..]), fetch64(&s[8..]).wrapping_add(K0)),
        );
    }

    return city_hash128_with_seed(&s[0..], len, U128(K0, K1));
}
