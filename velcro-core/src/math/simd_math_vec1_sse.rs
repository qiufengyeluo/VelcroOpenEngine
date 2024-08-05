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
    unsafe fn div(arg1:&FloatType, arg2: &mut FloatType) ->FloatType{
        let ones = sse::splat(1.0);
        *arg2 = sse::replace_first(ones.to_owned(),arg2.to_owned());
        return sse::div(arg1.to_owned(),arg2.to_owned())
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
    unsafe fn not(value:&FloatArgType)->FloatType{
        return  sse::not(value.to_owned());
    }

    fn and(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn add_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    fn sub_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    fn and_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    fn and_not(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn or(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn splat_i32(value: &i32) -> Int32Type {
        todo!()
    }

    fn splat_index0(value: &FloatArgType) -> FloatType {
        todo!()
    }

    fn select(arg1: &FloatArgType, arg2: &FloatArgType, mask: &FloatArgType) -> FloatType {
        todo!()
    }

    fn splat(value: &f32) -> FloatType {
        todo!()
    }

    fn sin(value: &FloatArgType) -> FloatType {
        todo!()
    }

    fn xor(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn abs(value: &FloatArgType) -> FloatType {
        todo!()
    }

    fn load_immediate(x: &f32) -> FloatType {
        todo!()
    }

    fn load_immediate_fourth_f32(x: &f32, y: &f32, z: &f32, w: &f32) -> FloatType {
        todo!()
    }

    fn sqrt(value: &FloatArgType) -> FloatType {
        todo!()
    }

    fn sqrt_estimate(value: &FloatArgType) -> FloatType {
        todo!()
    }

    fn sqrt_inv_estimate(value: &FloatArgType) -> FloatType {
        todo!()
    }

    fn atan(value: &FloatArgType) -> FloatType {
        todo!()
    }

    fn mod_calculate(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn cmp_eq(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn cmp_lt(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn cmp_gt(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn cmp_gt_eq(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn cmp_eq_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    fn cmp_all_lt(arg1: &FloatArgType, arg2: &FloatArgType) -> bool {
        todo!()
    }

    fn dot(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    fn convert_to_float(value: &Int32ArgType) -> FloatType {
        todo!()
    }

    fn convert_to_int(value: &FloatArgType) -> Int32Type {
        todo!()
    }

    fn convert_to_int_nearest(value: &FloatArgType) -> Int32Type {
        todo!()
    }

    fn cast_to_float(value: &Int32ArgType) -> FloatType {
        todo!()
    }

    fn cast_to_int(value: &FloatArgType) -> Int32Type {
        todo!()
    }

    fn zero_float() -> FloatType {
        todo!()
    }

    fn zero_int() -> Int32Type {
        todo!()
    }

    unsafe fn from_vec2(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn normalize(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn normalize_estimate(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn from_vec3(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn load_aligned(addr: *const f32) -> FloatType {
        todo!()
    }

    unsafe fn load_aligned_i128(addr: *const Int32Type) -> Int32Type {
        todo!()
    }

    unsafe fn load_unaligned(addr: &f32) -> FloatType {
        todo!()
    }

    unsafe fn load_unaligned_i128(addr: *const Int32Type) -> Int32Type {
        todo!()
    }

    unsafe fn store_aligned(addr: *mut f32, value: &FloatArgType) {
        todo!()
    }

    unsafe fn store_aligned_i128(addr: *mut Int32Type, value: &Int32ArgType) {
        todo!()
    }

    unsafe fn store_unaligned(addr: *mut f32, value: &FloatArgType) {
        todo!()
    }

    unsafe fn store_unaligned_i128(addr: *mut Int32Type, value: &Int32ArgType) {
        todo!()
    }

    unsafe fn stream_aligned(addr: *mut f32, value: &FloatArgType) {
        todo!()
    }

    unsafe fn stream_aligned_i128(addr: *mut Int32Type, value: &Int32ArgType) {
        todo!()
    }

    unsafe fn select_index0(value: &FloatArgType) -> f32 {
        todo!()
    }

    unsafe fn select_index1(value: &FloatArgType) -> f32 {
        todo!()
    }

    unsafe fn select_index2(value: &FloatArgType) -> f32 {
        todo!()
    }

    unsafe fn select_index3(value: &FloatArgType) -> f32 {
        todo!()
    }

    unsafe fn splat_index1(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn splat_index2(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn splat_index3(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn replace_index0_f32(a: &FloatArgType, b: &f32) -> FloatType {
        todo!()
    }

    unsafe fn replace_index0(a: &FloatArgType, b: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn replace_index1_f32(a: &FloatArgType, b: &f32) -> FloatType {
        todo!()
    }

    unsafe fn replace_index1(a: &FloatArgType, b: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn replace_index2_f32(a: &FloatArgType, b: &f32) -> FloatType {
        todo!()
    }

    unsafe fn replace_index2(a: &FloatArgType, b: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn replace_index3_f32(a: &FloatArgType, b: &f32) -> FloatType {
        todo!()
    }

    unsafe fn replace_index3(a: &FloatArgType, b: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn load_immediate_fourth_i32(x: &i32, y: &i32, z: &i32, w: &i32) -> Int32Type {
        todo!()
    }

    unsafe fn mul_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn madd_i32(mul1: &Int32ArgType, mul2: &Int32ArgType, add: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn abs_i32(value: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn not_i32(value: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn and_not_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn or_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn xor_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn floor(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn ceil(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn round(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn truncate(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn min(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn max(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn clamp(value: &FloatArgType, min: &FloatArgType, max: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn min_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn max_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn clamp_i32(value: &Int32ArgType, min: &Int32ArgType, max: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn cmp_neq(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn cmp_lt_eq(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn cmp_all_eq(arg1: &FloatArgType, arg2: &FloatArgType) -> bool {
        todo!()
    }

    unsafe fn cmp_all_lt_eq(arg1: &FloatArgType, arg2: &FloatArgType) -> bool {
        todo!()
    }

    unsafe fn cmp_all_gt(arg1: &FloatArgType, arg2: &FloatArgType) -> bool {
        todo!()
    }

    unsafe fn cmp_all_gt_eq(arg1: &FloatArgType, arg2: &FloatArgType) -> bool {
        todo!()
    }

    unsafe fn cmp_neq_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn cmp_gt_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn cmp_gt_eq_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn cmp_lt_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn cmp_lt_eq_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn cmp_all_eq_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> bool {
        todo!()
    }

    unsafe fn select_i32(arg1: &Int32ArgType, arg2: &Int32ArgType, mask: &Int32ArgType) -> Int32Type {
        todo!()
    }

    unsafe fn reciprocal(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn reciprocal_estimate(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn wrap(value: &FloatArgType, min_value: &FloatArgType, max_value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn angle_mod(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn sqrt_inv(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn cos(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn sin_cos(value: &FloatArgType, sin: &FloatType, cos: &FloatType) {
        todo!()
    }

    unsafe fn sin_cos_to_float_type(angles: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn acos(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn atan2(y: &FloatArgType, x: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn exp_estimate(value: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn normalize_safe(value: &FloatArgType, tolerance: &f32) -> FloatType {
        todo!()
    }

    unsafe fn normalize_safe_estimate(value: &FloatArgType, tolerance: &f32) -> FloatType {
        todo!()
    }

    unsafe fn quaternion_multiply(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn quaternion_transform(quat: &FloatArgType, vec3: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn mat4x4_transform_point3(rows: *const FloatType, vector: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn mat4x4_transpose_transform_vector(rows: *const FloatType, vector: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn construct_plane(normal: &FloatArgType, point: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn plane_distance(plane: &FloatArgType, point: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn mat3x4inverse_fast(rows: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat3x4transpose(rows: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat3x4multiply(rows_a: *const FloatType, rows_b: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat4x4inverse_fast(rows: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat4x4transpose(rows: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat4x4multiply(rows_a: *const FloatType, rows_b: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat4x4multiply_add(rows_a: *const FloatType, rows_b: *const FloatType, add: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat4x4transpose_multiply(rows_a: *const FloatType, rows_b: *const FloatType, out: *const FloatType) {
        todo!()
    }

    unsafe fn mat4x4transform_vector(rows: *const FloatType, vector: &FloatArgType) -> FloatType {
        todo!()
    }

    unsafe fn cmp_all_gt_eq_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> bool {
        todo!()
    }

    unsafe fn cmp_all_gt_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> bool {
        todo!()
    }

    unsafe fn cmp_all_lt_eq_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> bool {
        todo!()
    }

    unsafe fn cmp_all_lt_i32(arg1: &Int32ArgType, arg2: &Int32ArgType) -> bool {
        todo!()
    }

    unsafe fn load_immediate_i32(x: &i32) -> Int32Type {
        todo!()
    }
}

impl Vec1Type for  Vec1 {

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn load_aligned(addr :*f32)->FloatType{
        return  _mm_load_ps1(addr);
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
        return  _mm_load_ps1(addr);
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
        _mm_store_ss(addr, value.to_owned());
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
        _mm_store_ss(addr, value.to_owned());
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
     unsafe fn load_immediate(x:&f32)->FloatType{
        return sse::load_immediate(x.to_owned(),0.0,0.0,0.0);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn load_immediate_i32(x:&i32)->Int32Type{
        return sse::load_immediate_i32(x.to_owned(),0,0,0);
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
        return sse::cmp_all_eq(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_lt(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_lt_eq(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool{
        return sse::cmp_all_gt_eq(arg1.to_owned(),arg2.to_owned(),0b0001);
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
        return sse::cmp_all_eq_i32(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_lt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->bool{
        return sse::cmp_all_lt_i32(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_lt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->bool{
        return sse::cmp_all_lt_eq_i32(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_gt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->bool{
        return sse::cmp_all_gt_i32(arg1.to_owned(),arg2.to_owned(),0b0001);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn cmp_all_gt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->bool{
        return sse::cmp_all_gt_eq_i32(arg1.to_owned(),arg2.to_owned(),0b0001);
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
        let ones = sse::splat(1.0);
        return sse::reciprocal(sse::replace_first(ones,value.to_owned()));
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
     unsafe fn reciprocal_estimate(value:&FloatArgType)->FloatType{
        let ones = sse::splat(1.0);
        return sse::reciprocal_estimate(sse::replace_first(ones,value.to_owned()));
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
