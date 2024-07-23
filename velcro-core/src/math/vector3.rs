#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ops::*;
use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::vector::*;
use crate::math::{constants, vsimd};

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector3 {
    _value: FloatType,
}

impl Vector3 {
    #[allow(dead_code)]
    pub fn new() -> Vector3 {
        unsafe {
            Vector3 {
                _value: zero_float(),
            }
        }
    }

    pub unsafe fn new_splat(x:f32) ->Vector3{
        let mut result:Vector3 = Self.new();
        result._value = splat(x);
        result
    }

    pub unsafe fn new_load_immediate(x:f32, y:f32, z:f32) ->Vector3{
        let mut result:Vector3 = Self.new();
        result._value = load_immediate(x,y,z,0.0);
        result
    }
    pub fn new_float_type(v :  FloatType)->Vector3{
        let mut result:Vector3 = Self.new();
        result._value = v;
        result
    }
    pub unsafe fn create_zero() ->Vector3{
        let result:Vector3 = Self.new_float_type(zero_float());
        result
    }
    
    pub fn create_one(x:f32)->Vector3{
        let result:Vector3 = Self.new_splat(1.0);
        result
    }
    pub fn create_axis_x(length:f32)->Vector3{
        let result:Vector3 = Self.new_load_immediate(length, 0.0, 0.0);
        result
    }
    pub fn create_axis_y(length:f32)->Vector3{
        let result:Vector3 = Self.new_load_immediate(0.0, length, 0.0);
        result
    }
    pub fn create_axis_z(length:f32)->Vector3{
        let result:Vector3 = Self.new_load_immediate(0.0, 0.0, length);
        result
    }
    pub fn create_from_float_3(ptr :*const f32)->Vector3{
        let val =ptr as *[f32;3];
        let result:Vector3 = Self.new_load_immediate(val[0], val[1], val[2]);
        result
    }
    pub unsafe fn create_select_cmp_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = cmp_eq(cmp1._value, cmp2._value);
        let result = Self.new_float_type( select(va._value,vb._value,mask));
        result
    }
    pub unsafe fn create_select_cmp_greater_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = cmp_gt_eq(cmp1._value, cmp2._value);
        let result = Self.new_float_type( select(va._value,vb._value,mask));
        result
    }
    pub unsafe fn create_select_cmp_greater(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = cmp_gt(cmp1._value, cmp2._value);
        let result = Self.new_float_type(select(va._value,vb._value,mask));
        result
    }
    pub fn store_to_float_3(self, &mut  ptr :*const f32){
        let mut result = ptr as *[f32;3];
        let values:*const [f32;3] = (&self._value) as *const [f32;3];

        *result[0] = values[0];
        *result[1] = values[1];
        *result[2] = values[2];
    }
    pub unsafe fn store_to_float_4(self, &mut value :*const f32){
        store_unaligned(value, self._value);
    }
    pub fn get_x(self)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[0]
    }
    pub fn get_y(self)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[1]
    }
    pub fn get_z(self)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[2]
    }
    pub fn get_element(self,index:i32)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[index]
    }
    pub fn set_x(mut self, x :f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[0] = x
    }
    pub fn set_y(mut self, y:f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[1] = y
    }
    pub fn set_z(mut self, z:f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[2] = z
    }
    pub unsafe fn set_value_x(mut self, x :f32){
        self._value = splat(x);
    }
    pub fn set_element(mut self,index:i32,v:f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[index] = v
    }
    pub unsafe fn set_value_xyz(mut self, x:f32, y:f32, z:f32){
        self._value = load_immediate(x, y, z, 0.0);
    }
    pub unsafe fn set_value_ptr(mut self, ptr:*const f32){
        let val= ptr as [f32;3];
        self._value = load_immediate(val[0],val[1],val[2], 0.0);
    }
    pub unsafe fn get_length_sq(&self) ->f32{
        let result =  dot_to_f32(self,&Vector3{_value:self._value});
        result
    }

    pub unsafe fn get_length(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_estimate(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt_estimate(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_reciprocal(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt_inv(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_reciprocal_estimate(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt_inv_estimate(length);
        let result = select_first(length_sqrt);
        result
    }

    pub unsafe fn get_normalized(self) ->Vector3{
        let tmp = normalize(self._value);
        let result = Vector3::new_float_type(tmp);
        result
    }
    pub unsafe fn get_normalized_estimate(self)->Vector3{
        let tmp = normalize_estimate(self._value);
        let result = Vector3::new_float_type(tmp);
        result
    }
    pub unsafe fn normalize(mut self){
        self = self.get_normalized();
    }
    pub unsafe fn normalize_estimate(mut self){
        self = self.get_normalized_estimate();
    }
    pub unsafe fn normalize_with_length(mut self)->f32{
        let dot_val = dot_to_f32_type(self._value,self._value);
        let sqrt_val = sqrt(dot_val);
        let length = select_first(sqrt_val);
        let splat_val = splat(length);
        self._value = div(self._value,splat_val);
        length
    }
    pub unsafe fn normalize_with_length_estimate(mut self)->f32{
        let dot_val = dot_to_f32_type(self._value,self._value);
        let sqrt_val = sqrt_estimate(dot_val);
        let length = select_first(sqrt_val);
        let splat_val = splat(length);
        self._value = div(self._value,splat_val);
        length
    }
    pub unsafe fn get_normalized_safe( self,tolerance:f32)->Vector3{
        let tmp = normalize_safe(self._value,tolerance);
        let result = Vector3::new_float_type(tmp);
        result
    }
    pub unsafe fn get_normalized_safe_estimate(self,tolerance:f32)->Vector3{
        let tmp = normalize_safe_estimate(self._value,tolerance);
        let result = Vector3::new_float_type(tmp);
        result
    }
    pub unsafe fn normalize_safe(mut self, tolerance:f32){
        self._value = normalize_safe(self._value,tolerance)
    }
    pub unsafe fn normalize_safe_estimate(mut self, tolerance:f32){
        self._value = normalize_safe_estimate(self._value,tolerance);
    }
    pub unsafe fn normalize_safe_with_length(mut self, tolerance:f32)->f32{
        let dot_val = dot_to_f32_type(self._value,self._value);
        let length = sqrt(dot_val);
        if select_first(length) < tolerance{
            self._value = zero_float();
        }else {
            let from_val = from_vec_first(length);
            let splat_val = splat_first(from_val);
            self._value = div(self._value,splat_val);
        }

        let result = select_first(length);
        result
    }
    pub unsafe fn normalize_safe_with_length_estimate(mut self, tolerance:f32) ->f32{
        let dot_val = dot_to_f32_type(self._value,self._value);
        let length = sqrt_estimate(dot_val);
        if select_first(length) < tolerance{
            self._value = zero_float();
        }else {
            let from_val = from_vec_first(length);
            let splat_val = splat_first(from_val);
            self._value = div(self._value,splat_val);
        }
        let result = select_first(length);
        result
    }
    pub unsafe fn is_normalized(self,tolerance:f32)->bool{


        let splat_val = splat(self.get_length_sq()-1.0);
        let abs_val = abs(splat_val);
        let val = select_first(abs_val);
        return val <= tolerance;
    }
    pub unsafe fn set_length(mut self, length:f32){
        let scale =   self.get_length_reciprocal() * length;
        self *= splat(scale) ;
    }
    pub unsafe fn set_length_estimate(mut self, length:f32){
        let scale = length* self.get_length_reciprocal_estimate();
        self *= splat(scale) ;
    }
    pub unsafe fn get_distance_sq(&mut self, v :&Vector3)->f32{
        self -= v;
        let result = self.get_length_sq();
        result
    }
    pub unsafe fn get_distance(&mut self, v :&Vector3)->f32{
        self -= v;
        let result = self.get_length();
        result
    }
    pub unsafe fn get_distance_estimate(&mut self, v :&Vector3)->f32{
        self -= v;
        let result = self.get_length_estimate();
        result
    }
    pub unsafe fn lerp(self,dest :&Vector3,t :&f32)->Vector3{
        let sub_val = sub(dest._value,self._value);
        let splat_val = splat(t.to_owned());

        Vector3{
            _value : madd(sub_val,splat_val,self._value)
        }
    }
    pub unsafe fn slerp(self,dest :&Vector3,t :f32)->Vector3{
        let dot_val = clamp(dot_to_f32_type(self._value,dest._value),splat(-1.0),splat(1.0));
        let theta = mul(acos(dot_val),splat(t));
        let relativeVec = sub(dest.get_simd_value(),mul(self.get_simd_value(),from_vec_first(dot_val)));
        let relVecNorm = normalize_safe(relativeVec,constants::math_utils:: TOLERANCE);
        let sin_cos = from_vec_second()
    }
    AZ_MATH_INLINE Vector3 Vector3::Slerp(const Vector3& dest, float t) const
    {
    const Simd::Vec3::FloatType sinCos = Simd::Vec3::FromVec2(Simd::Vec2::SinCos(theta));
    const Simd::Vec3::FloatType relVecSinTheta = Simd::Vec3::Mul(relVecNorm, Simd::Vec3::SplatIndex0(sinCos));
    return Vector3(Simd::Vec3::Madd(GetSimdValue(), Simd::Vec3::SplatIndex1(sinCos), relVecSinTheta));
}

AZ_MATH_INLINE Vector3 Vector3::Nlerp(const Vector3& dest, float t) const
{
return Lerp(dest, t).GetNormalizedSafe(Constants::Tolerance);
}

    pub fn get_simd_value(&self)->FloatType{
        self._value
    }

    pub fn is_close(&self, v:&Vector3, tolerance :f32) ->bool
    {
        let dist:Vector3 = (v - (*self)).GetAbs();
        return dist.is_less_equal_than(Self.new_splat(tolerance));
    }
    pub  fn is_less_equal_than(rhs:&Vector3) ->bool
    {
        unsafe { return cmp_all_lt_eq(Self._value, rhs._value, 0b0111); }
    }


}

impl Add for Vector3 {
    type Output = ();
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self { _value: add(self._value, rhs._value) } }
    }
}

impl Sub for Vector3 {
    type Output = ();

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self { _value: sub(self._value, rhs._value) } }
    }
}

impl Mul for Vector3 {
    type Output = ();

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self { _value: mul(self._value, rhs._value) } }
    }

}

impl Div for Vector3 {
    type Output = ();

    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Self { _value: div(self._value, rhs._value) } }
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