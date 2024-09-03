#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ops::Add;

use crate::math::common_sse::{Vec4Type, VecFourthType, VecThirdType, VecTwoType, VecType};
use crate::math::matrix3x3::Matrix3x3;
use crate::math::matrix3x4::Matrix3x4;
use crate::math::quaternion::Quaternion;
use crate::math::simd_math::simd;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::FloatArgType;

const ROW_COUNT:usize = 4;
const COL_COUNT:usize = 4;
#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    _rows:[Vector4; ROW_COUNT]
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

Matrix4x4& operator+=(const Matrix4x4& rhs);
//! @}

//! Operator for matrix-matrix substraction.
//! @{
[[nodiscard]] Matrix4x4 operator-(const Matrix4x4& rhs) const;
Matrix4x4& operator-=(const Matrix4x4& rhs);
//! @}

//! Operator for matrix-matrix multiplication.
//! @{
[[nodiscard]] Matrix4x4 operator*(const Matrix4x4& rhs) const;
Matrix4x4& operator*=(const Matrix4x4& rhs);
//! @}

//! Operator for multiplying all matrix's elements with a scalar
//! @{
[[nodiscard]] Matrix4x4 operator*(float multiplier) const;
Matrix4x4& operator*=(float multiplier);
//! @}

//! Operator for dividing all matrix's elements with a scalar
//! @{
[[nodiscard]] Matrix4x4 operator/(float divisor) const;
Matrix4x4& operator/=(float divisor);
//! @}

//! Operator for negating all matrix's elements
[[nodiscard]] Matrix4x4 operator-() const;

//! Post-multiplies the matrix by a vector.
//! Assumes that the w-component of the Vector3 is 1.0.
Vector3 operator*(const Vector3& rhs) const;

