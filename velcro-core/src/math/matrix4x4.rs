#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::common_sse::{Vec4Type, VecFourthType, VecThirdType, VecTwoType, VecType};
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
    pub fn new_4float_type(row0:&FloatArgType,row1:&FloatArgType,row2:&FloatArgType,row3:&FloatArgType)->Matrix4x4{
        unsafe {
            Matrix4x4 {
                _rows: [Vector4::new_float_type(row0), Vector4::new_float_type(row1), Vector4::new_float_type(row2),Vector4::new_float_type(row3)]
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_identity() ->Matrix4x4{
    return Matrix4x4::new_4float_type(Vec4::load_aligned(simd::G_VEC1000.borrow()).borrow()
    , Vec4::load_aligned(simd::G_VEC0100.borrow()).borrow()
    , Vec4::load_aligned(simd::G_VEC0010.borrow()).borrow()
    , Vec4::load_aligned(simd::G_VEC0001.borrow()).borrow());
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
    pub unsafe  fn CreateProjectionOffset(float left, float right, float bottom, float top, float nearDist, float farDist){

    }

    //! Interpolates between two matrices; linearly for scale/translation, spherically for rotation.

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn CreateInterpolated(const Matrix4x4& m1, const Matrix4x4& m2, float t);

    //! Stores the matrix into to an array of 16 floats.
    //! The floats need only be 4 byte aligned, 16 byte alignment is not required.
    void StoreToRowMajorFloat16(float* values) const;

    //! Stores the matrix into to an array of 16 floats.
    //! The floats need only be 4 byte aligned, 16 byte alignment is not required.
    void StoreToColumnMajorFloat16(float* values) const;

    //! Indexed accessor functions.
    //! @{
    float GetElement(int32_t row, int32_t col) const;
    void SetElement(int32_t row, int32_t col, float value);
    //! @}

    //! Indexed access using operator().
    float operator()(int32_t row, int32_t col) const;

    //! Row access functions.
    //! @{
    Vector4 GetRow(int32_t row) const;
    Vector3 GetRowAsVector3(int32_t row) const;
    void GetRows(Vector4* row0, Vector4* row1, Vector4* row2, Vector4* row3) const;
    void SetRow(int32_t row, float x, float y, float z, float w);
    void SetRow(int32_t row, const Vector3& v);
    void SetRow(int32_t row, const Vector3& v, float w);
    void SetRow(int32_t row, const Vector4& v);
    void SetRows(const Vector4& row0, const Vector4& row1, const Vector4& row2, const Vector4& row3);
    //! @}

    //! Column access functions.
    //! @{
    Vector4 GetColumn(int32_t col) const;
    Vector3 GetColumnAsVector3(int32_t col) const;
    void GetColumns(Vector4* col0, Vector4* col1, Vector4* col2, Vector4* col3) const;
    void SetColumn(int32_t col, float x, float y, float z, float w);
    void SetColumn(int32_t col, const Vector3& v);
    void SetColumn(int32_t col, const Vector3& v, float w);
    void SetColumn(int32_t col, const Vector4& v);
    void SetColumns(const Vector4& col0, const Vector4& col1, const Vector4& col2, const Vector4& col3);
    //! @}

    //! Basis (column) access functions.
    //! @{
    Vector4 GetBasisX() const;
    Vector3 GetBasisXAsVector3() const;
    void SetBasisX(float x, float y, float z, float w);
    void SetBasisX(const Vector4& v);
    Vector4 GetBasisY() const;
    Vector3 GetBasisYAsVector3() const;
    void SetBasisY(float x, float y, float z, float w);
    void SetBasisY(const Vector4& v);
    Vector4 GetBasisZ() const;
    Vector3 GetBasisZAsVector3() const;
    void SetBasisZ(float x, float y, float z, float w);
    void SetBasisZ(const Vector4& v);
    void GetBasisAndTranslation(Vector4* basisX, Vector4* basisY, Vector4* basisZ, Vector4* pos) const;
    void SetBasisAndTranslation(const Vector4& basisX, const Vector4& basisY, const Vector4& basisZ, const Vector4& pos);
    //! @}

    //! Position (last column) access functions.
    //! @{
    Vector3 GetTranslation() const;
    void SetTranslation(float x, float y, float z);
    void SetTranslation(const Vector3& v);
    //! @}

    //! Operator for matrix-matrix addition.
    //! @{
    [[nodiscard]] Matrix4x4 operator+(const Matrix4x4& rhs) const;
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

    //! Pre-multiplies the matrix by a vector, using only the upper 3x3 submatrix.
    //! Note that this is not the usual multiplication order for transformations.
    Vector3 TransposedMultiply3x3(const Vector3& v) const;

    //! Post-multiplies the matrix by a vector, using only the upper 3x3 submatrix.
    Vector3 Multiply3x3(const Vector3& v) const;

    //! Transpose operations.
    //! @{
    Matrix4x4 GetTranspose() const;
    void Transpose();
    //! @}

    //! Performs a full inversion for an arbitrary 4x4 matrix.
    //! Using GetInverseTransform or GetFastInverse will often be possible, use them in preference to this.
    //! @{
    Matrix4x4 GetInverseFull() const;
    void  InvertFull();
    //! @}

    //! Gets the inverse of the matrix.
    //! Assumes that the last row is (0,0,0,1), use GetInverseFull if this is not true.
    //! @{
    Matrix4x4 GetInverseTransform() const;
    void InvertTransform();
    //! @}

    //! Fast inversion.
    //! Assumes the matrix consists of an upper 3x3 orthogonal matrix (i.e. a rotation) and a translation in the last column.
    //! @{
    Matrix4x4 GetInverseFast() const;
    void InvertFast();
    //! @}

    //! Gets the scale part of the transformation, i.e. the length of the scale components.
    [[nodiscard]] Vector3 RetrieveScale() const;

    //! Gets the squared scale part of the transformation (the squared length of the basis vectors).
    [[nodiscard]] Vector3 RetrieveScaleSq() const;

    //! Gets the scale part of the transformation as in RetrieveScale, and also removes this scaling from the matrix.
    Vector3 ExtractScale();

    //! Quick multiplication by a scale matrix, equivalent to m*=Matrix4x4::CreateScale(scale).
    void MultiplyByScale(const Vector3& scale);

    //! Returns a matrix with the reciprocal scale, keeping the same rotation and translation.
    [[nodiscard]] Matrix4x4 GetReciprocalScaled() const;

    bool IsClose(const Matrix4x4& rhs, float tolerance = Constants::Tolerance) const;

    bool operator==(const Matrix4x4& rhs) const;
    bool operator!=(const Matrix4x4& rhs) const;

    //! sets the upper 3x3 rotation part of the matrix from a quaternion.
    void SetRotationPartFromQuaternion(const Quaternion& q);

    Vector4 GetDiagonal() const;

    bool IsFinite() const;

    const Simd::Vec4::FloatType* GetSimdValues() const;

    Simd::Vec4::FloatType* GetSimdValues();

    private:


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn CreateProjectionInternal(float cotX, float cotY, float nearDist, float farDist);
}