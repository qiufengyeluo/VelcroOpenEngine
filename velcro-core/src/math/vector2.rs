#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ops::*;
use std::ops::{Add, Div, Mul, Sub};

#[cfg(target_arch = "arm")]
#[allow(dead_code)]
use vsimd::neon::*;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use vsimd::sse::FloatType;

use crate::math::*;
use crate::math::common_sse::{Vec2Type, VecTwoType, VecType};
use crate::math::constants::*;
use crate::math::math_utils::*;
use crate::math::simd_math::*;
use crate::math::simd_math_vec1_sse::Vec1;
use crate::math::simd_math_vec2_sse::Vec2;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::FloatArgType;

// PartialEq 是否相等
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    _value: FloatType,
}

impl Mul<f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        unsafe {
            Vector2 {
                _value: Vec2::mul(self._value.borrow(), Vec2::splat(rhs.borrow()).borrow()),
            }
        }
    }
}
impl Sub<Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        unsafe {
            Vector2 {
                _value: Vec2::sub(self._value.borrow(), rhs._value.borrow()),
            }
        }
    }
}
impl Vector2 {
    pub unsafe fn new()->Vector2{
        Vector2{
            _value:Vec2::zero_float(),
        }
    }
    pub unsafe  fn new_x(x:&f32)->Vector2{
        Vector2{
            _value: Vec2::splat(x),
        }
    }
    pub unsafe fn new_xy(x:&f32,y:&f32)->Vector2{
        Vector2{
            _value:Vec2::load_immediate(x,y),
        }
    }
    pub unsafe fn new_float_type(value:&FloatType)->Vector2{
        Vector2{
            _value:value.to_owned(),
        }
    }

    pub unsafe fn new_vector3(source:&Vector3)->Vector2{
        Vector2{
            _value:source.get_simd_value(),
        }
    }

    pub unsafe fn new_vector4(source:&Vector4)->Vector2{
        Vector2{
            _value:source.get_simd_value(),
        }
    }

    pub unsafe fn create_zero()->Vector2{
        return Vector2::new_x(0.0.borrow());
    }

    pub unsafe fn create_one() ->Vector2{
        return Vector2::new_x(1.0.borrow());
    }

    pub unsafe fn create_axis_x(length:&f32)->Vector2{
       let result = Vector2::new_xy(length,0.0.borrow());
        result
    }

    pub unsafe fn create_axis_y(length:&f32)->Vector2{
        let result = Vector2::new_xy(0.0.borrow(),length);
        result
    }


    pub unsafe fn create_from_float2(values:*const f32)->Vector2{
        let arr = values as *[f32;2];
        let result = Vector2::new_xy((*arr[0]).borrow(),(*arr[1]).borrow());
        result
    }

    pub unsafe fn create_from_angle(angle:&f32) ->Vector2{
        let mut sin : f32;
        let mut cos : f32;
        simd_sin_cos(angle.borrow(),sin.borrow_mut(),cos.borrow_mut());
        let result = Vector2::new_xy(sin.borrow(),cos.borrow());
        result
    }

    pub unsafe fn create_select_cmp_equal(cmp1:&Vector2,cmp2:&Vector2,va:&Vector2,vb:&Vector2)->Vector2{
        let mask = Vec2::cmp_eq(cmp1._value.borrow(),cmp2._value.borrow());
        let result = Vector2::new_float_type(Vec2::select(va._value.borrow(),vb._value.borrow(),mask.borrow()).borrow());
        result
    }

    pub unsafe fn create_select_cmp_greater_equal(cmp1:&Vector2,cmp2:&Vector2,va:&Vector2,vb:&Vector2)->Vector2{
        let mask = Vec2::cmp_gt_eq(cmp1._value.borrow(),cmp2._value.borrow());
        let result = Vector2::new_float_type(Vec2::select(va._value.borrow(),vb._value.borrow(),mask.borrow()).borrow());
        result
    }

    pub unsafe fn create_select_cmp_greater(cmp1:&Vector2, cmp2:&Vector2, va:&Vector2, vb:&Vector2) ->Vector2{
        let mask = Vec2::cmp_gt(cmp1._value.borrow(),cmp2._value.borrow());
        let result = Vector2::new_float_type(Vec2::select(va._value.borrow(),vb._value.borrow(),mask.borrow()).borrow());
        result
    }

