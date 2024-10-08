#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::fmt::Debug;
use std::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::math::common_sse::{Vec2Type, Vec3Type, Vec4Type, VecFourthType, VecThirdType, VecTwoType, VecType};
use crate::math::math_utils::constants;
use crate::math::math_utils::constants::Axis;
use crate::math::matrix3x3::Matrix3x3;
use crate::math::matrix4x4::Matrix4x4;
use crate::math::quaternion::Quaternion;
use crate::math::simd_math::simd;
use crate::math::simd_math_vec2_sse::Vec2;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::transform::Transform;
use crate::math::vector2::Vector2;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::{FloatArgType, FloatType};

const ROW_COUNT:usize = 3;
const COL_COUNT:usize = 4;
#[derive(Debug, Copy, Clone)]
pub struct Matrix3x4 {
    _rows:[Vector4; ROW_COUNT]
}
impl PartialEq<Self> for Matrix3x4 {
    unsafe fn eq(&self, rhs: &Self) -> bool {
        return self._rows[0] == rhs._rows[0] && self._rows[1] == rhs._rows[1] && self._rows[2] == rhs._rows[2];
    }
    unsafe fn ne(&self, rhs: &Self) -> bool {
        unsafe { return !(self == rhs); }
    }
}

impl Add<Matrix3x4> for Matrix3x4 {
    type Output = Matrix3x4;
    fn add(self, rhs: Matrix3x4) -> Self::Output {
        unsafe {
            unsafe {
                return Matrix3x4::new_3float_type (
                    Vec4::add(self._rows[0].get_simd_value(), rhs._rows[0].get_simd_value()),
                    Vec4::add(self._rows[1].get_simd_value(), rhs._rows[1].get_simd_value()),
                    Vec4::add(self._rows[2].get_simd_value(), rhs._rows[2].get_simd_value())
                )
            }
        }
    }
}

impl AddAssign<Matrix3x4> for Matrix3x4{
    fn add_assign(&mut self, rhs: Matrix3x4) {
        self._rows = (self.to_owned() + rhs)._rows;
    }
}

impl Sub<Matrix3x4> for Matrix3x4 {
    type Output = Matrix3x4;
    fn sub(self, rhs: Matrix3x4) -> Self::Output {
        unsafe {
          return Matrix3x4::new_3float_type (
                Vec4::sub(self._rows[0].get_simd_value(), rhs._rows[0].get_simd_value()),
                    Vec4::sub(self._rows[1].get_simd_value(), rhs._rows[1].get_simd_value()),
                    Vec4::sub(self._rows[2].get_simd_value(), rhs._rows[2].get_simd_value())
            )
        }
    }
}

impl SubAssign<Matrix3x4> for Matrix3x4{
    fn sub_assign(&mut self, rhs: Matrix3x4) {
        self._rows = (self.to_owned() - rhs)._rows;
    }
}

impl Mul<&Matrix3x4> for Matrix3x4 {
    type Output = Matrix3x4;
    fn mul(self, rhs: &Matrix3x4) -> Self::Output {
        unsafe {
            let mut result = Matrix3x4::new();
            Vec4::mat3x4multiply(self.get_simd_values(), rhs.get_simd_values(), *result._rows);
            result
        }
    }
}

impl MulAssign<&Matrix3x4> for Matrix3x4{
    fn mul_assign(&mut self, rhs: &Matrix3x4) {
        self._rows = (self.to_owned() * rhs)._rows;
    }
}

impl Mul<f32> for Matrix3x4 {
    type Output = Matrix3x4;
    fn mul(self, rhs: f32) -> Self::Output {
        unsafe {
            return Matrix3x4::new_3float_type (
                Vec4::mul(self._rows[0].get_simd_value(),Vec4::splat(rhs)),
                Vec4::mul(self._rows[1].get_simd_value(), Vec4::splat(rhs)),
                Vec4::mul(self._rows[2].get_simd_value(), Vec4::splat(rhs))
            )
        }
    }
}

impl MulAssign<f32> for Matrix3x4{
    fn mul_assign(&mut self, rhs: &Matrix3x4) {
        self._rows = (self.to_owned() * rhs)._rows;
    }
}

