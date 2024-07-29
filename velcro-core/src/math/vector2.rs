#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

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
use crate::math::vector4::Vector4;

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector2 {
    _value: FloatType,
}

impl Vector2 {
    pub unsafe fn new()->Vector2{
        Vector2{
            _value:zero_float(),
        }
    }
    pub unsafe  fn new_x(x:&f32)->Vector2{
        Vector2{
            _value: splat(x.to_owned()),
        }
    }
    pub unsafe fn new_xy(x:&f32,y:&f32)->Vector2{
        Vector2{
            _value:load_immediate(x.to_owned(),y.to_owned(),0.0,0.0),
        }
    }
    pub unsafe fn new_float_type(value:&FloatType)->Vector2{
        Vector2{
            _value:value.to_owned(),
        }
    }

    pub unsafe fn new_vector3(source:&Vector3)->Vector2{
        Vector2{
            _value:source.get_simd_value(),
        }
    }

    pub unsafe fn new_vector4(source:&Vector4)->Vector2{
        Vector2{
            _value:source.get_simd_value(),
        }
    }

    pub unsafe fn create_zero()->Vector2{
        return Vector2::new_x(0.0.borrow());
    }

    pub unsafe fn create_one() ->Vector2{
        return Vector2::new_x(1.0.borrow());
    }

    pub unsafe fn create_axis_x(length:&f32)->Vector2{
       let result = Vector2::new_xy(length,0.0.borrow());
        result
    }

    pub unsafe fn create_axis_y(length:&f32)->Vector2{
        let result = Vector2::new_xy(0.0.borrow(),length);
        result
    }


    pub unsafe fn create_from_float2(values:*const f32)->Vector2{
        let arr = values as [f32;2];
        let result = Vector2::new_xy(arr[0].borrow(),arr[1].borrow());
        result
    }

    pub unsafe fn create_from_angle(angle:&f32) ->Vector2{
        let mut sin : f32;
        let mut cos : f32;
        simd_sin_cos(angle.borrow(),sin.borrow_mut(),cos.borrow_mut());
        let result = Vector2::new_xy(sin.borrow(),cos.borrow());
        result
    }

    pub unsafe fn create_select_cmp_equal(cmp1:&Vector2,cmp2:&Vector2,va:&Vector2,vb:&Vector2)->Vector2{
        let mask = cmp_eq(cmp1._value,cmp2._value);
        let result = Vector2::new_float_type(select(va._value,vb._value,mask).borrow());
        result
    }

    pub unsafe fn create_select_cmp_greater_equal(cmp1:&Vector2,cmp2:&Vector2,va:&Vector2,vb:&Vector2)->Vector2{
        let mask = cmp_gt_eq(cmp1._value,cmp2._value);
        let result = Vector2::new_float_type(select(va._value,vb._value,mask).borrow());
        result
    }

    pub unsafe fn create_select_cmp_greater(cmp1:&Vector2, cmp2:&Vector2, va:&Vector2, vb:&Vector2) ->Vector2{
        let mask = cmp_gt(cmp1._value,cmp2._value);
        let result = Vector2::new_float_type(select(va._value,vb._value,mask).borrow());
        result
    }
}