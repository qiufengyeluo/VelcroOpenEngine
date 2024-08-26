#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::fmt::Debug;

use crate::math::common_sse::{Vec3Type, VecTwoType, VecType};
use crate::math::constants::{G_VEC0010, G_VEC0100, G_VEC1000, TOLERANCE};
use crate::math::math_utils::constants;
use crate::math::math_utils::constants::Axis;
use crate::math::matrix3x3::Matrix3x3;
use crate::math::quaternion::Quaternion;
use crate::math::simd_math::{simd, simd_sin_cos};
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::{FloatArgType, FloatType};

const ROW_COUNT:usize = 3;
const COL_COUNT:usize = 4;
#[derive(Debug, Copy, Clone)]
pub struct Matrix3x4 {
    _rows:[Vector4; ROW_COUNT]
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
    pub fn new_3float_type(row0:&FloatArgType,row1:&FloatArgType,row2:&FloatArgType)->Matrix3x4{
        unsafe {
            Matrix3x4 {
                _rows: [Vector4::new_float_type(row0), Vector4::new_float_type(row1), Vector4::new_float_type(row2)]
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_identity()->Matrix3x4{
        return Matrix3x4::new_3float_type(Vec4::load_aligned(simd::G_VEC1000.borrow()).borrow(),Vec4::load_aligned(simd::G_VEC0100.borrow()).borrow(),Vec4::load_aligned(simd::G_VEC0010.borrow()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_zero()->Matrix3x4{
        let zero_vec = Vec4::zero_float();
        return Matrix3x4::new_3float_type(zero_vec.borrow(), zero_vec.borrow(), zero_vec.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_value(value:&f32)->Matrix3x4{
        let values = Vec4::splat(value);
        return Matrix3x4::new_3float_type(values.borrow(), values.borrow(), values.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_row_major_float12(values:&[f32;12]){
        return Matrix3x4::create_from_rows(Vector4::create_from_float4(*values[0]).borrow(),
                                           Vector4::create_from_float4(*values[4]).borrow(),
                                           Vector4::create_from_float4(*values[8]).borrow());
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
            Vector3::create_from_float_3(&values[0]).borrow(),
            Vector3::create_from_float_3(&values[3]).borrow(),
            Vector3::create_from_float_3(&values[6]).borrow(),
            Vector3::create_from_float_3(&values[9]).borrow()
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
        return Matrix3x4::create_from_columns(Vector3::create_from_float_3(&values[0]).borrow(),
                                              Vector3::create_from_float_3(&values[4]).borrow(),
                                              Vector3::create_from_float_3(&values[8]).borrow(),
                                              Vector3::create_from_float_3(&values[12]).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_x(angle:&f32)->Matrix3x4{
        let mut result=Matrix3x4::new() ;
        let mut s:f32 = 0.0;
        let mut c:f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result._rows[0] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC1000.borrow()).borrow());
        result.set_row(1.borrow(), 0.0.borrow(), c.borrow(), (-s).borrow(), 0.0.borrow());
        result.set_row(2.borrow(), 0.0.borrow(), s.borrow(), c.borrow(), 0.0.borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_y(angle:&f32)->Matrix3x4{
        let mut result=Matrix3x4::new() ;
        let mut s:f32 = 0.0;
        let mut c:f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row(0.borrow(), c.borrow(), 0.0.borrow(), s.borrow(), 0.0.borrow());
        result._rows[1] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0100.borrow()).borrow());
        result.set_row(2.borrow(), (-s).borrow(), 0.0.borrow(), c.borrow(), 0.0.borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_z(angle:&f32)->Matrix3x4{
        let mut result=Matrix3x4::new() ;
        let mut s:f32 = 0.0;
        let mut c:f32;
        simd::sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row(0.borrow(), c.borrow(), (-s).borrow(),0.0.borrow(), 0.0.borrow());
        result.set_row(1.borrow(), s.borrow(), c.borrow(), 0.0.borrow(), 0.0.borrow());
        result._rows[2] = Vector4::new_float_type(Vec4::load_aligned(simd::G_VEC0010.borrow()).borrow());

        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_quaternion(quaternion:&Quaternion)->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_rotation_rart_from_quaternion(quaternion);
        result.set_translation(Vector3::CreateZero());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_quaternion_and_translation(quaternion:& Quaternion , translation: &Vector3)->Matrix3x4
    {
        let mut result=Matrix3x4::new();
        result.set_rotation_rart_from_quaternion(quaternion);
        result.set_translation(translation);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_matrix3x3(matrix3x3: &Matrix3x3) ->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_row(0, matrix3x3.GetRow(0), 0.0);
        result.set_row(1, matrix3x3.GetRow(1), 0.0);
        result.set_row(2, matrix3x3.GetRow(2), 0.0);
        result
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_matrix3x3and_translation(matrix3x3:&Matrix3x3, translation:&Vector3 )->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_rows(
        Vector4::create_from_vector3_and_float(matrix3x3.get_row(0), translation.get_element(0)),
        Vector4::create_from_vector3_and_float(matrix3x3.get_row(1), translation.get_element(1)),
        Vector4::create_from_vector3_and_float(matrix3x3.get_row(2), translation.get_element(2))
        );
        result
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn unsafe_create_from_matrix4x4(matrix4x4:&Matrix4x4) ->Matrix3x4{
        let mut result=Matrix3x4::new();
        result.set_row(0, matrix4x4.get_row(0));
        result.set_row(1, matrix4x4.get_row(1));
        result.set_row(2, matrix4x4.get_row(2));
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
        return Matrix3x4::create_from_rows(Vector4::create_axis_x(diagonal.get_x().borrow()).borrow(),
                                           Vector4::create_axis_y(diagonal.get_y().borrow()).borrow(),
                                           Vector4::create_axis_z(diagonal.get_z().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_translation(translation:&Vector3) ->Matrix3x4{
        let result = Matrix3x4::create_identity();
        result.set_translation(translation);
        result
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_look_at_default(from:&Vector3, to:&Vector3) ->Matrix3x4{
        return Matrix3x4::create_look_at(from,to,constants::Axis::YPositive.borrow());
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_look_at(from:&Vector3, to:&Vector3, forwardAxis:&constants::Axis) ->Matrix3x4{
            let result =Matrix3x4::create_identity();
            let mut targetForward = to - from;

        if targetForward.is_zero_default()
        {
            return result;
        }

        targetForward.normalize();

        let mut up = Vector3::create_axis_z(1.0.borrow());

        let abs_dot = simd::abs(targetForward.dot3(up.borrow()).borrow());
        if (abs_dot > 1.0 - 0.001)
        {
            up = targetForward.cross_y_axis();
        }

        let mut right = targetForward.cross(up.borrow());
        right.normalize();
        up = right.cross(targetForward.borrow());
        up.normalize();
        match forwardAxis {
            Axis::XPositive => {
                result.set_basis_and_translation(targetForward, -right, up, from);
            }
            Axis::XNegative => {
                result.set_basis_and_translation(-targetForward, right, up, from);
            }
            Axis::YPositive => {
                result.set_basis_and_translation(right, targetForward, up, from);
            }
            Axis::YNegative => {
                result.set_basis_and_translation(-right, -targetForward, up, from);
            }
            Axis::ZPositive => {
                result.set_basis_and_translation(right, -up, targetForward, from);
            }
            Axis::ZNegative => {
                result.set_basis_and_translation(right, up, -targetForward, from);
            }
            _ =>{
                result.set_basis_and_translation(right, targetForward, up, from);
            }
        }
         result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn identity()->Matrix3x4{
        return Matrix3x4::create_identity();;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_row_major_float12(self, values:&[f32;12]){
        self.get_row(0.borrow()).store_to_float_4((*values[0] as usize) as *mut f32);
        self.get_row(1.borrow()).store_to_float_4((*values[4] as usize) as *mut f32);
        self.get_row(2.borrow()).store_to_float_4((*values[8] as usize) as *mut f32);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_column_major_float12(self, values:&[f32;12]) {
        self.get_column(0).store_to_float_3(values);
        self.get_column(1).store_to_float_3(values + 3);
        self.get_column(2).store_to_float_3(values + 6);
        self.get_column(3).store_to_float_3(values + 9);
    }

    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_column_major_float16(self, values:&[f32;16]) {
        self.get_column(0).store_to_float_4(values);
        self.get_column(1).store_to_float_4(values + 4);
        self.get_column(2).store_to_float_4(values + 8);
        self.get_column(3).store_to_float_4(values + 12);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_element(self, row:&i32, col:&i32 ) ->f32{
        return self._rows[row].get_element(col);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(&mut self, row:&i32, col:&i32,   value:&f32){
        self._rows[row].set_element(col, value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_row(self, row:&i32 ) ->Vector4{
        return self._rows[row];
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_row_as_vector3(self, row:&i32)->Vector3{
        return self._rows[row].get_as_vector3();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_row_xyzw(&mut self, row:&i32 ,  x:&f32,  y:&f32,  z:&f32,  w:&f32){
        self._rows[row].set_xyzw(x, y, z, w);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_row_vec3_f32(&mut self,  row:&i32, v: &Vector3, w:&f32){
        self._rows[row].set_vec3_f32(v, w);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_row_vec4(&mut self, row:&i32 , v:& Vector4){
        self._rows[row] = v;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetRows(Vector4* row0, Vector4* row1, Vector4* row2) const;

    //! Sets all rows of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetRows(const Vector4& row0, const Vector4& row1, const Vector4& row2);

    //! Gets the specified column.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetColumn(int32_t col) const;

    //! Sets the specified column.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetColumn(int32_t col, float x, float y, float z);

    //! Sets the specified column.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetColumn(int32_t col, const Vector3& v);

    //! Gets all the columns of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetColumns(Vector3* col0, Vector3* col1, Vector3* col2, Vector3* col3) const;

    //! Sets all the columns of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetColumns(const Vector3& col0, const Vector3& col1, const Vector3& col2, const Vector3& col3);

    //! Gets the X basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetBasisX() const;

    //! Sets the X basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetBasisX(float x, float y, float z);

    //! Sets the X basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetBasisX(const Vector3& v);

    //! Gets the Y basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetBasisY() const;

    //! Sets the Y basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetBasisY(float x, float y, float z);

    //! Sets the Y basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetBasisY(const Vector3& v);

    //! Gets the Z basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetBasisZ() const;

    //! Sets the Z basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetBasisZ(float x, float y, float z);

    //! Sets the Z basis vector of the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetBasisZ(const Vector3& v);

    //! Gets the translation.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetTranslation() const;

    //! Sets the translation.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetTranslation(float x, float y, float z);

    //! Sets the translation.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetTranslation(const Vector3& v);

    //! Gets the three basis vectors and the translation.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetBasisAndTranslation(Vector3* basisX, Vector3* basisY, Vector3* basisZ, Vector3* translation) const;

    //! Sets the three basis vectors and the translation.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetBasisAndTranslation(const Vector3& basisX, const Vector3& basisY, const Vector3& basisZ, const Vector3& translation);

    //! Operator for matrix-matrix addition.
    //! @{
    [[nodiscard]] Matrix3x4 operator+(const Matrix3x4& rhs) const;
    Matrix3x4& operator+=(const Matrix3x4& rhs);
    //! @}

    //! Operator for matrix-matrix subtraction.
    //! @{
    [[nodiscard]] Matrix3x4 operator-(const Matrix3x4& rhs) const;
    Matrix3x4& operator-=(const Matrix3x4& rhs);
    //! @}

    //! Operator for matrix-matrix multiplication.
    //! @{
    [[nodiscard]] Matrix3x4 operator*(const Matrix3x4& rhs) const;
    Matrix3x4& operator*=(const Matrix3x4& rhs);
    //! @}

    //! Operator for multiplying all matrix's elements with a scalar
    //! @{
    [[nodiscard]] Matrix3x4 operator*(float multiplier) const;
    Matrix3x4& operator*=(float multiplier);
    //! @}

    //! Operator for dividing all matrix's elements with a scalar
    //! @{
    [[nodiscard]] Matrix3x4 operator/(float divisor) const;
    Matrix3x4& operator/=(float divisor);
    //! @}

    //! Operator for negating all matrix's elements
    [[nodiscard]] Matrix3x4 operator-() const;

    //! Operator for transforming a Vector3.
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn operator*(const Vector3& rhs) const;

    //! Operator for transforming a Vector4.
    [[nodiscard]] Vector4 operator*(const Vector4& rhs) const;

    //! Post-multiplies the matrix by a vector, using only the 3x3 part of the matrix.
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn Multiply3x3(const Vector3& rhs) const;

    //! Post-multiplies the matrix by a vector, using only the 3x3 part of the matrix.
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn TransformVector(const Vector3& rhs) const;

    //! Post-multiplies the matrix by a point, using the rotation and translation part of the matrix.
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn TransformPoint(const Vector3& rhs) const;

    //! Gets the result of transposing the 3x3 part of the matrix, setting the translation part to zero.
    [[nodiscard]] Matrix3x4 GetTranspose() const;

    //! Transposes the 3x3 part of the matrix, and sets the translation part to zero.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn Transpose();

    //! Gets the matrix obtained by transposing the 3x3 part of the matrix, leaving the translation untouched.
    [[nodiscard]] Matrix3x4 GetTranspose3x3() const;

    //! Transposes the 3x3 part of the matrix, leaving the translation untouched.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn Transpose3x3();

    //! Gets the inverse of the transformation represented by the matrix.
    //! This function works for any matrix, even if they have scaling or skew.
    //! If the 3x3 part of the matrix is orthogonal then \ref GetInverseFast is much faster.
    [[nodiscard]] Matrix3x4 GetInverseFull() const;

    //! Inverts the transformation represented by the matrix.
    //! This function works for any matrix, even if they have scaling or skew.
    //! If the 3x3 part of the matrix is orthogonal then \ref InvertFast is much faster.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn InvertFull();

    //! Gets the inverse of the transformation represented by the matrix, assuming the 3x3 part is orthogonal.
    [[nodiscard]] Matrix3x4 GetInverseFast() const;

    //! Inverts the transformation represented by the matrix, assuming the 3x3 part is orthogonal.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn InvertFast();

    //! Gets the scale part of the transformation (the length of the basis vectors).
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn RetrieveScale() const;

    //! Gets the squared scale part of the transformation (the squared length of the basis vectors).
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn RetrieveScaleSq() const;

    //! Gets the scale part of the transformation as in RetrieveScale, and also removes this scaling from the matrix.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn ExtractScale();

    //! Multiplies the basis vectors of the matrix by the elements of the scale specified.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn MultiplyByScale(const Vector3& scale);

    //! Returns a matrix with the reciprocal scale, keeping the same rotation and translation.
    [[nodiscard]] Matrix3x4 GetReciprocalScaled() const;

    //! Tests if the 3x3 part of the matrix is orthogonal.
    bool IsOrthogonal(float tolerance = Constants::Tolerance) const;

    //! Returns an orthogonal matrix based on this matrix.
    [[nodiscard]] Matrix3x4 GetOrthogonalized() const;

    //! Modifies the basis vectors of the matrix to be orthogonal and unit length.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn Orthogonalize();

    //! Tests element-wise whether this matrix is close to another matrix, within the specified tolerance.
    bool IsClose(const Matrix3x4& rhs, float tolerance = Constants::Tolerance) const;

    //! Tests whether this matrix is identical to another matrix.
    bool operator==(const Matrix3x4& rhs) const;

    //! Tests whether this matrix is not identical to another matrix.
    bool operator!=(const Matrix3x4& rhs) const;

    //! Converts the 3x3 part of the matrix to corresponding Euler angles (Z, then Y, then X), in degrees.
    //! @return Component-wise rotation angles in degrees.
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetEulerDegrees() const;

    //! Converts the 3x3 part of the matrix to corresponding Euler angles (Z, then Y, then X), in radians.
    //! @return Component-wise rotation angles in radians.
    [[nodiscard]] 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetEulerRadians() const;

    //! Sets the 3x3 part of the matrix from Euler Angles (rotation angles in Z, then Y, then X), in degrees.
    //! The translation is set to zero.
    //! @param eulerDegrees Component-wise rotation angles in degrees.
    //! @return A matrix calculated from the composition of rotations around Z, then Y, then X, with zero translation.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetFromEulerDegrees(const Vector3& eulerDegrees);

    //! Sets the 3x3 part of the matrix from Euler Angles (rotation angles in Z, then Y, then X), in radians.
    //! The translation is set to zero.
    //! @param 
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn eulerRadians Component-wise rotation angles in radians.
    //! @return A matrix calculated from the composition of rotations around Z, then Y, then X, with zero translation.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetFromEulerRadians(const Vector3& eulerRadians);

    //! Sets the 3x3 part of the matrix from a quaternion.
    
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn SetRotationPartFromQuaternion(const Quaternion& quaternion);

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetDeterminant3x3() ->f32;

    //! Checks whether the elements of the matrix are all finite.
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn IsFinite() ->bool;

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetSimdValuesConst() ->*const FloatType;

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn GetSimdValues() ->* FloatType;
}