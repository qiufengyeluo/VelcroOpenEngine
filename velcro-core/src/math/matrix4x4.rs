#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::math::common_sse::{Vec3Type, Vec4Type, VecFourthType, VecThirdType, VecTwoType, VecType};
use crate::math::math_utils::constants;
use crate::math::math_utils::constants::TOLERANCE;
use crate::math::matrix3x3::Matrix3x3;
use crate::math::matrix3x4::Matrix3x4;
use crate::math::quaternion::Quaternion;
use crate::math::simd_math::simd;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::{FloatArgType, FloatType};

const ROW_COUNT:usize = 4;
const COL_COUNT:usize = 4;
#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    _rows:[Vector4; ROW_COUNT]
}

impl PartialEq<Self> for Matrix4x4 {
    unsafe fn eq(&self, rhs: &Self) -> bool {
        return (Vec4::cmp_all_eq(self._rows[0].get_simd_value(), rhs._rows[0].get_simd_value())
            && Vec4::cmp_all_eq(self._rows[1].get_simd_value(), rhs._rows[1].get_simd_value())
            && Vec4::cmp_all_eq(self._rows[2].get_simd_value(), rhs._rows[2].get_simd_value())
            && Vec4::cmp_all_eq(self._rows[3].get_simd_value(), rhs._rows[3].get_simd_value()));
    }
    unsafe fn ne(&self, rhs: &Self) -> bool {
        unsafe { return !(self == rhs); }
    }
}

impl Add<&Matrix4x4> for Matrix4x4{
    type Output = Matrix4x4;

    fn add(self, rhs: &Matrix4x4) -> Self::Output {
        unsafe {
            return Matrix4x4::new_4float_type
                (
                    Vec4::add(self._rows[0].get_simd_value(), rhs._rows[0].get_simd_value()),
                    Vec4::add(self._rows[1].get_simd_value(), rhs._rows[1].get_simd_value()),
                    Vec4::add(self._rows[2].get_simd_value(), rhs._rows[2].get_simd_value()),
                    Vec4::add(self._rows[3].get_simd_value(), rhs._rows[3].get_simd_value())
                );
        }
    }
}

impl AddAssign<&Matrix4x4> for Matrix4x4{
    fn add_assign(&mut self, rhs: &Matrix4x4) {
        self._rows = (self.to_owned() + rhs)._rows
    }
}

impl Sub<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, rhs: &Matrix4x4) -> Self::Output {
        unsafe {
            return Matrix4x4::new_4float_type
                (
                    Vec4::sub(self._rows[0].get_simd_value(), rhs._rows[0].get_simd_value()),
                    Vec4::sub(self._rows[1].get_simd_value(), rhs._rows[1].get_simd_value()),
                    Vec4::sub(self._rows[2].get_simd_value(), rhs._rows[2].get_simd_value()),
                    Vec4::sub(self._rows[3].get_simd_value(), rhs._rows[3].get_simd_value())
                );
        }
    }
}

impl SubAssign<&Matrix4x4> for Matrix4x4{
    fn sub_assign(&mut self, rhs: &Matrix4x4) {
        self._rows = (self.to_owned() - rhs)._rows
    }
}
impl Mul<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: &Matrix4x4) -> Self::Output {
        let mut result= Matrix4x4::new();
        unsafe { Vec4::mat4x4multiply(*self._rows , *rhs._rows, *result._rows); }
        return result;
    }
}
impl MulAssign<&Matrix4x4>for Matrix4x4{
    fn mul_assign(&mut self, rhs: &Matrix4x4) {
        self._rows = (self.to_owned() * rhs)._rows
    }
}
impl Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, multiplier: f32) -> Self::Output {
        let mul_vec = unsafe { Vec4::splat(multiplier) };
        unsafe {
            return Matrix4x4::new_4float_type
                (
                    Vec4::mul(self._rows[0].get_simd_value(), mul_vec),
                    Vec4::mul(self._rows[1].get_simd_value(), mul_vec),
                    Vec4::mul(self._rows[2].get_simd_value(), mul_vec),
                    Vec4::mul(self._rows[3].get_simd_value(), mul_vec)
                );
        }
    }
}

impl MulAssign<f32> for Matrix4x4 {
    fn mul_assign(&mut self, multiplier: f32) {
        self._rows = (self.to_owned() * multiplier)._rows
    }
}

