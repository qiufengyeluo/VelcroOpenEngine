#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::math::common_sse::{Vec3Type, VecTwoType, VecType};
use crate::math::constants::{G_VEC0010, G_VEC0100, G_VEC1000, TOLERANCE};
use crate::math::frustum::is_close;
use crate::math::math_utils;
use crate::math::quaternion::Quaternion;
use crate::math::simd_math::simd_sin_cos;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::vsimd::{FloatArgType, FloatType};

const ROW_COUNT:usize = 3;
const COL_COUNT:usize = 4;
#[derive(Debug, Copy, Clone)]
pub struct Matrix3x4 {
    _rows:[Vector3; ROW_COUNT]
}

impl Matrix3x4{



    using Axis = Constants::Axis;

    //! Default constructor, which does not initialize the matrix.
    Matrix3x4() = default;
    Matrix3x4(const Matrix3x4& rhs) = default;

    Matrix3x4(Simd::Vec4::FloatArgType row0, Simd::Vec4::FloatArgType row1, Simd::Vec4::FloatArgType row2);

    //! Creates an identity Matrix3x4.
    static Matrix3x4 CreateIdentity();

    //! Creates a Matrix3x4 with all values zero.
    static Matrix3x4 CreateZero();

    //! Constructs a matrix with all components set to the specified value.
    static Matrix3x4 CreateFromValue(float value);

    //! Constructs from an array of 12 floats stored in row-major order.
    static Matrix3x4 CreateFromRowMajorFloat12(const float values[12]);

    //! Constructs from individual rows.
    static Matrix3x4 CreateFromRows(const Vector4& row0, const Vector4& row1, const Vector4& row2);

    //! Constructs from an array of 12 floats stored in column-major order.
    static Matrix3x4 CreateFromColumnMajorFloat12(const float values[12]);

    //! Constructs from individual columns.
    static Matrix3x4 CreateFromColumns(const Vector3& col0, const Vector3& col1, const Vector3& col2, const Vector3& col3);

    //! Constructs from an array of 16 floats stored in column-major order.
    //! The 16 floats are treated as 4 columns of 4 values, but the 4th value of each column is ignored.
    static Matrix3x4 CreateFromColumnMajorFloat16(const float values[16]);

    //! Sets the matrix to be a rotation around the X-axis, specified in radians.
    static Matrix3x4 CreateRotationX(float angle);

    //! Sets the matrix to be a rotation around the Y-axis, specified in radians.
    static Matrix3x4 CreateRotationY(float angle);

    //! Sets the matrix to be a rotation around the Z-axis, specified in radians.
    static Matrix3x4 CreateRotationZ(float angle);

    //! Sets the matrix from a quaternion, with translation set to zero.
    static Matrix3x4 CreateFromQuaternion(const Quaternion& quaternion);

    //! Sets the matrix from a quaternion and a translation.
    static Matrix3x4 CreateFromQuaternionAndTranslation(const Quaternion& quaternion, const Vector3& translation);

    //! Constructs from a Matrix3x3, with translation set to zero.
    static Matrix3x4 CreateFromMatrix3x3(const Matrix3x3& matrix3x3);

    //! Constructs from a Matrix3x3 and a translation.
    static Matrix3x4 CreateFromMatrix3x3AndTranslation(const Matrix3x3& matrix3x3, const Vector3& translation);

    //! Constructs from a Matrix4x4.
    static Matrix3x4 UnsafeCreateFromMatrix4x4(const Matrix4x4& matrix4x4);

    //! Constructs from a Transform.
    static Matrix3x4 CreateFromTransform(const Transform& transform);

    //! Sets the matrix to be a scale matrix, with translation set to zero.
    static Matrix3x4 CreateScale(const Vector3& scale);

    //! Sets the matrix to be a diagonal matrix, with translation set to zero.
    static Matrix3x4 CreateDiagonal(const Vector3& diagonal);

    //! Sets the matrix to be a translation matrix, with 3x3 part set to the identity.
    static Matrix3x4 CreateTranslation(const Vector3& translation);

    //! Creates a "look at" matrix.
    //! Given a source position and target position, computes a transform at the source position that points
    //! toward the target along a chosen local-space axis.
    //! @param from The source position (world space).
    //! @param to The target position (world space).
    //! @param forwardAxis The local-space basis axis that should "look at" the target.
    //! @return The resulting Matrix3x4 or the identity if the source and target coincide.
    static Matrix3x4 CreateLookAt(const Vector3& from, const Vector3& to, Matrix3x4::Axis forwardAxis = Matrix3x4::Axis::YPositive);

    //! Returns a reference to the identity Matrix3x4.
    static Matrix3x4 Identity();

