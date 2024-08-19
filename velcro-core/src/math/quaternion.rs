#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::math::common_sse::{Vec4Type, VecFourthType, VecThirdType, VecTwoType, VecType};
use crate::math::constants::{FLOAT_EPSILON, G_NEGATE_XMASK, G_NEGATE_XYZMASK};
use crate::math::math_utils::get_clamp;
use crate::math::simd_math::{simd_acos, simd_atan2, simd_inv_sqrt, simd_sin_cos, simd_sqrt};
use crate::math::simd_math_vec1_sse::Vec1;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::{FloatArgType, FloatType};

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
   _value:FloatType,
}

impl PartialEq<Self> for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return Vec4::cmp_all_eq(self._value.borrow(), other._value.borrow()); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !Vec4::cmp_all_eq(self._value.borrow(), other._value.borrow()); }
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self::Output {
        unsafe {
            return Quaternion {
                _value: Vec4::mul(self._value.borrow(),Vec3::splat(rhs.borrow()).borrow())
            }
        }
    }
}

impl Mul<&f32> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &f32) -> Self::Output {
        unsafe { return Quaternion::new_float_type(Vec4::mul(self._value.borrow(),Vec4::splat(rhs).borrow()).borrow()) }
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: &f32) -> Self::Output {
        unsafe { return Quaternion::new_float_type(Vec4::div(self._value.borrow(),Vec4::splat(rhs).borrow()).borrow()) }
    }
}



impl Sub for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Self) -> Self::Output {
        let negate_mask = unsafe { Vec4::load_aligned_i128(G_NEGATE_XMASK.borrow()) };
        unsafe { return Quaternion::new_float_type(Vec4::xor(self._value.borrow(), Vec4::cast_to_float(negate_mask.borrow()).borrow()).borrow()) }
    }
}

impl Add<Quaternion> for Quaternion{
    type Output = Quaternion;

    fn add(self, rhs: Quaternion) -> Self::Output {
        unsafe { return Quaternion::new_float_type(Vec4::add(self._value.borrow(), rhs._value.borrow()).borrow()); }
    }
}


impl Sub<Quaternion> for Quaternion{
    type Output = Quaternion;

    fn sub(self, rhs: Quaternion) -> Self::Output {
        unsafe { return Quaternion::new_float_type(Vec4::sub(self._value.borrow(), rhs._value.borrow()).borrow()); }
    }
}

impl Mul<Quaternion> for Quaternion{
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        unsafe { return Quaternion::new_float_type(Vec4::quaternion_multiply(self._value.borrow(), rhs._value.borrow()).borrow()); }
    }
}

impl Mul<f32> for Quaternion{
    type Output = Quaternion;

    fn mul(self, multiplier: f32) -> Self::Output {
        unsafe { return Quaternion::new_float_type(Vec4::mul(self._value.borrow(),Vec4::splat(multiplier.borrow()).borrow()).borrow()); }
    }
}



impl Div<f32> for Quaternion{
    type Output = Quaternion;

    fn div(self, divisor: f32) -> Self::Output {
        unsafe { return Quaternion::new_float_type(Vec4::div(self._value.borrow(),Vec4::splat((1.0/divisor).borrow()).borrow()).borrow()); }
    }
}


impl AddAssign<Quaternion> for Quaternion{
    fn add_assign(&mut self, rhs: Quaternion) {
        self._value = (self.to_owned() + rhs)._value;
    }
}

impl SubAssign<Quaternion> for Quaternion{
    fn sub_assign(&mut self, rhs: Quaternion) {
        self._value = (self.to_owned() - rhs)._value;
    }
}


impl MulAssign<Quaternion> for Quaternion{
    fn mul_assign(&mut self, rhs: Quaternion) {
        self._value = (self.to_owned() * rhs)._value;
    }
}

impl MulAssign<f32> for Quaternion{
    fn mul_assign(&mut self, rhs: f32) {
        self._value = (self.to_owned() * rhs)._value;
    }
}

