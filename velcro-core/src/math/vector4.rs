#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ops::{Mul, MulAssign, SubAssign};

#[cfg(target_arch = "arm")]
#[allow(dead_code)]
use vsimd::neon::*;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use vsimd::sse::{FloatArgType, FloatType};

use crate::math::*;
use crate::math::common_sse::{Vec2Type, Vec4Type, VecFourthType, VecThirdType, VecTwoType, VecType};
use crate::math::constants::{FLOAT_EPSILON, TOLERANCE};
use crate::math::math_utils::{get_clamp, is_finite_float, rad_to_deg};
use crate::math::simd_math::{simd_abs, simd_acos, simd_inv_sqrt};
use crate::math::simd_math_vec1_sse::Vec1;
use crate::math::simd_math_vec2_sse::Vec2;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::vector2::Vector2;
use crate::math::vector3::Vector3;

// PartialEq 是否相等
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    _value: FloatType,
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: &f32) {
        unsafe { self = Vector4::new_float_type(Vec4::mul(self.get_simd_value().borrow(), Vec4::splat(rhs).borrow()).borrow()).to_owned().borrow_mut(); }
    }
}
impl PartialEq<Self> for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return Vec4::cmp_all_eq(self._value.borrow(), other._value.borrow()); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !Vec4::cmp_all_eq(self._value.borrow(), other._value.borrow()); }
    }
}

impl MulAssign<FloatType> for Vector4 {
    fn mul_assign(&mut self, rhs: FloatType) {
        unsafe { self._value = Vec4::mul(self._value.borrow(), rhs.borrow()) }
    }
}

impl SubAssign<&Vector4> for &mut Vector4 {
    fn sub_assign(&mut self, rhs: &Vector4) {
        unsafe { self._value = Vec3::sub(self._value.borrow(), rhs._value.borrow()) }
    }
}

impl Mul<f32> for &mut Vector4 {
    type Output = Vector4;

    fn mul(self, multiplier: f32) -> Self::Output {
        unsafe { return Vector4::new_float_type(Vec3::mul(self._value.borrow(), Vec3::splat(multiplier.borrow()).borrow()).borrow()) }
    }
}

impl Mul<f32> for &Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        unsafe { return Vector4::new_float_type(Vec3::mul(self._value.borrow(), Vec3::splat(rhs.borrow()).borrow()).borrow()) }
    }
}