    pub unsafe fn store_to_float2(self,mut value : *const f32){
        let values = *self._value as *const f32;
        *value[0] = *values[0];
        *value[1] = *values[1];
    }

    pub fn get_x(self)->f32{
        let values = *self._value as *const f32;
        *values[0]
    }

    pub fn get_y(self)->f32{
        let values = *self._value as *const f32;
        *values[1]
    }
    pub fn set_x(mut self,x:&f32){
        let values = *self._value as *const f32;
        *values[0] = x
    }

    pub fn set_y(mut self,y :&f32){
        let values = *self._value as *const f32;
        *values[1] = y
    }

    pub fn get_element(self, index :&i32) ->f32{
        let values = *self._value as *const f32;
        values[index]
    }


    pub fn set_element(mut self,index:&i32,value:&f32){
        let values = *self._value as *const f32;
        *values[index] = value
    }

    pub unsafe  fn set_splat_x(mut self,x:&f32){
        self._value = Vec2::splat(x);
    }

    pub unsafe fn set_load_immediate(mut self,x:&f32,y:&f32){
        self._value = Vec2::load_immediate(x,y);
    }

    pub unsafe fn get_length_sq(self)->f32{
        return self.dot_vec2(self.borrow());
    }

    pub  unsafe fn get_length(self) ->f32{
        return Vec1::select_index0(Vec1::sqrt(Vec2::dot(self._value.borrow(),self._value.borrow()).borrow()).borrow());
    }

    pub unsafe fn get_length_estimate(self) ->f32{
        return Vec1::select_index0(Vec1::sqrt_estimate(Vec2::dot(self._value.borrow(),self._value.borrow()).borrow()).borrow());
    }

    pub unsafe fn get_length_reciprocal(self) ->f32{
        return Vec1::select_index0(Vec1::sqrt_inv(Vec2::dot(self._value.borrow(),self._value.borrow()).borrow()).borrow());
    }

    pub unsafe fn get_length_reciprocal_estimate(self) ->f32{
        return Vec1::select_index0(Vec1::sqrt_inv_estimate(Vec2::dot(self._value.borrow(),self._value.borrow()).borrow()).borrow());
    }

    pub unsafe fn get_normalized(self) ->Vector2{
        return Vector2::new_float_type(Vec2::normalize(self._value.borrow()).borrow());
    }

    pub unsafe fn get_normalized_estimate(self) ->Vector2{
        return  Vector2::new_float_type(Vec2::normalize_estimate(self._value.borrow()).borrow());
    }

    pub unsafe fn normalize(mut self){
        self._value = Vec2::normalize(self._value.borrow());
    }

    pub unsafe fn normalize_estimate(mut self){
        self._value = Vec2::normalize_estimate(self._value.borrow());
    }

    pub unsafe fn normalize_with_length(mut self)->f32{
        let length = self.get_length();
        self  *= 1.0/length;
        return length
    }

    pub unsafe fn normalize_with_length_estimate(mut self) ->f32{
        let length = self.get_length_estimate();
        self  *= 1.0/length;
        return length
    }

    pub unsafe fn get_normalized_safe(self, tolerance:&f32) ->Vector2{
        return Vector2::new_float_type(Vec2::normalize_safe(self._value.borrow(),tolerance).borrow());
    }

    pub  unsafe fn get_normalized_safe_estimate(self, tolerance:&f32) ->Vector2{
        return  Vector2::new_float_type(Vec2::normalize_safe_estimate(self._value.borrow(),tolerance).borrow())
    }

    pub unsafe fn normalize_safe(mut self, tolerance:&f32){
        self._value = Vec2::normalize_safe(self._value.borrow(),tolerance);
    }

    pub unsafe fn normalize_safe_estimate(mut self, tolerance:&f32){
        self._value = Vec2::normalize_safe_estimate(self._value.borrow(),tolerance);
    }

    pub unsafe fn normalize_safe_with_length(mut self, tolerance:&f32) ->f32{
        let length = Vec1::sqrt(Vec2::dot(self._value.borrow(),self._value.borrow()).borrow());
        if Vec1::select_index0(length.borrow()) < tolerance.to_owned(){
            self._value = Vec2::zero_float();
        }else {
            let mut val = Vec2::splat_index0(Vec2::from_vec1(length.borrow()).borrow());
            self._value = Vec2::div(self._value.borrow(),val.borrow_mut());
        }
        return Vec1::select_index0(length.borrow());

    }

