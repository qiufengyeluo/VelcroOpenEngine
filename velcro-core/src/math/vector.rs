#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]
use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::vector3::Vector3;
use crate::math::vsimd;

pub unsafe fn dot_to_f32(lhs :&Vector3,rhs :&Vector3)->f32{
    let x2  =   mul(lhs.get_simd_value(), rhs.get_simd_value()) ;
    let xy  =   add(splat_second(x2), x2);
    let xyz =   add(splat_third(x2), xy);
    let result   =   select_first(splat_first(xyz));
    result
}
pub unsafe fn dot_to_f32_type(lhs : FloatType,rhs:FloatType) ->FloatType{
    let x2  =   mul(*lhs, rhs) ;
    let xy  =   add(splat_second(x2), x2);
    let xyz =   add(splat_third(x2), xy);
    let result   =   splat_first(xyz);
    result
}

pub unsafe fn from_vec_first(value :FloatType ) ->FloatType
{
    let result   =   splat_first(value);
    result
}
pub unsafe fn from_vec_second(value :FloatType) ->FloatType{
    let x = replace_second_f32(value,0.0);
    let result   =   replace_third_f32(x,0.0);
    result
}
pub unsafe fn from_vec_third(value :FloatType)->FloatType{
    let result   =   replace_third_f32(value,0.0);
    result
}

pub unsafe fn normalize(value:FloatArgType)->FloatType{
    let dot_val = dot_to_f32_type(value,value);
    let from_val = from_vec_first(dot_val);
    let length_squared = splat_first(from_val);
    let  length = sqrt(length_squared);
    let result  = div(value,length);
    result
}
pub unsafe fn normalize_estimate(value:FloatArgType)->FloatType{
    let dot_val = dot_to_f32_type(value,value);
    let from_val = from_vec_first(dot_val);
    let length_squared = splat_first(from_val);
    let inv_length = sqrt_inv_estimate(length_squared);
    let result  = mul(value,inv_length);
    result
}
pub unsafe fn normalize_safe(value:FloatArgType,tolerance :f32)->FloatType{
    let float_epsilon = splat(tolerance*tolerance);
    let dot_val = dot_to_f32_type(value,value);
    let from_val = from_vec_first(dot_val);
    let length_squared = splat_first(from_val);
    if cmp_all_lt(length_squared,float_epsilon) {
        let result = zero_float();
        result
    } else {
        let sqrt_val = sqrt(length_squared);
        let result = div(value,sqrt_val);
        result
    }
}
pub unsafe fn normalize_safe_estimate(value :FloatArgType,tolerance:f32)->FloatType{
    let float_epsilon = splat(tolerance*tolerance);
    let dot_val = dot_to_f32_type(value,value);
    let from_val = from_vec_first(dot_val);
    let length_squared = splat_first(from_val);
    if cmp_all_lt(length_squared,float_epsilon) {
        let result = zero_float();
        result
    } else {
        let sqrt_val = sqrt_inv_estimate(length_squared);
        let result = mul(value,sqrt_val);
        result
    }
}