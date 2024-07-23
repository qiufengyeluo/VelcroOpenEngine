#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::f32::consts::PI;
use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::vector3::Vector3;
use crate::math::vsimd;


const G_SIN_COEF1:[f32;4] = [ -0.0001950727, -0.0001950727, -0.0001950727, -0.0001950727 ];
const G_SIN_COEF2:[f32;4] = [0.0083320758,  0.0083320758,  0.0083320758,  0.0083320758 ];
const G_SIN_COEF3:[f32;4] =[-0.1666665247, -0.1666665247, -0.1666665247, -0.1666665247];
const G_COS_COEF1:[f32;4] = [-0.0013602249, -0.0013602249, -0.0013602249, -0.0013602249 ];
const G_COS_COEF2:[f32;4] = [0.0416566950,  0.0416566950,  0.0416566950,  0.0416566950];
const G_COS_COEF3:[f32;4] =[-0.4999990225, -0.4999990225, -0.4999990225, -0.4999990225];
const G_ACOS_HI_COEF1:[f32;4] = [ -0.0012624911, -0.0012624911, -0.0012624911, -0.0012624911 ];
const G_ACOS_HI_COEF2:[f32;4] = [  0.0066700901,  0.0066700901,  0.0066700901,  0.0066700901 ];
const G_ACOS_HI_COEF3:[f32;4] = [ -0.0170881256, -0.0170881256, -0.0170881256, -0.0170881256 ];
const G_ACOS_HI_COEF4:[f32;4] = [  0.0308918810,  0.0308918810,  0.0308918810,  0.0308918810 ];
const G_ACOS_LO_COEF1:[f32;4] = [ -0.0501743046, -0.0501743046, -0.0501743046, -0.0501743046 ];
const G_ACOS_LO_COEF2:[f32;4] = [  0.0889789874,  0.0889789874,  0.0889789874,  0.0889789874 ];
const G_ACOS_LO_COEF3:[f32;4] = [ -0.2145988016, -0.2145988016, -0.2145988016, -0.2145988016 ];
const G_ACOS_LO_COEF4:[f32;4] = [  1.5707963050,  1.5707963050,  1.5707963050,  1.5707963050 ];
const G_ACOS_COEF1:[f32;4]   = [ -0.0200752200, -0.0200752200, -0.0200752200, -0.0200752200 ];
const G_ACOS_COEF2:[f32;4]   = [  0.0759031500,  0.0759031500,  0.0759031500,  0.0759031500 ];
const G_ACOS_COEF3:[f32;4]   = [ -0.2126757000, -0.2126757000, -0.2126757000, -0.2126757000 ];
const G_ATAN_HI_RANGE:[f32;4] = [  2.4142135624,  2.4142135624,  2.4142135624,  2.4142135624 ];
const G_ATAN_LO_RANGE:[f32;4] = [  0.4142135624,  0.4142135624,  0.4142135624,  0.4142135624 ];
const G_ATAN_COEF1:[f32;4]   = [  8.05374449538e-2,  8.05374449538e-2,  8.05374449538e-2,  8.05374449538e-2 ];
const G_ATAN_COEF2:[f32;4]   = [ -1.38776856032e-1, -1.38776856032e-1, -1.38776856032e-1, -1.38776856032e-1 ];
const G_ATAN_COEF3:[f32;4]   = [  1.99777106478e-1,  1.99777106478e-1,  1.99777106478e-1,  1.99777106478e-1 ];
const G_ATAN_COEF4:[f32;4]   = [ -3.33329491539e-1, -3.33329491539e-1, -3.33329491539e-1, -3.33329491539e-1 ];
const G_EXP_COEF1:[f32;4]    = [  1.2102203e7, 1.2102203e7, 1.2102203e7, 1.2102203e7 ];
const G_EXP_COEF2:[i32;4]  = [ -8388608, -8388608, -8388608, -8388608];
const G_EXP_COEF3:[f32;4]    = [  1.1920929e-7, 1.1920929e-7, 1.1920929e-7, 1.1920929e-7 ];
const G_EXP_COEF4:[f32;4]    = [  3.371894346e-1, 3.371894346e-1, 3.371894346e-1, 3.371894346e-1 ];
const G_EXP_COEF5:[f32;4]    = [  6.57636276e-1, 6.57636276e-1, 6.57636276e-1, 6.57636276e-1 ];
const G_EXP_COEF6:[f32;4]    = [  1.00172476, 1.00172476, 1.00172476, 1.00172476 ];

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