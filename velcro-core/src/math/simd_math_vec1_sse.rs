#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use std::arch::x86_64::*;

use crate::math::common_sse::*;
use crate::math::vsimd::*;

pub struct Vec1 {

}

impl VecType for Vec1 {
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn load_aligned(addr :*f32)->FloatType{
        unsafe { return _mm_load_ps1(addr); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn load_aligned_i128(addr :*const Int32Type)->Int32Type{
        unsafe { return sse::load_aligned_i128(addr); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn load_unaligned(addr:&f32)->FloatType{
        unsafe { return _mm_load_ps1(addr); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn load_unaligned_i128(addr:*const Int32Type)->Int32Type{
        unsafe { return sse::load_unaligned_i128(addr); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn store_aligned( addr:*mut f32,value:FloatArgType){
        unsafe { _mm_store_ss(addr, value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn store_aligned_i128(addr :*mut Int32Type,value:Int32ArgType){
        unsafe { sse::store_aligned_i128(addr as *mut Int32ArgType, value.to_owned()) }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn store_unaligned(addr :*mut f32,value:FloatArgType){
        unsafe { _mm_store_ss(addr, value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn store_unaligned_i128(addr:*mut Int32Type,value:Int32ArgType){
        unsafe { sse::store_unaligned_i128(addr as *mut Int32Type, value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn stream_aligned(addr :*mut f32,value:FloatArgType){
        unsafe { sse::stream_aligned(addr, value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn stream_aligned_i128(addr:*mut Int32Type,value:Int32ArgType){
        unsafe { sse::stream_aligned_i128(addr as *mut Int32Type, value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn select_index0(value:FloatArgType)->f32{
        unsafe { return sse::select_first(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn splat(value:f32)->FloatType{
        unsafe { return _mm_set_ps1(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn splat_i32(value:&i32)->Int32Type{
        unsafe { return sse::splat_i32(value.to_owned()); }
    }


    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn add(arg1:FloatArgType,arg2:FloatArgType)->FloatType{
        unsafe { return sse::add(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sub(arg1:FloatArgType,arg2:FloatArgType)->FloatType{
         unsafe { return sse::sub(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn mul(arg1:FloatArgType,arg2:FloatArgType)->FloatType{
        unsafe { return sse::mul(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn madd(mul1:FloatArgType,mul2:FloatArgType,add:FloatArgType)->FloatType{
        unsafe { return sse::madd(mul1.to_owned(), mul2.to_owned(), add.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe  fn div(arg1:&FloatType, arg2: &mut FloatType) ->FloatType{
        let ones = sse::splat(1.0);
        *arg2 = sse::replace_first(ones.to_owned(),arg2.to_owned());
        return sse::div(arg1.to_owned(),arg2.to_owned())
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn abs(value:FloatArgType)->FloatType{
        unsafe { return sse::abs(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn add_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type{
        unsafe { return sse::add_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sub_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type{
        unsafe { return sse::sub_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn mul_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type{
        unsafe { return sse::mul_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn madd_i32(mul1:Int32ArgType,mul2:Int32ArgType,add:Int32ArgType)->Int32Type{
        unsafe { return sse::madd_i32(mul1.to_owned(), mul2.to_owned(), add.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn abs_i32(value:Int32ArgType)->Int32Type{
        unsafe { return sse::abs_i32(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn not(value:FloatArgType)->FloatType{
        unsafe { return sse::not(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn and(arg1:FloatArgType,arg2:FloatArgType)->FloatType{
        unsafe { return sse::and(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn and_not(arg1:FloatArgType,arg2:FloatArgType)->FloatType{
        unsafe { return sse::and_not(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn or(arg1:FloatArgType,arg2:FloatArgType)->FloatType{
        unsafe { return sse::or(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn xor(arg1:FloatArgType,arg2:FloatArgType)->FloatType{
        unsafe { return sse::xor(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn not_i32(value:Int32ArgType)->Int32Type{
        unsafe { return sse::not_i32(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn and_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type{
        unsafe { return sse::and_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn or_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type{
        unsafe { return sse::or_i32(arg1.to_owned(), arg2.to_owned()); }
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn xor_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type{
        unsafe { return sse::xor_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn and_not_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type{
        unsafe { return sse::and_not_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn floor(value:FloatArgType)->FloatType{
        unsafe { return sse::floor(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn ceil(value:FloatArgType)->FloatType{
        unsafe { return sse::ceil(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn round(value:FloatArgType)->FloatType{
        unsafe { return sse::round(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn truncate(value:FloatArgType) ->FloatType{
        unsafe { return sse::truncate(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn min(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::min(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn max(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::max(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn clamp(value:FloatArgType,min:FloatArgType,max:FloatArgType) ->FloatType{
        unsafe { return sse::clamp(value.to_owned(), min.to_owned(), max.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn min_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::min_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn max_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::max_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn clamp_i32(value:Int32ArgType,min:Int32ArgType,max:Int32ArgType) ->Int32Type{
        unsafe { return sse::clamp_i32(value.to_owned(), min.to_owned(), max.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_eq(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::cmp_eq(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_neq(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::cmp_neq(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_gt(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::cmp_gt(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_gt_eq(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::cmp_gt_eq(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_lt(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::cmp_lt(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_lt_eq(arg1:FloatArgType,arg2:FloatArgType) ->FloatType{
        unsafe { return sse::cmp_lt_eq(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_all_eq(arg1:FloatArgType,arg2:FloatArgType) ->bool{
        unsafe { return sse::cmp_all_eq(arg1.to_owned(), arg2.to_owned(), 0b0001); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_all_lt(arg1:FloatArgType,arg2:FloatArgType) ->bool{
        unsafe { return sse::cmp_all_lt(arg1.to_owned(), arg2.to_owned(), 0b0001); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_all_lt_eq(arg1:FloatArgType,arg2:FloatArgType) -> bool {
        unsafe { return sse::cmp_all_lt_eq(arg1.to_owned(), arg2.to_owned(), 0b0001); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_all_gt(arg1:FloatArgType,arg2:FloatArgType) ->bool{
        unsafe { return sse::cmp_all_gt(arg1.to_owned(), arg2.to_owned(), 0b0001); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_all_gt_eq(arg1:FloatArgType,arg2:FloatArgType) ->bool{
        unsafe { return sse::cmp_all_gt_eq(arg1.to_owned(), arg2.to_owned(), 0b0001); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::cmp_eq_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_neq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::cmp_neq_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_gt_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::cmp_gt_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_gt_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::cmp_gt_eq_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_lt_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::cmp_lt_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_lt_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type{
        unsafe { return sse::cmp_lt_eq_i32(arg1.to_owned(), arg2.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cmp_all_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->bool{
        unsafe { return sse::cmp_all_eq_i32(arg1.to_owned(), arg2.to_owned(), 0b0001); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn select(arg1:FloatArgType,arg2:FloatArgType,mask:FloatArgType)->FloatType{
        unsafe { return sse::select(arg1.to_owned(), arg2.to_owned(), mask.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn select_i32(arg1:Int32ArgType,arg2:Int32ArgType,mask:Int32ArgType)->Int32Type{
        unsafe { return sse::select_i32(arg1.to_owned(), arg2.to_owned(), mask.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn reciprocal(value:FloatArgType)->FloatType{
        let ones = unsafe { sse::splat(1.0) };
        unsafe { return sse::reciprocal(sse::replace_first(ones, value.to_owned())); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn reciprocal_estimate(value:FloatArgType)->FloatType{
        let ones = unsafe { sse::splat(1.0) };
        unsafe { return sse::reciprocal_estimate(sse::replace_first(ones, value.to_owned())); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn mod_calculate(value:FloatArgType,divisor:FloatArgType)->FloatType{
        unsafe { return sse::mod_calculate(value.to_owned(), divisor.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn  wrap(value:FloatArgType, min_value:FloatArgType, max_value:FloatArgType) ->FloatType{
        return Common::wrap(value, min_value, max_value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn angle_mod(value:FloatArgType) ->FloatType{
        return  Common::angle_mod(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sqrt(value:FloatArgType)->FloatType{
        unsafe { return sse::sqrt(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sqrt_estimate(value:FloatArgType)->FloatType{
        unsafe { return sse::sqrt_estimate(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sqrt_inv(value:FloatArgType)->FloatType{
        unsafe { return sse::sqrt_inv(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sqrt_inv_estimate(value:FloatArgType) ->FloatType{
        unsafe { return sse::sqrt_inv_estimate(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sin(value:FloatArgType)->FloatType{
        return Common::sin(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cos(value:FloatArgType)->FloatType{
        return Common::cos(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn sin_cos(value:FloatArgType,mut sin:&FloatType,mut cos:&FloatType){
        Common::sin_cos(value,sin,cos)
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn acos(value:FloatArgType)->FloatType{
        return Common::acos(value);
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn atan(value:FloatArgType) ->FloatType{
        return Common::atan(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn atan2(y:FloatArgType,x:FloatArgType) ->FloatType{
        return Common::atan2(y,x);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn exp_estimate(x:FloatArgType)->FloatType{
        return Common::exp_estimate(x);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn convert_to_float(value:Int32ArgType)->FloatType{
        unsafe { return sse::convert_to_float(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn convert_to_int(value:FloatArgType)->Int32Type{
        unsafe { return sse::convert_to_int(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn convert_to_int_nearest(value:FloatArgType)->Int32Type{
        unsafe { return sse::convert_to_int_nearest(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cast_to_float(value:Int32ArgType)->FloatType{
        unsafe { return sse::cast_to_float(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn cast_to_int(value:FloatArgType)->Int32Type{
        unsafe { return sse::cast_to_int(value.to_owned()); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn zero_float() ->FloatType{
        unsafe { return sse::zero_float(); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn zero_int() ->Int32Type{
        unsafe { return sse::zero_int(); }
    }
}

impl Vec1Type for  Vec1 {
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn load_immediate(x:&f32)->FloatType{
        unsafe { return sse::load_immediate(x.to_owned(), 0.0, 0.0, 0.0); }
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     fn load_immediate_i32(x:&i32)->Int32Type{
        unsafe { return sse::load_immediate_i32(x.to_owned(), 0, 0, 0); }
    }
}
