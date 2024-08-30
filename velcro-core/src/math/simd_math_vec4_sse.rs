#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::arch::x86_64::{_mm_hadd_ps, _mm_shuffle_ps};

use crate::math::common_sse::*;
use crate::math::constants::{G_NEGATE_MASK, G_VEC1111, HALF_PI};
use crate::math::math_utils::constants;
use crate::math::vsimd::*;

pub struct Vec4{
    
}

impl VecType for  Vec4 {

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn load_aligned(addr :*f32)->FloatType{
        return  sse::load_aligned(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn load_aligned_i128(addr :*const Int32Type)->Int32Type{
        return sse::load_aligned_i128(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn load_unaligned(addr:*const f32)->FloatType{
        return  sse::load_unaligned(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn load_unaligned_i128(addr:*const Int32Type)->Int32Type{
        return sse::load_aligned_i128(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn store_aligned( addr:*mut f32,value:&FloatArgType){
        sse::store_aligned(addr, value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn store_aligned_i128(addr :*mut Int32Type,value:&Int32ArgType){
        sse::store_aligned_i128(addr as *mut Int32ArgType,value.to_owned())
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn store_unaligned(addr :*mut f32,value:&FloatArgType){
        sse::store_unaligned(addr, value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn store_unaligned_i128(addr:*mut Int32Type,value:&Int32ArgType){
        sse::store_unaligned_i128(addr as *mut Int32Type,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn stream_aligned(addr :*mut f32,value:&FloatArgType){
        sse::stream_aligned(addr,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn stream_aligned_i128(addr:*mut Int32Type,value:&Int32ArgType){
        sse::stream_aligned_i128(addr as *mut Int32Type,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn select_index0(value:&FloatArgType)->f32{
        return sse::select_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn splat(value:&f32)->FloatType{
        return sse::splat(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn splat_i32(value:&i32)->Int32Type{
        return sse::splat_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn add(arg1:FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::add(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sub(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::sub(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn mul(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::mul(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn madd(mul1:&FloatArgType, mul2:&FloatArgType,add:&FloatArgType) ->FloatType{
        return sse::madd(mul1.to_owned(),mul2.to_owned(),add.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn div(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::div(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn abs(value:&FloatArgType)->FloatType{
        return sse::abs(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn add_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::add_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sub_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::sub_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn mul_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::mul_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn madd_i32(mul1:&Int32ArgType,mul2:&Int32ArgType,add:&Int32ArgType)->Int32Type{
        return sse::madd_i32(mul1.to_owned(),mul2.to_owned(),add.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn abs_i32(value:&Int32ArgType)->Int32Type{
        return  sse::abs_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn not(value:&FloatArgType)->FloatType{
        return sse::not(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn and(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::and(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn and_not(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::and_not(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn or(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::or(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn xor(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::xor(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn not_i32(value:&Int32ArgType)->Int32Type{
        return sse::not_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn and_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::and_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn and_not_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::and_not_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn or_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::or_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn xor_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::xor_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn floor(value:&FloatArgType)->FloatType{
        return sse::floor(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn ceil(value:&FloatArgType)->FloatType{
        return sse::ceil(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn round(value:&FloatArgType)->FloatType{
        return sse::round(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn truncate(value:&FloatArgType) ->FloatType{
        return sse::truncate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn min(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::min(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn max(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::max(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn clamp(value:&FloatArgType,min:&FloatArgType,max:&FloatArgType) ->FloatType{
        return sse::clamp(value.to_owned(),min.to_owned(),max.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn min_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::min_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn max_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::max_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn clamp_i32(value:&Int32ArgType,min:&Int32ArgType,max:&Int32ArgType) ->Int32Type{
        return sse::clamp_i32(value.to_owned(),min.to_owned(),max.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_eq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_neq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_neq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_gt(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_gt_eq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_lt(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_lt_eq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_eq(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_lt(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_lt_eq(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt_eq(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_eq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_neq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_neq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_gt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_gt_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_gt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_gt_eq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_lt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_lt_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_lt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_lt_eq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->bool{
        return sse::cmp_all_eq_i32(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn select(arg1:&FloatArgType,arg2:&FloatArgType,mask:&FloatArgType)->FloatType{
        return  sse::select(arg1.to_owned(),arg2.to_owned(),mask.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn select_i32(arg1:&Int32ArgType,arg2:&Int32ArgType,mask:&Int32ArgType)->Int32Type{
        return  sse::select_i32(arg1.to_owned(),arg2.to_owned(),mask.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn reciprocal(value:&FloatArgType)->FloatType{
        return sse::reciprocal(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn reciprocal_estimate(value:&FloatArgType)->FloatType{
        return sse::reciprocal_estimate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn mod_calculate(value:&FloatArgType,divisor:&FloatArgType)->FloatType{
        return sse::mod_calculate(value.to_owned(),divisor.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn  wrap(value:&FloatArgType, min_value:&FloatArgType, max_value:&FloatArgType) ->FloatType{
        return Common::wrap(value, min_value, max_value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn angle_mod(value:&FloatArgType) ->FloatType{
        return  Common::angle_mod(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sqrt(value:&FloatArgType)->FloatType{
        return sse::sqrt(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sqrt_estimate(value:&FloatArgType)->FloatType{
        return sse::sqrt_estimate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sqrt_inv(value:&FloatArgType)->FloatType{
        return sse::sqrt_inv(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sqrt_inv_estimate(value:&FloatArgType) ->FloatType{
        return sse::sqrt_inv_estimate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sin(value:&FloatArgType)->FloatType{
        return Common::sin(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cos(value:&FloatArgType)->FloatType{
        return Common::cos(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn sin_cos(value:FloatArgType,mut sin:&FloatType,mut cos:&FloatType){
        Common::sin_cos(value,sin,cos)
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn acos(value:&FloatArgType)->FloatType{
        return Common::acos(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn atan(value:&FloatArgType) ->FloatType{
        return Common::atan(value);
    }


    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn atan2(y:&FloatArgType,x:&FloatArgType) ->FloatType{
        return Common::atan2(y,x);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn exp_estimate(value:&FloatArgType)->FloatType{
        return Common::exp_estimate(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn convert_to_float(value: &Int32ArgType) -> FloatType {
        return sse::convert_to_float(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn convert_to_int(value: &FloatArgType) -> Int32Type {
        return  sse::convert_to_int(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn convert_to_int_nearest(value: &FloatArgType) -> Int32Type {
        return sse::convert_to_int_nearest(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cast_to_float(value: &Int32ArgType) -> FloatType {
        return  sse::cast_to_float(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cast_to_int(value: &FloatArgType) -> Int32Type {
        return sse::cast_to_int(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn zero_float() -> FloatType {
        return  sse::zero_float();
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn zero_int() -> Int32Type {
        return sse::zero_int();
    }
}

impl VecTwoType for Vec4 {

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn value_to_vec1(value:&FloatArgType) ->FloatType{
        return value.to_owned();
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn from_vec1(value:&FloatArgType) ->FloatType{
        return sse::splat_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn select_index1(value:FloatArgType)->f32{
        return sse::select_first(sse::splat_second(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn splat_index0(value:&FloatArgType)->FloatType{
        return  sse::splat_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn splat_index1(value:&FloatArgType)->FloatType{
        return  sse::splat_second(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index0_f32(a:&FloatArgType,b:&f32) ->FloatType{
        return  sse::replace_first_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index0(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_first(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index1_f32(a:&FloatArgType, b:&f32) ->FloatType{
        return  sse::replace_second_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index1(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_second(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn dot(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        let x2 = Vec4::mul(arg1,arg2);
        let tmp = Vec4::add(x2.borrow(),_mm_shuffle_ps(x2, x2, _MM_SHUFFLE(2, 3, 0, 1)).borrow());
        return  Vec4::add(tmp.borrow(),_mm_shuffle_ps(tmp, tmp, _MM_SHUFFLE(1, 0, 2, 3)).borrow());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn normalize(value:&FloatArgType)->FloatType{
        return Common::normalize(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn normalize_estimate(value:&FloatArgType) ->FloatType{
        return Common::normalize_estimate(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn normalize_safe(value:&FloatArgType,tolerance:&f32) ->FloatType{
        return  Common::normalize_safe(value,tolerance);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn normalize_safe_estimate(value:&FloatArgType, tolerance:&f32) ->FloatType{
        return Common::normalize_safe_estimate(value,tolerance);
    }
}

impl VecThirdType for Vec4 {

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn value_to_vec2(value: &FloatArgType) -> FloatType {
        return  value.to_owned();
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn from_vec2(value:&FloatArgType) ->FloatType{
        return sse::replace_fourth_f32(sse::replace_third_f32(value.to_owned(),0.0) ,0.0);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn select_index2(value:FloatArgType)->f32{
        return sse::select_first(sse::splat_third(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn splat_index2(value:&FloatArgType)->FloatType{
        return  sse::splat_third(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index2_f32(a:&FloatArgType,b:&f32) ->FloatType{
        return  sse::replace_third_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index2(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_third(a.to_owned(),b.to_owned());
    }


}

impl VecFourthType for Vec4 {

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn value_to_vec3(value: &FloatArgType) -> FloatType {
       return  value.to_owned();
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn from_vec3(value: FloatArgType) -> FloatType {
        return sse::replace_fourth_f32(value.to_owned(), 0.0)
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn select_index3(value:FloatArgType)->f32{
        return sse::select_first(sse::splat_fourth(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn splat_index3(value:FloatArgType)->FloatType{
        return  sse::splat_fourth(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index3_f32(a:FloatArgType,b:f32) ->FloatType{
        return  sse::replace_fourth_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index3(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_fourth(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn quaternion_multiply(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        let flip_wsign = Common::fast_load_constant(G_NEGATE_MASK.borrow());
        let val1 = _mm_shuffle_ps(arg1.to_owned(), arg1.to_owned(), _MM_SHUFFLE(3, 0, 2, 1));
        let val2 = _mm_shuffle_ps(arg2.to_owned(), arg2.to_owned(), _MM_SHUFFLE(3, 1, 0, 2));
        let val3 = _mm_shuffle_ps(arg1.to_owned(), arg1.to_owned(), _MM_SHUFFLE(0, 1, 0, 2));
        let val4 = _mm_shuffle_ps(arg2.to_owned(), arg2.to_owned(), _MM_SHUFFLE(0, 0, 2, 1));
        let val5 = _mm_shuffle_ps(arg1.to_owned(), arg1.to_owned(), _MM_SHUFFLE(1, 3, 3, 3));
        let val6 = _mm_shuffle_ps(arg2.to_owned(), arg2.to_owned(), _MM_SHUFFLE(1, 2, 1, 0));
        let val7 = _mm_shuffle_ps(arg1.to_owned(), arg1.to_owned(), _MM_SHUFFLE(2, 2, 1, 0));
        let val8 = _mm_shuffle_ps(arg2.to_owned(), arg2.to_owned(), _MM_SHUFFLE(2, 3, 3, 3));
        let first_term = Vec4::mul(val1.borrow(), val2.borrow());
        let second_term = Vec4::mul(val3.borrow(), val4.borrow());
        let third_term = Vec4::mul(val5.borrow(), val6.borrow());
        let fourth_term = Vec4::sub(val7.borrow(), val8.borrow());
        let partial_one = Vec4::sub(first_term.borrow(), second_term.borrow());
        let partial_two = Vec4::xor(Vec4::xor(third_term.borrow(), fourth_term.borrow()).borrow(), flip_wsign.borrow());
        return Vec4::add(partial_one.borrow(), partial_two.borrow());

    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn quaternion_transform(quat:&FloatArgType,vec3:&FloatArgType)->FloatType{
        return Common::quaternion_transform(quat,vec3);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn construct_plane(normal:&FloatArgType,point:&FloatArgType)->FloatType{
        return Common::construct_plane(normal,point);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn plane_distance(plane:&FloatArgType,point:&FloatArgType)->FloatType{
        return Common::plane_distance(plane, point);
    }
}

impl Vec4Type for Vec4 {
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn load_immediate(x:f32,y:f32,z:f32,w:f32)->FloatType{
        return  sse::load_immediate(x.to_owned(),y.to_owned(),z.to_owned(),w.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn load_immediate_i32(x:&i32,y:&i32,z:&i32,w:&i32)->Int32Type{
        return  sse::load_immediate_i32(x.to_owned(),y.to_owned(),z.to_owned(),w.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn sin_cos_to_float_type(angles:FloatArgType)->FloatType{
        let angle_offset = Vec4::load_immediate_fourth_f32(0.0.borrow(),constants:: HALF_PI.borrow(), 0.0.borrow(), constants::HALF_PI.borrow());
        let sin_angles = Vec4::add(angles, angle_offset.borrow());
        return Vec4::sin(sin_angles.borrow());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat3x4inverse_fast(rows:*const FloatType,mut out:&*const FloatType){
        let pos = Vec4::sub(Vec4::zero_float().borrow(), Vec4::madd(*rows[0], Vec4::splat_index3(*rows[0]).borrow(), Vec4::madd(*rows[1], Vec4::splat_index3(*rows[1]).borrow(), Vec4::mul(*rows[2], Vec4::splat_index3(*rows[2]).borrow()).borrow()).borrow()).borrow());
        let tmp0 = _mm_shuffle_ps(rows[0], rows[1], 0x44);
        let tmp2 = _mm_shuffle_ps(rows[0], rows[1], 0xEE);
        let tmp1 = _mm_shuffle_ps(rows[2], pos, 0x44);
        let tmp3 = _mm_shuffle_ps(rows[2], pos, 0xEE);
        *out[0] = _mm_shuffle_ps(tmp0, tmp1, 0x88);
        *out[1] = _mm_shuffle_ps(tmp0, tmp1, 0xDD);
        *out[2] = _mm_shuffle_ps(tmp2, tmp3, 0x88);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat3x4transpose(rows:*const FloatType,mut out:&*const FloatType){
        let fourth = Common::fast_load_constant(G_VEC1111.borrow());
        let tmp0 = _mm_shuffle_ps(rows[0], rows[1], 0x44);
        let tmp2 = _mm_shuffle_ps(rows[0], rows[1], 0xEE);
        let tmp1 = _mm_shuffle_ps(rows[2], fourth, 0x44);
        let tmp3 = _mm_shuffle_ps(rows[2], fourth, 0xEE);
        *out[0] = _mm_shuffle_ps(tmp0, tmp1, 0x88);
        *out[1] = _mm_shuffle_ps(tmp0, tmp1, 0xDD);
        *out[2] = _mm_shuffle_ps(tmp2, tmp3, 0x88);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat3x4multiply(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        Common::mat3x4multiply(rows_a,rows_b,out);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4inverse_fast(rows:*const FloatType,mut out :&*const FloatType){
        Common::mat4x4inverse_fast(rows,out);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4transpose(rows:*const FloatType,mut out :&*const FloatType){
        let tmp0 = _mm_shuffle_ps(rows[0], rows[1], 0x44);
        let tmp2 = _mm_shuffle_ps(rows[0], rows[1], 0xEE);
        let tmp1 = _mm_shuffle_ps(rows[2], rows[3], 0x44);
        let tmp3 = _mm_shuffle_ps(rows[2], rows[3], 0xEE);
        *out[0] = _mm_shuffle_ps(tmp0, tmp1, 0x88);
        *out[1] = _mm_shuffle_ps(tmp0, tmp1, 0xDD);
        *out[2] = _mm_shuffle_ps(tmp2, tmp3, 0x88);
        *out[3] = _mm_shuffle_ps(tmp2, tmp3, 0xDD);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4multiply(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        Common::mat4x4multiply(rows_a,rows_b,out);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4multiply_add(rows_a:*const FloatType, rows_b:*const FloatType, add:*const FloatType, mut out:&*const FloatType){
        Common::mat4x4multiply_add(rows_a,rows_b,add,out);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4transpose_multiply(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        Common::mat4x4transpose_multiply(rows_a, rows_b, out);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4transform_vector(rows:*const FloatType,vector:&FloatArgType)->FloatType{
        let prod1 = Vec4::mul(*rows[0],vector);
        let prod2 = Vec4::mul(*rows[1],vector);
        let prod3 = Vec4::mul(*rows[2],vector);
        let prod4 = Vec4::mul(*rows[3],vector);
        return _mm_hadd_ps(_mm_hadd_ps(prod1, prod2), _mm_hadd_ps(prod3, prod4));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4_transpose_transform_vector(rows:*const FloatType,vector:&FloatArgType)->FloatType{
        return  Common::mat4x4transpose_transform_vector(rows,vector);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat4x4_transform_point3(rows:*const FloatType,vector:&FloatArgType)->FloatType{
        let vec_xyz = Vec4::replace_index3_f32(vector, 1.0.borrow());
        let prod1 = Vec4::mul(rows[0], vec_xyz.borrow());
        let prod2 = Vec4::mul(rows[1], vec_xyz.borrow());
        let prod3 = Vec4::mul(rows[2], vec_xyz.borrow());
        return  _mm_hadd_ps(_mm_hadd_ps(prod1, prod2), _mm_hadd_ps(prod3, Vec4::zero_float()));
    }

}