    //! Stores the matrix into to an array of 12 floats.
    //! The floats need only be 4 byte aligned, 16 byte alignment is not required.
    void StoreToRowMajorFloat12(float values[12]) const;

    //! Stores the matrix into to an array of 12 floats.
    //! The floats need only be 4 byte aligned, 16 byte alignment is not required.
    void StoreToColumnMajorFloat12(float values[12]) const;

    //! Stores the matrix into to an array of 16 floats.
    //! Because the matrix contains only 12 elements, there are 4 padding values in the array of 16 which may have
    //! arbitrary values written to them.
    //! The floats need only be 4 byte aligned, 16 byte alignment is not required.
    void StoreToColumnMajorFloat16(float values[16]) const;

    //! Gets the element in the specified row and column.
    //! Accessing individual elements can be slower than working with entire rows.
    float GetElement(int32_t row, int32_t col) const;

    //! Sets the element in the specified row and column.
    //! Accessing individual elements can be slower than working with entire rows.
    void SetElement(int32_t row, int32_t col, const float value);

    //! Accesses the element in the specified row and column.
    //! Accessing individual elements can be slower than working with entire rows.
    float operator()(int32_t row, int32_t col) const;

    //! Gets the specified row.
    Vector4 GetRow(int32_t row) const;

    //! Gets the specified row as a Vector3.
    Vector3 GetRowAsVector3(int32_t row) const;

    //! Sets the specified row.
    void SetRow(int32_t row, float x, float y, float z, float w);

    //! Sets the specified row.
    void SetRow(int32_t row, const Vector3& v, float w);

    //! Sets the specified row.
    void SetRow(int32_t row, const Vector4& v);

    //! Gets all rows of the matrix.
    void GetRows(Vector4* row0, Vector4* row1, Vector4* row2) const;

    //! Sets all rows of the matrix.
    void SetRows(const Vector4& row0, const Vector4& row1, const Vector4& row2);

    //! Gets the specified column.
    Vector3 GetColumn(int32_t col) const;

    //! Sets the specified column.
    void SetColumn(int32_t col, float x, float y, float z);

    //! Sets the specified column.
    void SetColumn(int32_t col, const Vector3& v);

    //! Gets all the columns of the matrix.
    void GetColumns(Vector3* col0, Vector3* col1, Vector3* col2, Vector3* col3) const;

    //! Sets all the columns of the matrix.
    void SetColumns(const Vector3& col0, const Vector3& col1, const Vector3& col2, const Vector3& col3);

    //! Gets the X basis vector of the matrix.
    Vector3 GetBasisX() const;

    //! Sets the X basis vector of the matrix.
    void SetBasisX(float x, float y, float z);

    //! Sets the X basis vector of the matrix.
    void SetBasisX(const Vector3& v);

    //! Gets the Y basis vector of the matrix.
    Vector3 GetBasisY() const;

    //! Sets the Y basis vector of the matrix.
    void SetBasisY(float x, float y, float z);

    //! Sets the Y basis vector of the matrix.
    void SetBasisY(const Vector3& v);

    //! Gets the Z basis vector of the matrix.
    Vector3 GetBasisZ() const;

    //! Sets the Z basis vector of the matrix.
    void SetBasisZ(float x, float y, float z);

    //! Sets the Z basis vector of the matrix.
    void SetBasisZ(const Vector3& v);

    //! Gets the translation.
    Vector3 GetTranslation() const;

    //! Sets the translation.
    void SetTranslation(float x, float y, float z);

    //! Sets the translation.
    void SetTranslation(const Vector3& v);

    //! Gets the three basis vectors and the translation.
    void GetBasisAndTranslation(Vector3* basisX, Vector3* basisY, Vector3* basisZ, Vector3* translation) const;

    //! Sets the three basis vectors and the translation.
    void SetBasisAndTranslation(const Vector3& basisX, const Vector3& basisY, const Vector3& basisZ, const Vector3& translation);

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
    [[nodiscard]] Vector3 operator*(const Vector3& rhs) const;

    //! Operator for transforming a Vector4.
    [[nodiscard]] Vector4 operator*(const Vector4& rhs) const;

    //! Post-multiplies the matrix by a vector, using only the 3x3 part of the matrix.
    [[nodiscard]] Vector3 Multiply3x3(const Vector3& rhs) const;

    //! Post-multiplies the matrix by a vector, using only the 3x3 part of the matrix.
    [[nodiscard]] Vector3 TransformVector(const Vector3& rhs) const;

    //! Post-multiplies the matrix by a point, using the rotation and translation part of the matrix.
    [[nodiscard]] Vector3 TransformPoint(const Vector3& rhs) const;