impl DivAssign<f32> for Quaternion{
    fn div_assign(&mut self, rhs: f32) {
        self._value = (self.to_owned() / rhs)._value;
    }
}
impl Quaternion {

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new() ->Quaternion{
        Quaternion{
            _value:Vec3::zero_float()
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub fn new_q(q:&Quaternion)->Quaternion{
        Quaternion{
            _value:q._value,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn new_x(x:&f32)->Quaternion{
        Quaternion{
            _value:Vec4::splat(x),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn new_xyzw(x:&f32,y:&f32,z:&f32,w:&f32)->Quaternion{
        Quaternion{
            _value:Vec4::load_immediate(x,y,z,w),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn new_vec3_w(v:&Vector3,w:&f32)->Quaternion{
        let result = Vector4::new_vec3_w(v,w);
        Quaternion{
            _value:result.get_simd_value(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn new_float_type(value:&FloatArgType)->Quaternion{
        Quaternion{
            _value:value.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_identity()->Quaternion{
        return Quaternion::new_float_type(Vec4::load_immediate(0.0.borrow(),0.0.borrow(),0.0.borrow(),1.0.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_zero()->Quaternion{
        return Quaternion::new_float_type(Vec4::zero_float().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_float4(values:*const f32)->Quaternion{
        return Quaternion::new_float_type(Vec4::load_unaligned(values).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3(v:&Vector3)->Quaternion{
        return Quaternion::new_float_type(Vec4::from_vec3(v.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_euler_degrees_xyz(euler_degrees:&Vector3) ->Quaternion{
        return Quaternion::create_from_euler_radians_xyz(Vector3::vector3deg_to_rad(euler_degrees).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_euler_degrees_yxz(euler_degrees:&Vector3) ->Quaternion{
        return Quaternion::create_from_euler_radians_yxz(Vector3::vector3deg_to_rad(euler_degrees).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_euler_degrees_zyx(euler_degrees:&Vector3) ->Quaternion{
        return Quaternion::create_from_euler_radians_zyx(Vector3::vector3deg_to_rad(euler_degrees).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3and_value(v:&Vector3,w:&f32)->Quaternion{
        return Quaternion::new_vec3_w(v,w);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_x(angle_in_radians:&f32) ->Quaternion{
        let half_angle = 0.5f32 * angle_in_radians;
        let mut sin:f32;
        let mut cos:f32;
        simd_sin_cos(half_angle.borrow(), sin.borrow_mut(), cos.borrow_mut());
        return Quaternion::new_xyzw(sin.borrow(),0.0.borrow(),0.0.borrow(),cos.borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_y(angle_in_radians:&f32) ->Quaternion{
        let half_angle = 0.5f32 * angle_in_radians;
        let mut sin:f32;
        let mut cos:f32;
        simd_sin_cos(half_angle.borrow(), sin.borrow_mut(), cos.borrow_mut());
        return Quaternion::new_xyzw(0.0.borrow(),sin.borrow(),0.0.borrow(),cos.borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_z(angle_in_radians:&f32) ->Quaternion{
        let half_angle = 0.5f32 * angle_in_radians;
        let mut sin:f32;
        let mut cos:f32;
        simd_sin_cos(half_angle.borrow(), sin.borrow_mut(), cos.borrow_mut());
        return Quaternion::new_xyzw(0.0.borrow(),0.0.borrow(),sin.borrow(),cos.borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_euler_radians_xyz(euler_radians:&Vector3) ->Quaternion{
        let half = Vec3::splat(0.5.borrow());
        let angles = Vec3::mul(half.borrow(), euler_radians.get_simd_value().borrow());
        let mut sin:FloatType;
        let mut cos :FloatType;
        Vec3::sin_cos(angles.borrow(),sin.borrow_mut(),cos.borrow_mut());
        let sx = Vec3::select_index0(sin.borrow());
        let cx = Vec3::select_index0(cos.borrow());
        let sy = Vec3::select_index1(sin.borrow());
        let cy = Vec3::select_index1(cos.borrow());
        let sz = Vec3::select_index2(sin.borrow());
        let cz = Vec3::select_index2(cos.borrow());
        return Quaternion::new_xyzw((cx * sy * sz + sx * cy * cz).borrow(),
                                    (cx * sy * cz - sx * cy * sz).borrow(),
                                    (cx * cy * sz + sx * sy * cz).borrow(),
                                    (cx * cy * cz - sx * sy * sz).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_euler_radians_yxz(euler_radians:&Vector3) ->Quaternion{
        let half = Vec3::splat(0.5.borrow());
        let angles = Vec3::mul(half.borrow(), euler_radians.get_simd_value().borrow());
        let mut sin:FloatType;
        let mut cos :FloatType;
        Vec3::sin_cos(angles.borrow(),sin.borrow_mut(),cos.borrow_mut());
        let sx = Vec3::select_index0(sin.borrow());
        let cx = Vec3::select_index0(cos.borrow());
        let sy = Vec3::select_index1(sin.borrow());
        let cy = Vec3::select_index1(cos.borrow());
        let sz = Vec3::select_index2(sin.borrow());
        let cz = Vec3::select_index2(cos.borrow());
        return Quaternion::new_xyzw((cy * sx * cz + sy * cx * sz).borrow(),
                                    (sy * cx * cz - cy * sx * sz).borrow(),
                                    (cy * cx * sz - sy * sx * cz).borrow(),
                                    (cy * cx * cz + sy * sx * sz).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_euler_radians_zyx(euler_radians:&Vector3) ->Quaternion{
        let half = Vec3::splat(0.5.borrow());
        let angles = Vec3::mul(half.borrow(), euler_radians.get_simd_value().borrow());
        let mut sin:FloatType;
        let mut cos :FloatType;
        Vec3::sin_cos(angles.borrow(),sin.borrow_mut(),cos.borrow_mut());
        let sx = Vec3::select_index0(sin.borrow());
        let cx = Vec3::select_index0(cos.borrow());
        let sy = Vec3::select_index1(sin.borrow());
        let cy = Vec3::select_index1(cos.borrow());
        let sz = Vec3::select_index2(sin.borrow());
        let cz = Vec3::select_index2(cos.borrow());
        return Quaternion::new_xyzw((sx * cy * cz - cx * sy * sz).borrow(),
                                    (cx * sy * cz + sx * cy * sz).borrow(),
                                    (cx * cy * sz - sx * sy * cz).borrow(),
                                    (cx * cy * cz + sx * sy * sz).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_matrix3x3(m:&Matrix3x3)->Quaternion{
        return Quaternion::create_from_basis(m.get_basis_x().borrow(),m.get_basis_y().borrow(),m.get_basis_z().borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_matrix3x4(m:&Matrix3x4) ->Quaternion{
        return Quaternion::create_from_basis(m.get_basis_x().borrow(),m.get_basis_y().borrow(),m.get_basis_z().borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_matrix4x4(m:&Matrix4x4) ->Quaternion{
        return Quaternion::create_from_basis(m.get_basis_x_as_vector3().borrow(),m.get_basis_y_as_vector3().borrow(),m.get_basis_z_as_vector3().borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_basis(basis_x:&Vector3, basis_y:&Vector3, basis_z:&Vector3) ->Quaternion{
        let mut trace:f32;
        let mut result:Quaternion = Quaternion::new();
        if basis_z.get_z() < 0.0f32 {
            if basis_x.get_x() > basis_y.get_y() {
                trace = 1.0f32 + basis_x.get_x() - basis_y.get_y() - basis_z.get_z();
                result =Quaternion::new_xyzw(trace.borrow(), (basis_x.get_y() + basis_y.get_x()).borrow(), (basis_z.get_x() + basis_x.get_z()).borrow(), (basis_y.get_z() - basis_z.get_y()).borrow());
            }else {
                trace = 1.0f32 - basis_x.get_x() + basis_y.get_y() - basis_z.get_z();
                result = Quaternion::new_xyzw((basis_x.get_y() + basis_y.get_x()).borrow(), trace.borrow(), (basis_y.get_z() + basis_z.get_y()).borrow(), (basis_z.get_x() - basis_x.get_z()).borrow());
            }
        }
        else
        {
            if (basis_x.get_x() < -basis_y.get_y())
            {
                trace = 1.0f32 - basis_x.get_x() - basis_y.get_y() + basis_z.get_z();
                result = Quaternion::new_xyzw((basis_z.get_x() + basis_x.get_z()).borrow(), (basis_y.get_z() + basis_z.get_y()).borrow(), trace.borrow(), (basis_x.get_y() - basis_y.get_x()).borrow());
            }
            else
            {
                trace = 1.0f32 + basis_x.get_x() + basis_y.get_y() + basis_z.get_z();
                result = Quaternion::new_xyzw((basis_y.get_z() - basis_z.get_y()).borrow(), (basis_z.get_x() - basis_x.get_z()).borrow(), (basis_x.get_y() - basis_y.get_x()).borrow(), trace.borrow());
            }
        }
        return result * (0.5f32 *simd_inv_sqrt(trace.borrow()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_axis_angle(axis:&Vector3,angle:&f32)->Quaternion{
        let half_angle = 0.5f32*angle;
        let mut sin:f32;
        let mut cos:f32;
        simd_sin_cos(half_angle.borrow(), sin.borrow_mut(), cos.borrow_mut());
        return Quaternion::create_from_vector3and_value((axis*sin.to_owned()).borrow(),cos.borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_scaled_axis_angle(scaledAxisAngle:&Vector3)->Quaternion{
        let exponential_map = scaledAxisAngle / 2.0f32;
        let half_angle = exponential_map.get_length();
        if half_angle < FLOAT_EPSILON {
            return Quaternion::create_from_vector3and_value(exponential_map.borrow(),1.0.borrow()).get_normalized();
        }else {
            let mut sin:f32;
            let mut cos:f32;
            simd_sin_cos(half_angle.borrow(), sin.borrow_mut(), cos.borrow_mut());
            return Quaternion::create_from_vector3and_value(((sin.to_owned() / half_angle)*exponential_map).borrow(), cos.borrow())
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_float4(mut self, values:*mut f32){
        Vec4::store_unaligned(values,self._value.borrow())
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
    pub unsafe fn get_element(self,index:i32)->f32{
        let values = *self._value as *const f32;
        *values[index]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_x(self, x:f32) {
        let mut values = *self._value as *const f32;
        *values[0] = x
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_y(self, y:f32) {
        let mut values = *self._value as *const f32;
        *values[1] = y
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_z(self, z:f32) {
        let mut values = *self._value as *const f32;
        *values[2] = z
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_w(self, w:f32) {
        let mut values = *self._value as *const f32;
        *values[3] = w
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(self,index:&i32, v:&f32) {
        let mut values = *self._value as *const f32;
        *values[index] = v
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_value(mut self,x:&f32){
        self._value = Vec4::splat(x);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_xyzw(mut self,x:&f32,y:&f32,z:&f32,w:&f32){
        self._value = Vec4::load_immediate(x,y,z,w);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_vec3_w(mut self,v:&Vector3,w:&f32){
        self._value = Vector4::new_vec3_w(v,w).get_simd_value()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_value_ptr(mut self, values:*const f32){
        self._value = Vec4::load_unaligned(values);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_conjugate(self)->Quaternion{
        let conjugate_mask = Vec4::load_aligned_i128(G_NEGATE_XYZMASK.borrow());
        return Quaternion::new_float_type(Vec4::xor(self._value.borrow(),Vec4::cast_to_float(conjugate_mask.borrow()).borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_inverse_fast(self)->Quaternion{
        return self.get_conjugate();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn invert_fast(mut self){
        self._value = self.get_inverse_fast()._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_inverse_full(self)->Quaternion{
        return self.get_conjugate() / self.get_length_sq()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn invert_full(mut self){
        self._value = self.get_inverse_full()._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn dot(self, q:&Quaternion)->f32{
        return  Vec1::select_index0(Vec4::dot(self._value.borrow(),q._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length_sq(self)->f32{
        return self.dot(self.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length(self)->f32{
        let length_sq = Vec4::dot(self._value.borrow(), self._value.borrow());
        return  Vec1::select_index0(Vec1::sqrt(length_sq.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length_estimate(self)->f32{
        let length_sq = Vec4::dot(self._value.borrow(), self._value.borrow());
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
    pub unsafe fn get_length_reciprocal_estimate(self) ->f32{
        let length_sq = Vec4::dot(self._value.borrow(), self._value.borrow());
        return Vec1::select_index0(Vec1::sqrt_inv_estimate(length_sq.borrow()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_normalized(self)->Quaternion{
        return Quaternion::new_float_type(Vec4::normalize(self._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_normalized_estimate(self) ->Quaternion{
        return  Quaternion::new_float_type(Vec4::normalize_estimate(self._value.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize(mut self){
        self._value = self.get_normalized()._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_estimate(mut self){
        self._value = self.get_normalized_estimate()._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_with_length(mut self)->f32{
        let length = Vec1::select_index0(Vec1::sqrt(Vec4::dot(self._value.borrow(),self._value.borrow()).borrow()).borrow());
        self._value = Vec4::div(self._value.borrow(),Vec4::splat(length.borrow()).borrow());
        return length;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize_with_length_estimate(mut self)->f32{
        let length = Vec1::select_index0(Vec1::sqrt_estimate(Vec4::dot(self._value.borrow(),self._value.borrow()).borrow()).borrow());
        self._value = Vec4::div(self._value.borrow(),Vec4::splat(length.borrow()).borrow());
        return length;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_shortest_equivalent(self)->Quaternion{
        if self.get_w() < 0.0f32{
            return -(*self);
        }else {
            return *self;
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn shortest_equivalent(mut self){
        self._value = self.get_shortest_equivalent()._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn lerp(self,dest:&Quaternion,t:&f32)->Quaternion{
        if self.dot(dest) >= 0.0f32 {
            return (*self) * (1.0 -t) + dest * t;
        }
        return  (*self) *(1.0 -t) - dest *t ;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn nlerp(self,dest:&Quaternion,t:&f32)->Quaternion{
        let result = self.lerp(dest,t);
        result.normalize();
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn squad(self,dest:&Quaternion,inv :&Quaternion,out:&Quaternion,t:&f32)->Quaternion{
        let k = 2.0 * (1.0 - t) *t;
        let temp1 = inv.slerp(out,t);
        let temp2 = self.slerp(dest,t);
        return temp1.slerp(temp2.borrow(),k.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(self,q:&Quaternion,tolerance:&f32)->bool{
        let abs_diff = Vec4::abs(Vec4::sub(q._value.borrow(), self._value.borrow()).borrow());
        return Vec4::cmp_all_lt(abs_diff.borrow(), Vec4::splat(tolerance).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_identity(self,tolerance:&f32)->bool{
        return self.is_close(self.create_identity().borrow(),tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_zero(self,tolerance:&f32)->bool{
        let abs_diff = Vec4::abs(self._value.borrow());
        return Vec4::cmp_all_lt(abs_diff.borrow(), Vec4::splat(tolerance).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transform_vector(self,v:&Vector3)->Vector3{
        return  Vector3::new_float_type(Vec4::quaternion_transform(self._value.borrow(),v.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_angle(self)->f32{
        return  2.0f32 * simd_acos(get_clamp(self.get_w().borrow(),(-1.0).borrow(),1.0.borrow()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_euler_degrees(self)->Vector3{
        return Vector3::vector3rad_to_deg(self.get_euler_radians().borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_euler_radians(self)->Vector3{
        let sinp = 2.0 *(self.get_x() * self.get_y() + self.get_z() * self.get_w());
        if sinp *sinp < 0.5 {
            let roll = simd_atan2((2.0*(self.get_w() * self.get_x() - self.get_z() * self.get_y()).borrow(),(1.0 - 2.0 *(self.get_x(),self.get_x() + self.get_y() * self.get_y())).borrow()));
            let pitch = asinf(sinp);
            let yaw = simd_atan2((2.0 *(self.get_w()*self.get_z() - self.get_x() * self.get_y())).borrow(),(1.0 - 2.0*(self.get_y()*self.get_y() + self.get_z() * self.get_z())).borrow());
            return Vector3::new_xyz(roll.borrow(),pitch.borrow(),yaw.borrow());
        }else {
           let mut sign = -1.0f32;
            if sinp > 0.0{
                sign = 1.0f32;
            }
            let m12 = 2.0f32 * (self.get_z()*self.get_y()-self.get_w()*self.get_x());
            let m22 = 1.0f32 - 2.0f32 * (self.get_x() * self.get_x() + self.get_y() * self.get_y());
            let cospSq = m12 * m12 + m22 * m22;
            let cosp =simd_sqrt(cospSq);
            let pitch = sign * acosf(cosp);
            if (cospSq > FLOAT_EPSILON)
            {
                let roll = simd_atan2(-m12, m22);
                let yaw = simd_atan2((2.0 * (self.get_w() * self.get_z() - self.get_x() * self.get_y())).borrow(), (1.0 - 2.0 * (self.get_y() * self.get_y() + self.get_z() * self.get_z())).borrow());
                return Vector3::new_xyz(roll.borrow(),pitch.borrow(),yaw.borrow());
            }
            else
            {
                let m21 = 2.0 * (self.get_y() * self.get_z() + self.get_x() * self.get_w());
                let m11 = 1.0 - 2.0 * (self.get_x() * self.get_x() + self.get_z() * self.get_z());
                let roll = simd_atan2(m21, m11);
                return Vector3::new_xyz(roll.borrow(),pitch.borrow(),0.0.borrow());
            }
        }
    }



AZ_MATH_INLINE void Quaternion::SetFromEulerDegrees(const Vector3& eulerDegrees)
{
SetFromEulerRadians(Vector3DegToRad(eulerDegrees));
}


AZ_MATH_INLINE Vector3 Quaternion::GetImaginary() const
{
return Vector3(Simd::Vec4::ToVec3(m_value));
}


AZ_MATH_INLINE bool Quaternion::IsFinite() const
{
return IsFiniteFloat(GetX()) && IsFiniteFloat(GetY()) && IsFiniteFloat(GetZ()) && IsFiniteFloat(GetW());
}


AZ_MATH_INLINE Simd::Vec4::FloatType Quaternion::GetSimdValue() const
{
return m_value;
}


AZ_MATH_INLINE Quaternion Quaternion::GetAbs() const
{
return Quaternion(Simd::Vec4::Abs(m_value));
}


// Non-member functionality belonging to the AZ namespace
AZ_MATH_INLINE Vector3 ConvertQuaternionToEulerDegrees(const Quaternion& q)
{
return q.GetEulerDegrees();
}


AZ_MATH_INLINE Vector3 ConvertQuaternionToEulerRadians(const Quaternion& q)
{
return q.GetEulerRadians();
}


AZ_MATH_INLINE Quaternion ConvertEulerRadiansToQuaternion(const Vector3& eulerRadians)
{
Quaternion q;
q.SetFromEulerRadians(eulerRadians);
return q;
}


AZ_MATH_INLINE Quaternion ConvertEulerDegreesToQuaternion(const Vector3& eulerDegrees)
{
Quaternion q;
q.SetFromEulerDegrees(eulerDegrees);
return q;
}


AZ_MATH_INLINE void ConvertQuaternionToAxisAngle(const Quaternion& quat, Vector3& outAxis, float& outAngle)
{
quat.ConvertToAxisAngle(outAxis, outAngle);
}

}