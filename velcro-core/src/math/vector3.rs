#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use vsimd::neon::*;
use vsimd::sse::*;
use crate::math::vsimd;

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector3 {
    _x: f32,
    _y: f32,
    _z: f32,
    _value: FloatType,
    _values:[f32; 3],
}

impl Vector3 {
    #[allow(dead_code)]
    pub fn new() -> Vector3 {
        unsafe {
            Vector3 {
                _x: 0.0,
                _y: 0.0,
                _z: 0.0,
                _value: zero_float(),
                _values: [0.0; 3],
            }
        }
    }

    pub fn new_splat(x:f32)->Vector3{
        let mut result:Vector3 = Self.new();
        unsafe { result._value = splat(x); }
        result
    }

    pub fn new_load_immediate(x:f32,y:f32,z:f32)->Vector3{
        let mut result:Vector3 = Self.new();
        unsafe { result._value = load_immediate(x,y,z,0.0); }
        result
    }
    pub fn new_float_type(v :  FloatType)->Vector3{
        let mut result:Vector3 = Self.new();
        result._value = v;
        result
    }
    pub fn create_zero() ->Vector3{
        let result:Vector3 = Self.new_float_type(unsafe { zero_float() });
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
    pub  fn create_select_cmp_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = unsafe { cmp_eq(cmp1._value, cmp2._value) };
        let result = Self.new_float_type( unsafe { select(va._value,vb._value,mask)});
        result
    }
    pub fn create_select_cmp_greater_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = unsafe { cmp_gt_eq(cmp1._value, cmp2._value) };
        let result = Self.new_float_type( unsafe { select(va._value,vb._value,mask)});
        result
    }
    pub fn create_select_cmp_greater(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = unsafe { cmp_gt(cmp1._value, cmp2._value) };
        let result = Self.new_float_type( unsafe { select(va._value,vb._value,mask)});
        result
    }
    pub fn store_to_float_3(self, &mut  ptr :*const f32){
        let mut result = ptr as *[f32;3];
        *result[0] = self._values[0];
        *result[1] = self._values[1];
        *result[2] = self._values[2];
    }
    pub fn store_to_float_4(self,&mut value :*const f32){
        unsafe { store_unaligned(value, self._value) }
    }
    pub fn get_x(self)->f32{
        self._x
    }
    pub fn get_y(self)->f32{
        self._y
    }
    pub fn get_z(self)->f32{
        self._z
    }
    pub fn get_element(self,index:i32)->f32{
        self._values[index]
    }
    pub fn set_x(mut self, x :f32){
        self._x = x
    }
    pub fn set_y(mut self, y:f32){
        self._y = y
    }
    pub fn set_z(mut self, z:f32){
        self._z = z
    }
    pub fn set_splat(mut self,x :f32){
        unsafe { self._value = splat(x) }
    }
    pub fn set_element(mut self,index:i32,v:f32){
        self._values[index] = v
    }
    pub fn set_load_immediate(mut self,x:f32,y:f32,z:f32){
        unsafe { self._value = load_immediate(x, y, z, 0.0) }
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