    //! Gets the result of transposing the 3x3 part of the matrix, setting the translation part to zero.
    [[nodiscard]] Matrix3x4 GetTranspose() const;

    //! Transposes the 3x3 part of the matrix, and sets the translation part to zero.
    void Transpose();

    //! Gets the matrix obtained by transposing the 3x3 part of the matrix, leaving the translation untouched.
    [[nodiscard]] Matrix3x4 GetTranspose3x3() const;

    //! Transposes the 3x3 part of the matrix, leaving the translation untouched.
    void Transpose3x3();

    //! Gets the inverse of the transformation represented by the matrix.
    //! This function works for any matrix, even if they have scaling or skew.
    //! If the 3x3 part of the matrix is orthogonal then \ref GetInverseFast is much faster.
    [[nodiscard]] Matrix3x4 GetInverseFull() const;

    //! Inverts the transformation represented by the matrix.
    //! This function works for any matrix, even if they have scaling or skew.
    //! If the 3x3 part of the matrix is orthogonal then \ref InvertFast is much faster.
    void InvertFull();

    //! Gets the inverse of the transformation represented by the matrix, assuming the 3x3 part is orthogonal.
    [[nodiscard]] Matrix3x4 GetInverseFast() const;

    //! Inverts the transformation represented by the matrix, assuming the 3x3 part is orthogonal.
    void InvertFast();

    //! Gets the scale part of the transformation (the length of the basis vectors).
    [[nodiscard]] Vector3 RetrieveScale() const;

    //! Gets the squared scale part of the transformation (the squared length of the basis vectors).
    [[nodiscard]] Vector3 RetrieveScaleSq() const;

    //! Gets the scale part of the transformation as in RetrieveScale, and also removes this scaling from the matrix.
    Vector3 ExtractScale();

    //! Multiplies the basis vectors of the matrix by the elements of the scale specified.
    void MultiplyByScale(const Vector3& scale);

    //! Returns a matrix with the reciprocal scale, keeping the same rotation and translation.
    [[nodiscard]] Matrix3x4 GetReciprocalScaled() const;

    //! Tests if the 3x3 part of the matrix is orthogonal.
    bool IsOrthogonal(float tolerance = Constants::Tolerance) const;

    //! Returns an orthogonal matrix based on this matrix.
    [[nodiscard]] Matrix3x4 GetOrthogonalized() const;

    //! Modifies the basis vectors of the matrix to be orthogonal and unit length.
    void Orthogonalize();

    //! Tests element-wise whether this matrix is close to another matrix, within the specified tolerance.
    bool IsClose(const Matrix3x4& rhs, float tolerance = Constants::Tolerance) const;

    //! Tests whether this matrix is identical to another matrix.
    bool operator==(const Matrix3x4& rhs) const;

    //! Tests whether this matrix is not identical to another matrix.
    bool operator!=(const Matrix3x4& rhs) const;

    //! Converts the 3x3 part of the matrix to corresponding Euler angles (Z, then Y, then X), in degrees.
    //! @return Component-wise rotation angles in degrees.
    [[nodiscard]] Vector3 GetEulerDegrees() const;

    //! Converts the 3x3 part of the matrix to corresponding Euler angles (Z, then Y, then X), in radians.
    //! @return Component-wise rotation angles in radians.
    [[nodiscard]] Vector3 GetEulerRadians() const;

    //! Sets the 3x3 part of the matrix from Euler Angles (rotation angles in Z, then Y, then X), in degrees.
    //! The translation is set to zero.
    //! @param eulerDegrees Component-wise rotation angles in degrees.
    //! @return A matrix calculated from the composition of rotations around Z, then Y, then X, with zero translation.
    void SetFromEulerDegrees(const Vector3& eulerDegrees);

    //! Sets the 3x3 part of the matrix from Euler Angles (rotation angles in Z, then Y, then X), in radians.
    //! The translation is set to zero.
    //! @param Vector3 eulerRadians Component-wise rotation angles in radians.
    //! @return A matrix calculated from the composition of rotations around Z, then Y, then X, with zero translation.
    void SetFromEulerRadians(const Vector3& eulerRadians);

    //! Sets the 3x3 part of the matrix from a quaternion.
    void SetRotationPartFromQuaternion(const Quaternion& quaternion);

    //! Calculates the determinant of the 3x3 part of the matrix.
    [[nodiscard]] float GetDeterminant3x3() const;

    //! Checks whether the elements of the matrix are all finite.
    bool IsFinite() const;

    const Simd::Vec4::FloatType* GetSimdValues() const;

    Simd::Vec4::FloatType* GetSimdValues();
}