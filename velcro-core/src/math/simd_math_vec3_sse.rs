#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::arch::x86_64::{_mm_set_ps1, _mm_shuffle_ps};

use crate::math::common_sse::{Common, Vec2Type, Vec3Type, VecThirdType, VecTwoType, VecType};
use crate::math::simd_math_vec2_sse::Vec2;
use crate::math::vsimd::{FloatArgType, FloatType, Int32ArgType, Int32Type, sse};

pub struct Vec3{

}
impl VecType for Vec3 {

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
    unsafe fn load_unaligned(addr:&f32)->FloatType{
        return  sse::load_unaligned(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn load_unaligned_i128(addr:*const Int32Type)->Int32Type{
        return sse::load_unaligned_i128(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn store_aligned( addr:*mut f32,value:&FloatArgType){
        sse::store_aligned(addr,value.to_owned())
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
        sse::store_unaligned(addr,value.to_owned());
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
        return _mm_set_ps1(value.to_owned());
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
    unsafe fn add(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
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
    unsafe fn madd(mul1:&FloatArgType,mul2:&FloatArgType,add:&FloatArgType)->FloatType{
        return sse::madd(mul1.to_owned(),mul2.to_owned(),add.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn div(arg1:&FloatArgType, arg2: &mut FloatArgType) ->FloatType{
        *arg2 = sse::replace_fourth_f32(arg2.to_owned(),1.0);
        return sse::div(arg1.to_owned(),arg2.to_owned())
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
    unsafe fn madd_i32(mul1:&Int32ArgType,mul2:Int32ArgType,add:&Int32ArgType)->Int32Type{
        return sse::madd_i32(mul1.to_owned(),mul2.to_owned(),add.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn abs_i32(value:&Int32ArgType)->Int32Type{
        return sse::abs_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn not(value:&FloatArgType)->FloatType{
        return  sse::not(value.to_owned());
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
    unsafe fn and_not_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::and_not_i32(arg1.to_owned(),arg2.to_owned());
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
        return sse::cmp_all_eq(arg1.to_owned(),arg2.to_owned(),0b0111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cmp_all_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_lt(arg1.to_owned(),arg2.to_owned(),0b0111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cmp_all_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) -> bool {
        return sse::cmp_all_lt_eq(arg1.to_owned(),arg2.to_owned(),0b0111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cmp_all_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt(arg1.to_owned(),arg2.to_owned(),0b0111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cmp_all_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt_eq(arg1.to_owned(),arg2.to_owned(),0b0111);
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
        return sse::cmp_all_eq_i32(arg1.to_owned(),arg2.to_owned(),0b0111);
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
        let  val = sse::replace_fourth_f32(sse::replace_fourth_f32(value.to_owned(),1.0),1.0);
        return sse::reciprocal(val);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn reciprocal_estimate(value:&FloatArgType)->FloatType{
        let val = sse::replace_fourth_f32(sse::replace_third_f32(value.to_owned(),1.0),1.0);
        return sse::reciprocal_estimate(val);
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
        let val = sse::replace_fourth_f32(value.to_owned(),1.0);
        return sse::sqrt_inv(val);
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
    unsafe fn sin_cos(value:&FloatArgType,mut sin:&FloatType,mut cos:&FloatType){
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
    unsafe fn exp_estimate(x:&FloatArgType)->FloatType{
        return Common::exp_estimate(x);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn convert_to_float(value:&Int32ArgType)->FloatType{
        return sse::convert_to_float(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn convert_to_int(value:&FloatArgType)->Int32Type{
        return sse::convert_to_int(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn convert_to_int_nearest(value:&FloatArgType)->Int32Type{
        return sse::convert_to_int_nearest(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cast_to_float(value:&Int32ArgType)->FloatType{
        return sse::cast_to_float(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cast_to_int(value:&FloatArgType)->Int32Type{
        return sse::cast_to_int(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn zero_float() ->FloatType{
        return sse::zero_float();
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn zero_int() ->Int32Type{
        return sse::zero_int();
    }
}

impl VecTwoType for Vec3 {
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
    unsafe fn select_index1(value:&FloatArgType)->f32{
        return sse::select_first(sse::splat_second(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn splat_index0(value:&FloatArgType)->FloatType{
        return sse::splat_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn splat_index1(value:&FloatArgType)->FloatType{
        return sse::splat_second(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index0_f32(a:&FloatArgType,b:&f32)->FloatType{
        return sse::replace_first_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index0(a:&FloatArgType,b:&FloatArgType)->FloatType{
        return sse::replace_first(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index1_f32(a:&FloatArgType,b:&f32)->FloatType{
        return sse::replace_second_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index1(a:&FloatArgType,b:&FloatArgType)->FloatType{
        return sse::replace_second(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn dot(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        let x2= Vec3::mul(arg1,arg2);
        let xy = Vec3::add(Vec3::splat_index1(x2.borrow()).borrow(),x2.borrow());
        let xyz = Vec3::add(Vec3::splat_index2(x2.borrow()).borrow(),xy.borrow());
        return Vec2::splat_index0(xyz.borrow());
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
    unsafe fn normalize_estimate(value:&FloatArgType)->FloatType{
        return Common::normalize_estimate(value);
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn normalize_safe(value:&FloatArgType,tolerance:&f32)->FloatType{
        return Common::normalize_safe(value,tolerance);
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn normalize_safe_estimate(value:&FloatArgType,tolerance:&f32)->FloatType{
        return Common::normalize_safe_estimate(value,tolerance);
    }
}

impl VecThirdType for Vec3{

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn value_to_vec2(value:&FloatArgType)->FloatType{
        return value.to_owned();
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn from_vec2(value:&FloatArgType)->FloatType{
        return sse::replace_third_f32(value.to_owned(),0.0);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn select_index2(value:&FloatArgType)->f32{
        return sse::select_first(sse::splat_third(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn splat_index2(value:&FloatArgType)->FloatType{
        return sse::splat_third(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index2_f32(a:&FloatArgType,b:&f32)->FloatType{
        return sse::replace_third_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn replace_index2(a:&FloatArgType,b:&FloatArgType)->FloatType{
        return sse::replace_third(a.to_owned(),b.to_owned());
    }

}
impl Vec3Type for Vec3 {
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn load_immediate(x:&f32,y:&f32,z:&f32)->FloatType{
        return sse::load_immediate(x.to_owned(),y.to_owned(),z.to_owned(),0.0);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn load_immediate_i32(x:&i32,y:&i32,z:&i32)->Int32Type{
        return sse::load_immediate_i32(x.to_owned(),y.to_owned(),z.to_owned(),0);
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn cross(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        let arg1_yzx = _mm_shuffle_ps(arg1.to_owned(), arg1.to_owned(), _MM_SHUFFLE(3, 0, 2, 1));
        let arg2_yzx = _mm_shuffle_ps(arg2.to_owned(), arg2.to_owned(), _MM_SHUFFLE(3, 0, 2, 1));
        let partial= Vec3::sub(Vec3::mul(arg1,arg2_yzx.borrow()).borrow(),Vec3::mul(arg1_yzx.borrow(),arg2).borrow());
        return _mm_shuffle_ps(partial, partial, _MM_SHUFFLE(3, 0, 2, 1));
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    unsafe fn mat3x3inverse(rows:*const FloatType,mut out :*const FloatType){
        const FloatType row0YZX = _mm_shuffle_ps(rows[0], rows[0], _MM_SHUFFLE(0, 0, 2, 1));
        const FloatType row0ZXY = _mm_shuffle_ps(rows[0], rows[0], _MM_SHUFFLE(0, 1, 0, 2));
        const FloatType row1YZX = _mm_shuffle_ps(rows[1], rows[1], _MM_SHUFFLE(0, 0, 2, 1));
        const FloatType row1ZXY = _mm_shuffle_ps(rows[1], rows[1], _MM_SHUFFLE(0, 1, 0, 2));
        const FloatType row2YZX = _mm_shuffle_ps(rows[2], rows[2], _MM_SHUFFLE(0, 0, 2, 1));
        const FloatType row2ZXY = _mm_shuffle_ps(rows[2], rows[2], _MM_SHUFFLE(0, 1, 0, 2));

        FloatType cols[3] = { Sub(Mul(row1YZX, row2ZXY), Mul(row1ZXY, row2YZX))
            , Sub(Mul(row2YZX, row0ZXY), Mul(row2ZXY, row0YZX))
            , Sub(Mul(row0YZX, row1ZXY), Mul(row0ZXY, row1YZX)) };

        const FloatType detXYZ = Mul(rows[0], cols[0]);
        const FloatType detTmp = Add(detXYZ, _mm_shuffle_ps(detXYZ, detXYZ, _MM_SHUFFLE(0, 1, 0, 1)));
        const FloatType det    = Add(detTmp, _mm_shuffle_ps(detXYZ, detXYZ, _MM_SHUFFLE(0, 0, 2, 2)));

        Mat3x3Transpose(cols, cols);

        out[0] = Div(cols[0], det);
        out[1] = Div(cols[1], det);
        out[2] = Div(cols[2], det);
    }
}