impl Vector4 {
    #[inline]
    #[allow(dead_code)]
    pub fn new()->Vector4{
        unsafe {
            Vector4 {
                _value:Vec4::zero_float(),
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_x(x:&f32)->Vector4{
        Vector4{
            _value:Vec4::splat(x),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_x_y_z_w(x:&f32,y:&f32,z:&f32,w:&f32)->Vector4{
        Vector4{
            _value:Vec4::load_immediate(x,y,z,w),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_float_type(value:&FloatArgType)->Vector4{
        Vector4{
            _value:value.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2(source:&Vector2)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec2(source.get_simd_value().borrow())};
        let mut tmp = *result._value as *const f32;
        *tmp[2] = 0.0;
        *tmp[3] = 1.0;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2_z(source:&Vector2,z:&f32)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec2(source.get_simd_value().borrow())};
        let mut tmp = *result._value as *const f32;
        *tmp[2] = z;
        *tmp[3] = 1.0;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2_z_w(source:&Vector2,z:&f32,w:&f32)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec2(source.get_simd_value().borrow())};
        let mut tmp = *result._value as *const f32;
        *tmp[2] = z;
        *tmp[3] = w;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec3(source:&Vector3)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec3(source.get_simd_value().borrow())};
        let mut tmp = *result._value as [f32;4];
        *tmp[3] = 1.0;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec3_w(source:&Vector3,w:&f32)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec3(source.get_simd_value().borrow())};
        let mut tmp = *result._value as [f32;4];
        *tmp[3] = w;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_zero()->Vector4{
        unsafe {
            Vector4 {
                _value: Vec4::zero_float(),
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_one()->Vector4{
        return Vector4::new_x(1.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_x(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(length,0.0.borrow(),0.0.borrow(),0.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_y(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(0.0.borrow(),length,0.0.borrow(),0.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_z(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(0.0.borrow(),0.0.borrow(),length,0.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_w(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(0.0.borrow(),0.0.borrow(),0.0.borrow(),length);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_float4(values:*const f32)->Vector4{
        let result = Vector4::new();
        result.set(values);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3(v:&Vector3)->Vector4{
        let mut result = Vector4::new();
        result.set_vec3(v);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3_and_float(v:&Vector3,w:&f32)->Vector4{
        let mut result = Vector4::new();
        result.set_vec3_f32(v,w);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_select_cmp_equal(cmp1:&Vector4,cmp2:&Vector4,va:&Vector4,vb:&Vector4)->Vector4{
        let mask = Vec4::cmp_eq(cmp1._value.borrow(),cmp2._value.borrow());
        return Vector4::new_float_type(Vec4::select(va._value.borrow(),vb._value.borrow(),mask.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_select_cmp_greater_equal(cmp1:&Vector4, cmp2:&Vector4, va:&Vector4, vb:&Vector4) ->Vector4{
        let mask = Vec4::cmp_gt_eq(cmp1._value.borrow(),cmp2._value.borrow());
        return Vector4::new_float_type(Vec4::select(va._value.borrow(),vb._value.borrow(),mask.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_select_cmp_greater(cmp1:&Vector4, cmp2:&Vector4, va:&Vector4, vb:&Vector4) ->Vector4{
        let mask = Vec4::cmp_gt(cmp1._value.borrow(),cmp2._value.borrow());
        return Vector4::new_float_type(Vec4::select(va._value.borrow(),vb._value.borrow(),mask.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_float_4(self, values:*mut f32){
        Vec4::store_unaligned(values,self._value.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_x(self)->f32{
        let values = *self._value as *const f32;
        *values[0]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_y(self)->f32{
        let values = *self._value as *const f32;
        *values[1]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_z(self)->f32{
        let values = *self._value as *const f32;
        *values[2]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_w(self)->f32{
        let values = *self._value as *const f32;
        *values[3]
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_element(self,index:i32)->f32{
        let values = *self._value as *const f32;
        *values[index]
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_x(mut self, x :f32){
        let values = *self._value as *const f32;
        *values[0] = x
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_y(mut self, y:f32){
        let values = *self._value as *const f32;
        *values[1] = y
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_z(mut self, z:f32){
        let values = *self._value as *const f32;
        *values[2] = z
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_w(mut self, w:f32){
        let values = *self._value as *const f32;
        *values[3] = w
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_f32(mut self,x:&f32){
        self._value = Vec4::splat(x);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_x_y_z_w(mut self,x:&f32,y:&f32,z:&f32,w:&f32){
        self._value = Vec4::load_immediate(x,y,z,w);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set(mut self, values:*const f32){
        self._value = Vec4::load_aligned(values);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_vec3(&mut self,v:&Vector3){
        self._value = Vector4::new_vec3_w(v,1.0.borrow())._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_vec3_f32(&mut self,v:&Vector3,w:&f32){
        self._value = Vector4::new_vec3_w(v,w)._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_float_type(mut self,v:&FloatArgType){
        self._value = v.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(mut self,index:&i32,v:&f32){
        let values = *self._value as *const f32;
        *values[index] = v
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_as_vector3(self)->Vector3{
        return Vector3::new_float_type(self._value.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length_sq(self)->f32{
        return self.dot4(*self)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length(self)->f32{
        let length_sq = Vec4::dot(self._value.borrow(), self._value.borrow());
        return Vec1::select_index0(Vec1::sqrt(length_sq.borrow()).borrow());
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length_estimate(self)->f32{
        let length_sq = Vec4::dot(self._value.borrow(),self._value.borrow());
        return Vec1::select_index0(Vec1::sqrt_estimate(length_sq.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length_reciprocal(self) ->f32{
        let length_sq = Vec4::dot(self._value.borrow(), self._value.borrow());
        return Vec1::select_index0(Vec1::sqrt_inv(length_sq.borrow()).borrow());
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_length_reciprocal_estimate(self) ->f32{
        let length_sq = Vec4::dot(self._value.borrow(), self._value.borrow());
        return Vec1::select_index0(Vec1::sqrt_inv_estimate(length_sq.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_normalized(self) ->Vector4{
        let result = Vector4::new_float_type(Vec4::normalize(self._value.borrow()).borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_normalized_estimate(self)->Vector4{
        let result = Vector4::new_float_type( Vec4::normalize_estimate(self._value.borrow()).borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize(mut self){
        self._value = self.get_normalized()._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize_estimate(mut self){
        self._value = self.get_normalized_estimate()._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_normalized_safe(self, tolerance:&f32)->Vector4{
        return  Vector4::new_float_type(Vec4::normalize_safe(self._value.borrow(),tolerance).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_normalized_safe_estimate(self, tolerance:&f32)->Vector4{
        return Vector4::new_float_type(Vec4::normalize_safe_estimate(self._value.borrow(),tolerance).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize_safe(mut self, tolerance:&f32){
        self._value = self.get_normalized_safe(tolerance)._value
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_safe_estimate(mut self, tolerance:&f32){
        self._value = self.get_normalized_safe_estimate(tolerance)._value
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_with_length(mut self)->f32{
        let length = Vec1::select_index0(
            Vec1::sqrt(Vec4::dot(self._value.borrow(), self._value.borrow()).borrow()).borrow());
        self._value = Vec4::div(self._value.borrow(), Vec4::splat(length.borrow()).borrow_mut());
        return length;
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_with_length_estimate(mut self) ->f32{
        let length = Vec1::select_index0(
            Vec1::sqrt_estimate(Vec4::dot(self._value.borrow(), self._value.borrow()).borrow()).borrow());
        self._value = Vec4::div(self._value.borrow(), Vec4::splat(length.borrow()).borrow_mut());
        return length;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_safe_with_length(mut self,tolerance:&f32)->f32{
        let length = Vec1::sqrt( Vec4::dot(self._value.borrow(),self._value.borrow()).borrow());
        if Vec1::select_index0(length.borrow()) < tolerance.to_owned(){
            self._value = Vec4::zero_float();
        }else {
            self._value = Vec4::div(self._value.borrow(),Vec4::splat_index0(Vec4::from_vec1(length.borrow()).borrow()).borrow_mut());
        }
        let result = Vec1::select_index0(length.borrow());
        result
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_safe_with_length_estimate(mut self,tolerance:&f32) ->f32{
        let length = Vec1::sqrt_estimate(Vec4::dot(self._value.borrow(),self._value.borrow()).borrow());
        if Vec1::select_index0(length.borrow()) < tolerance.to_owned(){
            self._value = Vec4::zero_float();
        }else {
            self._value = Vec4::div(self._value.borrow(),Vec4::splat_index0(Vec4::from_vec1(length.borrow()).borrow()).borrow_mut());
        }
        let result = Vec1::select_index0(length.borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_normalized(self,tolerance:&f32)->bool{
        return (simd_abs((self.get_length_sq() - 1.0).borrow()) <= tolerance.to_owned());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_length(mut self, length:&f32){
        let scale =   self.get_length_reciprocal() * length;
        self *= scale ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_length_estimate(mut self, length:&f32){
        let scale = length* self.get_length_reciprocal_estimate();
        self *= scale;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance_sq(mut self, v :&Vector4)->f32{
        return (*self - v).get_length_sq();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance(mut self, v :&Vector4)->f32{
        return (*self - v).get_length();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance_estimate(mut self, v :&Vector4)->f32{
        return (*self - v).get_length_estimate();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(&self, v:&Vector4, tolerance :&f32)->bool{
        let dist:Vector4 = (v - (*self)).get_abs();
        return dist.is_less_equal_than(Vector4::new_x(tolerance));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_zero(self, tolerance:&f32)->bool{
        let dist = self.get_abs();
        return  dist.is_less_equal_than(Vector4::new_x(tolerance).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_zero_with_default(self)->bool{
        let dist = self.get_abs();
        return  dist.is_less_equal_than(Vector3::new_x(FLOAT_EPSILON.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_less_than(self, rhs :&Vector4)->bool{
        return Vec4::cmp_all_lt(self.get_simd_value().borrow(),rhs.get_simd_value().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_less_equal_than(self, rhs:&Vector4) ->bool{
        return Vec4::cmp_all_lt_eq(self._value.borrow(), rhs._value.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_greater_than(self,rhs:&Vector4)->bool{
        return  Vec4::cmp_all_gt(self.get_simd_value().borrow(),rhs.get_simd_value().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_greater_equal_than(self,rhs:&Vector4)->bool{
        return  Vec4::cmp_all_gt_eq(self.get_simd_value().borrow(),rhs.get_simd_value().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_floor(self)->Vector4{
        return Vector4::new_float_type(Vec4::floor(self.get_simd_value().borrow()).borrow()) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_ceil(self)->Vector4{
        return Vector4::new_float_type(Vec4::ceil(self.get_simd_value().borrow()).borrow()) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_round(self)->Vector4{
        return  Vector4::new_float_type(Vec4::round(self.get_simd_value().borrow()).borrow()) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_min(self,v :&Vector4)->Vector4{
        return  Vector4::new_float_type(Vec4::min(self.get_simd_value().borrow(),v.get_simd_value().borrow()).borrow()) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_max(self,v :&Vector4)->Vector4{
        return  Vector4::new_float_type(Vec4::max(self.get_simd_value().borrow(),v.get_simd_value().borrow()).borrow()) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_clamp(self, min:&Vector4,max:&Vector4)->Vector4{
        return self.get_min(max).get_max(min);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  lerp(self,dest :&Vector4,t :&f32)->Vector4{
        return Vector4::new_float_type(Vec4::madd(Vec4::sub(dest._value.borrow(),self._value.borrow()).borrow(),Vec4::splat(t).borrow(),self._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  slerp(self,dest :&Vector4,t :&f32)->Vector4{
        let dot_val = Vec1::clamp(Vec4::dot(self._value.borrow(),dest._value.borrow()).borrow(),Vec1::splat((-1.0).borrow()).borrow(),Vec1::splat(1.0.borrow()).borrow());
        let theta = Vec1::mul(Vec1::acos(dot_val.borrow()).borrow(),Vec1::splat(t).borrow());
        let relative_vec = Vec4::sub(dest.get_simd_value().borrow(), Vec4::mul(self.get_simd_value().borrow(), Vec4::from_vec1(dot_val.borrow()).borrow()).borrow());
        let rel_vec_norm = Vec4::normalize_safe(relative_vec.borrow(), TOLERANCE.borrow());
        let sin_cos = Vec4::from_vec2(Vec2::sin_cos_to_float_type(theta.borrow()).borrow());
        let rel_vec_sin_theta = Vec4::mul(rel_vec_norm.borrow(), Vec4::splat_index0(sin_cos.borrow()).borrow());
        let result = Vector4::new_float_type(Vec4::madd(self.get_simd_value().borrow(), Vec3::splat_index1(sin_cos.borrow()).borrow(),rel_vec_sin_theta.borrow()).borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  nlerp(self, dest :&Vector4,t:&f32)->Vector4{
        return  self.lerp(dest.borrow(),t).get_normalized_safe(TOLERANCE.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  dot4(self,rhs:&Vector4)->f32{
        return Vec1::select_index0(Vec4::dot(self.get_simd_value().borrow(),rhs.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn dot3(self,rhs:&Vector4)->f32{
        return Vec1::select_index0(Vec3::dot(Vec4::value_to_vec3(self._value.borrow()).borrow(),rhs.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn homogenize(mut self){
        let divisor = Vec4::splat_index3(self._value.borrow());
        self._value = Vec4::div(self._value.borrow(),divisor.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_homogenized(self)->Vector3{
        let mut divisor = Vec4::value_to_vec3(Vec4::splat_index3(self._value.borrow()).borrow());
        return Vector3::new_float_type(Vec3::div(Vec4::value_to_vec3(self._value.borrow()).borrow(),divisor.borrow_mut()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_sin(self)->Vector4{
        return  Vector4::new_float_type(Vec4::sin(self._value.borrow()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_cos(self)->Vector4{
        return Vector4::new_float_type(Vec4::cos(self._value.borrow()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_sin_cos(self,mut sin: &Vector4,mut cos :&Vector4){
        let mut sin_values:FloatType;
        let mut cos_values:FloatType;
        Vec4::sin_cos(self._value.borrow(), sin_values.borrow_mut(), cos_values.borrow_mut());
        sin = &mut Vector4::new_float_type(sin_values.borrow());
        cos = &mut Vector4::new_float_type(cos_values.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_acos(self)->Vector4{
        return Vector4::new_float_type(Vec4::acos(self._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_atan(self)->Vector4{
        return Vector4::new_float_type(Vec4::atan(self._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_exp_estimate(self) ->Vector4{
        return Vector4::new_float_type(Vec4::exp_estimate(self._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_angle_mod(self) ->Vector4{
        return Vector4::new_float_type(Vec4::angle_mod(self._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle(self, v:&Vector4)->f32{
        let cos =self.dot4(v.borrow())*simd_inv_sqrt((self.get_length_sq()*v.get_length_sq()).borrow());
        let res = simd_acos(get_clamp(cos.borrow(), (-1.0) .borrow(), 1.0.borrow()));
        res
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle_deg(self,v:&Vector4)->f32{
        return rad_to_deg(self.angle(v).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle_safe(self, v:&Vector4)->f32{
        return   if !self.is_zero_with_default()&&!v.is_zero_with_default(){
            let result = self.angle(v.borrow());
            result
        }else {
            0.0
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle_safe_deg(self,v:&Vector4)->f32{
        return if !self.is_zero_with_default() && !v.is_zero_with_default() {
            let result = self.angle_deg(v);
            result
        }else{
            0.0
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_abs(self)->Vector4{
        return  Vector4::new_float_type(Vec4::abs(self.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_reciprocal(self)->Vector4{
        return  Vector4::new_float_type(Vec4::reciprocal(self.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_reciprocal_estimate(self)->Vector4{
        return Vector4::new_float_type(Vec4::reciprocal_estimate(self.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_finite(self)->bool{
        return is_finite_float(self.get_x().borrow())&&is_finite_float(self.get_y().borrow())&&is_finite_float(self.get_z().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_simd_value(&self)->FloatType{
        self._value
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_simd_value(mut self, value :FloatArgType ){
        self._value = value;
    }

}