    pub unsafe fn normalize_safe_with_length_estimate(mut self, tolerance:&f32) ->f32{
        let length = Vec1::sqrt_estimate(Vec2::dot(self._value.borrow(),self._value.borrow()).borrow());
        if Vec1::select_index0(length.borrow()) < tolerance.to_owned(){
            self._value = Vec2::zero_float();
        }else {
            let mut val = Vec2::splat_index0(Vec2::from_vec1(length.borrow()).borrow());
            self._value = Vec2::div(self._value.borrow(),val.borrow_mut());
        }
        return Vec1::select_index0(length.borrow());

    }

    pub unsafe fn is_normalized(self, tolerance:&f32) ->bool{
        return simd_abs((self.get_length_sq()-1.0).borrow())<=tolerance.to_owned();
    }

    pub unsafe fn set_length(mut self, length:&f32){
        let scale = length/self.get_length();
        self._value = Vec2::mul(self._value.borrow(),Vec2::splat(scale.borrow()).borrow());
    }

    pub unsafe fn set_length_estimate(mut self,length:&f32){
        let scale = length / self.get_length_estimate();
        self._value = Vec2::mul(self._value.borrow(),Vec2::splat(scale.borrow()).borrow());
    }

    pub unsafe fn get_distance_sq(mut self, v:&Vector2) ->f32{
        return  (*self - v).get_length_sq();
    }

    pub unsafe fn get_distance(mut self, v:&Vector2) ->f32{
        return  (*self - v).get_length();
    }

    pub unsafe fn get_distance_estimate(self, v:&Vector2) ->f32{
        return  (*self - v).get_length_estimate();
    }

    pub unsafe fn lerp(self,dest:&Vector2,t:&f32)->Vector2{
        return Vector2::new_float_type(Vec2::madd(Vec2::sub(dest._value.borrow(),self._value.borrow()).borrow(),Vec2::splat(t).borrow(),self._value.borrow()).borrow());

    }

    pub unsafe fn slerp(self,dest:&Vector2,t:&f32)->Vector2{
        let dot = Vec1::clamp(Vec2::dot(self._value.borrow(),dest._value.borrow()).borrow(),Vec1::splat(-1.0.borrow()).borrow(),Vec1::splat(1.0.borrow()).borrow());
        let theta = Vec1::mul(Vec1::acos(dot.borrow()).borrow(),Vec1::splat(t).borrow());
        let relative_vec = Vec2::sub(dest.get_simd_vaue().borrow(), Vec2::mul(self.get_simd_value().borrow(), Vec2::from_vec1(dot.borrow())));
        let rel_vec_norm = Vec2::normalize_safe(relative_vec.borrow(), TOLERANCE.borrow());
        let sin_cos_val = Vec2::sin_cos_to_float_type(theta.borrow());
        let rel_vec_sin_theta = Vec2::mul(rel_vec_norm.borrow(), Vec2::splat_index0(sin_cos_val.borrow()).borrow());
        return  Vector2::new_float_type(Vec2::madd(self.get_simd_value().borrow(), Vec2::splat_index1(sin_cos_val.borrow()).borrow(), rel_vec_sin_theta.borrow()).borrow());

    }

    pub unsafe fn nlerp(self,dest:&Vector2,t:&f32)->Vector2{
        return self.lerp(dest,t).get_normalized_safe(TOLERANCE.borrow());
    }

    pub unsafe fn get_perpendicular(self) ->Vector2{
        return Vector2::new_xy((-self.get_y()).borrow(), self.get_x().borrow());
    }

    pub unsafe fn is_close(self,v:&Vector2,tolerance:&f32)->bool{
        let dist = (v - (*self)).get_abs();
        return dist.is_less_equal_than(Vector2::new_x(tolerance).borrow());
    }

    pub unsafe fn is_zero(self, tolerance:&f32) ->bool{
        let dist = self.get_abs();
        return  dist.is_less_equal_than(Vector2::new_x(tolerance).borrow());
    }

    pub unsafe fn is_less_than(self,v:&Vector2)->bool{
        return  Vec2::cmp_all_lt(self._value.borrow(),v._value.borrow());
    }

    pub unsafe fn is_less_equal_than(self, v:&Vector2) ->bool{
        return  Vec2::cmp_all_lt_eq(self._value.borrow(),v._value.borrow());
    }

