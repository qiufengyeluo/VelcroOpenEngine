#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::arch::x86_64::{_mm_load_ps1, _mm_set_ps1, _mm_shuffle_ps, _mm_store_ss};
use crate::math::common_sse::Common;
use crate::math::constants::HALF_PI;
use crate::math::vsimd::*;

struct Vec4{
    
}

impl Vec4 {

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn from_vec1(value:&FloatArgType) ->FloatType{
        return sse::splat_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn from_vec2(value:&FloatArgType) ->FloatType{
        return sse::replace_fourth_f32(sse::replace_third_f32(value.to_owned(),0.0) ,0.0);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn from_vec3(value:&FloatArgType) ->FloatType{
        return sse::replace_fourth_f32(value.to_owned(),0.0);
    }
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_aligned(addr :*f32)->FloatType{
        return  sse::load_aligned(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_aligned_i128(addr :*const Int32Type)->Int32Type{
        return sse::load_aligned_i128(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_unaligned(addr:&f32)->FloatType{
        return  sse::load_unaligned(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_unaligned_i128(addr:*const Int32Type)->Int32Type{
        return sse::load_aligned_i128(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_aligned( addr:*mut f32,value:&FloatArgType){
        sse::store_aligned(addr, value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_aligned_i128(addr :*mut Int32Type,value:&Int32ArgType){
        sse::store_aligned_i128(addr as *mut Int32ArgType,value.to_owned())
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_unaligned(addr :*mut f32,value:&FloatArgType){
        sse::store_unaligned(addr, value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_unaligned_i128(addr:*mut Int32Type,value:&Int32ArgType){
        sse::store_unaligned_i128(addr as *mut Int32Type,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn stream_aligned(addr :*mut f32,value:&FloatArgType){
        sse::stream_aligned(addr,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn stream_aligned_i128(addr:*mut Int32Type,value:&Int32ArgType){
        sse::stream_aligned_i128(addr as *mut Int32Type,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn select_index0(value:&FloatArgType)->f32{
        return sse::select_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn select_index1(value:&FloatArgType)->f32{
        return sse::select_first(sse::splat_second(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn select_index2(value:&FloatArgType)->f32{
        return sse::select_first(sse::splat_third(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn select_index3(value:&FloatArgType)->f32{
        return sse::select_first(sse::splat_fourth(value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat(value:&f32)->FloatType{
        return sse::splat(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat_i32(value:&i32)->Int32Type{
        return sse::splat_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat_index0(value:&FloatArgType)->FloatType{
        return  sse::splat_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat_index1(value:&FloatArgType)->FloatType{
        return  sse::splat_second(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat_index2(value:&FloatArgType)->FloatType{
        return  sse::splat_third(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat_index3(value:&FloatArgType)->FloatType{
        return  sse::splat_fourth(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index0_f32(a:&FloatArgType,b:&f32) ->FloatType{
        return  sse::replace_first_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index0(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_first(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index1_f32(a:&FloatArgType,b:&f32) ->FloatType{
        return  sse::replace_second_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index1(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_second(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index2_f32(a:&FloatArgType,b:&f32) ->FloatType{
        return  sse::replace_third_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index2(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_third(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index3_f32(a:&FloatArgType,b:&f32) ->FloatType{
        return  sse::replace_fourth_f32(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn replace_index3(a:&FloatArgType,b:&FloatArgType) ->FloatType{
        return  sse::replace_fourth(a.to_owned(),b.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_immediate_fourth_f32(x:&f32,y:&f32,z:&f32,w:&f32)->FloatType{
        return  sse::load_immediate(x.to_owned(),y.to_owned(),z.to_owned(),w.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_immediate_fourth_i32(x:&i32,y:&i32,z:&i32,w:&i32)->Int32Type{
        return  sse::load_immediate_i32(x.to_owned(),y.to_owned(),z.to_owned(),w.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn add(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::add(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sub(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::sub(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn mul(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::mul(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn madd(mul1:&FloatArgType, mul2:&FloatArgType,add:&FloatArgType) ->FloatType{
        return sse::madd(mul1.to_owned(),mul2.to_owned(),add.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn div(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::div(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn abs(value:&FloatArgType)->FloatType{
        return sse::abs(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn add_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::add_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sub_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::sub_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn mul_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::mul_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn madd_i32(mul1:&Int32ArgType,mul2:&Int32ArgType,add:&Int32ArgType)->Int32Type{
        return sse::madd_i32(mul1.to_owned(),mul2.to_owned(),add.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn abs_i32(value:&Int32ArgType)->Int32Type{
        return  sse::abs_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn not(value:&FloatArgType)->FloatType{
        return sse::not(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn and(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::and(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn and_not(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::and_not(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn or(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::or(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn xor(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::xor(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn not_i32(value:&Int32ArgType)->Int32Type{
        return sse::not_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn and_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::and_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn and_not_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::and_not_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn or_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::or_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn xor_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type{
        return sse::xor_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn floor(value:&FloatArgType)->FloatType{
        return sse::floor(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn ceil(value:&FloatArgType)->FloatType{
        return sse::ceil(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn round(value:&FloatArgType)->FloatType{
        return sse::round(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn truncate(value:&FloatArgType) ->FloatType{
        return sse::truncate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn min(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::min(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn max(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::max(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn clamp(value:&FloatArgType,min:&FloatArgType,max:&FloatArgType) ->FloatType{
        return sse::clamp(value.to_owned(),min.to_owned(),max.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn min_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::min_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn max_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::max_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn clamp_i32(value:&Int32ArgType,min:&Int32ArgType,max:&Int32ArgType) ->Int32Type{
        return sse::clamp_i32(value.to_owned(),min.to_owned(),max.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_eq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_neq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_neq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_gt(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_gt_eq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_lt(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType{
        return sse::cmp_lt_eq(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_all_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_eq(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_all_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_lt(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_all_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_lt_eq(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_all_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_all_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt_eq(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_eq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_neq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_neq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_gt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_gt_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_gt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_gt_eq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_lt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_lt_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_lt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type{
        return sse::cmp_lt_eq_i32(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cmp_all_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->bool{
        return sse::cmp_all_eq_i32(arg1.to_owned(),arg2.to_owned(),0b1111);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn select(arg1:&FloatArgType,arg2:&FloatArgType,mask:&FloatArgType)->FloatType{
        return  sse::select(arg1.to_owned(),arg2.to_owned(),mask.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn select_i32(arg1:&Int32ArgType,arg2:&Int32ArgType,mask:&Int32ArgType)->Int32Type{
        return  sse::select_i32(arg1.to_owned(),arg2.to_owned(),mask.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn reciprocal(value:&FloatArgType)->FloatType{
        return sse::reciprocal(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn reciprocal_estimate(value:&FloatArgType)->FloatType{
        return sse::reciprocal_estimate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn mod_calculate(value:&FloatArgType,divisor:&FloatArgType)->FloatType{
        return sse::mod_calculate(value.to_owned(),divisor.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  wrap(value:&FloatArgType, min_value:&FloatArgType, max_value:&FloatArgType) ->FloatType{
        return Common::wrap(value, min_value, max_value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn angle_mod(value:&FloatArgType) ->FloatType{
        return  Common::angle_mod(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sqrt(value:&FloatArgType)->FloatType{
        return sse::sqrt(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sqrt_estimate(value:&FloatArgType)->FloatType{
        return sse::sqrt_estimate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sqrt_inv(value:&FloatArgType)->FloatType{
        return sse::sqrt_inv(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sqrt_inv_estimate(value:&FloatArgType) ->FloatType{
        return sse::sqrt_inv_estimate(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sin(value:&FloatArgType)->FloatType{
        return Common::sin(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn cos(value:&FloatArgType)->FloatType{
        return Common::cos(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sin_cos(value:&FloatArgType,mut sin:&FloatType,mut cos:&FloatType){
        Common::sin_cos(value,sin,cos)
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sin_cos_to_float_type(angles:&FloatArgType)->FloatType{
        let angle_offset = Vec4::load_immediate_fourth_f32(0.0.borrow(), HALF_PI.borrow(), 0.0.borrow(), HALF_PI.borrow());
        let sin_angles = Vec4::add(angles, angle_offset.borrow());
        return Vec4::sin(sin_angles.borrow());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn acos(value:&FloatArgType)->FloatType{
        return Common::acos(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn atan(value:&FloatArgType) ->FloatType{
        return Common::atan(value);
    }


    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn atan2(y:&FloatArgType,x:&FloatArgType) ->FloatType{
        return Common::atan2(y,x);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn exp_estimate(value:&FloatArgType)->FloatType{
        return Common::exp_estimate(value);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn dot(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        let x2 = Vec4::mul(arg1,arg2);
        let tmp = Vec4::add(x2.borrow(),_mm_shuffle_ps(x2, x2, _MM_SHUFFLE(2, 3, 0, 1)).borrow());
        return  Vec4::add(tmp.borrow(),_mm_shuffle_ps(tmp, tmp, _MM_SHUFFLE(1, 0, 2, 3)).borrow());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize(value:&FloatArgType)->FloatType{
        return Common::normalize(value);
    }
    AZ_MATH_INLINE Vec4::FloatType Vec4::NormalizeEstimate(FloatArgType value)
    {
    return Common::NormalizeEstimate<Vec4>(value);
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::NormalizeSafe(FloatArgType value, float tolerance)
    {
    return Common::NormalizeSafe<Vec4>(value, tolerance);
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::NormalizeSafeEstimate(FloatArgType value, float tolerance)
    {
    return Common::NormalizeSafeEstimate<Vec4>(value, tolerance);
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::QuaternionMultiply(FloatArgType arg1, FloatArgType arg2)
    {
    // _x = (arg1.y * arg2.z) - (arg1.z * arg2.y) + (arg1.w * arg2.x) + (arg1.x * arg2.w);
    // _y = (arg1.z * arg2.x) - (arg1.x * arg2.z) + (arg1.w * arg2.y) + (arg1.y * arg2.w);
    // _z = (arg1.x * arg2.y) - (arg1.y * arg2.x) + (arg1.w * arg2.z) + (arg1.z * arg2.w);
    // _w = (arg1.w * arg2.w) - (arg1.x * arg2.x) - (arg1.y * arg2.y) - (arg1.z * arg2.z);
    const FloatType flipWSign = Common::FastLoadConstant<Vec4>((const float*)g_negateWMask);
    const FloatType val1 = _mm_shuffle_ps(arg1, arg1, _MM_SHUFFLE(3, 0, 2, 1)); // arg1 y z x w
    const FloatType val2 = _mm_shuffle_ps(arg2, arg2, _MM_SHUFFLE(3, 1, 0, 2)); // arg2 z x y w
    const FloatType val3 = _mm_shuffle_ps(arg1, arg1, _MM_SHUFFLE(0, 1, 0, 2)); // arg1 z x y x
    const FloatType val4 = _mm_shuffle_ps(arg2, arg2, _MM_SHUFFLE(0, 0, 2, 1)); // arg2 y z x x
    const FloatType val5 = _mm_shuffle_ps(arg1, arg1, _MM_SHUFFLE(1, 3, 3, 3)); // arg1 w w w y
    const FloatType val6 = _mm_shuffle_ps(arg2, arg2, _MM_SHUFFLE(1, 2, 1, 0)); // arg2 x y z y
    const FloatType val7 = _mm_shuffle_ps(arg1, arg1, _MM_SHUFFLE(2, 2, 1, 0)); // arg1 x y z z
    const FloatType val8 = _mm_shuffle_ps(arg2, arg2, _MM_SHUFFLE(2, 3, 3, 3)); // arg2 w w w z
    const FloatType firstTerm  = Mul(val1, val2); // (arg1.y * arg2.z), (arg1.z * arg2.x), (arg1.x * arg2.y), (arg1.w * arg2.w)
    const FloatType secondTerm = Mul(val3, val4); // (arg1.z * arg2.y), (arg1.x * arg2.z), (arg1.y * arg2.x), (arg1.x * arg2.x)
    const FloatType thirdTerm  = Mul(val5, val6); // (arg1.w * arg2.x), (arg1.w * arg2.y), (arg1.w * arg2.z), (arg1.y * arg2.y)
    const FloatType fourthTerm = Mul(val7, val8); // (arg1.x * arg2.w), (arg1.y * arg2.w), (arg1.z * arg2.w), (arg1.z * arg2.z)
    const FloatType partialOne = Sub(firstTerm, secondTerm);
    const FloatType partialTwo = Xor(Add(thirdTerm, fourthTerm), flipWSign);
    return Add(partialOne, partialTwo);
    }

    AZ_MATH_INLINE Vec3::FloatType Vec4::QuaternionTransform(FloatArgType quat, Vec3::FloatArgType vec3)
    {
    return Common::QuaternionTransform<Vec4, Vec3>(quat, vec3);
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::ConstructPlane(Vec3::FloatArgType normal, Vec3::FloatArgType point)
    {
    return Common::ConstructPlane<Vec4, Vec3>(normal, point);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec4::PlaneDistance(FloatArgType plane, Vec3::FloatArgType point)
    {
    return Common::PlaneDistance<Vec4, Vec3>(plane, point);
    }

    AZ_MATH_INLINE void Vec4::Mat3x4InverseFast(const FloatType* __restrict rows, FloatType* __restrict out)
    {
    const FloatType pos = Sub(ZeroFloat(), Madd(rows[0], SplatIndex3(rows[0]), Madd(rows[1], SplatIndex3(rows[1]), Mul(rows[2], SplatIndex3(rows[2])))));
    const FloatType tmp0 = _mm_shuffle_ps(rows[0], rows[1], 0x44);
    const FloatType tmp2 = _mm_shuffle_ps(rows[0], rows[1], 0xEE);
    const FloatType tmp1 = _mm_shuffle_ps(rows[2], pos, 0x44);
    const FloatType tmp3 = _mm_shuffle_ps(rows[2], pos, 0xEE);
    out[0] = _mm_shuffle_ps(tmp0, tmp1, 0x88);
    out[1] = _mm_shuffle_ps(tmp0, tmp1, 0xDD);
    out[2] = _mm_shuffle_ps(tmp2, tmp3, 0x88);
    }

    AZ_MATH_INLINE void Vec4::Mat3x4Transpose(const FloatType* __restrict rows, FloatType* __restrict out)
    {
    const FloatType fourth = Common::FastLoadConstant<Vec4>(g_vec0001);
    const FloatType tmp0 = _mm_shuffle_ps(rows[0], rows[1], 0x44);
    const FloatType tmp2 = _mm_shuffle_ps(rows[0], rows[1], 0xEE);
    const FloatType tmp1 = _mm_shuffle_ps(rows[2], fourth, 0x44);
    const FloatType tmp3 = _mm_shuffle_ps(rows[2], fourth, 0xEE);
    out[0] = _mm_shuffle_ps(tmp0, tmp1, 0x88);
    out[1] = _mm_shuffle_ps(tmp0, tmp1, 0xDD);
    out[2] = _mm_shuffle_ps(tmp2, tmp3, 0x88);
    }

    AZ_MATH_INLINE void Vec4::Mat3x4Multiply(const FloatType* __restrict rowsA, const FloatType* __restrict rowsB, FloatType* __restrict out)
    {
    Common::Mat3x4Multiply<Vec4>(rowsA, rowsB, out);
    }

    AZ_MATH_INLINE void Vec4::Mat4x4InverseFast(const FloatType* __restrict rows, FloatType* __restrict out)
    {
    Common::Mat4x4InverseFast<Vec4>(rows, out);
    }

    AZ_MATH_INLINE void Vec4::Mat4x4Transpose(const FloatType* __restrict rows, FloatType* __restrict out)
    {
    const FloatType tmp0 = _mm_shuffle_ps(rows[0], rows[1], 0x44);
    const FloatType tmp2 = _mm_shuffle_ps(rows[0], rows[1], 0xEE);
    const FloatType tmp1 = _mm_shuffle_ps(rows[2], rows[3], 0x44);
    const FloatType tmp3 = _mm_shuffle_ps(rows[2], rows[3], 0xEE);
    out[0] = _mm_shuffle_ps(tmp0, tmp1, 0x88);
    out[1] = _mm_shuffle_ps(tmp0, tmp1, 0xDD);
    out[2] = _mm_shuffle_ps(tmp2, tmp3, 0x88);
    out[3] = _mm_shuffle_ps(tmp2, tmp3, 0xDD);
    }

    AZ_MATH_INLINE void Vec4::Mat4x4Multiply(const FloatType* __restrict rowsA, const FloatType* __restrict rowsB, FloatType* __restrict out)
    {
    Common::Mat4x4Multiply<Vec4>(rowsA, rowsB, out);
    }

    AZ_MATH_INLINE void Vec4::Mat4x4MultiplyAdd(const FloatType* __restrict rowsA, const FloatType* __restrict rowsB, const FloatType* __restrict add, FloatType* __restrict out)
    {
    Common::Mat4x4MultiplyAdd<Vec4>(rowsA, rowsB, add, out);
    }

    AZ_MATH_INLINE void Vec4::Mat4x4TransposeMultiply(const FloatType* __restrict rowsA, const FloatType* __restrict rowsB, FloatType* __restrict out)
    {
    Common::Mat4x4TransposeMultiply<Vec4>(rowsA, rowsB, out);
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::Mat4x4TransformVector(const FloatType* __restrict rows, FloatArgType vector)
    {
    const FloatType prod1 = Mul(rows[0], vector);
    const FloatType prod2 = Mul(rows[1], vector);
    const FloatType prod3 = Mul(rows[2], vector);
    const FloatType prod4 = Mul(rows[3], vector);
    return _mm_hadd_ps(_mm_hadd_ps(prod1, prod2), _mm_hadd_ps(prod3, prod4));
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::Mat4x4TransposeTransformVector(const FloatType* __restrict rows, FloatArgType vector)
    {
    return Common::Mat4x4TransposeTransformVector<Vec4>(rows, vector);
    }

    AZ_MATH_INLINE Vec3::FloatType Vec4::Mat4x4TransformPoint3(const FloatType* __restrict rows, Vec3::FloatArgType vector)
    {
    // The hadd solution is profiling slightly faster than transpose + transposetransform
    const FloatType vecXYZ = ReplaceIndex3(vector, 1.0f); // Explicitly set the w cord to 1 to include translation
    const FloatType prod1 = Mul(rows[0], vecXYZ);
    const FloatType prod2 = Mul(rows[1], vecXYZ);
    const FloatType prod3 = Mul(rows[2], vecXYZ);
    return _mm_hadd_ps(_mm_hadd_ps(prod1, prod2), _mm_hadd_ps(prod3, ZeroFloat()));
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::ConvertToFloat(Int32ArgType value)
    {
    return Sse::ConvertToFloat(value);
    }

    AZ_MATH_INLINE Vec4::Int32Type Vec4::ConvertToInt(FloatArgType value)
    {
    return Sse::ConvertToInt(value);
    }

    AZ_MATH_INLINE Vec4::Int32Type Vec4::ConvertToIntNearest(FloatArgType value)
    {
    return Sse::ConvertToIntNearest(value);
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::CastToFloat(Int32ArgType value)
    {
    return Sse::CastToFloat(value);
    }

    AZ_MATH_INLINE Vec4::Int32Type Vec4::CastToInt(FloatArgType value)
    {
    return Sse::CastToInt(value);
    }

    AZ_MATH_INLINE Vec4::FloatType Vec4::ZeroFloat()
    {
    return Sse::ZeroFloat();
    }

    AZ_MATH_INLINE Vec4::Int32Type Vec4::ZeroInt()
    {
    return Sse::ZeroInt();
    }
}