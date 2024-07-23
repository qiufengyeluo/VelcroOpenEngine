#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

#[cfg(target_arch = "arm")]
use std::arch::asm::*;
use crate::math::vsimd::{load_aligned_i128, load_immediate};

#[cfg(target_arch = "arm")] 
type __m128 = float32x4_t;
#[cfg(target_arch = "arm")] 
type __m128i = int32x4_t;

#[cfg(target_arch = "arm")] 
pub type FloatType = __m128;
#[cfg(target_arch = "arm")] 
pub type Int32Type = __m128i;
#[cfg(target_arch = "arm")] 
pub type FloatArgType = FloatType;
#[cfg(target_arch = "arm")] 
pub type Int32ArgType = Int32Type;

#[cfg(target_arch = "arm")] 
#[inline]
pub unsafe  fn load_aligned(ptr: *const f32) -> FloatType {
    return vld1_f32(ptr);
}
#[cfg(target_arch = "arm")]
#[inline]
pub unsafe  fn Splat(ptr: *const f32) ->FloatType
{
    return vdupq_n_f32(ptr);
}


#[cfg(any(target_arch = "arm"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn fast_load_constant_f32(values :*const f32)->FloatType{
    return   load_aligned(values);
}
#[cfg(any(target_arch = "arm"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn fast_load_constant_i32(value :*const i32)->Int32Type{
    return   load_aligned_i128(values);
}