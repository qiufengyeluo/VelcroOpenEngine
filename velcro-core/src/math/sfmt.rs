#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::{mem, sync::{atomic::AtomicI32, Arc, atomic, Mutex}, ptr::{self}};
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

// a PARITY check vector which certificate the period of 2^{MEXP}
const PARITY: [u32; 4] = [PARITY1, PARITY2, PARITY3, PARITY4];

macro_rules! velcor_fmt_func1 {
    ($x:expr) => {
        ($x ^ ($x >> 27)).wrapping_mul(1664525)
    };
}

macro_rules! velcor_fmt_func2 {
    ($x:expr) => {
        ($x ^ ($x >> 27)).wrapping_mul(1566083941)
    };
}

macro_rules! idxof {
    ($x:expr) => {
        $x 
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
union W128T {
    si: Int32Type,
    u: [u32; 4]
}

impl W128T {
    pub fn new() -> Self {
        W128T {u: [0, 0, 0, 0]}
    }
}



//assert_eq!(N, MEXP / (mem::size_of::<W128T>() as i32 * 8 ) + 1 , "The smft member array must fit all iterations of the correct 128-bit size.");

pub struct Sfmt {
    sfmt: [W128T; N as usize],
    index: AtomicI32,
    psfmt32: *mut u32,
    psfmt64: *mut u64,
    generation_mutex: Arc::<Mutex<u32>>
}

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
fn simd_recursion(a: *mut Int32Type, b: *mut Int32Type, c: Int32Type, d: Int32Type, mask: Int32Type) -> Int32Type {
    let mut x = unsafe { *a.as_mut().unwrap() };
    let mut y = unsafe { _mm_srli_epi32( *b.as_mut().unwrap(), SR1) }; 
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
fn gen_rand_all(g:&mut Sfmt) {
    let mut r:    Int32Type;
    let mask: Int32Type = unsafe { load_immediate_i32(MSK4 as i32, MSK3 as i32, MSK2 as i32, MSK1 as i32) };
    
    let mut r1: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 2) as usize].si) };
    let mut r2: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 1) as usize].si) };

    let iloop: usize = (N - POS1) as usize;
    let mut i: usize = 0;
    while i < iloop {
        r = simd_recursion(unsafe {g.sfmt.as_mut_ptr().offset(i as isize) as *mut Int32Type}, 
                            unsafe {g.sfmt.as_mut_ptr().offset(i .wrapping_add(POS1 as usize) as isize) as *mut Int32Type }, r1, r2, mask);
        unsafe { 
            let tmp = g.sfmt.as_mut_ptr().offset(i as isize) as *mut Int32Type;
            store_aligned_i128(tmp , r);
        };
        r1 = r2;
        r2 = r;

        i += 1;
    }
    
    while i < N as usize {
        r = simd_recursion(unsafe {g.sfmt.as_mut_ptr().offset(i as isize) as *mut Int32Type}, 
                                unsafe{g.sfmt.as_mut_ptr().offset(i .wrapping_add(POS1 as usize).wrapping_sub(N as usize) as isize) as *mut Int32Type}, r1, r2, mask);
        unsafe { store_aligned_i128(g.sfmt.as_mut_ptr().offset(i as isize) as *mut Int32Type , r) };
        r1 = r2;
        r2 = r;
        i += 1;
    }
}

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
fn gen_rand_array(g: &mut Sfmt, array: *mut W128T, size: usize) {
    let mut r:    Int32Type;
    let mask = unsafe { load_immediate_i32(MSK4 as i32, MSK3 as i32, MSK2 as i32, MSK1 as i32) };

    let mut r1: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 2) as usize].si) };
    let mut r2: Int32Type = unsafe { load_aligned_i128(&g.sfmt[(N - 1) as usize].si) };

    let mut iloop: usize = (N - POS1) as usize;
    let mut i: usize = 0;
    while i < iloop {

        r = simd_recursion(unsafe {  g.sfmt.as_mut_ptr().offset(i as isize) as *mut Int32Type }, 
                            unsafe {  g.sfmt.as_mut_ptr().offset((i + POS1 as usize) as isize) as *mut Int32Type }, r1, r2, mask);
        
        unsafe { store_aligned_i128(array.offset(i as isize).cast::<Int32Type>() , r) };
        r1 = r2;
        r2 = r;

        i += 1;
    }
    i = 0;
    while i < N as usize {
        r = simd_recursion(unsafe { g.sfmt.as_mut_ptr().offset(i as isize) as *mut Int32Type },
            unsafe{ g.sfmt.as_mut_ptr().offset((i + POS1 as usize - N as usize) as isize) as *mut Int32Type}, r1, r2, mask);
        unsafe { store_aligned_i128(array.offset(i as isize).cast::<Int32Type>() , r) };
        r1 = r2;
        r2 = r;
        i += 1;
    }
    iloop = size.wrapping_sub(N as usize);
    i = 0;
    while i < iloop {
        
        r = simd_recursion(unsafe { array.offset(i.wrapping_sub(N as usize) as isize ).cast::<Int32Type>() },
                            unsafe { array.offset(i.wrapping_sub(POS1 as usize).wrapping_sub(N as usize) as isize ).cast::<Int32Type>() }, r1, r2, mask);
        unsafe { store_aligned_i128(array.offset(i as isize).cast::<Int32Type>(), r) };
        r1 = r2;
        r2 = r;
        i += 1;
    }

    iloop = 2 * N as usize - size;
    let mut j: usize = 0;
    while j < iloop {
        r = unsafe { load_aligned_i128(array.offset(j.wrapping_add(size as usize).wrapping_sub(N as usize) as isize ).cast::<Int32Type>())};
        unsafe { store_aligned_i128(&mut g.sfmt[j].si , r) };
        
        j += 1;
    }
    i = 0;
    while i < size {
        r = simd_recursion(unsafe { array.offset(i.wrapping_sub(N as usize) as isize ).cast::<Int32Type>() }, 
            unsafe { array.offset(i.wrapping_add(POS1 as usize).wrapping_sub(N as usize) as isize).cast::<Int32Type>()}, r1, r2, mask);
        unsafe { store_aligned_i128(array.offset(i as isize ).cast::<Int32Type>(), r) };
        unsafe { store_aligned_i128(g.sfmt.as_mut_ptr().offset(j as isize) as *mut Int32Type , r) };
        j += 1;
        r1 = r2;
        r2 = r;
        i += 1;
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
fn rshift128(out_param: *mut W128T, in_param: &W128T, shift: i32) {
    let th = unsafe {(in_param.u[3] as u64) << 32 | in_param.u[2] as u64};
    let tl = unsafe {(in_param.u[1] as u64) << 32 | in_param.u[0] as u64};

    let oh = th >> (shift.wrapping_mul(8));
    let mut ol = tl >> (shift.wrapping_mul(8));
    ol |= th << (64 - shift * 8);
    unsafe { (*out_param).u[1] = (ol >> 32) as u32 };
    unsafe { (*out_param).u[0] = ol as u32 };
    unsafe { (*out_param).u[3] = (oh >> 32) as u32 };
    unsafe { (*out_param).u[2] = oh as u32 };
}

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
fn lshift128(out_param: *mut W128T, in_param: &W128T, shift: i32) {
    let th = unsafe {(in_param.u[3] as u64) << 32 | in_param.u[2] as u64};
    let tl = unsafe {(in_param.u[1] as u64) << 32 | in_param.u[0] as u64};

    let oh = th >> (shift.wrapping_mul(8));
    let mut ol = tl >> (shift.wrapping_mul(8));
    ol |= tl << (64 - shift * 8);
    unsafe { (*out_param).u[1] = (ol >> 32) as u32 };
    unsafe { (*out_param).u[0] = ol as u32 };
    unsafe { (*out_param).u[3] = (oh >> 32) as u32 };
    unsafe { (*out_param).u[2] = oh as u32 };
}

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
fn do_recursion(r: *mut W128T, a: &W128T, b: &W128T, c: &W128T, d: &W128T) {
    let mut x:W128T = W128T{u: [0 ,0, 0, 0]};
    let mut y:W128T = W128T{u: [0 ,0, 0, 0]};

    lshift128(&mut x, a, SL2);
    rshift128(&mut y, c, SR2);

    unsafe { (*r).u[0] = a.u[0] ^ x.u[0] ^ ((b.u[0] >> SR1) & MSK1) ^ y.u[0] ^ (d.u[0] << SL1) };
    unsafe { (*r).u[1] = a.u[1] ^ x.u[1] ^ ((b.u[1] >> SR1) & MSK2) ^ y.u[1] ^ (d.u[1] << SL1) };
    unsafe { (*r).u[2] = a.u[2] ^ x.u[2] ^ ((b.u[2] >> SR1) & MSK3) ^ y.u[2] ^ (d.u[2] << SL1) };
    unsafe { (*r).u[3] = a.u[3] ^ x.u[3] ^ ((b.u[3] >> SR1) & MSK4) ^ y.u[3] ^ (d.u[3] << SL1) };
}

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
fn gen_rand_all(g: &mut Sfmt) {
    let mut i:usize = 0;

    let mut r1: *mut W128T;
    let mut r2: *mut W128T;

    r1 = &mut g.sfmt[(N - 2) as usize];
    r2 = &mut g.sfmt[(N - 1) as usize];

    let iloop = (N - POS1) as usize;
    while i < iloop {
        do_recursion(&mut g.sfmt[i],  &g.sfmt[i], &g.sfmt[i + POS1 as usize], unsafe {&(*r1)}, unsafe {&(*r2)});
        i += 1;
    }
    while i < N as usize {
        do_recursion(&mut g.sfmt[i],  &g.sfmt[i], &g.sfmt[i + POS1 as usize - N as usize], unsafe {&(*r1)}, unsafe {&(*r2)});
        r1 = r2;
        r2 = &mut g.sfmt[i];
    }
}

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
fn gen_rand_array(g: &mut Sfmt, array: &mut [W128T], size: usize) {
    let mut i:usize = 0;
    let mut j:usize = 0;

    let mut r1: *mut W128T;
    let mut r2: *mut W128T;

    r1 = &mut g.sfmt[(N - 2) as usize];
    r2 = &mut g.sfmt[(N - 1) as usize];
    let mut iloop = (N - POS1) as usize;
    while i < iloop {
        do_recursion(&mut array[i],  &g.sfmt[i], &g.sfmt[i + POS1 as usize], unsafe {&(*r1)}, unsafe {&(*r2)});
        r1 = r2;
        r2 = &mut array[i];
        i += 1;
    }
    while i < N as usize {
        do_recursion(&mut array[i],  &g.sfmt[i], &array[i + POS1 as usize - N as usize], unsafe {&(*r1)}, unsafe {&(*r2)});
        r1 = r2;
        r2 = &mut array[i];
        i += 1;
    }
    iloop = size - N as usize;
    while i < iloop {
        do_recursion(&mut array[i],  &array[i - N as usize], &array[i + POS1 as usize - N as usize], unsafe {&(*r1)}, unsafe {&(*r2)});
        r1 = r2;
        r2 = &mut array[i];
        i += 1;
    }
    iloop = 2 * N as usize - size;
    while j < iloop {
        g.sfmt[j] = array[j + size - N as usize];
        j += 1;
    }
    while i < size {
        do_recursion(&mut array[i],  &array[i - N as usize], &array[i + POS1 as usize - N as usize], unsafe {&(*r1)}, unsafe {&(*r2)});
        r1 = r2;
        r2 = &mut array[i];
        g.sfmt[j] = array[i];
        i += 1;
        j += 1;
    }

}

impl Sfmt {
    pub fn new() -> Self {
        use std::ptr::NonNull;
        let mut r = Sfmt{sfmt: [ W128T::new() ;N as usize], 
                            index: AtomicI32::new(0), 
                            psfmt32: NonNull::<u32>::dangling().as_ptr(),
                            psfmt64: NonNull::<u64>::dangling().as_ptr(),
                            generation_mutex: Arc::new(Mutex::new(0))};
        r.psfmt32 = r.sfmt.as_mut_ptr() as *mut u32;
        r.psfmt64 = r.sfmt.as_mut_ptr() as *mut u64;
        r.seed_init();
        return r;
    }

 

    

  

    unsafe fn get_fmt_element(&mut self, index: isize) -> u32 {
        return ptr::read_unaligned(self.psfmt32.offset( index) );
    }

    unsafe fn set_fmt_element(&mut self, index: isize, value: u32) {
        ptr::write_unaligned(self.psfmt32.offset(index), value);
    }

    unsafe fn get_fmt_element64(&mut self, index: isize) -> u64 {
        return ptr::read_unaligned(self.psfmt64.offset( index) );
    }

    fn period_certification(&mut self) {
        let mut inner: i32 = 0;
        let mut i: isize = 0;
        let mut j;
        let mut work;

        while i < 4{
            inner ^= (unsafe { self.get_fmt_element(idxof!(i)) & PARITY[i as usize]}) as i32;
            i += 1;
        }
        i = 16;
        while i > 0 {
            inner ^= inner >> i;
            i >>= 1;
        }
        inner &= 1;
        if inner == 1 {
            return;
        }
        
        i = 0;
        while i < 4 {
            work = 1;
            j = 0;
            while j < 32 {
                if work & PARITY[i as usize] != 0 {
                    unsafe {
                        let tmp = self.get_fmt_element(idxof!(i));
                        self.set_fmt_element(idxof!(i), tmp ^ work);
                    }
                    return;
                }
                work = work << 1;
                j += 1;
            }
            i += 1;
        }
    }

    fn seed_init(&mut self) {
        let mut buffer: [u32; 32] = [0; 32];
        let r = crypto_api_osrandom::to_slice( unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, 32 * std::mem::size_of::<u32>()) });
        if r.is_err() {
            assert!(false, "sfmt init failed.");
        }
        self.seed(&buffer, 32);
    }

    fn seed(&mut self, keys: &[u32], num_keys: i32) {
        let mut i: isize;
        let mut j: isize;
        let mut count: i32;
        let lag: i32;
        let size = N * 4;
        if size >= 623 {
            lag = 11;
        } else if size >= 68 {
            lag = 7;
        } else if size >= 39 {
            lag = 5;
        } else {
            lag = 3;
        }
        let mid = (size - lag).wrapping_div(2);
        
        unsafe { ptr::write_bytes(self.sfmt.as_mut_ptr() as *mut u8, 0x8b, mem::size_of::<W128T>().wrapping_mul(self.sfmt.len())) };
        if num_keys + 1 > N32 {
            count = num_keys + 1;
        } else {
            count = N32;
        }


        let mut r: u32 = velcor_fmt_func1!(unsafe {self.get_fmt_element(idxof!(0)) ^ self.get_fmt_element(idxof!(mid as isize)) ^ self.get_fmt_element(idxof!((N32 - 1) as isize))});
        unsafe { 
            let tmp = self.get_fmt_element(idxof!(mid as isize));
            self.set_fmt_element(idxof!(mid as isize),  tmp.wrapping_add(r));
        };
        
        r = ((r as i32) + num_keys) as u32;
        unsafe {
            let tmp = self.get_fmt_element((mid + lag) as isize); 
            self.set_fmt_element(idxof!((mid + lag) as isize), tmp.wrapping_add(r))
        };
        unsafe { self.set_fmt_element(idxof!(0), r)};

        count -= 1;
        i = 1;
        j = 0;
        while j < count as isize && j < num_keys as isize {
            r = velcor_fmt_func1!(unsafe {self.get_fmt_element(idxof!(i as isize)) ^ self.get_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32))  as isize)) ^ self.get_fmt_element(idxof!(((i as i32 + N32 - 1).wrapping_rem(N32))  as isize))});
            unsafe {
                let tmp = self.get_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize));
                self.set_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize), tmp.wrapping_add(r));
            }
    
            r = r.wrapping_add(keys[j as usize].wrapping_add(i as u32));
            unsafe {
                let tmp = self.get_fmt_element(idxof!(((i as i32 + mid + lag).wrapping_rem(N32)) as isize));
                self.set_fmt_element(idxof!(((i as i32 + mid + lag).wrapping_rem(N32)) as isize), tmp.wrapping_add(r));
                self.set_fmt_element(idxof!(i as isize), r);
            }
            
            j += 1;
            i = ((i as i32 + 1).wrapping_rem(N32)) as isize;
        }

        while j < count as isize {
            r = velcor_fmt_func1!(unsafe { self.get_fmt_element(idxof!(i as isize)) ^ self.get_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize)) ^ self.get_fmt_element(idxof!(((i as i32 + N32 - 1).wrapping_rem(N32))  as isize))});
            unsafe {
                let tmp = self.get_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize));
                self.set_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize), tmp.wrapping_add(r));
            }
            r += i as u32;
            unsafe {
                let tmp = self.get_fmt_element(idxof!(((i as i32 + mid + lag).wrapping_rem(N32)) as isize));
                self.set_fmt_element(idxof!(((i as i32 + mid + lag).wrapping_rem(N32)) as isize), tmp.wrapping_add(r));
                self.set_fmt_element(idxof!(i as isize), r);
            }
            

            j += 1;
            i = ((i as i32 + 1).wrapping_rem(N32)) as isize;
        }
        j = 0;
        while j < N32 as isize {
            r = velcor_fmt_func2!(unsafe { self.get_fmt_element(idxof!(i as isize)).
                wrapping_add(self.get_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize))).
                wrapping_add(self.get_fmt_element(idxof!(((i as i32 + N32 - 1).wrapping_rem(N32))  as isize)))});

            unsafe {
                let tmp = self.get_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize));
                self.set_fmt_element(idxof!(((i as i32 + mid).wrapping_rem(N32)) as isize), tmp ^ r);
            }
            r -= i as u32;
            unsafe {
                let tmp = self.get_fmt_element(idxof!(((i as i32 + mid + lag).wrapping_rem(N32)) as isize));
                self.set_fmt_element(idxof!(((i as i32 + mid + lag).wrapping_rem(N32)) as isize), tmp ^ r);
                self.set_fmt_element(idxof!(i as isize), r);
            }
            
            j += 1;
            i = ((i as i32 + 1).wrapping_rem(N32)) as isize;
        }
        self.index.store(N32, atomic::Ordering::Relaxed);
        self.period_certification();
    }

    fn rand32(&mut self) -> u32 {
        let mut idx = self.index.fetch_add(1, atomic::Ordering::Relaxed);
        if idx >= N32 {
            let _locker = *self.generation_mutex.lock().unwrap();
            idx += 1;

            if self.index.compare_exchange(idx, 0, atomic::Ordering::Relaxed , atomic::Ordering::Relaxed).is_ok() {
                gen_rand_all(self);
            }

            return self.rand32();
        }

        return unsafe { self.get_fmt_element(idx as isize) };
    }

    fn rand64(&mut self) -> u64 {
        let mut idx = self.index.fetch_add(2, atomic::Ordering::Relaxed);
        if idx >= N32 - 1 {
            let _locker = *self.generation_mutex.lock().unwrap();
            idx += 2;

            if self.index.compare_exchange(idx, 0, atomic::Ordering::Relaxed , atomic::Ordering::Relaxed).is_ok() {
                gen_rand_all(self);
            }

            return self.rand64();
        }
        
        return unsafe {  self.get_fmt_element64(idx.wrapping_div(2) as isize) };
    }

    pub fn fill_array32(&mut self, array: *mut u32, size: usize) {
        assert_eq!(self.index.load(atomic::Ordering::Relaxed), N32, "Invalid index Reinitialize!");
        assert_eq!(size.wrapping_rem(4), 0, "Size must be multiple of 4!");
        assert!(size >= N32 as usize, "Size must be bigger than {} get_min_array32_size()!", N32);

        gen_rand_array(self, array as *mut W128T, size.wrapping_div(4));
        self.index.store(N32, atomic::Ordering::Relaxed);
    }

    pub fn fill_array64(&mut self, array: *mut u64, size: usize) {
        assert_eq!(self.index.load(atomic::Ordering::Relaxed), N32, "Invalid index Reinitialize!");
        assert_eq!(size.wrapping_rem(4), 0, "Size must be multiple of 4!");
        assert!(size >= N64 as usize, "Size must be bigger than {} get_min_array64_size()!", N32);
        
        gen_rand_array(self, array as *mut W128T, size.wrapping_div(2));
        self.index.store(N32, atomic::Ordering::Relaxed);
    }

    pub const fn get_min_array32_size() -> i32 {
        return N32;
    }

    pub const fn get_min_array64_size() -> i32 {
        return N64;
    }

    // return [0, 1]
    pub fn rand_r32(&mut self) -> f64 {
        return self.rand32() as f64 * (1.0 / 4294967295.0);
    }

    // return [0, 1]
    pub fn rand_r32_1(&mut self) -> f64 {
        return self.rand32() as f64 * (1.0 / 4294967296.0);
    }

    pub fn rand_r32_2(&mut self) -> f64 {
        return (self.rand_r32() as f64 + 0.5) * (1.0 / 4294967296.0);
    }
}