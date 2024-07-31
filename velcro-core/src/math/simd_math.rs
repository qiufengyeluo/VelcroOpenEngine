
#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::arch::x86_64::{_MM_SHUFFLE, _mm_shuffle_ps};

use crate::math::vector::{acos, angle_mod, atan, atan2, cos, sin, sin_cos_float_type, wrap};
use crate::math::vector3::Vector3;
use crate::math::vsimd::*;

mod simd {
    use crate::math::vsimd::select_first;

    pub unsafe fn abs(value:&f32) ->f32{
        return select_first(abs(value)); crate::math::vsimd::sse::abs(abs(splat(value.to_owned())))
    }

}

pub unsafe fn simd_mod(value:&f32,divisor:&f32)->f32{
    return  select_first(mod_calculate(splat(value.to_owned()),splat(divisor.to_owned())));
}

pub unsafe fn simd_wrap_max(value:&f32, max_value:&f32) ->f32{
    return select_first(wrap(splat(value.to_owned()).borrow(),zero_float().borrow(),splat(max_value.to_owned()).borrow()))
}
pub unsafe fn simd_wrap_max_min(value:&f32,min_value:&f32, max_value:&f32)->f32{
    return select_first(wrap(splat(value.to_owned()).borrow(),splat(min_value.to_owned()).borrow(),splat(max_value.to_owned()).borrow()))
}

pub unsafe fn simd_angle_mod(value:&f32)->f32{
    return  select_first(angle_mod(splat(value.to_owned()).borrow()));
}

pub unsafe fn simd_sin_cos(angle:&f32,mut sin:&f32,mut cos:&f32){
    let values = sin_cos_float_type(splat(angle.to_owned()));
    sin = select_first(values).borrow();
    cos = select_first(splat_first(values)).borrow();
}

pub unsafe fn simd_sin(angle :&f32)->f32{
    return select_first(sin(splat(angle.to_owned())));
}
pub unsafe fn simd_cos(angle :&f32)->f32{
    return select_first(cos(splat(angle.to_owned())));
}

pub unsafe fn simd_acos(value:&f32)->f32{
    return  select_first(acos(splat(value.to_owned()).borrow()));
}
pub unsafe fn simd_atan(value:&f32)->f32{
    return select_first(atan(splat(value.to_owned()).borrow()));
}

pub unsafe fn simd_atan2(y:&f32,x:&f32)->f32{
    return select_first(atan2(splat(y.to_owned()).borrow(),splat(x.to_owned()).borrow()));
}

pub unsafe fn simd_sqrt(value :&f32)->f32{
    return  select_first(sqrt(splat(value.to_owned())));
}

pub unsafe fn simd_inv_sqrt(value:&f32)->f32{
    return select_first(sqrt_inv(splat(value.to_owned())));
}

struct Vec2{
    
}
impl Vec2 {
    pub unsafe fn dot_to_float_type(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        let x2 = mul(arg1.to_owned(),arg2.to_owned());
        return splat_first(add(splat_second(x2),x2));
    }
}
struct Vec3{

}
impl Vec3{
    pub unsafe fn dot_to_f32(lhs :&Vector3,rhs :&Vector3)->f32{
        let x2  =   mul(lhs.get_simd_value(), rhs.get_simd_value()) ;
        let xy  =   add(splat_second(x2), x2);
        let xyz =   add(splat_third(x2), xy);
        let result   =   select_first(splat_first(xyz));
        result
    }
    pub unsafe fn dot_to_float_type(lhs : FloatArgType,rhs:FloatArgType) ->FloatType{
        let x2  =   mul(lhs, rhs) ;
        let xy  =   add(splat_second(x2), x2);
        let xyz =   add(splat_third(x2), xy);
        let result   =   splat_first(xyz);
        result
    }

}
struct Vec4{

}
impl Vec4 {
    pub unsafe fn dot_to_float_type(lhs : FloatArgType,rhs:FloatArgType) ->FloatType{
        let x2  =   mul(lhs, rhs) ;
        let tmp  =   add(x2,_mm_shuffle_ps(x2,x2,_MM_SHUFFLE(2, 3, 0, 1)));
        return  add(tmp,_mm_shuffle_ps(tmp, tmp, _MM_SHUFFLE(1, 0, 2, 3)));
    }
}