//! Post-multiplies the matrix by a vector.
Vector4 operator*(const Vector4& rhs) const;

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
        return Matrix4x4::new_4float_type(zero_vec.borrow(), zero_vec.borrow(), zero_vec.borrow(), zero_vec.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_value(value:&f32 ) ->Matrix4x4{
        let values = Vec4::splat(value);
        return Matrix4x4::new_4float_type(values.borrow(), values.borrow(), values.borrow(), values.borrow());
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
    pub unsafe  fn create_rotation_x(angle:&f32 ) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        let mut s :f32 = 0f32;
        let mut c :f32 = 0f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result._rows[0] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC1000.borrow()).borrow());
        result.set_row_xyzw(1.borrow(), 0.0.borrow(), c.borrow(), (-s).borrow(), 0.0.borrow());
        result.set_row_xyzw(2.borrow(), 0.0.borrow(), s.borrow(), c.borrow(), 0.0.borrow());
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_y(angle:&f32)->Matrix4x4{
        let mut result=Matrix4x4::new();
        let mut s :f32 = 0f32;
        let mut c :f32 = 0f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row_xyzw(0.borrow(), c, 0.0.borrow(), s.borrow(), 0.0.borrow());
        result._rows[1] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0100.borrow()).borrow());
        result.set_row_xyzw(2.borrow(), (-s).borrow(), 0.0.borrow(), c.borrow(), 0.0.borrow());
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_z(angle:&f32) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        let mut s :f32 = 0f32;
        let mut c :f32 = 0f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row_xyzw(0.borrow(), c.borrow(), (-s).borrow(), 0.0.borrow(), 0.0.borrow());
        result.set_row_xyzw(1.borrow(), s.borrow(), c.borrow(), 0.0.borrow(), 0.0.borrow());
        result._rows[2] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0010.borrow()).borrow());
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_quaternion(q:&Quaternion) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_rotation_part_from_quaternion(q);
        result.set_translation(Vector3::new_float_type(Vec3::zero_float().borrow()).borrow());
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
        return result;
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_quaternion_and_translation(q :&Quaternion, p:& Vector3)->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_rotation_part_from_quaternion(q);
        result.set_translation(p);
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix3x4(matrix3x4: &Matrix3x4) ->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.SetRow(0, matrix3x4.GetRow(0));
        result.SetRow(1, matrix3x4.GetRow(1));
        result.SetRow(2, matrix3x4.GetRow(2));
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
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
        return Matrix4x4::create_diagonal(Vector4::new_xyzw(scale.get_x().borrow(), scale.get_y().borrow(), scale.get_z().borrow(), 1.0.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_diagonal(diagonal:&Vector4)->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_row_xyzw(0.borrow(), diagonal.get_x().borrow(), 0.0.borrow(), 0.0.borrow(), 0.0.borrow());
        result.set_row_xyzw(1.borrow(), 0.0.borrow(), diagonal.get_y().borrow(), 0.0.borrow(), 0.0.borrow());
        result.set_row_xyzw(2.borrow(), 0.0.borrow(), 0.0.borrow(), diagonal.get_z().borrow(), 0.0.borrow());
        result.set_row_xyzw(3.borrow(), 0.0.borrow(), 0.0.borrow(), 0.0.borrow(), diagonal.get_w().borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_translation(translation:&Vector3 )->Matrix4x4{
        let mut result=Matrix4x4::new();
        result.set_row_xyzw(0.borrow(), 1.0.borrow(), 0.0.borrow(), 0.0.borrow(), translation.GetX());
        result.set_row_xyzw(1.borrow(), 0.0.borrow(), 1.0.borrow(), 0.0.borrow(), translation.GetY());
        result.set_row_xyzw(2.borrow(), 0.0.borrow(), 0.0.borrow(), 1.0.borrow(), translation.GetZ());
        result._rows[3] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_projection(fov_y:&f32, aspect_ratio:&f32, near_dist:&f32, far_dist:&f32) ->Matrix4x4{
        let mut sin:f32 = 0f32;
        let mut cos:f32 = 0f32;
        simd::sin_cos((0.5 * fov_y).borrow(), sin.borrow_mut(), cos.borrow_mut());
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
        let  result = Matrix4x4::new();
        let invfn = 1.0 / (far_dist - near_dist);
        result.set_row_xyzw(0, -2.0 * near_dist / (right - left), 0.0, (left + right) / (left - right), 0.0);
        result.set_row_xyzw(1, 0.0, 2.0 * near_dist / (top - bottom), (top + bottom) / (bottom - top), 0.0);
        result.set_row_xyzw(2, 0.0, 0.0, (far_dist + near_dist) * invfn, -2.0 * far_dist * near_dist * invfn);
        result.set_row_xyzw(3, 0.0, 0.0, 1.0, 0.0);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_interpolated(m1:&Matrix4x4 , m2:&Matrix4x4, t:f32 ){
        let mut m1Copy = Matrix3x3::create_from_matrix4x4(m1);
        let mut s1 = m1Copy.extract_scale();
        let mut t1 = m1.get_translation();
        let mut q1 = Quaternion::create_from_matrix3x3(m1Copy);

        let mut m2Copy = Matrix3x3::create_from_matrix4x4(m2);
        let s2 = m2Copy.extract_scale();
        let t2 = m2.get_translation();
        let q2 = Quaternion::create_from_matrix3x3(m2Copy);

        s1 = s1.lerp(s2, t);
        t1 = t1.lerp(t2, t);
        q1 = q1.slerp(q2, t);
        q1.normalize();
        let mut result = Matrix4x4::create_from_quaternion_and_translation(q1, t1);
        result.multiply_by_scale(s1);
        return result;
    }

    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn store_to_row_major_float16(self, values:*f32){
        self.get_row(0).store_to_float4(values);
        self.get_row(1).store_to_float4(values + 4);
        self.get_row(2).store_to_float4(values + 8);
        self.get_row(3).store_to_float4(values + 12);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn store_to_column_major_float16(self, values:*f32){
        self.get_column(0).store_to_float4(values);
        self.get_column(1).store_to_float4(values + 4);
        self.get_column(2).store_to_float4(values + 8);
        self.get_column(3).store_to_float4(values + 12);
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
        self._rows[row] = Vector4::new_float_type( Vec4::from_vec3(v.get_simd_value()).borrow());
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
    pub unsafe  fn set_translation_vec3(&mut self, v:&Vector3){
        self.set_column_vec3(3, v);
    }


    //! Pre-multiplies the matrix by a vector, using only the upper 3x3 submatrix.
    //! Note that this is not the usual multiplication order for transformations.
    Vector3 TransposedMultiply3x3(const Vector3& v) const;

    //! Post-multiplies the matrix by a vector, using only the upper 3x3 submatrix.
    Vector3 Multiply3x3(const Vector3& v) const;

    //! Transpose operations.
    //! @{
    Matrix4x4 GetTranspose() const;
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn Transpose();
    //! @}

    //! Performs a full inversion for an arbitrary 4x4 matrix.
    //! Using GetInverseTransform or GetFastInverse will often be possible, use them in preference to this.
    //! @{
    Matrix4x4 GetInverseFull() const;
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn  InvertFull();
    //! @}

    //! Gets the inverse of the matrix.
    //! Assumes that the last row is (0,0,0,1), use GetInverseFull if this is not true.
    //! @{
    Matrix4x4 GetInverseTransform() const;
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn InvertTransform();
    //! @}

    //! Fast inversion.
    //! Assumes the matrix consists of an upper 3x3 orthogonal matrix (i.e. a rotation) and a translation in the last column.
    //! @{
    Matrix4x4 GetInverseFast() const;
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn InvertFast();
    //! @}

    //! Gets the scale part of the transformation, i.e. the length of the scale components.
    [[nodiscard]] Vector3 RetrieveScale() const;

    //! Gets the squared scale part of the transformation (the squared length of the basis vectors).
    [[nodiscard]] Vector3 RetrieveScaleSq() const;

    //! Gets the scale part of the transformation as in RetrieveScale, and also removes this scaling from the matrix.
    Vector3 ExtractScale();

    //! Quick multiplication by a scale matrix, equivalent to m*=Matrix4x4::CreateScale(scale).
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn MultiplyByScale(const Vector3& scale);

    //! Returns a matrix with the reciprocal scale, keeping the same rotation and translation.
    [[nodiscard]] Matrix4x4 GetReciprocalScaled() const;

    bool IsClose(const Matrix4x4& rhs, float tolerance = Constants::Tolerance) const;

    bool operator==(const Matrix4x4& rhs) const;
    bool operator!=(const Matrix4x4& rhs) const;

    //! sets the upper 3x3 rotation part of the matrix from a quaternion.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn SetRotationPartFromQuaternion(const Quaternion& q);

    Vector4 GetDiagonal() const;

    bool IsFinite() const;

    const Simd::Vec4::FloatType* GetSimdValues() const;

    Simd::Vec4::FloatType* GetSimdValues();

    private:


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn CreateProjectionInternal(float cotX, float cotY, float nearDist, float farDist);
}