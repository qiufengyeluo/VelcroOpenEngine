#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crypto_api_osrandom;
use std::ptr::read_unaligned;

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