impl Div<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn div(self, divisor: f32) -> Self::Output {
        let div_vec = unsafe { Vec4::splat(divisor) };
        unsafe {
            return Matrix4x4::new_4float_type
                (
                    Vec4::div(self._rows[0].get_simd_value(), div_vec),
                    Vec4::div(self._rows[1].get_simd_value(), div_vec),
                    Vec4::div(self._rows[2].get_simd_value(), div_vec),
                    Vec4::div(self._rows[3].get_simd_value(), div_vec)
                );
        }
    }
}

impl DivAssign<f32> for Matrix4x4 {
    fn div_assign(&mut self, multiplier: f32) {
        self._rows = (self.to_owned() / multiplier)._rows
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, rhs: Self) -> Self::Output {
        let zero_vec = unsafe { Vec4::zero_float() };
        unsafe {
            return Matrix4x4::new_4float_type
                (
                    Vec4::sub(zero_vec, self._rows[0].get_simd_value()),
                    Vec4::sub(zero_vec, self._rows[1].get_simd_value()),
                    Vec4::sub(zero_vec, self._rows[2].get_simd_value()),
                    Vec4::sub(zero_vec, self._rows[3].get_simd_value())
                );
        }
    }
}

impl Mul<&Vector3> for Matrix4x4 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        unsafe { return Vector3::new_float_type(Vec4::mat4x4transform_point3(*self._rows, rhs.get_simd_value())); }
    }
}
impl Mul<&Vector4> for Matrix4x4 {
    type Output = Vector4;

    fn mul(self, rhs: &Vector4) -> Self::Output {
        unsafe {  return Vector4::new_float_type(Vec4::mat4x4transform_vector(*self._rows, rhs.GetSimdValue())); }
    }
}

