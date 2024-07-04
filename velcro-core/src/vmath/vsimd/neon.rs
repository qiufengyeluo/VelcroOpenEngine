#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

#[cfg(target_arch = "arm")]
use std::arch::asm::*;

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