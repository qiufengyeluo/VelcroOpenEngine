#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::vsimd;

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
    pub unsafe fn get_length_sq(self) ->f32{

        self.dot(&Vector3{_value:self._value})
    }
    pub unsafe fn get_length(self) ->f32{
        let length = self.dot_f32_type();
        let length_sqrt =  sqrt(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_estimate(self) ->f32{
        let length = self.dot_f32_type();
        let length_sqrt =  sqrt_estimate(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_reciprocal(self) ->f32{
        let length = self.dot_f32_type();
        let length_sqrt =  sqrt_inv(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_reciprocal_estimate(self) ->f32{
        let length = self.dot_f32_type();
        let length_sqrt =  sqrt_inv_estimate(length);
        let result = select_first(length_sqrt);
        result
    }

    pub fn get_normalized()->Vector3{

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

    pub unsafe fn dot_f32(lhs:&Vector3, rhs:&Vector3) ->f32{
        let x2  =   mul(lhs.get_simd_value(), rhs.get_simd_value()) ;
        let xy  =   add(splat_second(x2), x2);
        let xyz =   add(splat_third(x2), xy);
        let result   =   select_first(splat_first(xyz));
        result
    }
    pub unsafe fn dot_f32_type(self) ->FloatType{
        let x2  =   mul(self._value,self._value) ;
        let xy  =   add(splat_second(x2), x2);
        let xyz =   add(splat_third(x2), xy);
        let result   =   splat_first(xyz);
        result
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