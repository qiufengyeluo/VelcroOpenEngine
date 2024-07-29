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
pub unsafe fn acos(value:&FloatArgType) ->FloatType{
    let  xabs = abs(value.to_owned());
    let xabs2 = mul(xabs,xabs);
    let xabs4 = mul(xabs2,xabs2);
    let t1 = sqrt(sub(splat(1.0),xabs));
    let select_val = cmp_lt(value.to_owned(),zero_float());

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

pub unsafe fn atan(value:&FloatArgType)->FloatType{
    let mut x = value.to_owned();
    let signbit = and(x,cast_to_float(fast_load_constant_i32(G_NEGATE_MASK as *const i32)));
    let xabs = abs(x);

    let cmp0 = cmp_gt(xabs,fast_load_constant_f32(G_ATAN_HI_RANGE as *const f32));
    let mut cmp1 = cmp_gt(xabs,fast_load_constant_f32(G_ATAN_LO_RANGE as *const f32));
    let cmp2 = and_not(cmp0,cmp1);

    let xabs_safe = add(xabs, and(cmp_eq(xabs, zero_float()), fast_load_constant_f32(G_VEC1111 as *const f32)));
    let y0 = and(cmp0,fast_load_constant_f32(G_HALF_PI as *const f32));

    let mut x0 = div(fast_load_constant_f32(G_VEC1111 as *const f32), xabs_safe);
    x0 = xor(x0,cast_to_float(fast_load_constant_i32(G_NEGATE_MASK as *const i32)));
    let y1 = and(cmp2,fast_load_constant_f32(G_QUARTER_PI as *const f32));
    let x1_numer = sub(xabs,fast_load_constant_f32(G_VEC1111 as *const f32));
    let x1_denom = add(xabs,fast_load_constant_f32(G_VEC1111 as *const f32));
    let x1 = div(x1_numer,x1_denom);
    let mut x2 = and(cmp2,x1);
    x0 = add(cmp0,x0);
    x2 = or(x2,x0);
    cmp1 = or(cmp0, cmp2);

    x2 = and(cmp1, x2);
    x = and_not(cmp1, xabs);
    x = or(x2, x);

    let mut y = or(y0,y1);
    let x_sqr = mul(x,x);
    let x_cub = mul(x_sqr,x);
    let result = madd(x_cub,
    madd(x_sqr,
    madd(x_sqr,
    madd(x_sqr,
    fast_load_constant_f32(G_ATAN_COEF1 as *const f32),
    fast_load_constant_f32(G_ATAN_COEF2 as *const f32)),
    fast_load_constant_f32(G_ATAN_COEF3 as *const f32)),
    fast_load_constant_f32(G_ATAN_COEF4 as *const f32)),
    x);
    y = add(y, result);

    y = xor(y, signbit);
    y
}

pub unsafe fn atan2(y:&FloatArgType,x:&FloatArgType)->FloatType{
    let x_eq_0 = cmp_eq(x.to_owned(),zero_float());
    let x_ge_0 = cmp_gt_eq(x.to_owned(),zero_float());
    let x_lt_0 = cmp_lt(x.to_owned(),zero_float());

    let y_eq_0 = cmp_eq(y.to_owned(),zero_float());
    let y_lt_0 = cmp_lt(y.to_owned(),zero_float());

    let zero_mask = and(x_ge_0,y_eq_0);

    let pio2_mask = and_not(y_eq_0,x_eq_0);
    let pio2_mask_sign = and(y_lt_0,cast_to_float(fast_load_constant_i32(G_NEGATE_MASK as *const i32)));
    let mut pio2_result = fast_load_constant_f32(G_HALF_PI as *const f32);
    pio2_result = xor(pio2_result, pio2_mask_sign);
    pio2_result = and(pio2_mask, pio2_result);

    let pi_mask = and(y_eq_0, x_lt_0);
    let mut pi_result = fast_load_constant_f32(G_PI as *const f32);
    pi_result = and(pi_mask, pi_result);

    let mut swap_sign_mask_offset = and(x_lt_0, y_lt_0);
    swap_sign_mask_offset = and(swap_sign_mask_offset, cast_to_float(fast_load_constant_i32(G_NEGATE_MASK as *const i32)));

    let mut offset1 = fast_load_constant_f32(G_PI as *const f32);
    offset1 = xor(offset1, swap_sign_mask_offset);

    let offset = and(x_lt_0, offset1);
    let x_safe = add(x.to_owned(), and(x_eq_0, fast_load_constant_f32(G_VEC1111 as *const f32)));
    let atan_mask = not(or(x_eq_0, y_eq_0));
    let atan_arg = div(y.to_owned(), x_safe);
    let mut atan_result = atan(atan_arg.borrow());
    atan_result = add(atan_result, offset);
    atan_result = and_not(pio2_mask, atan_result);
    atan_result = and(atan_mask, atan_result);

    let mut result = and_not(zero_mask, pio2_result);
    result = or(result, pio2_result);
    result = or(result, pi_result);
    result = or(result, atan_result);

    result
}
pub unsafe fn wrap(value:&FloatArgType, min_value:&FloatArgType, max_value:&FloatArgType) ->FloatType {
    let value_adjust = sub(value.to_owned(), min_value.to_owned());
    let max_adjust = sub(max_value.to_owned(), min_value.to_owned());
    let value_offset = select(max_adjust, zero_float(), cmp_lt(value_adjust, zero_float()));
    return add(min_value.to_owned(), add(value_offset.to_owned(), mod_calculate(value_adjust, max_adjust)));
}

pub unsafe fn angle_mod(value:&FloatArgType)->FloatType {
    let vec_pi = splat(PI);
    let vec_two_pi = splat(TWO_PI);
    let positive_angles = sub(mod_calculate(add(value.to_owned(), vec_pi), vec_two_pi), vec_pi);
    let negative_angles = add(mod_calculate(sub(value.to_owned(), vec_pi), vec_two_pi), vec_pi);
    let mask = cmp_gt_eq(value.to_owned(),zero_float());
    return select(positive_angles, negative_angles, mask);
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
pub unsafe fn sin_cos_float_type(angle:FloatArgType) ->FloatType{
    return sin(add(from_vec_first(angle),load_immediate(0.0, HALF_PI, 0.0, 0.0)))
}

pub unsafe fn  sin_cos(value:FloatArgType,mut sin:&FloatArgType,mut cos:&FloatArgType){
    let mut x = mul(value,fast_load_constant_f32(G_TWO_OVER_PI as *const f32));

    let intx = convert_to_int_nearest(x);
    let offset_sin = and_i32(intx, splat_i32(3));
    let offset_cos = add_i32(add_i32(intx, splat_i32(1)), splat_i32(3));
    let intx_float = convert_to_float(intx);
    x = sub(value,mul(intx_float, fast_load_constant_f32(G_HALF_PI as *const f32)));
    let mut sinx:FloatType = zero_float();
    let mut cosx:FloatType = zero_float();
    compute_sinx_cosx(x.borrow(),sinx.borrow(),cosx.borrow());
    let mut sin_mask = cast_to_float(cmp_eq_i32(and_i32(offset_sin, splat_i32(1)), zero_int()));
    let mut cos_mask = cast_to_float(cmp_eq_i32(and_i32(offset_cos, splat_i32(1)), zero_int()));
    sin = select(sinx.to_owned(), cosx.to_owned(), sin_mask.to_owned()).borrow();
    cos = select(sinx.to_owned(), cosx.to_owned(), cos_mask.to_owned()).borrow();
    sin_mask = cast_to_float(cmp_eq_i32(and_i32(offset_sin,splat_i32(2)),zero_int()));
    cos_mask = cast_to_float(cmp_eq_i32(and_i32(offset_cos,splat_i32(2)),zero_int()));
    sin = select(sin.to_owned(),xor(sin.to_owned(),fast_load_constant_f32(G_NEGATE_MASK as *const f32)),sin_mask).borrow();
    cos = select(cos.to_owned(),xor(cos.to_owned(),fast_load_constant_f32(G_NEGATE_MASK as *const f32)),cos_mask).borrow();

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