impl Matrix4x4{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Matrix4x4{
        Matrix4x4{
            _rows:[Vector4::new(),Vector4::new(),Vector4::new(),Vector4::new()]
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_4float_type(row0:FloatArgType,row1:FloatArgType,row2:FloatArgType,row3:FloatArgType)->Matrix4x4{
        unsafe {
            Matrix4x4 {
                _rows: [Vector4::new_float_type(row0), Vector4::new_float_type(row1), Vector4::new_float_type(row2),Vector4::new_float_type(row3)]
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_identity() ->Matrix4x4{
    return Matrix4x4::new_4float_type(Vec4::load_aligned(simd::G_VEC1000.borrow())
    , Vec4::load_aligned(simd::G_VEC0100.borrow())
    , Vec4::load_aligned(simd::G_VEC0010.borrow())
    , Vec4::load_aligned(simd::G_VEC0001.borrow()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_zero() ->Matrix4x4{
        let zero_vec = Vec4::zero_float();
        return Matrix4x4::new_4float_type(zero_vec, zero_vec, zero_vec, zero_vec);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_value(value:f32) ->Matrix4x4{
        let values = Vec4::splat(value);
        return Matrix4x4::new_4float_type(values, values, values, values);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_row_major_float16(values:*f32)->Matrix4x4{
        return Matrix4x4::create_from_rows(Vector4::create_from_float4(values[0]).borrow()
                              , Vector4::create_from_float4(&values[4]).borrow()
                              , Vector4::create_from_float4(&values[8]).borrow()
                              , Vector4::create_from_float4(&values[12]).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_rows(row0:&Vector4, row1:&Vector4, row2:&Vector4, row3:&Vector4)->Matrix4x4{
        let mut m= Matrix4x4::new();
        m.set_rows_vec4(row0, row1, row2, row3);
        m
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_column_major_float16(values:*f32 )->Matrix4x4{
        return Matrix4x4::create_from_columns(Vector4::create_from_float4(&values[0]).borrow()
                                 , Vector4::create_from_float4(&values[4]).borrow()
                                 , Vector4::create_from_float4(&values[8]).borrow()
                                 , Vector4::create_from_float4(&values[12]).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_columns(col0:&Vector4, col1:&Vector4, col2:&Vector4, col3:&Vector4)->Matrix4x4{
        let mut m =Matrix4x4::new();
        m.set_columns(col0, col1, col2, col3);
        return m;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_x(angle:f32 ) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        let mut s :f32 = 0f32;
        let mut c :f32 = 0f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result._rows[0] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC1000.borrow()));
        result.set_row_xyzw(1, 0.0, c, (-s), 0.0);
        result.set_row_xyzw(2, 0.0, s, c, 0.0);
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_y(angle:f32)->Matrix4x4{
        let mut result=Matrix4x4::new();
        let mut s :f32 = 0f32;
        let mut c :f32 = 0f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row_xyzw(0, c, 0.0, s, 0.0);
        result._rows[1] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0100.borrow()));
        result.set_row_xyzw(2, (-s), 0.0, c, 0.0);
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_z(angle:f32) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        let mut s :f32 = 0f32;
        let mut c :f32 = 0f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row_xyzw(0, c, (-s), 0.0, 0.0);
        result.set_row_xyzw(1, s, c, 0.0, 0.0);
        result._rows[2] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0010.borrow()));
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_quaternion(q:&Quaternion) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_rotation_part_from_quaternion(q);
        result.set_translation(Vector3::new_float_type(Vec3::zero_float()));
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()));
        return result;
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_quaternion_and_translation(q :&Quaternion, p:& Vector3)->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_rotation_part_from_quaternion(q);
        result.set_translation(p);
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix3x4(matrix3x4: &Matrix3x4) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_row_vec4(0, matrix3x4.get_row(0));
        result.set_row_vec4(1, matrix3x4.get_row(1));
        result.set_row_vec4(2, matrix3x4.get_row(2));
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_transform(transform: &Transform) ->Matrix4x4{
        return Matrix4x4::create_from_matrix3x4(Matrix3x4::create_from_transform(transform).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_scale(scale:& Vector3)->Matrix4x4{
        return Matrix4x4::create_diagonal(Vector4::new_xyzw(scale.get_x(), scale.get_y(), scale.get_z(), 1.0).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_diagonal(diagonal:&Vector4)->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_row_xyzw(0, diagonal.get_x(), 0.0, 0.0, 0.0);
        result.set_row_xyzw(1, 0.0, diagonal.get_y(), 0.0, 0.0);
        result.set_row_xyzw(2, 0.0, 0.0, diagonal.get_z(), 0.0);
        result.set_row_xyzw(3, 0.0, 0.0, 0.0, diagonal.get_w());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_translation(translation:&Vector3 )->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_row_xyzw(0, 1.0, 0.0, 0.0, translation.GetX());
        result.set_row_xyzw(1, 0.0, 1.0, 0.0, translation.GetY());
        result.set_row_xyzw(2, 0.0, 0.0, 1.0, translation.GetZ());
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_projection(fov_y:&f32, aspect_ratio:&f32, near_dist:f32, far_dist:f32) ->Matrix4x4{
        let mut sin:f32 = 0f32;
        let mut cos:f32 = 0f32;
        simd::sin_cos((0.5 * fov_y), sin.borrow_mut(), cos.borrow_mut());
        let cot_y = cos / sin;
        let cot_x = cot_y / aspect_ratio;
        return Matrix4x4::create_projection_internal(cot_x, cot_y, near_dist, far_dist);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_projection_fov(fov_x:f32, fov_y:f32, near_dist:f32, far_dist:f32) ->Matrix4x4{
        let angles = Vec4::load_immediate(0.5 * fov_x, 0.5 * fov_x, 0.5 * fov_y, 0.5 * fov_y);
        let values = Vec4::sin_cos_to_float_type(angles);
        let cot_x = Vec4::select_index1(values) / Vec4::select_index1(values);
        let cot_y = Vec4::select_index3(values) / Vec4::select_index2(values);
        return Matrix4x4::create_projection_internal(cot_x, cot_y, near_dist, far_dist);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_projection_offset(left:f32, right:f32, bottom :f32, top :f32, near_dist:f32, far_dist:f32 ){
        let mut result = Matrix4x4::new();
        let invfn = 1.0 / (far_dist - near_dist);
        result.set_row_xyzw(0, -2.0 * near_dist / (right - left), 0.0, (left + right) / (left - right), 0.0);
        result.set_row_xyzw(1, 0.0, 2.0 * near_dist / (top - bottom), (top + bottom) / (bottom - top), 0.0);
        result.set_row_xyzw(2, 0.0, 0.0, (far_dist + near_dist) * invfn, -2.0 * far_dist * near_dist * invfn);
        result.set_row_xyzw(3, 0.0, 0.0, 1.0, 0.0);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_interpolated(m1:&Matrix4x4 , m2:&Matrix4x4, t:f32 )->Matrix4x4{
        let mut m1copy = Matrix3x3::create_from_matrix4x4(m1);
        let mut s1 = m1copy.extract_scale();
        let mut t1 = m1.get_translation();
        let mut q1 = Quaternion::create_from_matrix3x3(m1copy.borrow());

        let mut m2copy = Matrix3x3::create_from_matrix4x4(m2);
        let s2 = m2copy.extract_scale();
        let t2 = m2.get_translation();
        let q2 = Quaternion::create_from_matrix3x3(m2copy.borrow());

        s1 = s1.lerp(s2, t);
        t1 = t1.lerp(t2, t);
        q1 = q1.slerp(q2.borrow(), t);
        q1.normalize();
        let mut result = Matrix4x4::create_from_quaternion_and_translation(q1.borrow(), t1.borrow());
        result.multiply_by_scale(s1.borrow());
        return result;
    }

    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn store_to_row_major_float16(self, values:*f32){
        self.get_row(0).store_to_float_4((*values[0] as usize) as *mut f32);
        self.get_row(1).store_to_float_4((*values[4] as usize) as *mut f32);
        self.get_row(2).store_to_float_4((*values[8] as usize) as *mut f32);
        self.get_row(3).store_to_float_4((*values[12] as usize) as *mut f32);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn store_to_column_major_float16(self, values:*f32){
        self.get_column(0).store_to_float_4((*values[0] as usize) as *mut f32);
        self.get_column(1).store_to_float_4((*values[4] as usize) as *mut f32);
        self.get_column(2).store_to_float_4((*values[8] as usize) as *mut f32);
        self.get_column(3).store_to_float_4((*values[12] as usize) as *mut f32);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_element(self, row:i32 , col:i32)->f32{
        return self._rows[row].get_element(col);
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_element(&mut self,row:i32 , col:i32 , value:f32 ){
        self._rows[row].set_element(col, value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_row(self,row:i32 )->Vector4{
        return self._rows[row];
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_row_as_vector3(self, row:i32) ->Vector3{
        return self._rows[row].GetAsVector3();
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_rows(self, row0:&*mut Vector4, row1:&*mut Vector4, row2:&*mut Vector4, row3:&*mut Vector4){
        row0.set_simd_value(self.get_row(0).get_simd_value());
        row1.set_simd_value(self.get_row(1).get_simd_value()) ;
        row2.set_simd_value(self.get_row(2).get_simd_value()) ;
        row3.set_simd_value(self.get_row(3).get_simd_value());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_row_xyzw(&mut self,row:i32 , x:f32 , y:f32 , z:f32 , w:f32){
        self.set_row_vec4(row, Vector4::new_xyzw(x, y, z, w).borrow());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_row_vec3(&mut self,row:i32 , v:&Vector3 ){
        self._rows[row].set_element(0, v.get_x());
        self._rows[row].set_element(1, v.get_y());
        self._rows[row].set_element(2, v.get_z());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_row_vec3f32(&mut self, row:i32, v:&Vector3, w:f32){
        self._rows[row] = Vector4::new_float_type( Vec4::from_vec3(v.get_simd_value()));
        self._rows[row].set_element(3, w);
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_row_vec4(&mut self, row:i32, v:&Vector4){
        self._rows[row] = v;
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_rows(&mut self, row0:&Vector4, row1:&Vector4, row2:&Vector4, row3:&Vector4 ){
        self.set_row_vec4(0, row0);
        self.set_row_vec4(1, row1);
        self.set_row_vec4(2, row2);
        self.set_row_vec4(3, row3);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_column(self,col:i32)->Vector4{
        return Vector4::new_xyzw(self._rows[0].get_element(col), self._rows[1].get_element(col), self._rows[2].get_element(col), self._rows[3].get_element(col));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_column_as_vector3(self,col:i32) ->Vector3{
        return Vector3::new_xyz(self._rows[0].get_element(col), self._rows[1].get_element(col), self._rows[2].get_element(col));
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_columns(self, col0:&*mut Vector4, col1:&*mut Vector4, col2:&*mut Vector4, col3:&*mut Vector4){
        col0.set_simd_value(self.get_column(0).get_simd_value());
        col1.set_simd_value(self.get_column(1).get_simd_value()) ;
        col2.set_simd_value(self.get_column(2).get_simd_value()) ;
        col3.set_simd_value(self.get_column(3).get_simd_value());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_column_xyzw(&mut self, col:i32, x:f32, y:f32, z:f32, w:f32 ){
        self.set_column_vec4(col, Vector4::new_xyzw(x, y, z, w).borrow());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_column_vec3(&mut self, col:i32, v:&Vector3 ){
        self._rows[0].set_element(col, v.get_x());
        self._rows[1].set_element(col, v.get_y());
        self._rows[2].set_element(col, v.get_z());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_column_vec3f32(&mut self, col:i32, v:&Vector3, w:f32 ){
        self._rows[0].set_element(col, v.get_x());
        self._rows[1].set_element(col, v.get_y());
        self._rows[2].set_element(col, v.get_z());
        self._rows[3].set_element(col, w);
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_column_vec4(&mut self, col:i32, v:&Vector4 ){
        self._rows[0].set_element(col, v.get_x());
        self._rows[1].set_element(col, v.get_y());
        self._rows[2].set_element(col, v.get_z());
        self._rows[3].set_element(col, v.get_w());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_columns(&mut self, col0:&Vector4, col1:&Vector4, col2:&Vector4, col3:&Vector4){
        self.set_column_vec4(0, col0);
        self.set_column_vec4(1, col1);
        self.set_column_vec4(2, col2);
        self.set_column_vec4(3, col3);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_x(self) ->Vector4{
        return self.get_column(0);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_xas_vector3(self) ->Vector3{
        return self.get_column_as_vector3(0);
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_xxyzw(&mut self, x:f32, y:f32, z:f32, w:f32){
        self.set_column_xyzw(0, x, y, z, w);
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_xvec4(&mut self, v:&Vector4){
        self.set_column_vec4(1, v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_y(self) ->Vector4{
        return self.get_column(1);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_yas_vector3(self) ->Vector3{
        return self.get_column_as_vector3(1);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_yxyzw(&mut self, x:f32, y:f32, z:f32, w:f32){
        self.set_column_xyzw(0, x, y, z, w);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_yvec4(&mut self, v:&Vector4){
        self.set_column_vec4(1, v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_z(self) ->Vector4{
        return self.get_column(2);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_zas_vector3(self) ->Vector3{
        return self.get_column_as_vector3(2);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_zxyzw(&mut self, x:f32, y:f32, z:f32, w:f32){
        self.set_column_xyzw(2, x, y, z, w);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_zvec4(&mut self, v:&Vector4){
        self.set_column_vec4(2, v);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_basis_and_translation(self, basis_x:&*mut Vector4, basis_y:&*mut Vector4, basis_z:&*mut Vector4, pos:&*mut Vector4) {
        self.get_columns(basis_x, basis_y, basis_z, pos);
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_and_translation(&mut self, basis_x:&Vector4, basis_y:&Vector4, basis_z:&Vector4, pos:&Vector4){
        self.set_columns(basis_x, basis_y, basis_z, pos);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_translation(self) ->Vector3{
        return self.get_column_as_vector3(3);
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_translation_4f32(&mut self, x:f32, y:f32, z:f32 ){
        self.set_translation_vec3(Vector3::new_xyz(x, y, z).borrow());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_translation_vec3(&mut self, v:&Vector3){
        self.set_column_vec3(3, v);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transposed_multiply3x3(self, v:&Vector3 ) ->Vector3{
    let rows:[FloatType;3] = [ Vec4::value_to_vec3(self._rows[0].get_simd_value()), Vec4::value_to_vec3(self._rows[1].get_simd_value()), Vec4::value_to_vec3(self._rows[2].get_simd_value()) ];
    return Vector3::new_float_type(Vec3::mat3x3transpose_transform_vector(rows.borrow(), v.get_simd_value()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn multiply3x3(self, v:&Vector3 ) ->Vector3{
        let rows:[FloatType;3] = [ Vec4::ToVec3(self._rows[0].get_simd_value()), Vec4::ToVec3(self._rows[1].get_simd_value()), Vec4::ToVec3(self._rows[2].get_simd_value()) ];
        return Vector3::new_float_type(Vec3::mat3x3transform_vector(rows.borrow(), v.get_simd_value()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transpose(self) ->Matrix4x4 {
        let result=Matrix4x4::new();
        Vec4::mat4x4transpose(self.get_simd_values(), *result._rows);
        return result;
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn transpose(&mut self){
        self._rows= self.get_transpose()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_inverse_full(self)->Matrix4x4{
        let mut out =Matrix4x4::new();


        let mut d12 = (self.get_element(2,0) * self.get_element(3, 1) -self.get_element(3, 0) * self.get_element(2, 1));
        let mut d13 = (self.get_element(2, 0) * self.get_element(3, 2) - self.get_element(3, 0) * self.get_element(2, 2));
        let mut d23 = (self.get_element(2, 1) * self.get_element(3, 2) - self.get_element(3, 1) * self.get_element(2, 2));
        let mut d24 = (self.get_element(2, 1) * self.get_element(3, 3) - self.get_element(3, 1) * self.get_element(2, 3));
        let mut d34 = (self.get_element(2, 2) * self.get_element(3, 3) - self.get_element(3, 2) * self.get_element(2, 3));
        let mut d41 = (self.get_element(2, 3) * self.get_element(3, 0) - self.get_element(3, 3) * self.get_element(2, 0));

        out.set_element(0, 0,  (self.get_element(1, 1) * d34 - self.get_element(1, 2) * d24 + self.get_element(1, 3) * d23));
        out.set_element(1, 0, -(self.get_element(1, 0) * d34 + self.get_element(1, 2) * d41 + self.get_element(1, 3) * d13));
        out.set_element(2, 0,  (self.get_element(1, 0) * d24 + self.get_element(1, 1) * d41 + self.get_element(1, 3) * d12));
        out.set_element(3, 0, -(self.get_element(1, 0) * d23 - self.get_element(1, 1) * d13 + self.get_element(1, 2) * d12));

        // Compute determinant as early as possible using these cofactors.
        let det = self.get_element(0, 0) * out.get_element(0, 0) + self.get_element(0, 1) * out.get_element(1, 0) + self.get_element(0, 2) * out.get_element(2, 0) + self.get_element(0, 3) * out.get_element(3, 0);

        // Run singularity test.
        if (det == 0.0)
        {
            out = Matrix4x4::create_identity();
        }
        else
        {
            let invDet = 1.0 / det;

            // Compute rest of inverse.
            out.set_element(0, 0, out.get_element(0, 0) * invDet);
            out.set_element(1, 0, out.get_element(1, 0) * invDet);
            out.set_element(2, 0, out.get_element(2, 0) * invDet);
            out.set_element(3, 0, out.get_element(3, 0) * invDet);

            out.set_element(0, 1, -(self.get_element(0, 1) * d34 - self.get_element(0, 2) * d24 + self.get_element(0, 3) * d23) * invDet);
            out.set_element(1, 1,  (self.get_element(0, 0) * d34 + self.get_element(0, 2) * d41 + self.get_element(0, 3) * d13) * invDet);
            out.set_element(2, 1, -(self.get_element(0, 0) * d24 + self.get_element(0, 1) * d41 + self.get_element(0, 3) * d12) * invDet);
            out.set_element(3, 1,  (self.get_element(0, 0) * d23 - self.get_element(0, 1) * d13 + self.get_element(0, 2) * d12) * invDet);

            // Pre-compute 2x2 dets for first two rows when computing cofactors of last two rows.
            d12 = self.get_element(0, 0) * self.get_element(1, 1) - self.get_element(1, 0) * self.get_element(0, 1);
            d13 = self.get_element(0, 0) * self.get_element(1, 2) - self.get_element(1, 0) * self.get_element(0, 2);
            d23 = self.get_element(0, 1) * self.get_element(1, 2) - self.get_element(1, 1) * self.get_element(0, 2);
            d24 = self.get_element(0, 1) * self.get_element(1, 3) - self.get_element(1, 1) * self.get_element(0, 3);
            d34 = self.get_element(0, 2) * self.get_element(1, 3) - self.get_element(1, 2) * self.get_element(0, 3);
            d41 = self.get_element(0, 3) * self.get_element(1, 0) - self.get_element(1, 3) * self.get_element(0, 0);

            out.set_element(0, 2,  (self.get_element(3, 1) * d34 - self.get_element(3, 2) * d24 + self.get_element(3, 3) * d23) * invDet);
            out.set_element(1, 2, -(self.get_element(3, 0) * d34 + self.get_element(3, 2) * d41 + self.get_element(3, 3) * d13) * invDet);
            out.set_element(2, 2,  (self.get_element(3, 0) * d24 + self.get_element(3, 1) * d41 + self.get_element(3, 3) * d12) * invDet);
            out.set_element(3, 2, -(self.get_element(3, 0) * d23 - self.get_element(3, 1) * d13 + self.get_element(3, 2) * d12) * invDet);
            out.set_element(0, 3, -(self.get_element(2, 1) * d34 - self.get_element(2, 2) * d24 + self.get_element(2, 3) * d23) * invDet);
            out.set_element(1, 3,  (self.get_element(2, 0) * d34 + self.get_element(2, 2) * d41 + self.get_element(2, 3) * d13) * invDet);
            out.set_element(2, 3, -(self.get_element(2, 0) * d24 + self.get_element(2, 1) * d41 + self.get_element(2, 3) * d12) * invDet);
            out.set_element(3, 3,  (self.get_element(2, 0) * d23 - self.get_element(2, 1) * d13 + self.get_element(2, 2) * d12) * invDet);
        }

        return out;
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn invert_full(&mut self){
        self._rows = self.get_inverse_full()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_inverse_transform(self) ->Matrix4x4{
        let mut out =Matrix4x4::new();

        assert!(self.get_element(3, 0) == 0.0 && self.get_element(3, 1) == 0.0 && self.get_element(3, 2) == 0.0 && self.get_element(3, 3) == 1.0, "For this matrix you should use GetInverseFull");

        out.set_element(0, 0, (self.get_element(1, 1) * self.get_element(2, 2) - self.get_element(1, 2) * self.get_element(2, 1)));
        out.set_element(1, 0, (self.get_element(1, 2) * self.get_element(2, 0) - self.get_element(1, 0) * self.get_element(2, 2)));
        out.set_element(2, 0, (self.get_element(1, 0) * self.get_element(2, 1) - self.get_element(1, 1) * self.get_element(2, 0)));

        out.set_element(0, 1, (self.get_element(2, 1) * self.get_element(0, 2) - self.get_element(2, 2) * self.get_element(0, 1)));
        out.set_element(1, 1, (self.get_element(2, 2) * self.get_element(0, 0) - self.get_element(2, 0) * self.get_element(0, 2)));
        out.set_element(2, 1, (self.get_element(2, 0) * self.get_element(0, 1) - self.get_element(2, 1) * self.get_element(0, 0)));

        out.set_element(0, 2, (self.get_element(0, 1) * self.get_element(1, 2) - self.get_element(0, 2) * self.get_element(1, 1)));
        out.set_element(1, 2, (self.get_element(0, 2) * self.get_element(1, 0) - self.get_element(0, 0) * self.get_element(1, 2)));
        out.set_element(2, 2, (self.get_element(0, 0) * self.get_element(1, 1) - self.get_element(0, 1) * self.get_element(1, 0)));

        let det = self.get_element(0, 0) * out.get_element(0, 0) + self.get_element(1, 0) * out.get_element(0, 1) + self.get_element(2, 0) * out.get_element(0, 2);

        let mut f = 10000000.0;
        if (simd::abs(det) > constants::TOLERANCE) {
            f = 1.0 / det;
        }

        out.set_row_vec4(0, (out.get_row(0) * f).borrow());
        out.set_row_vec4(1, (out.get_row(1) * f).borrow());
        out.set_row_vec4(2, (out.get_row(2) * f).borrow());

        out.set_element(0, 3, -(self.get_element(0, 3) * out.get_element(0, 0) + self.get_element(1, 3) * out.get_element(0, 1) + self.get_element(2, 3) * out.get_element(0, 2)));
        out.set_element(1, 3, -(self.get_element(0, 3) * out.get_element(1, 0) + self.get_element(1, 3) * out.get_element(1, 1) + self.get_element(2, 3) * out.get_element(1, 2)));
        out.set_element(2, 3, -(self.get_element(0, 3) * out.get_element(2, 0) + self.get_element(1, 3) * out.get_element(2, 1) + self.get_element(2, 3) * out.get_element(2, 2)));

        out.set_row_xyzw(3, 0.0, 0.0, 0.0, 1.0);

        return out;    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn invert_transform(&mut self){
        self._rows = self.get_inverse_transform()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_inverse_fast(self) ->Matrix4x4{
        let mut out =Matrix4x4::new();
        Vec4::mat4x4inverse_fast(self.get_simd_values(), *out._rows);
        return out;
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn invert_fast(&mut self){
        self._rows = self.get_inverse_fast()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn retrieve_scale(self)->Vector3{
        return Vector3::new_xyz(self.get_basis_x().get_length(), self.get_basis_y().get_length(), self.get_basis_z().get_length());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn retrieve_scale_sq(self) ->Vector3{
        return Vector3::new_xyz(self.get_basis_x().get_length_sq(), self.get_basis_y().get_length_sq(), self.get_basis_z().get_length_sq());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn extract_scale(&mut self) ->Vector3{
        let x = self.get_basis_x();
        let y = self.get_basis_y();
        let z = self.get_basis_z();
        let length_x = x.normalize_with_length();
        let length_y = y.normalize_with_length();
        let length_z = z.normalize_with_length();
        self.set_basis_xvec4(x.borrow());
        self.set_basis_yvec4(y.borrow());
        self.set_basis_zvec4(z.borrow());
        return Vector3::new_xyz(length_x, length_y, length_z);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn multiply_by_scale(&mut self, scale:&Vector3){
        let scalars = Vector4::create_from_vector3_and_float(scale, 1.0);
        self._rows[0] = self._rows[0] * scalars;
        self._rows[1] = self._rows[1] * scalars;
        self._rows[2] = self._rows[2] * scalars;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_reciprocal_scaled(self) ->Matrix4x4{
        let mut result = Matrix4x4::new();
        result._rows = self._rows.to_owned();
        result.multiply_by_scale(self.retrieve_scale_sq().get_reciprocal().borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn is_close_default(self,rhs:&Matrix4x4)->bool{
        return  self.is_close(rhs,TOLERANCE);
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn is_close(self, rhs:&Matrix4x4, tolerance:f32) ->bool{
        let vec_tolerance = Vec4::Splat(tolerance);
        for row in 0..ROW_COUNT{
            let compare = Vec4::abs(Vec4::sub(self._rows[row].get_simd_value(), rhs._rows[row].get_simd_value()));
            if (!Vec4::CmpAllLt(compare, vec_tolerance))
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_rotation_part_from_quaternion(&mut self, q:&Quaternion ){
        let tx = q.get_x() * 2.0;
        let ty = q.get_y() * 2.0;
        let tz = q.get_z() * 2.0;
        let twx = q.get_w() * tx;
        let twy = q.get_w() * ty;
        let twz = q.get_w() * tz;
        let txx = q.get_x() * tx;
        let txy = q.get_x() * ty;
        let txz = q.get_x() * tz;
        let tyy = q.get_y() * ty;
        let tyz = q.get_y() * tz;
        let tzz = q.get_z() * tz;

        self.set_element(0, 0, 1.0 - (tyy + tzz)); // 1.0-2yy-2zz   2xy-2wz       2xz+2wy
        self.set_element(0, 1, txy - twz);
        self.set_element(0, 2, txz + twy);

        self.set_element(1, 0, txy + twz);          // 2xy+2wz   1.0-2xx-2zz       2yz-2wx
        self.set_element(1, 1, 1.0 - (txx + tzz));
        self.set_element(1, 2, tyz - twx);

        self.set_element(2, 0, txz - twy);          // 2xz-2wy       2yz+2wx   1.0-2xx-2yy
        self.set_element(2, 1, tyz + twx);
        self.set_element(2, 2, 1.0 - (txx + tyy));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_diagonal(self) ->Vector4{
        return Vector4::new_xyzw(self.get_element(0, 0), self.get_element(1, 1), self.get_element(2, 2), self.get_element(3, 3));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn is_finite(self) ->bool{
        return self.get_row(0).is_finite() && self.get_row(1).is_finite() && self.get_row(2).is_finite() && self.get_row(3).is_finite();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_simd_values_const(self) ->*const FloatType{
        return *self._rows as *const FloatType
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_simd_values(self) ->* FloatType{
        return *self._rows as * FloatType
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_projection_internal(cot_x:f32, cot_y:f32, near_dist:f32, far_dist:f32 )->Matrix4x4{
        let mut result =Matrix4x4::new();
        let invfn = 1.0 / (far_dist - near_dist);
        result.set_row_xyzw(0, -cot_x, 0.0, 0.0, 0.0);
        result.set_row_xyzw(1, 0.0, cot_y, 0.0, 0.0);
        result.set_row_xyzw(2, 0.0, 0.0, (far_dist + near_dist) * invfn, -2.0 * far_dist * near_dist * invfn);
        result.set_row_xyzw(3, 0.0, 0.0, 1.0,0.0);
        return result;
    }
}