    pub unsafe fn is_greater_than(self, v:&Vector2) ->bool{
        return Vec2::cmp_all_gt(self._value.borrow(),v._value.borrow());
    }

    pub unsafe fn is_greater_equal_than(self, v:&Vector2) ->bool{
        return Vec2::cmp_all_gt_eq(self._value.borrow(),v._value.borrow());
    }

    pub unsafe fn get_floor(self) ->Vector2{
        return Vector2::new_float_type(Vec2::floor(self._value.borrow()).borrow());
    }

    pub unsafe fn get_ceil(self) ->Vector2{
        return  Vector2::new_float_type(Vec2::ceil(self._value.borrow()).borrow());
    }

    pub unsafe fn get_round(self) ->Vector2{
        return Vector2::new_float_type(Vec2::round(self._value.borrow()).borrow());
    }

    pub unsafe fn get_min(self, v:&Vector2) ->Vector2{
        return  Vector2::new_float_type(Vec2::min(self._value.borrow(),v._value.borrow()).borrow())
    }

    pub unsafe fn get_max(self,v:&Vector2)->Vector2{
        return Vector2::new_float_type(Vec2::max(self._value.borrow(),v._value.borrow()).borrow());
    }

    pub unsafe fn get_clamp(self,min:&Vector2,max:&Vector2)->Vector2{
        return self.get_min(max).get_max(min);
    }

    pub unsafe fn get_select(self, v_cmp:&Vector2, vb:&Vector2) ->Vector2{
        let mask = Vec2::cmp_eq(v_cmp._value.borrow(),Vec2::zero_float().borrow());
        return  Vector2::new_float_type(Vec2::select(self._value.borrow(),vb._value.borrow(),mask.borrow()).borrow());
    }

    pub unsafe fn select(mut self, v_cmp:&Vector2, vb:&Vector2){
        self._value = self.get_select(v_cmp,vb)._value;
    }

    pub unsafe fn get_abs(self) ->Vector2{
        return Vector2::new_float_type(Vec2::abs(self._value.borrow()).borrow());
    }

    pub unsafe fn get_sin(self) ->Vector2{
        return Vector2::new_float_type(Vec2::sin(self._value.borrow()).borrow());
    }

    pub unsafe fn get_cos(self) ->Vector2{
        return Vector2::new_float_type(Vec2::cos(self._value.borrow()).borrow());

    }

    pub unsafe fn get_sin_cos(self, mut sin: &mut Vector2, mut cos: &mut Vector2){
        let mut sin_values:FloatType;
        let mut cos_values:FloatType;
        Vec2::sin_cos(self._value.borrow(), sin_values.borrow_mut(), cos_values.borrow_mut());
        sin._value = Vector2::new_float_type(sin_values.borrow())._value;
        cos._value = Vector2::new_float_type(cos_values.borrow())._value;
    }

    pub unsafe fn get_acos(self) ->Vector2{
        return Vector2::new_float_type(Vec2::acos(self._value.borrow()).borrow());
    }

    pub unsafe fn get_atan(self) ->Vector2{
        return  Vector2::new_float_type(Vec2::atan(self._value.borrow()).borrow());
    }

    pub unsafe fn get_atan2(self) ->f32{
        return Vec1::select_index0(Vec2::atan2_float_type(self._value.borrow()).borrow());
    }

    pub unsafe fn get_angle_mod(self) ->Vector2{
        return Vector2::new_float_type(Vec2::angle_mod(self._value.borrow()).borrow());
    }

    pub unsafe fn angle(self,v:&Vector2) ->f32{
        let cos = self.dot2(v) * simd_inv_sqrt(self.get_length_sq() * v.GetLengthSq());
        let res = simd_acos(get_clamp(cos.borrow(),-1.0.borrow(),1.0.borrow()).borrow());
        return res;
    }

    pub unsafe fn angle_deg(self, v:&Vector2) ->f32{
        return rad_to_deg(self.angle(v).borrow());
    }

    pub unsafe fn angle_safe(self, v:&Vector2) ->f32{
        return  if !self.is_zero(FLOAT_EPSILON.borrow())&& !v.is_zero(FLOAT_EPSILON.borrow()){
            let result =self.angle(v);
            result
        }else {
            0.0
        }
    }

    pub unsafe fn angle_safe_deg(self,v:&Vector2)->f32{
        return if !self.is_zero(FLOAT_EPSILON.borrow()) && !v.is_zero(FLOAT_EPSILON.borrow()){
            let result =self.angle_deg(v);
            result
        }else {
            0.0
        }
    }