impl Div<f32> for Matrix3x4 {
    type Output = Matrix3x4;
    fn div(self, rhs: f32) -> Self::Output {
        unsafe {
            return Matrix3x4::new_3float_type (
                Vec4::div(self._rows[0].get_simd_value(),Vec4::splat(rhs)),
                Vec4::div(self._rows[1].get_simd_value(), Vec4::splat(rhs)),
                Vec4::div(self._rows[2].get_simd_value(), Vec4::splat(rhs))
            )
        }
    }
}

impl DivAssign<f32> for Matrix3x4{
    fn div_assign(&mut self, rhs: &Matrix3x4) {
        self._rows = (self.to_owned() * rhs)._rows;
    }
}

impl Sub for Matrix3x4 {
    type Output = Matrix3x4;

    fn sub(self, _rhs: Self) -> Self::Output {
       let zero_vec = unsafe { Vec4::zero_float() };
        unsafe {
            return Matrix3x4::new_3float_type
                (
                    Vec4::Sub(zero_vec, self._rows[0].get_simd_value()),
                    Vec4::Sub(zero_vec, self._rows[1].get_simd_value()),
                    Vec4::Sub(zero_vec, self._rows[2].get_simd_value())
                );
        }
    }
}

impl Mul<&Vector3> for Matrix3x4 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        unsafe {
            return Vector3::new_xyz
                (
                    (self._rows[0].dot3(rhs) + self._rows[0].get_element(3)),
                    (self._rows[1].dot3(rhs) + self._rows[1].get_element(3)),
                    (self._rows[2].dot3(rhs) + self._rows[2].get_element(3))
                );
        }
    }
}

impl Mul<&Vector4> for Matrix3x4 {
    type Output = Vector4;

    fn mul(self, rhs: &Vector4) -> Self::Output {
        unsafe {
            return Vector4::new_xyzw
                (
                    self._rows[0].dot4(rhs),
                    self._rows[1].dot4(rhs),
                    self._rows[2].dot4(rhs),
                    rhs.get_element(3)
                );
        }
    }
}

