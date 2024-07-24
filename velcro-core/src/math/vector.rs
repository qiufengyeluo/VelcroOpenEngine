#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::arch::x86_64::{_MM_SHUFFLE, _mm_shuffle_ps};
use std::f32::consts::PI;
use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::constants::*;
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

pub unsafe fn cross_f32_type(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
    // Vec3(y * vector.z - z * vector.y, z * vector.x - x * vector.z, x * vector.y - y * a_Vector.x);
    let arg1_yzx = _mm_shuffle_ps(arg1.to_owned(),arg1.to_owned(),_MM_SHUFFLE(3,0,2,1));
    let arg2_yzx = _mm_shuffle_ps(arg2.to_owned(),arg2.to_owned(),_MM_SHUFFLE(3, 0, 2, 1));
    let partial = sub(mul(arg1.to_owned(),arg2_yzx),mul(arg1_yzx,arg2.to_owned()));
    return _mm_shuffle_ps(partial, partial, _MM_SHUFFLE(3, 0, 2, 1));

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

pub unsafe fn compute_sinx_cosx(x:&FloatArgType,mut sinx :&FloatArgType,mut cosx :&FloatArgType){
    let x2 = mul(x.to_owned(),x.to_owned());
    let x3 = mul(x2.to_owned(),x.to_owned());
    sinx = madd(x2,
                madd(x2,
                     madd(x2,
                          fast_load_constant_f32(G_SIN_COEF1 as *const f32),
                          fast_load_constant_f32(G_SIN_COEF2 as *const f32)),fast_load_constant_f32(G_SIN_COEF3 as *const f32)),x.to_owned()).borrow();
    cosx =  madd(x2,
                 madd(x2,
                      madd(x2,
                           fast_load_constant_f32(G_COS_COEF1 as *const f32),
                           fast_load_constant_f32(G_COS_COEF2 as *const f32)),fast_load_constant_f32(G_COS_COEF3 as *const f32)),splat(1.0)).borrow();

}
pub unsafe fn sin(value:FloatArgType)->FloatType{
    let mut x = mul(value,fast_load_constant_f32(G_TWO_OVER_PI as (*const f32)));
    let intx = convert_to_int_nearest(x);
    let offset = and_i32(intx,splat_i32(3));
    let intx_float = convert_to_float(intx);
    x = sub(value,mul(intx_float,fast_load_constant_f32(G_HALF_PI as *const f32)));
    let sinx : FloatType;
    let cosx : FloatType;
    compute_sinx_cosx(x.borrow(),sinx.borrow(),cosx.borrow());
    let mut mask = cmp_eq_i32(and_i32(offset,splat_i32(1)),zero_int());
    let mut result = select(sinx.to_owned(),cosx.to_owned(),cast_to_float(mask));
    mask = cmp_eq_i32(and_i32(offset,splat_i32(2)),zero_int());
    result = select(result,xor(result.to_owned(),splat(-0.0)),cast_to_float(mask));
    result
}

pub unsafe fn cos(value:FloatArgType)->FloatType{
    let mut x = mul(value,fast_load_constant_f32(G_TWO_OVER_PI as *const f32));
    let intx = convert_to_int_nearest(x);
    let offset = and_i32(and_i32(intx,splat_i32(1)),splat_i32(3));
    let intx_float = convert_to_float(intx);

    x = sub(value,mul(intx_float, fast_load_constant_f32(G_HALF_PI as *const f32)));
    let sinx:FloatType;
    let cosx:FloatType;
    compute_sinx_cosx(x.borrow(),sinx.borrow(),cosx.borrow());
    let mut mask = cmp_eq_i32(and_i32(offset,splat_i32(1)),zero_int());
    let mut result = select(sinx.to_owned(),cosx.to_owned(),cast_to_float(mask));
    mask = cmp_eq_i32(and_i32(offset,splat_i32(2)),zero_int());
    result = select(result,xor(result,splat(0.0)),cast_to_float(mask));
    return result;
}
pub unsafe fn sin_cos(angle:FloatArgType) ->FloatType{
    let angle_offset = load_immediate(0.0, HALF_PI, 0.0, 0.0);
    let angles = add(from_vec_first(angle),angle_offset);
    return sin(angles)
}

pub unsafe fn min_f32(left:&f32,right:&f32)->f32{
    if left > right{
        let result = right.to_owned();
        return  result;
    }
    let result = left.to_owned();
    return result;
}
pub unsafe fn max_f32(left:&f32,right:&f32)->f32{
    if left > right{
        let result = left.to_owned();
        return  result;
    }
    let result = right.to_owned();
    return result;
}