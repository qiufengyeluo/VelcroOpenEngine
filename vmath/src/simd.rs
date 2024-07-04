#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "arm")]
use std::arch::asm::*;




#[cfg(target_arch = "arm")] 
type __m128 = float32x4_t;
#[cfg(target_arch = "arm")] 
type __m128i = int32x4_t;

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))] 
struct __m128 (f32, f32, f32, f32);
#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
struct __m128i (i64, i64);



pub type FloatType = __m128;
pub type Int32Type = __m128i;
pub type FloatArgType = FloatType;
pub type Int32ArgType = Int32Type;


#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe  fn load_aligned(ptr: *const f32) -> FloatType {
    return _mm_load_ps(ptr);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn load_aligned_i128(ptr: *const Int32Type) -> Int32Type {
    return _mm_load_si128(ptr);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn load_unaligned(ptr: *const f32) -> FloatType {
    return _mm_loadu_ps(ptr); 
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn load_unaligned_i128(ptr: *const Int32Type) -> Int32Type {
    return _mm_loadu_si128(ptr);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn store_aligned(addr: *mut f32, value: FloatType) {
    _mm_store_ps(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn store_aligned_i128(addr: *mut Int32Type, value: Int32Type) {
    _mm_store_si128(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn store_unaligned(addr: *mut f32, value: FloatType) {
    _mm_storeu_ps(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn store_unaligned_i128(addr: *mut Int32Type, value: Int32Type) {
    _mm_storeu_si128(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn stream_aligned(addr: *mut f32, value: FloatType) {
    _mm_stream_ps(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn stream_aligned_i128(addr: *mut Int32Type, value: Int32Type) {
    _mm_stream_si128(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn convert_to_float(value: Int32Type) -> FloatType {
    return _mm_cvtepi32_ps(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
pub unsafe fn convert_to_int(value: FloatType) -> Int32Type {
    return _mm_cvttps_epi32(value);
}