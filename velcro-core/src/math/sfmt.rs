#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::{mem, sync::{atomic::AtomicI32, Arc, Mutex}};
use crate::math::vsimd::*;

const MEXP: i32 = 19937;
const N: i32    = MEXP / 128 + 1;

const N32:  i32    = N * 4;
const N64:  i32    = N * 2;
const POS1: i32    = 122;
const SL1:  i32    = 18;
const SR1:  i32    = 11;
const SL2:  i32    = 1;
const SR2:  i32    = 1;

const MSK1:    u32  = 0xdfff_ffef;
const MSK2:    u32  = 0xddfe_cb7f;
const MSK3:    u32  = 0xbffa_ffff;
const MSK4:    u32  = 0xbfff_fff6;
const PARITY1: u32  = 0x0000_0001;
const PARITY2: u32  = 0x0000_0000;
const PARITY3: u32  = 0x0000_0000;
const PARITY4: u32  = 0x13c9_e684;

// a parity check vector which certificate the period of 2^{MEXP}
const parity: [u32; 4] = [PARITY1, PARITY2, PARITY3, PARITY4];

#[repr(C)]
union W128T {
    si: Int32Type,
    u: [u32; 4]
}



//assert_eq!(N, MEXP / (mem::size_of::<W128T>() as i32 * 8 ) + 1 , "The smft member array must fit all iterations of the correct 128-bit size.");

pub struct Sfmt {
    sfmt: [W128T; N as usize],
    index: AtomicI32,
    generation_mutex: Arc<Mutex<u32>>
}

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
fn simd_recursion(a: &Int32Type, b: &Int32Type, c: Int32Type, d: Int32Type, mask: Int32Type) -> Int32Type {
    let mut x = *a;

    let mut y = unsafe { _mm_srli_epi32(*b, SR1) }; 
    let mut z: __m128i = unsafe { _mm_srli_si128(c,  SR2) };
    let v = unsafe { _mm_slli_epi32(d, SL1) };

    z = unsafe { xor_i32(z, x) };
    z = unsafe { xor_i32(z, v) };
    x = unsafe { _mm_slli_si128(x, SL2)   };

    y = unsafe { and_i32(y, mask) };
    z = unsafe { xor_i32(z, x) };
    z = unsafe { xor_i32(z, y) };

    return z;
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
fn gen_rand_all(g: &mut Sfmt) {
    let mut r:    Int32Type;
    let mask: Int32Type = unsafe { load_immediate_i32(MSK4 as i32, MSK3 as i32, MSK2 as i32, MSK1 as i32) };

    let mut r1: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 2) as usize].si) };
    let mut r2: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 1) as usize].si) };

    let iloop: usize = (N - POS1) as usize;
    let mut i: usize = 0;
    while i < iloop {
        r = simd_recursion(unsafe { &g.sfmt[i].si }, unsafe { &g.sfmt[i + POS1 as usize].si}, r1, r2, mask);
        unsafe { store_aligned_i128(&mut g.sfmt[i].si , r) };
        r1 = r2;
        r2 = r;

        i += 1;
    }
    i = 0;
    while i < N as usize {
        r = simd_recursion(unsafe {&g.sfmt[i].si}, unsafe{&g.sfmt[i + POS1 as usize - N as usize].si}, r1, r2, mask);
        unsafe { store_aligned_i128(&mut g.sfmt[i].si , r) };
        r1 = r2;
        r2 = r;
        i += 1;
    }
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
fn gen_rand_array(g: &mut Sfmt, array: &mut [W128T], size: usize) {
    let mut r:    Int32Type;
    let mask = unsafe { load_immediate_i32(MSK4 as i32, MSK3 as i32, MSK2 as i32, MSK1 as i32) };

    let mut r1: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 2) as usize].si) };
    let mut r2: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 1) as usize].si) };

    let mut iloop: usize = (N - POS1) as usize;
    let mut i: usize = 0;
    while i < iloop {
        r = simd_recursion(unsafe { &g.sfmt[i].si }, unsafe { &g.sfmt[i + POS1 as usize].si}, r1, r2, mask);
        unsafe { store_aligned_i128(&mut array[i].si , r) };
        r1 = r2;
        r2 = r;

        i += 1;
    }
    i = 0;
    while i < N as usize {
        r = simd_recursion(unsafe {&g.sfmt[i].si}, unsafe{&g.sfmt[i + POS1 as usize - N as usize].si}, r1, r2, mask);
        unsafe { store_aligned_i128(&mut array[i].si , r) };
        r1 = r2;
        r2 = r;
        i += 1;
    }
    iloop = size.wrapping_sub(N as usize);
    i = 0;
    while i < iloop {
        r = simd_recursion(unsafe {&array[i - N as usize].si}, unsafe {&array[i + POS1 as usize - N as usize].si}, r1, r2, mask);
        unsafe { store_aligned_i128(&mut array[i].si, r) };
        r1 = r2;
        r2 = r;
        i += 1;
    }

    iloop = 2 * N as usize - size;
    let mut j: usize = 0;
    while j < iloop {
        r = unsafe { load_aligned_i128(&array[j + size - N as usize].si) };
        unsafe { store_aligned_i128(&mut g.sfmt[j].si , r) };
        
        j += 1;
    }
    i = 0;
    while i < size {
        r = simd_recursion(unsafe {&array[i - N as usize].si}, unsafe {&array[i + POS1 as usize - N as usize].si}, r1, r2, mask);
        unsafe { store_aligned_i128(&mut array[i].si, r) };
        unsafe { store_aligned_i128(&mut g.sfmt[j].si , r) };
        j += 1;
        r1 = r2;
        r2 = r;
        i += 1;
    }
}

//#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
fn rshift128(out_param: &mut W128T, in_param: &W128T, shift: i32) {
    
}