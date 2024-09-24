#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ops::*;
use std::ops::{Mul, MulAssign, SubAssign};

#[cfg(target_arch = "arm")]
#[allow(dead_code)]
use vsimd::neon::*;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use vsimd::sse::FloatType;

use crate::math::*;
use crate::math::common_sse::{Vec2Type, Vec3Type, VecFourthType, VecThirdType, VecTwoType, VecType};
use crate::math::math_utils::*;
use crate::math::simd_math::*;
use crate::math::simd_math_vec1_sse::Vec1;
use crate::math::simd_math_vec2_sse::Vec2;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::vector2::Vector2;
use crate::math::vector4::Vector4;
use crate::math::vsimd::FloatArgType;

// PartialEq 是否相等
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    _value: FloatType,
}

impl MulAssign<FloatType> for Vector3 {
    fn mul_assign(&mut self, rhs: FloatType) {
        unsafe { self._value = Vec3::mul(self._value, rhs) }
    }
}
impl MulAssign<&Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: &Vector3) {
        unsafe { self._value = Vec3::mul(self._value, rhs._value)}
    }
}
impl SubAssign<&Vector3> for &mut Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        unsafe { self._value = Vec3::sub(self._value, rhs._value) }
    }
}

impl Mul<f32> for &mut Vector3 {
    type Output = Vector3;

    fn mul(self, multiplier: f32) -> Self::Output {
        unsafe { return Vector3::new_float_type(Vec3::mul(self._value, Vec3::splat(multiplier))) }
    }
}


impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        unsafe { return Vector3::new_float_type(Vec3::mul(self._value, Vec3::splat(rhs))) }
    }
}
impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        unsafe { return Vector3::new_float_type(Vec3::div(self._value, Vec3::splat(rhs).borrow_mut())) }
    }
}
impl Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        unsafe {
            Vector3 {
                _value: Vec3::add(self._value, rhs._value)
            }
        }
    }
}
impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        unsafe { return Vector3::new_float_type(Vec3::sub(self._value, rhs._value)) }
    }
}
impl Vector3 {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Vector3 {
        unsafe {
            Vector3 {
                _value: Vec3::zero_float(),
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_x(x:f32) ->Vector3{
       Vector3{
           _value:Vec3::splat(x),
       }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_xyz(x:f32, y:f32, z:f32) ->Vector3{
        Vector3{
            _value:Vec3::load_immediate(x,y,z),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2(source:&Vector2)->Vector3{
       return Vector3::new_vec2_f32(source,0.0);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2_f32(source:&Vector2,z:f32)->Vector3{
        let mut result =  Vector3{
            _value: Vec3::value_to_vec2(source.get_simd_value())
        };
        result.set_z(z);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec4(source:&Vector4)->Vector3{
        Vector3{
            _value:Vec4::value_to_vec3(source.get_simd_value()),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_float_type(v :FloatType)->Vector3{
        Vector3{
            _value:v.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub  fn create_zero() ->Vector3{
        let result:Vector3 =Vector3::new_float_type(Vec3::zero_float());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_one()->Vector3{
        let result:Vector3 = Vector3::new_x(1.0);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_x(length:f32)->Vector3{
        let result:Vector3 = Vector3::new_xyz(length, 0.0, 0.0);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  create_axis_y(length:f32)->Vector3{
        let result:Vector3 =  Vector3::new_xyz(0.0, length, 0.0);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  create_axis_z(length:f32)->Vector3{
        let result:Vector3 =  Vector3::new_xyz(0.0, 0.0, length);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  create_from_float_3(ptr :*const f32)->Vector3{
        let result:Vector3 =  Vector3::new_xyz((*ptr[0]), (*ptr[1]), (*ptr[2]));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  create_select_cmp_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = Vec3::cmp_eq(cmp1._value, cmp2._value);
        let result = Vector3::new_float_type( Vec3::select(va._value,vb._value,mask));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  create_select_cmp_greater_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = Vec3::cmp_gt_eq(cmp1._value, cmp2._value);
        let result = Vector3::new_float_type( Vec3::select(va._value,vb._value,mask));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  create_select_cmp_greater(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = Vec3::cmp_gt(cmp1._value, cmp2._value);
        let result = Vector3::new_float_type(Vec3::select(va._value,vb._value,mask));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  store_to_float_3(self,  ptr :&*mut f32){
        let values:*const [f32;3] = (*self._value) as *[f32;3];

        *ptr[0] = values[0];
        *ptr[1] = values[1];
        *ptr[2] = values[2];
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  store_to_float_4(self,  value : *mut f32){
        Vec3::store_unaligned(value, self._value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_x(self)->f32{
        let values = *self._value as *const f32;
       *values[0]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_y(self)->f32{
        let values = *self._value as *const f32;
        *values[1]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_z(self)->f32{
        let values = *self._value as *const f32;
        *values[2]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_element(self,index:i32)->f32{
        let values = *self._value as *const f32;
        *values[index]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_simd_value(&self)->FloatType{
        self._value
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_x(mut self, x :f32){
        let values = *self._value as *const f32;
        *values[0] = x
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_y(mut self, y:f32){
        let values = *self._value as *const f32;
        *values[1] = y
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_z(mut self, z:f32){
        let values = *self._value as *const f32;
        *values[2] = z
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_value_x(mut self, x :f32){
        self._value = Vec3::splat(x);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_element(mut self,index:i32,v:f32){
        let values = *self._value as *const f32;
        *values[index] = v
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_value_xyz(mut self, x:f32, y:f32, z:f32){
        self._value = Vec3::load_immediate(x, y, z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_value_ptr(mut self, values:*const f32){
        self._value = Vec3::load_immediate(*values[0],*values[1],*values[2]);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_length_sq(&self) ->f32{
        let result =  Vec3::dot_vec3(Vector3{_value:self._value});
        result
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_length(self) ->f32{
        let length_sq = Vec3::dot(self._value, self._value);
        return Vec1::select_index0(Vec1::sqrt(length_sq));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_length_estimate(self) ->f32{
        let length_sq = Vec3::dot(self._value, self._value);
        return Vec1::select_index0(Vec1::sqrt_estimate(length_sq));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_length_reciprocal(self) ->f32{
        let length_sq = Vec3::dot(self._value, self._value);
        return Vec1::select_index0(Vec1::sqrt_inv(length_sq));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_length_reciprocal_estimate(self) ->f32{
        let length_sq = Vec3::dot(self._value, self._value);
        return Vec1::select_index0(Vec1::sqrt_inv_estimate(length_sq));
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_normalized(self) ->Vector3{
        let result = Vector3::new_float_type(Vec3::normalize(self._value));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_normalized_estimate(self)->Vector3{
        let result = Vector3::new_float_type( Vec3::normalize_estimate(self._value));
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
    pub unsafe fn  normalize_with_length(mut self)->f32{
        let length = Vec1::select_index0(
            Vec1::sqrt(Vec3::dot(self._value, self._value)));
        self._value = Vec3::div(self._value, Vec3::splat(length).borrow_mut());
        return length;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize_with_length_estimate(mut self)->f32{
        let length = Vec1::select_index0(
            Vec1::sqrt_estimate(Vec3::dot(self._value, self._value)));
        self._value = Vec3::div(self._value, Vec3::splat(length).borrow_mut());
        return length;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_normalized_safe( self,tolerance:f32)->Vector3{
        let result = Vector3::new_float_type(Vec3::normalize_safe(self._value,tolerance));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_normalized_safe_estimate(self,tolerance:f32)->Vector3{
        let result = Vector3::new_float_type(Vec3::normalize_safe_estimate(self._value,tolerance));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize_safe(mut self, tolerance:f32){
        self._value = Vec3::normalize_safe(self._value,tolerance)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize_safe_estimate(mut self, tolerance:f32){
        self._value = Vec3::normalize_safe_estimate(self._value,tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize_safe_with_length(mut self, tolerance:f32)->f32{
        let length = Vec1::sqrt( Vec3::dot(self._value,self._value));
        if Vec1::select_index0(length) < tolerance.to_owned(){
            self._value = Vec3::zero_float();
        }else {
            self._value = Vec3::div(self._value,Vec3::splat_index0(Vec3::from_vec1(length)).borrow_mut());
        }
        let result = Vec1::select_index0(length);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  normalize_safe_with_length_estimate(mut self, tolerance:f32) ->f32{
        let length = Vec1::sqrt_estimate(Vec3::dot(self._value,self._value));
        if Vec1::select_index0(length) < tolerance.to_owned(){
            self._value = Vec3::zero_float();
        }else {
            self._value = Vec3::div(self._value,Vec3::splat_index0(Vec3::from_vec1(length)).borrow_mut());
        }
        let result = Vec1::select_index0(length);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_normalized(self,tolerance:f32)->bool{
        return (simd::abs((self.get_length_sq() - 1.0)) <= tolerance.to_owned());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_length(mut self, length:f32){
        let scale =   self.get_length_reciprocal() * length;
        self *= scale ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_length_estimate(mut self, length:f32){
        let scale = length* self.get_length_reciprocal_estimate();
        self *= scale;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_distance_sq(mut self, v :&Vector3)->f32{
        return (*self - v).get_length_sq();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_distance(mut self, v :&Vector3)->f32{
        return (*self - v).get_length();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_distance_estimate(mut self, v :&Vector3)->f32{
        return (*self - v).get_length_estimate();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  lerp(self,dest :Vector3,t :f32)->Vector3{
        return Vector3::new_float_type(Vec3::madd(Vec3::sub(dest._value,self._value),Vec3::splat(t),self._value));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  slerp(self,dest :&Vector3,t :f32)->Vector3{
        let dot_val = Vec1::clamp(Vec3::dot(self._value,dest._value),Vec1::splat((-1.0)),Vec1::splat(1.0));
        let theta = Vec1::mul(Vec1::acos(dot_val),Vec1::splat(t));
        let relative_vec = Vec3::sub(dest.get_simd_value(), Vec3::mul(self.get_simd_value(), Vec3::from_vec1(dot_val)));
        let rel_vec_norm = Vec3::normalize_safe(relative_vec, constants::TOLERANCE);
        let sin_cos = Vec3::from_vec2(Vec2::sin_cos_to_float_type(theta));
        let rel_vec_sin_theta = Vec3::mul(rel_vec_norm, Vec3::splat_index0(sin_cos));
        let result = Vector3::new_float_type(Vec3::madd(self.get_simd_value(), Vec3::splat_index1(sin_cos),rel_vec_sin_theta));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  nlerp(self, dest :&Vector3,t:f32)->Vector3{
        let result = self.lerp(dest.to_owned(),t);
        return  result.get_normalized_safe(constants::TOLERANCE);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  dot3(self,rhs:&Vector3)->f32{
        return Vec1::select_index0(Vec3::dot(self.get_simd_value(),rhs.get_simd_value()));
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  cross(self,rhs :&Vector3)->Vector3{
        let result = Vector3::new_float_type(Vec3::cross(self.get_simd_value(),rhs.get_simd_value()));
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  cross_x_axis(self)->Vector3{
        return Vector3::new_load_immediate(0.0,self.get_z(),-self.get_y());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  cross_y_axis(self)->Vector3{
        return  Vector3::new_load_immediate(-self.get_z(),0.0,self.get_z());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  cross_z_axis(self)->Vector3{
        return  Vector3::new_load_immediate(self.get_y(),-self.get_x(),0.0);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  x_axis_cross(self)->Vector3{
        return Vector3::new_load_immediate(0.0,-self.get_z(),self.get_y());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  y_axis_cross(self)->Vector3{
        return Vector3::new_load_immediate(self.get_z(),0.0,-self.get_x());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  z_axis_cross(self)->Vector3{
        return  Vector3::new_load_immediate(-self.get_y(),self.get_x(),0.0);
    }

    pub unsafe  fn is_close(&self, v:&Vector3, tolerance :f32) ->bool
    {
        let dist:Vector3 = (v - (*self)).get_abs();
        return dist.is_less_equal_than(Vector3::new_x(tolerance).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_close_default(&self, v:&Vector3)->bool{
        let dist:Vector3 = (v - (*self)).get_abs();
        return dist.is_less_equal_than(Vector3::new_x(constants::TOLERANCE).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_zero(self, tolerance:f32) ->bool{
        let dist = self.get_abs();
        return  dist.is_less_equal_than(Vector3::new_x(tolerance).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_zero_default(self)->bool{
        let dist = self.get_abs();
        return  dist.is_less_equal_than(Vector3::new_x(constants::FLOAT_EPSILON).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_less_than(self, rhs :&Vector3)->bool{
        return Vec3::cmp_all_lt(self.get_simd_value(),rhs.get_simd_value());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_less_equal_than(self, rhs:&Vector3) ->bool
    {
        return Vec3::cmp_all_lt_eq(self._value, rhs._value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_greater_than(self,rhs:&Vector3)->bool{
        return  Vec3::cmp_all_gt(self.get_simd_value(),rhs.get_simd_value());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_greater_equal_than(self,rhs:&Vector3)->bool{
        return  Vec3::cmp_all_gt_eq(self.get_simd_value(),rhs.get_simd_value());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_floor(self)->Vector3{
        return Vector3::new_float_type(Vec3::floor(self.get_simd_value())) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_ceil(self)->Vector3{
        return Vector3::new_float_type(Vec3::ceil(self.get_simd_value())) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_round(self)->Vector3{
        return  Vector3::new_float_type(Vec3::round(self.get_simd_value())) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_min(self,v :&Vector3)->Vector3{
        return  Vector3::new_float_type(Vec3::min(self.get_simd_value(),v.get_simd_value())) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_max(self,v :&Vector3)->Vector3{
        return  Vector3::new_float_type(Vec3::max(self.get_simd_value(),v.get_simd_value())) ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_clamp(self, min:&Vector3,max:&Vector3)->Vector3{
        let min_val = self.get_min(max);
        return min_val.get_max(min);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_max_element(self)->f32{
       return  constants::max(self.get_x(),constants::max(self.get_y(),self.get_z()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_min_element(self)->f32{
        return constants::min(self.get_x(),constants::min(self.get_y(),self.get_z()));
    }
    pub  unsafe fn get_sin(self)->Vector3{
        return Vector3::new_float_type(Vec3::sin(self.get_simd_value()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_cos(self)->Vector3{
        return  Vector3::new_float_type(Vec3::cos(self.get_simd_value()))
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_sin_cos(self,mut sin:&Vector3, mut cos :&Vector3){
        Vec3::sin_cos(self.get_simd_value(),sin._value.borrow(),cos._value.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_acos(self)->Vector3{
        return Vector3::new_float_type(Vec3::acos(self.get_simd_value()))
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_atan(self)->Vector3{
        return Vector3::new_float_type(Vec3::atan(self.get_simd_value()))
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_angle_mod(self)->Vector3{
        return  Vector3::new_float_type(Vec3::angle_mod(self.get_simd_value()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle(self, v:&Vector3)->f32{
        let cos =self.dot3(v)*simd::inv_sqrt((self.get_length_sq()*v.get_length_sq()));
        let res = simd::acos(constants::get_clamp(cos, (-1.0) , 1.0));
        res
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle_deg(self,v:&Vector3)->f32{
        return constants::rad_to_deg(self.angle(v))
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle_safe(self, v:&Vector3)->f32{
       return   if !self.is_zero_default()&&!v.is_zero_with_default(){
            let result = self.angle(v);
           result
        }else {
            0.0
         }
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  angle_safe_deg(self,v:&Vector3)->f32{
        return if !self.is_zero_default() && !v.is_zero_with_default() {
            let result = self.angle_deg(v);
            result
        }else{
            0.0
        }
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_abs(self)->Vector3{
        return  Vector3::new_float_type(Vec3::abs(self.get_simd_value()));
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_reciprocal(self)->Vector3{
        return  Vector3::new_float_type(Vec3::reciprocal(self.get_simd_value()));
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_reciprocal_estimate(self)->Vector3{
        return Vector3::new_float_type(Vec3::reciprocal_estimate(self.get_simd_value()));
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_m_add(self,mul :&Vector3,add :&Vector3)->Vector3{
        return  Vector3::new_float_type(Vec3::madd(self.get_simd_value(),mul.get_simd_value(),add.get_simd_value()));
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  m_add(mut self,mul :&Vector3,add :&Vector3){
        self._value = self.get_m_add(mul,add)._value;
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_perpendicular(self,v:&Vector3,tolerance:f32)->bool{
        let abs_length_sq = Vec1::abs(Vec3::dot(self.get_simd_value(), v.get_simd_value()));
        return  Vec1::select_index0(abs_length_sq)< tolerance.to_owned();
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_orthogonal_vector(self)->Vector3{
        let mut axis:Vector3 = Vector3::new();
        let val = (self.get_x() * self.get_x());
        if val < 0.5 * self.get_length_sq(){
            axis = Vector3::create_axis_x(1.0);
        }else{
            axis = Vector3::create_axis_y(1.0);
        }
        return self.cross(axis.borrow());
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  project(&mut self, rhs: &mut Vector3){
        self._value = (rhs * (self.dot3(rhs) / rhs.dot3(rhs)))._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  project_on_normal(mut self,mut normal :&Vector3){
        self._value = (normal * self.dot3(normal))._value;
    }
    pub  unsafe fn get_projected(self,mut rhs:&Vector3)->Vector3{
        return rhs * (self.dot3(rhs) / rhs.dot3(rhs));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  get_projected_on_normal(self,normal:&Vector3)->Vector3{
        return normal * self.dot3(normal);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  is_finite(self)->bool{
        return constants::is_finite_float(self.get_x())&&constants::is_finite_float(self.get_y())&&constants::is_finite_float(self.get_z());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  set_simd_value(mut self, value :FloatArgType ){
        self._value = value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  vector3rad_to_deg(radians:&Vector3)->Vector3{
        return Vector3::new_float_type(Vec3::mul(radians.get_simd_value(),Vec3::splat((180.0/constants::PI))));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn  vector3deg_to_rad(degrees:&Vector3)->Vector3{
        return  Vector3::new_float_type(Vec3::mul(degrees.get_simd_value(),Vec3::splat((constants::PI/180.0))));
    }
}

impl PartialEq<Self> for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return Vec3::cmp_all_eq(self._value, other._value); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !Vec3::cmp_all_eq(self._value, other._value); }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Self::Output {
        unsafe { Vector3 { _value: Vec3::add(self._value, rhs._value) } }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        unsafe { Vector3 { _value: Vec3::sub(self._value, rhs._value) } }
    }
}
impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(&self, rhs: &Vector3) -> Self::Output {
        unsafe { Vector3 { _value: Vec3::sub(self._value, rhs._value) } }
    }
}


impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        unsafe { Vector3 { _value: Vec3::mul(self._value, rhs._value) } }
    }

}
impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        unsafe { return Vector3::new_float_type(Vec3::mul(self.get_simd_value(), Vec3::splat(rhs))) }
    }
}
impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: &Vector3) -> Self::Output {
        let mut val =rhs._value;
        unsafe { Vector3 { _value: Vec3::div(self._value,val.borrow_mut()) } }
    }
}
impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        unsafe { Vector3 { _value: Vec3::div(self._value,Vec3::splat(rhs).borrow_mut()) } }
    }
}
impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self._value = self.add(rhs)._value;
    }
    /* */
}

impl SubAssign<Vector3> for  Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        self._value = self.sub(rhs)._value;
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: &Vector3) {
        unsafe { self._value = Vec3::mul(self._value, rhs._value); }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self = Vector3::new_float_type(Vec3::mul(self.get_simd_value(), Vec3::splat(rhs))).to_owned().borrow_mut(); }
    }
}
impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: &Vector3) {
        self._value = self.div(rhs)._value;
    }
}
impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self = Vector3::new_float_type(Vec3::div(self.get_simd_value(), Vec3::splat(rhs).borrow_mut())).to_owned().borrow_mut(); }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ve3_get_length() {
        let vec3 = Vector3::new();
        unsafe { println!("crc32 from string:{}", vec3.get_length()); }
    }
}