#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::{mem, sync::{atomic::AtomicI32, Arc, Mutex}};
use crate::vmath::vsimd::*;

const MEXP: i32 = 19937;
const N: i32    = MEXP / 128 + 1;

const N32:  i32    = N * 4;
const N64:  i32    = N * 2;
const POS1: i32    = 122;
const SL1:  i32    = 18;
const SR1:  i32    = 11;
const SL2:  i32    = 1;
const SR2:  i32    = 1;

const MSK:     u32  = 0xdfff_ffef;
const MSK2:    u32  = 0xddfe_cb7f;
const MSK3:    u32  = 0xbffa_ffff;
const MSK4:    u32  = 0xbfff_fff6;
const PARITY1: u32  = 0x0000_0001;
const PARITY2: u32  = 0x0000_0000;
const PARITY3: u32  = 0x0000_0000;
const PARITY4: u32  = 0x13c9_e684;

// a parity check vector which certificate the period of 2^{MEXP}
const parity: [u32; 4] = [PARITY1, PARITY2, PARITY3, PARITY4];

union W128T {
    si: Int32Type,
    u: [u32; 4]
}


assert_eq!(N, MEXP / (mem::size_of::<W128_T>() * 8) + 1, "The smft member array must fit all iterations of the correct 128-bit size.");


pub struct Sfmt {
    _sfmt: [W128T; N as usize],
    _index: AtomicI32,
    _generation_mutex: Arc<Mutex<u32>>
}