    pub unsafe fn get_reciprocal(self) ->Vector2{
        return  Vector2::new_float_type(Vec2::reciprocal(self._value.borrow()).borrow());
    }

    pub unsafe fn get_reciprocal_estimate(self) ->Vector2{
        return Vector2::new_float_type(Vec2::reciprocal_estimate(self._value.borrow()).borrow());
    }
    pub unsafe fn dot2(self,rhs:&Vector2)->f32{
        return Vec1::select_index0(Vec2::dot(self._value.borrow(),rhs._value.borrow()).borrow());
    }

    pub unsafe fn get_m_add(self, mul:&Vector2, add:&Vector2) ->Vector2{
        return Vector2::new_float_type(Vec2::madd(self._value.borrow(),mul._value.borrow(),add._value.borrow()).borrow());
    }

    pub unsafe fn m_add(mut self,mul:&Vector2,add:&Vector2){
        self._value = self.get_m_add(mul,add)._value;
    }

    pub unsafe fn project(mut self, rhs:&Vector2){
        self._value = (rhs * (self.dot2(rhs)/rhs.dot2(rhs)))._value;
    }

    pub unsafe fn project_on_normal(mut self, normal:&Vector2){
        self._value = (normal * self.dot2(normal))._value;
    }

    pub unsafe fn get_projected(self, rhs:&Vector2) ->Vector2{
        return rhs*(self.dot2(rhs)/rhs.dot2(rhs));
    }
    pub unsafe fn get_projected_on_normal(self, normal:&Vector2) ->Vector2{
        return normal *self.dot2(normal);
    }

    pub unsafe fn is_finite(self) ->bool{
        return is_finite_float(self.get_x().borrow())&&is_finite_float(self.get_y().borrow());
    }

    pub fn get_simd_value(self) ->FloatType{
        self._value
    }

    pub fn set_simd_value(mut self, value:&FloatArgType){
        self._value = value.to_owned();
    }
}
impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Vector2 { _value: Vec2::add(self._value.borrow(), rhs._value.borrow()) } }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Vector2 { _value: Vec2::sub(self._value.borrow(), rhs._value.borrow()) } }
    }
}

impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: &Vector2) -> Self::Output {
        unsafe { Vector2 { _value: Vec2::mul(self._value.borrow(), rhs._value.borrow()) } }
    }

}

impl Div for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: &mut Vector2) -> Self::Output {
        unsafe { Vector2 { _value: Vec2::div(self._value.borrow(), rhs._value.borrow_mut()) } }
    }
}

impl Add<Vector2> for &mut Vector2 {
    type Output = Vector2;

    fn add(self, mut rhs: Vector2) -> Self::Output {
        unsafe {
            Vector2 {
                _value: Vec2::add(self._value.borrow(), rhs._value.borrow_mut()),
            }
        }
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self = &mut (self + rhs);
    }
    /* */
}

impl Sub<Vector2> for &mut Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        unsafe {
            Vector2 {
                _value: Vec2::sub(self._value.borrow(), rhs._value.borrow()),
            }
        }
    }
}

impl SubAssign<Vector2> for  Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self = &mut (self - rhs);
    }
}

impl Mul<Vector2> for &mut Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        unsafe {
            Vector2 {
                _value: Vec2::mul(self._value.borrow(), rhs._value.borrow()),
            }
        }
    }
}

impl MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self = &mut (self * rhs);
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self._value = Vec2::mul(self.get_simd_value().borrow(), Vec2::splat(rhs.borrow()).borrow()); }
    }
}

impl Div<Vector2> for &mut Vector2 {
    type Output = Vector2;

    fn div(self, mut rhs: Vector2) -> Self::Output {
        unsafe {
            Vector2 {
                _value: Vec2::div(self._value.borrow(), rhs._value.borrow_mut()),
            }
        }
    }
}

impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self = &mut (self / rhs);
    }
}
impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self = Vector2::new_float_type(Vec2::div(self.get_simd_value().borrow(), Vec2::splat(rhs.borrow()).borrow_mut()).borrow()).to_owned().borrow_mut(); }
    }
}

impl PartialEq<Self> for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return Vec2::cmp_all_eq(self._value.borrow(), other._value.borrow()); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !Vec2::cmp_all_eq(self._value.borrow(), other._value.borrow()); }
    }
}
