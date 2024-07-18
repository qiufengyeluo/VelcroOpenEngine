#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::vsimd;

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector3 {
    _x: f32,
    _y: f32,
    _z: f32,
    _value: FloatType,
    _values:[f32; 3],
}

impl Vector3 {
    #[allow(dead_code)]
    pub fn new() -> Vector3 {
        Vector3 {
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _value: FloatType,
            _values: [f32;3],
        }
    }

    pub fn new_splat(x:f32)->Vector3{
        let mut result:Vector3 = new();
        unsafe { result._value = splat(x); }
        result
    }

    pub fn new_load_immediate(x:f32,y:f32,z:f32)->Vector3{
        let mut result:Vector3 = new();
        unsafe { result._value = load_immediate(x,y,z,0.0); }
        result
    }


    pub fn new_zero()->Vector3{
        Vector3{
            _x: 0.0,
            _y: 0.0,
            _z: 0.0,
            _value: FloatType,
            _values: [f32;3],
        }
    }
    
    pub fn new_one(x:f32)->Vector3{
        Vector3{
            _x : x,
            _y: 0.0,
            _z: 0.0,
            _value: FloatType,
            _values: [f32;3],
        }
    }

    pub fn is_close(v:&Vector3, tolerance :f32) ->bool
    {
        let dist:Vector3 = (v - (*Self)).GetAbs();
        return dist.is_less_equal_than(new_splat(tolerance));
    }
    pub unsafe fn is_less_equal_than(rhs:&Vector3) ->bool
    {
        return  cmp_all_lt_eq(Self._value,rhs._value,0b0111);
    }
}