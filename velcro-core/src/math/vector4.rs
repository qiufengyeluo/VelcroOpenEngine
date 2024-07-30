#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

#[cfg(target_arch = "arm")]
#[allow(dead_code)]
use vsimd::neon::*;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use vsimd::sse::*;

use crate::math::*;

// PartialEq 是否相等
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    _value: FloatType,
}

impl Vector4 {
    pub fn new()->Vector4{
        unsafe {
            Vector4 {
                _value: zero_float(),
            }
        }
    }

    pub fn new_zero()->Vector4{
        unsafe {
            Vector4 {
                _value: zero_float(),
            }
        }
    }
    pub fn get_simd_value(self)->FloatType{
        self._value
    }
}