#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

// PartialEq 是否相等
use std::ops::*;
use crypto_api_osrandom::to_vec;
#[cfg(target_arch = "arm")]
#[allow(dead_code)]
use vsimd::neon::*;

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use vsimd::sse::*;

use crate::math::vector::*;
use crate::math::*;
use crate::math::constants::*;
use crate::math::simd_math::*;
use crate::math::math_utils::*;
use crate::math::vector3::Vector3;

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector4 {
    _value: FloatType,
}

impl Vector4 {
    pub fn get_simd_value(self)->FloatType{
        self._value
    }
}