impl Matrix3x4{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Matrix3x4{
        Matrix3x4{
            _rows:[Vector4::new(),Vector4::new(),Vector4::new()]
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_3float_type(row0:FloatArgType,row1:FloatArgType,row2:FloatArgType)->Matrix3x4{
        unsafe {
            Matrix3x4 {
                _rows: [Vector4::new_float_type(row0), Vector4::new_float_type(row1), Vector4::new_float_type(row2)]
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_identity()->Matrix3x4{
        return Matrix3x4::new_3float_type(Vec4::load_aligned(simd::G_VEC1000.borrow()),Vec4::load_aligned(simd::G_VEC0100.borrow()),Vec4::load_aligned(simd::G_VEC0010.borrow()))
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_zero()->Matrix3x4{
        let zero_vec = Vec4::zero_float();
        return Matrix3x4::new_3float_type(zero_vec, zero_vec, zero_vec);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_value(value:f32)->Matrix3x4{
        let values = Vec4::splat(value);
        return Matrix3x4::new_3float_type(values, values, values);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_row_major_float12(values:&[f32;12]){
        return Matrix3x4::create_from_rows(Vector4::create_from_float4(*values[0]),
                                           Vector4::create_from_float4(*values[4]),
                                           Vector4::create_from_float4(*values[8]));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_rows(row0:&Vector4,row1:&Vector4,row2:&Vector4)->Matrix3x4{
        Matrix3x4{
            _rows:[row0.to_owned(),row1.to_owned(),row2.to_owned()],
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_column_major_float12(values:[f32;12])->Matrix3x4{
        return Matrix3x4::create_from_columns(
            Vector3::create_from_float_3(*values[0]),
            Vector3::create_from_float_3(*values[3]),
            Vector3::create_from_float_3(*values[6]),
            Vector3::create_from_float_3(*values[9])
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_columns(col0:&Vector3,col1:&Vector3,col2:&Vector3,col3:&Vector3)->Matrix3x4{
        let mut result = Matrix3x4::new();
        result.set_columns(col0, col1, col2, col3);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_column_major_float16(values:&[f32;16])->Matrix3x4{
        return Matrix3x4::create_from_columns(Vector3::create_from_float_3(*values[0]),
                                              Vector3::create_from_float_3(*values[4]),
                                              Vector3::create_from_float_3(*values[8]),
                                              Vector3::create_from_float_3(*values[12]));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_x(angle:f32)->Matrix3x4{
        let mut result=Matrix3x4::new() ;
        let mut s:f32 = 0.0;
        let mut c:f32 = 0.0;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result._rows[0] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC1000.borrow()));
        result.set_row_xyzw(1, 0.0, c, (-s), 0.0);
        result.set_row_xyzw(2, 0.0, s, c, 0.0);
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_y(angle:f32)->Matrix3x4{
        let mut result=Matrix3x4::new() ;
        let mut s:f32 = 0.0;
        let mut c:f32 = 0.0;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row_xyzw(0, c, 0.0, s, 0.0);
        result._rows[1] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0100.borrow()));
        result.set_row_xyzw(2, (-s), 0.0, c, 0.0);
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_z(angle:f32)->Matrix3x4{
        let mut result=Matrix3x4::new() ;
        let mut s:f32 = 0.0;
        let mut c:f32 = 0.0;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row_xyzw(0, c, (-s),0.0, 0.0);
        result.set_row_xyzw(1, s, c, 0.0, 0.0);
        result._rows[2] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0010.borrow()));

        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_quaternion(quaternion:&Quaternion)->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_rotation_part_from_quaternion(quaternion);
        result.set_translation_vec3(Vector3::create_zero().borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_quaternion_and_translation(quaternion:& Quaternion , translation: &Vector3)->Matrix3x4
    {
        let mut result=Matrix3x4::new();
        result.set_rotation_part_from_quaternion(quaternion);
        result.set_translation_vec3(translation);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_matrix3x3(matrix3x3: &Matrix3x3) ->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_row_vec3_f32(0, matrix3x3.get_row(0).borrow(), 0.0);
        result.set_row_vec3_f32(1, matrix3x3.get_row(1).borrow(), 0.0);
        result.set_row_vec3_f32(2, matrix3x3.get_row(2).borrow(), 0.0);
        result
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_matrix3x3and_translation(matrix3x3:&Matrix3x3, translation:&Vector3 )->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_rows(
        Vector4::create_from_vector3_and_float(matrix3x3.get_row(0).borrow(), translation.get_element(0)).borrow(),
        Vector4::create_from_vector3_and_float(matrix3x3.get_row(1).borrow(), translation.get_element(1)).borrow(),
        Vector4::create_from_vector3_and_float(matrix3x3.get_row(2).borrow(), translation.get_element(2)).borrow()
        );
        result
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn unsafe_create_from_matrix4x4(matrix4x4:&Matrix4x4) ->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_row_vec4(0, matrix4x4.get_row(0).borrow());
        result.set_row_vec4(1, matrix4x4.get_row(1).borrow());
        result.set_row_vec4(2, matrix4x4.get_row(2).borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_transform(transform:&Transform) ->Matrix3x4{
    return Matrix3x4::create_from_columns(transform.get_basis_x().borrow(), transform.get_basis_y().borrow(), transform.get_basis_z().borrow(), transform.get_translation().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_scale(scale:&Vector3) ->Matrix3x4{
        return Matrix3x4::create_diagonal(scale);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_diagonal(diagonal:&Vector3) ->Matrix3x4{
        return Matrix3x4::create_from_rows(Vector4::create_axis_x(diagonal.get_x()).borrow(),
                                           Vector4::create_axis_y(diagonal.get_y()).borrow(),
                                           Vector4::create_axis_z(diagonal.get_z()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_translation(translation:&Vector3) ->Matrix3x4{
        let mut result = Matrix3x4::create_identity();
        result.set_translation_vec3(translation);
        result
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_look_at_default(from:&Vector3, to:&Vector3) ->Matrix3x4{
        return Matrix3x4::create_look_at(from,to,constants::Axis::YPositive.borrow());
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_look_at(from:&Vector3, to:&Vector3, forward_axis:&constants::Axis) ->Matrix3x4{
        let mut result =Matrix3x4::create_identity();
        let mut target_forward = to - from;

        if target_forward.is_zero_default()
        {
            return result;
        }

        target_forward.normalize();

        let mut up = Vector3::create_axis_z(1.0);

        let abs_dot = simd::abs(target_forward.dot3(up.borrow()));
        if (abs_dot > 1.0 - 0.001)
        {
            up = target_forward.cross_y_axis();
        }

        let mut right = target_forward.cross(up.borrow());
        right.normalize();
        up = right.cross(target_forward.borrow());
        up.normalize();
        match forward_axis {
            Axis::XPositive => {
                result.set_basis_and_translation(target_forward.borrow(), -right, up.borrow(), from);
            }
            Axis::XNegative => {
                result.set_basis_and_translation(-target_forward, right.borrow(), up.borrow(), from);
            }
            Axis::YPositive => {
                result.set_basis_and_translation(right.borrow(), target_forward.borrow(), up.borrow(), from);
            }
            Axis::YNegative => {
                result.set_basis_and_translation(-right, -target_forward, up.borrow(), from);
            }
            Axis::ZPositive => {
                result.set_basis_and_translation(right.borrow(), -up, target_forward.borrow(), from);
            }
            Axis::ZNegative => {
                result.set_basis_and_translation(right.borrow(), up.borrow(), -target_forward, from);
            }
            _ =>{
                result.set_basis_and_translation(right.borrow(), target_forward.borrow(), up.borrow(), from);
            }
        }
         result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn identity()->Matrix3x4{
        return Matrix3x4::create_identity();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_row_major_float12(self, values:&[f32;12]){
        self.get_row(0).store_to_float_4((*values[0] as usize) as *mut f32);
        self.get_row(1).store_to_float_4((*values[4] as usize) as *mut f32);
        self.get_row(2).store_to_float_4((*values[8] as usize) as *mut f32);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_column_major_float12(self, values:&[f32;12]) {
        self.get_column(0).store_to_float_3((*values[0] as usize) as *mut f32);
        self.get_column(1).store_to_float_3((*values[3] as usize) as *mut f32);
        self.get_column(2).store_to_float_3((*values[6] as usize) as *mut f32);
        self.get_column(3).store_to_float_3((*values[9] as usize) as *mut f32);
    }

    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_column_major_float16(self, values:&[f32;16]) {
        self.get_column(0).store_to_float_4((*values[0] as usize) as *mut f32);
        self.get_column(1).store_to_float_4((*values[4] as usize) as *mut f32);
        self.get_column(2).store_to_float_4((*values[8] as usize) as *mut f32);
        self.get_column(3).store_to_float_4((*values[12] as usize) as *mut f32);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_element(self, row:i32, col:i32 ) ->f32{
        return self._rows[row].get_element(col);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(&mut self, row:i32, col:i32,value:f32){
        self._rows[row].set_element(col, value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_row(self, row:i32 ) ->Vector4{
        return self._rows[row];
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_row_as_vector3(self, row:i32)->Vector3{
        return self._rows[row].get_as_vector3();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_row_xyzw(&mut self, row:i32 ,  x:f32,  y:f32,  z:f32,  w:f32){
        self._rows[row].set_xyzw(x, y, z, w);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_row_vec3_f32(&mut self,  row:i32, v: &Vector3, w:f32){
        self._rows[row].set_vec3_f32(v, w);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_row_vec4(&mut self, row:i32 , v:& Vector4){
        self._rows[row] = v;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_rows(self, row0:*mut Vector4, row1:*mut Vector4, row2:*mut Vector4){
        *row0 = self._rows[0];
        *row1 = self._rows[1];
        *row2 = self._rows[2];
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_rows(&mut self,row0:*const Vector4, row1:*const Vector4 ,row2:* const Vector4 ){
        self._rows[0] = *row0;
        self._rows[1] = *row1;
        self._rows[2] = *row2;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_column(self, col:i32 )->Vector3{
        return Vector3::new_xyz(self._rows[0].get_element(col), self._rows[1].get_element(col), self._rows[2].get_element(col));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_column(&mut self,col:i32 ,x:f32, y:f32,z:f32){
        self._rows[0].set_element(col, x);
        self._rows[1].set_element(col, y);
        self._rows[2].set_element(col, z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_column_vec3(&mut self,col:i32 , v:& Vector3){
        self.set_column(col, v.get_x(), v.get_y(), v.get_z());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_columns(self,col0:*mut Vector3, col1:*mut Vector3,col2:*mut Vector3,col3:*mut Vector3){
        *col0 = self.get_column(0);
        *col1 = self.get_column(1);
        *col2 = self.get_column(2);
        *col3 = self.get_column(3);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_columns(&mut self,col0:&Vector3,col1:&Vector3, col2:&Vector3, col3:&Vector3){
        for row in  0.. Matrix3x4::RowCount
        {
            self._rows[row].set_x_y_z_w(col0.get_element((row as i32)), col1.get_element((row as i32)), col2.get_element((row as i32)), col3.get_element((row as i32)));
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_x(self)->Vector3{
        return self.get_column(0);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_x(&mut self,x:f32, y:f32, z:f32){
        self.set_column(0, x, y, z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_x_vec3(&mut self,v:&Vector3){
        self.set_column(0, v.get_x(), v.get_y(), v.get_z());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_y(self)->Vector3{
        return self.get_column(1);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_y(&mut self,x:f32, y:f32, z:f32){
        self.set_column(1, x, y, z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_y_vec3(&mut self,v:&Vector3){
        self.set_column(1, v.get_x(), v.get_y(), v.get_z());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_z(self)->Vector3{
        return self.get_column(2);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_z(&mut self,x:f32, y:f32, z:f32){
        self.set_column(2, x, y, z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_z_vec3(&mut self,v:&Vector3){
        self.set_column(2, v.get_x(), v.get_y(), v.get_z());
    }
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_translation(self)->Vector3{
        return self.get_column(3);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_translation(&mut self,x:f32, y:f32, z:f32){
        self.set_column(3, x, y, z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_translation_vec3(&mut self,v:&Vector3){
        self.set_column_vec3(3, v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_and_translation(self, basis_x:*mut Vector3, basis_y:*mut Vector3, basis_z:*mut Vector3, translation:*mut Vector3){
        *basis_x = self.get_column(0);
        *basis_y = self.get_column(1);
        *basis_z = self.get_column(2);
        *translation = self.get_column(3);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_basis_and_translation(&mut self, basis_x:&Vector3, basis_y:&Vector3, basis_z:&Vector3, translation:&Vector3){
        self.set_columns(basis_x, basis_y, basis_z, translation);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn multiply3x3(self,rhs:&Vector3 )->Vector3{
        return Vector3::new_xyz
        (
            self._rows[0].dot3(rhs),
            self._rows[1].dot3(rhs),
            self._rows[2].dot3(rhs)
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transform_vector(self, rhs: &Vector3)->Vector3{
        return self.multiply3x3(rhs);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transform_point(self, rhs: &Vector3) ->Vector3{
        return self.multiply3x3(rhs) + self.get_translation();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transpose(self) ->Matrix3x4{
        let result =Matrix3x4::new();
        Vec4::mat3x4transpose(self.get_simd_values(), *result._rows);
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transpose(&mut self){
        self._rows = self.get_transpose()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transpose3x3(self) ->Matrix3x4{
        let mut result =Matrix3x4::new();
        result.set_row_vec3_f32(0, self.get_column(0).borrow(), self._rows[0].get_element(3));
        result.set_row_vec3_f32(1, self.get_column(1).borrow(), self._rows[1].get_element(3));
        result.set_row_vec3_f32(2, self.get_column(2).borrow(), self._rows[2].get_element(3));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transpose3x3(&mut self){
        self._rows =self.get_transpose3x3()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_inverse_full(&mut self)->Matrix3x4{
        let mut result =Matrix3x4::new();

        // compute the first row of the matrix of cofactors
        result.set_row_xyzw(0,
                       (self.get_element(1, 1) * self.get_element(2, 2) - self.get_element(1, 2) * self.get_element(2, 1)),
                       (self.get_element(2, 1) * self.get_element(0, 2) - self.get_element(2, 2) * self.get_element(0, 1)),
                       (self.get_element(0, 1) * self.get_element(1, 2) - self.get_element(0, 2) * self.get_element(1, 1)),
                      0.0
        );

       let determinant = result._rows[0].dot3(self.get_column(0).borrow());

        if (!constants::is_close_f32(determinant, 0.0, constants::FLOAT_EPSILON))
        {
            let determinant_inv = 1.0 / determinant;
            result._rows[0] *= determinant_inv;
            result.set_row_xyzw(1,
                                (determinant_inv * (self.get_element(1, 2) * self.get_element(2, 0) - self.get_element(1, 0) * self.get_element(2, 2))),
                                (determinant_inv * (self.get_element(2, 2) * self.get_element(0, 0) - self.get_element(2, 0) * self.get_element(0, 2))),
                                (determinant_inv * (self.get_element(0, 2) * self.get_element(1, 0) - self.get_element(0, 0) * self.get_element(1, 2))),
                                0.0
            );
            result.set_row_xyzw(2,
                                (determinant_inv * (self.get_element(1, 0) * self.get_element(2, 1) - self.get_element(1, 1) * self.get_element(2, 0))),
                                (determinant_inv * (self.get_element(2, 0) * self.get_element(0, 1) - self.get_element(2, 1) * self.get_element(0, 0))),
                                (determinant_inv * (self.get_element(0, 0) * self.get_element(1, 1) - self.get_element(0, 1) * self.get_element(1, 0))),
                                0.0
            );
        }
        else
        {
            result = Matrix3x4::create_identity();
        }

        let translation =self.get_translation();
        result.set_element(0, 3, (-result._rows[0].dot3(translation.borrow())));
        result.set_element(1, 3, (-result._rows[1].dot3(translation.borrow())));
        result.set_element(2, 3, (-result._rows[2].dot3(translation.borrow())));

        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn invert_full(&mut self){
        self._rows =self.get_inverse_full()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_inverse_fast(self) ->Matrix3x4{
        let result =Matrix3x4::new();
        Vec4::mat3x4inverse_fast(self.get_simd_values(), *result._rows);
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn invert_fast(&mut self){
        self._rows = self.get_inverse_fast()._rows;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn retrieve_scale(self)->Vector3{
        return Vector3::new_xyz(self.get_column(0).get_length(), self.get_column(1).get_length(), self.get_column(2).get_length());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn retrieve_scale_sq(self)->Vector3{
        return Vector3::new_xyz(self.get_column(0).get_length_sq(), self.get_column(1).get_length_sq(), self.get_column(2).get_length_sq());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn extract_scale(&mut self)->Vector3{
        let scale = self.retrieve_scale();
        self.multiply_by_scale(scale.get_reciprocal().borrow());
        return scale;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn multiply_by_scale(&mut self, scale:&Vector3){
        let vector4scale = Vec4::replace_index3_f32(Vec4::from_vec3(scale.get_simd_value()),1.0);
        self._rows[0].set_float_type(Vec4::mul(self._rows[0].get_simd_value(), vector4scale).borrow());
        self._rows[1].set_float_type(Vec4::mul(self._rows[1].get_simd_value(), vector4scale).borrow());
        self._rows[2].set_float_type(Vec4::mul(self._rows[2].get_simd_value(), vector4scale).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_reciprocal_scaled(self) ->Matrix3x4{
    let mut result = Matrix3x4::create_from_rows(self._rows[0].borrow(),self._rows[1].borrow(),self._rows[2].borrow());
    result.multiply_by_scale(self.retrieve_scale_sq().get_reciprocal().borrow());
    return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_orthogonal_default(self)->bool{
        return self.is_orthogonal(constants::TOLERANCE);
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_orthogonal(self,tolerance:f32) ->bool{
        let row0 = self._rows[0].get_as_vector3();
        let row1 = self._rows[1].get_as_vector3();
        let row2 = self._rows[2].get_as_vector3();
        return
        row0.is_normalized(tolerance) &&
        row1.is_normalized(tolerance) &&
        row2.is_normalized(tolerance) &&
        row0.is_perpendicular(row1.borrow(), tolerance) &&
        row0.is_perpendicular(row2.borrow(), tolerance) &&
        row1.is_perpendicular(row2.borrow(), tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_orthogonalized(self) ->Matrix3x4{
        let mut result= Matrix3x4::new();
        let translation = self.get_translation();
        let row0 = self.get_row_as_vector3(1).cross(self.get_row_as_vector3(2).borrow()).get_normalized_safe(constants::TOLERANCE);
        let row1 = self.get_row_as_vector3(2).cross(row0.borrow()).get_normalized_safe(constants::TOLERANCE);
        let row2 = row0.cross(row1.borrow());
        result.set_row_vec3_f32(0, row0.borrow(), translation.get_x());
        result.set_row_vec3_f32(1, row1.borrow(), translation.get_y());
        result.set_row_vec3_f32(2, row2.borrow(), translation.get_z());
        return result;
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn orthogonalize(&mut self){
        self._rows = self.get_orthogonalized()._rows;
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close_default(self,rhs:&Matrix3x4)->bool{
        return self.is_close(rhs,constants::TOLERANCE);
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(self,rhs:&Matrix3x4,tolerance:f32)->bool{

        return self._rows[0].is_close(rhs._rows[0].borrow(), tolerance) && self._rows[1].is_close(rhs._rows[1].borrow(), tolerance) && self._rows[2].is_close(rhs._rows[2].borrow(), tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_euler_degrees(self) ->Vector3
    {
        return Vector3::vector3_rad_to_reg(self.get_euler_radians());
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_euler_radians(self) ->Vector3{
        let mut result = Vector3::new();
        let c2 = Vector2::new_xy(self.get_element(0, 0), self.get_element(0, 1)).get_length();
        result.set_x((-simd::atan2(self.get_element(1, 2), self.get_element(2, 2))));
        result.set_y((-simd::atan2((-self.get_element(0, 2)), c2)));
        let angles = Vector2::new_float_type(Vec2::sin(Vec2::load_immediate((-result.get_x()), (result.get_x() + constants::HALF_PI))));
        let s1 = angles.get_x();
        let c1 = angles.get_y();
        result.set_z((-simd::atan2((-c1 * self.get_element(1, 0) + s1 * self.get_element(2, 0)), (c1 * self.get_element(1, 1) - s1 * self.get_element(2, 1)))));
        return Vector3::new_float_type(Vec3::wrap(result.get_simd_value(), Vec3::zero_float(), Vec3::splat(constants::TWO_PI)));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_from_euler_degrees(&mut self, euler_degrees:&Vector3 ){
        self.set_from_euler_radians(Vector3::vector3deg_to_rad(euler_degrees).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_from_euler_radians(&mut self, euler_radians:&Vector3 ){
        let mut sin:FloatType = Vec3::zero_float();
        let mut cos:FloatType = Vec3::zero_float();
        Vec3::sin_cos(euler_radians.get_simd_value(), sin.borrow_mut(), cos.borrow_mut());

        let sx = Vec3::select_index0(sin);
        let sy = Vec3::select_index1(sin);
        let sz = Vec3::select_index2(sin);
        let cx = Vec3::select_index0(cos);
        let cy = Vec3::select_index1(cos);
        let cz = Vec3::select_index2(cos);

        self.set_row_xyzw(0, (cy * cz), (-cy * sz), sy, 0.0);
        self.set_row_xyzw(1, (cx * sz + sx * sy * cz), (cx * cz - sx * sy * sz), (-sx * cy), 0.0);
        self.set_row_xyzw(2, (sx * sz - cx * sy * cz), (sx * cz + cx * sy * sz), (cx * cy), 0.0);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_rotation_part_from_quaternion(&mut self, quaternion:&Quaternion){
        let tx = quaternion.get_x() * 2.0;
        let ty = quaternion.get_y() * 2.0;
        let tz = quaternion.get_z() * 2.0;
        let twx = quaternion.get_w() * tx;
        let twy = quaternion.get_w() * ty;
        let twz = quaternion.get_w() * tz;
        let txx = quaternion.get_x() * tx;
        let txy = quaternion.get_x() * ty;
        let txz = quaternion.get_x() * tz;
        let tyy = quaternion.get_y() * ty;
        let tyz = quaternion.get_y() * tz;
        let tzz = quaternion.get_z() * tz;

        self.set_row_xyzw(0, (1.0 - (tyy + tzz)), (txy - twz), (txz + twy), self._rows[0].get_w());
        self.set_row_xyzw(1, (txy + twz), (1.0 - (txx + tzz)), (tyz - twx), self._rows[1].get_w());
        self.set_row_xyzw(2, (txz - twy), (tyz + twx), (1.0 - (txx + tyy)), self._rows[2].get_w());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_determinant3x3(self) ->f32{
        return self._rows[0].get_element(0) * (self._rows[1].get_element(1) * self._rows[2].get_element(2) - self._rows[1].get_element(2) * self._rows[2].get_element(1))
            + self._rows[1].get_element(0) * (self._rows[2].get_element(1) * self._rows[0].get_element(2) - self._rows[2].get_element(2) * self._rows[0].get_element(1))
            + self._rows[2].get_element(0) * (self._rows[0].get_element(1) * self._rows[1].get_element(2) - self._rows[0].get_element(2) * self._rows[1].get_element(1));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_finite(self) ->bool{
        return self._rows[0].is_finite() && self._rows[1].is_finite() && self._rows[2].is_finite();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_simd_values_const(self) ->*const FloatType{
        return *self._rows as *const FloatType
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_simd_values(self) ->* FloatType{
        return *self._rows as * FloatType
    }
}