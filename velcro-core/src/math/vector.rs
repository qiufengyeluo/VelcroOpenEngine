#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::f32::consts::PI;
use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::constants::{HALF_PI, TWO_OVER_PI};
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
pub unsafe fn acos(value:FloatArgType) ->FloatType{
    let  xabs = abs(value);
    let xabs2 = mul(xabs,xabs);
    let xabs4 = mul(xabs2,xabs2);
    let t1 = sqrt(sub(splat(1.0),xabs));
    let select_val = cmp_lt(value,zero_float());

    let hi = madd(xabs,
                            madd(xabs,
                                        madd(xabs,
                                             fast_load_constant_f32(G_ACOS_HI_COEF1 as (*const f32)),
                                              fast_load_constant_f32(G_ACOS_HI_COEF2 as (*const f32))),
                                    fast_load_constant_f32(G_ACOS_HI_COEF3 as (*const f32))),
                  fast_load_constant_f32(G_ACOS_HI_COEF4 as (*const f32)));

    let lo = madd(xabs,
                            madd(xabs,
                                 madd(xabs,fast_load_constant_f32(G_ACOS_LO_COEF1 as (*const f32)),
                                      fast_load_constant_f32(G_ACOS_LO_COEF2 as (*const f32)))
                                        ,fast_load_constant_f32(G_ACOS_LO_COEF3 as (*const f32)))
                            ,fast_load_constant_f32(G_ACOS_LO_COEF4 as (*const f32)));

    let result = madd(hi,xabs4,lo);
    let positive = mul(t1,result);
    let negative = sub(splat(PI),positive);
    return select(negative,positive,select_val);

}
AZ_MATH_INLINE typename VecType::FloatType Sin(typename VecType::FloatArgType value)
{
// Range Reduction
typename VecType::FloatType x = VecType::Mul(value, FastLoadConstant<VecType>(Simd::g_TwoOverPi));

// Find offset mod 4
const typename VecType::Int32Type intx = VecType::ConvertToIntNearest(x);
const typename VecType::Int32Type offset = VecType::And(intx, VecType::Splat(3));

const typename VecType::FloatType intxFloat = VecType::ConvertToFloat(intx);
x = VecType::Sub(value, VecType::Mul(intxFloat, FastLoadConstant<VecType>(Simd::g_HalfPi)));

typename VecType::FloatType sinx, cosx;
ComputeSinxCosx<VecType>(x, sinx, cosx);

// Choose sin for even offset, cos for odd
typename VecType::Int32Type mask = VecType::CmpEq(VecType::And(offset, VecType::Splat(1)), VecType::ZeroInt());
typename VecType::FloatType result = VecType::Select(sinx, cosx, VecType::CastToFloat(mask));

// Change sign for result if offset is 1 or 2
mask = VecType::CmpEq(VecType::And(offset, VecType::Splat(2)), VecType::ZeroInt());
result = VecType::Select(result, VecType::Xor(result, VecType::Splat(-0.0f)), VecType::CastToFloat(mask));

return result;
}
pub unsafe fn sin(value:FloatArgType)->FloatType{
    let x = mul(value,fast_load_constant_i32(G_));
    let result = select()
    result
}

pub unsafe fn sin_cos(angle:FloatArgType) ->FloatType{
    let angle_offset = load_immediate(0.0, HALF_PI, 0.0, 0.0);
    let angles = add(from_vec_first(angle),angle_offset);
    return sin(angles)
}