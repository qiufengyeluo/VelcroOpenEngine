#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ptr::read_unaligned;

use crypto_api_osrandom;

use crate::math::common_sse::VecType;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::vsimd::{FloatType, Int32ArgType, Int32Type};

pub struct SimpleLcgRandom {
    _seed: u64
}

impl SimpleLcgRandom {
    pub fn new(seed: u64) -> Self {
        SimpleLcgRandom { _seed: ((seed ^ 0x5DEECE66D) & ((1 << 48) - 1)) }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self._seed =  (seed ^ 0x5DEECE66D) & ((1 << 48) - 1);
    }

    pub fn get64_random(&mut self) -> u64 {
        self._seed = (self._seed * 0x5DEECE66D + 0b11) & ((1 << 48) - 1);
        return self._seed;
    }

    pub fn get_random(&mut self) -> u32 {
        return (self.get64_random() >> 16) as u32;
    }

    pub fn get_random_float(&mut self) -> f32 {
        let mut r = self.get_random();
        r &= 0x007fffff;
        r |= 0x3f800000;

        union U {
            f: f32,
            i: u32,
        }
        let mut uv = U{i: 0};
        uv.i = r;
        return unsafe { uv.f - 1.0 };
    }
}

/// 获取随机数
pub fn get_random<T>() -> Option<T> {
    let length = std::mem::size_of::<T>();
    let result: Result<Vec<u8>, crypto_api_osrandom::error::Error> = crypto_api_osrandom::to_vec(length);
    if result.is_err() {
        return None;
    }

    return Some(unsafe { read_unaligned(result.unwrap().as_ptr().cast::<T>()) });
}
pub struct SimpleLcgRandomVec4{
    _seed:Int32Type,
}
impl SimpleLcgRandomVec4 {
    pub fn new()->SimpleLcgRandomVec4{
        SimpleLcgRandomVec4{
            _seed
        }
    }

    pub unsafe  fn set_seed(&mut self,seed:Int32ArgType){
        let mask = Vec4::splat_i32(0x7FFFFFFF); // 2^31 - 1
        self._seed = Vec4::and_i32(seed, mask);
    }

    pub unsafe  fn get_random_int4(&mut self) ->Int32Type{
        let scalar = Vec4::splat_i32(1103515245);
        let constant = Vec4::splat_i32(12345);
        let mask = Vec4::splat_i32((0x7FFFFFFF));
        self._seed = Vec4::and_i32(Vec4::madd_i32(self._seed, scalar, constant), mask);
        self._seed
    }

    pub unsafe  fn get_random_float4(&mut self) ->FloatType{
        let mut rand_val = self.get_random_int4();
        rand_val = Vec4::and_i32(rand_val, Vec4::splat_i32(0x007fffff)); // Sets mantissa to random bits
        rand_val = Vec4::or_i32(rand_val, Vec4::splat_i32(0x3f800000)); // Result is in [1,2), uniformly distributed
        return Vec4::sub(Vec4::cast_to_float(rand_val), Vec4::splat(1.0));
    }
}
