
#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::common_sse::{Vec2Type, VecTwoType};
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
use crate::math::common_sse::VecType;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
use crate::math::simd_math_vec1_sse::Vec1;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
use crate::math::simd_math_vec2_sse::Vec2;

#[inline]
pub unsafe  fn simd_abs(value:&f32)->f32{
    return Vec1::select_index0(Vec1::abs(Vec1::splat(value).borrow()).borrow())
}

#[inline]
pub unsafe  fn simd_mod_calculate(value:&f32,divisor:&f32)->f32{
    return Vec1::select_index0(Vec1::mod_calculate(Vec1::splat(value.borrow()).borrow(),Vec1::splat(divisor).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_wrap2(value:&f32, max_value:&f32) ->f32{
    return Vec1::select_index0(Vec1::wrap(Vec1::splat(value).borrow(),Vec1::zero_float().borrow(),Vec1::splat(max_value).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_wrap3(value:&f32, min_value:&f32, max_value:&f32) ->f32{
    return Vec1::select_index0(Vec1::wrap(Vec1::splat(value).borrow(), Vec1::splat(min_value).borrow(), Vec1::splat(max_value).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_angle_mod(value:&f32)->f32{
    return Vec1::select_index0(Vec1::angle_mod(Vec1::splat(value).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_sin_cos(angle:&f32,mut sin:&f32,mut cos:&f32){
    let values = Vec2::sin_cos_to_float_type(Vec1::splat(angle).borrow());
    sin = Vec2::select_index0(values.borrow()).borrow_mut();
    cos = Vec2::select_index1(values.borrow()).borrow_mut();
}

#[inline]
pub unsafe  fn simd_sin(angle:&f32)->f32{
    return Vec1::select_index0(Vec1::sin(Vec1::splat(angle).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_cos(angle:&f32)->f32{
    return Vec1::select_index0(Vec1::cos(Vec1::splat(angle).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_acos(value:&f32)->f32{
    return Vec1::select_index0(Vec1::acos(Vec1::splat(value).borrow()).borrow())
}

#[inline]
pub unsafe  fn simd_atan(value:&f32)->f32{
    return Vec1::select_index0(Vec1::atan(Vec1::splat(value).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_atan2(y:&f32,x:&f32)->f32{
    return Vec1::select_index0(Vec1::atan2(Vec1::splat(y).borrow(),Vec1::splat(x).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_sqrt(value:&f32)->f32{
    return Vec1::select_index0(Vec1::sqrt(Vec1::splat(value).borrow()).borrow());
}

#[inline]
pub unsafe  fn simd_inv_sqrt(value:&f32)->f32{
    return Vec1::select_index0(Vec1::sqrt_inv(Vec1::splat(value).borrow()).borrow());
}