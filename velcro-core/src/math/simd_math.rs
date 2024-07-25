
#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]


use crate::math::vector::{acos, angle_mod, atan, atan2, cos, sin, sin_cos_float_type, wrap};
use crate::math::vsimd::*;

pub unsafe fn simd_abs(value:&f32) ->f32{
    return select_first(abs(splat(value.to_owned())))
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