#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::fmt::Debug;
use std::num::{Add, Mul};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::math::common_sse::{Vec3Type, VecType};
use crate::math::constants::{G_VEC0010, G_VEC0100, G_VEC1000};
use crate::math::quaternion::Quaternion;
use crate::math::simd_math::simd_sin_cos;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::sphere::Sphere;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::vsimd::FloatArgType;

#[derive(Debug, Copy, Clone)]
pub struct Matrix3x3 {
    _rows:[Vector3;3]
}

impl Mul<&Vector3> for Matrix3x3{
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        unsafe { return Vector3::new_float_type(Vec3::mat3x3transform_vector(self.get_simd_values().borrow(), rhs.get_simd_values().borrow()).borrow()) }
    }
}

impl Add<&Matrix3x3> for Matrix3x3{
    type Output = Matrix3x3;

    fn add(self, rhs: &Matrix3x3) -> Self::Output {
        unsafe {
            return Matrix3x3::new_3float_type(Vec3::add(self._rows[0].get_simd_value().borrow(), rhs._rows[0].get_simd_value().borrow()).borrow(),
                                              Vec3::add(self._rows[1].get_simd_value().borrow(), rhs._rows[1].get_simd_value().borrow()).borrow(),
                                              Vec3::add(self._rows[2].get_simd_value().borrow(), rhs._rows[2].get_simd_value().borrow()).borrow());
        }

    }
}

impl AddAssign<&Matrix3x3> for Matrix3x3{
    fn add_assign(&mut self, rhs: &Matrix3x3) {
        self._rows = (self.borrow() + rhs)._rows;
    }
}

impl Sub<&Matrix3x3> for Matrix3x3{
    type Output = Matrix3x3;

    fn sub(self, rhs: &Matrix3x3) -> Self::Output {
        unsafe {
            return Matrix3x3::new_3float_type(Vec3::sub(self._rows[0].get_simd_value().borrow(), rhs._rows[0].get_simd_value().borrow()).borrow(),
                                              Vec3::sub(self._rows[1].get_simd_value().borrow(), rhs._rows[1].get_simd_value().borrow()).borrow(),
                                              Vec3::sub(self._rows[2].get_simd_value().borrow(), rhs._rows[2].get_simd_value().borrow()).borrow());
        }
    }
}

impl SubAssign<&Matrix3x3> for Matrix3x3{
    fn sub_assign(&mut self, rhs: &Matrix3x3) {
        self._rows = (self.borrow() - rhs)._rows;
    }
}

impl Mul<&Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: &Matrix3x3) -> Self::Output {
        let mut result =Matrix3x3::new();
        unsafe { Vec3::mat3x3multiply(self. get_simd_values(), rhs.get_simd_values(), result._rows.borrow_mut()); }
        result
    }
}

impl MulAssign<&Matrix3x3> for Matrix3x3{
    fn mul_assign(&mut self, rhs: &Matrix3x3) {
        self._rows = (self.borrow() * rhs)._rows;
    }
}
impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, multiplier: f32) -> Self::Output {
        let mul_vec = unsafe { Vec3::splat(multiplier.borrow()) };
        unsafe {
            return Matrix3x3::new_3float_type
                (
                    Vec3::mul(self._rows[0].get_simd_values().borrow(), mul_vec.borrow()),
                    Vec3::mul(self._rows[1].get_simd_values().borrow(), mul_vec.borrow()),
                    Vec3::mul(self._rows[2].get_simd_values().borrow(), mul_vec.borrow())
                );
        }
    }
}

impl MulAssign<f32> for Matrix3x3{
    fn mul_assign(&mut self, multiplier: f32) {
        self._rows = (self.borrow() * multiplier)._rows;
    }
}

impl Div<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    fn div(self, divisor: f32) -> Self::Output {
        let div_vec = unsafe { Vec3::splat(divisor.borrow()) };
        unsafe {
            return Matrix3x3::new_3float_type
                (
                    Vec3::div(self._rows[0].get_simd_values().borrow(), div_vec.borrow()),
                    Vec3::div(self._rows[1].get_simd_values().borrow(), div_vec.borrow()),
                    Vec3::div(self._rows[2].get_simd_values().borrow(), div_vec.borrow())
                );
        }
    }
}

impl DivAssign<f32> for Matrix3x3{
    fn div_assign(&mut self, divisor: f32) {
        self._rows = (self.borrow() / divisor)._rows;
    }
}
impl Sub<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

   unsafe  fn sub(self, rhs: Matrix3x3) -> Self::Output {
        let zero_vec = Vec3::zero_float();
        return Matrix3x3::new_3float_type
            (
                Vec3::sub(zero_vec.borrow(), self._rows[0].get_simd_value().borrow()).borrow(),
                Vec3::sub(zero_vec.borrow(), self._rows[1].get_simd_value().borrow()).borrow(),
                Vec3::sub(zero_vec.borrow(), self._rows[2].get_simd_value().borrow()).borrow()
            );
    }
}
impl PartialEq<Self> for Matrix3x3 {
   unsafe fn eq(&self, rhs: &Self) -> bool {
        return (Vec3::cmp_all_eq(self._rows[0].get_simd_value().borrow(), rhs._rows[0].get_simd_value().borrow())
            && Vec3::cmp_all_eq(self._rows[1].get_simd_value().borrow(), rhs._rows[1].get_simd_value().borrow())
            && Vec3::cmp_all_eq(self._rows[2].get_simd_value().borrow(), rhs._rows[2].get_simd_value().borrow()));
    }
    unsafe fn ne(&self, rhs: &Self) -> bool {
        unsafe { return !(self == rhs); }
    }
}

impl Matrix3x3 {

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Matrix3x3{
        Matrix3x3{
            _rows:[Vector3;3]
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_3float_type(row0:&FloatArgType,row1:&FloatArgType,row2:&FloatArgType)->Matrix3x3{
        Matrix3x3{
            _rows:[Vector3::new_float_type(row0),Vector3::new_float_type(row1),Vector3::new_float_type(row2)]
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_quaternion(quaternion:&Quaternion)->Matrix3x3{
        return Matrix3x3::create_from_quaternion(quaternion);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_identity()->Matrix3x3{
        Matrix3x3{
            _rows:[Vector3::new_float_type(Vec3::load_aligned(G_VEC1000.borrow()).borrow()),
                Vector3::new_float_type(Vec3::load_aligned(G_VEC0100.borrow()).borrow()),
                Vector3::new_float_type(Vec3::load_aligned(G_VEC0010.borrow()).borrow())]
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_zero()->Matrix3x3{
        let zero_vec = Vec3::zero_float();
        return Matrix3x3::new_3float_type(zero_vec.borrow(), zero_vec.borrow(), zero_vec.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_value(value:&f32)->Matrix3x3{
        let values = Vec3::splat(value);
        return Matrix3x3::new_3float_type(values.borrow(), values.borrow(), values.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_row_major_float9(values:*const f32)->Matrix3x3{
        return Matrix3x3::create_from_rows(Vector3::create_from_float_3(values[0]).borrow(),
                                           Vector3::create_from_float_3(values[3]).borrow(),
                                           Vector3::create_from_float_3(values[6]).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_rows(row0:&Vector3,row1:&Vector3,row2:&Vector3)->Matrix3x3{
        return Matrix3x3::new_3float_type(row0.get_simd_value().borrow(),row1.get_simd_value().borrow(),row2.get_simd_value().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_column_major_float9(values:*const f32)->Matrix3x3{
        return Matrix3x3::create_from_columns(Vector3::create_from_float_3(values[0]).borrow(),
                                           Vector3::create_from_float_3(values[3]).borrow(),
                                           Vector3::create_from_float_3(values[6]).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_columns(col0:&Vector3,col1:&Vector3,col2:&Vector3)->Matrix3x3{
        let mut result = Matrix3x3::new();
        result.set_columns(col0, col1, col2);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_x(angle:&f32)->Matrix3x3{
        let mut result =Matrix3x3::new();
        let mut s:f32 = 0f32;
        let mut c:f32 = 0f32;
        simd_sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result._rows[0] =Vector3::new_float_type(Vec3::load_aligned(G_VEC1000.borrow()).borrow());
        result.set_row(1.borrow(), 0.0.borrow(), c.borrow(), (-s).borrow());
        result.set_row(2.borrow(), 0.0.borrow(), s.borrow(), c.borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_y(angle:&f32) ->Matrix3x3{
        let mut result =Matrix3x3::new();
        let mut s:f32 = 0f32;
        let mut c:f32 = 0f32;
        simd_sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row(0.borrow(), c.borrow(), 0.0.borrow(), s.borrow());
        result._rows[1] =Vector3::new_float_type(Vec3::load_aligned(G_VEC0100.borrow()).borrow());
        result.set_row(2.borrow(),(-s).borrow(), 0.0.borrow(), c.borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_rotation_z(angle:&f32) ->Matrix3x3{
        let mut result =Matrix3x3::new();
        let mut s:f32 = 0f32;
        let mut c:f32 = 0f32;
        simd_sin_cos(angle, s.borrow_mut(), c.borrow_mut());
        result.set_row(0.borrow(), c.borrow(), (-s).borrow(), 0.0.borrow());
        result.set_row(1.borrow(),s.borrow(), c.borrow(), 0.0.borrow());
        result._rows[2] =Vector3::new_float_type(Vec3::load_aligned(G_VEC0010.borrow()).borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix3x4(m:&Matrix3x4)->Matrix3x3{
        let mut result = Matrix3x3::new() ;
        result.set_row_vec3(0.borrow(), m.GetRowAsVector3(0));
        result.set_row_vec3(1.borrow(), m.GetRowAsVector3(1));
        result.set_row_vec3(2.borrow(), m.GetRowAsVector3(2));
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix4x4(m:&Matrix4x4) ->Matrix3x3{
        let mut result = Matrix3x3::new() ;
        result._rows[0] = m.GetRow(0).GetAsVector3();
        result._rows[1] = m.GetRow(1).GetAsVector3();
        result._rows[2] = m.GetRow(2).GetAsVector3();
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_transform(t:&Transform)->Matrix3x3{
        return Matrix3x3::create_from_columns(t.get_basis_x().borrow(),t.get_basis_y().borrow(),t.get_basis_z().borrow())
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_quaternion(q:&Quaternion)->Matrix3x3{
        let mut result = Matrix3x3::new();
        result.set_rotation_part_from_quaternion(q);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_scale(scale:&Vector3)->Matrix3x3{
        return Matrix3x3::create_diagonal(scale);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_diagonal(diagonal:&Vector3)->Matrix3x3{
        let mut result =Matrix3x3::new();
        result.set_row(0.borrow(), diagonal.get_x().borrow(), 0.0.borrow(), 0.0.borrow());
        result.set_row(1.borrow(), 0.0.borrow(), diagonal.get_y().borrow(), 0.0.borrow());
        result.set_row(2.borrow(), 0.0.borrow(), 0.0.borrow(), diagonal.get_z().borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_cross_product(p:Vector3)->Matrix3x3{
        let mut result =Matrix3x3::new();
        result.set_row(0.borrow(), 0.0.borrow(), (-p.get_z()).borrow(), p.get_y().borrow());
        result.set_row(1.borrow(), p.get_z().borrow(), 0.0.borrow(),(-p.get_x()).borrow());
        result.set_row(2.borrow(), (-p.get_z()).borrow(), p.get_x().borrow(), 0.0.borrow());
        return result;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn store_to_row_major_float9(self,values:&*mut f32){
        self.get_row(0.borrow()).store_to_float_3(values);
        self.get_row(0.borrow()).store_to_float_3(*((*values as usize)+3) as &*mut f32);
        self.get_row(0.borrow()).store_to_float_3(*((*values as usize)+6) as &*mut f32);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn store_to_column_major_float9(self,mut values:*const f32){

    }
    // AZ_MATH_INLINE void Matrix3x3::StoreToColumnMajorFloat9(float* values) const
// {
// GetColumn(0).StoreToFloat4(values);
// GetColumn(1).StoreToFloat4(values + 3);
// GetColumn(2).StoreToFloat3(values + 6);
// }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_element(self,row:&i32,col:&i32)->f32{
        return self._rows[row].get_element(col);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_element(&mut self,row:&i32,col:&i32,value:&f32){
        self._rows[row].set_element(col,value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_row(self,row:&i32)->Vector3{
        return self._rows[row];
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_row(&mut self,row:&i32,x:&f32,y:&f32,z:&f32){
        self.set_row_vec3(row,Vector3::new_xyz(x,y,z).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_row_vec3(&mut self,row:&i32,v:&Vector3){
        self._rows[row] = v.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_rows(self,row0:*const Vector3,row1:*const Vector3,row2:*const Vector3){
        row0.set_simd_value(self.get_row(0.borrow()).get_simd_value());
        row1.set_simd_value(self.get_row(1.borrow()).get_simd_value());
        row2.set_simd_value(self.get_row(2.borrow()).get_simd_value());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_rows(&mut self,row0:&Vector3,row1:&Vector3,row2:&Vector3){
        self.set_row_vec3(0.borrow(),row0);
        self.set_row_vec3(1.borrow(),row1);
        self.set_row_vec3(2.borrow(),row2);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_column(self,col :&i32)->Vector3{
        return Vector3::new_xyz(self._rows[0].get_element(col).borrow(),self._rows[1].get_element(col).borrow(),self._rows[2].get_element(col).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_column(&mut self,col:&i32,x:&f32,y:&f32,z:&f32){
        self._rows[0].set_element(col,x);
        self._rows[1].set_element(col,y);
        self._rows[2].set_element(col,z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_column_vec3(&mut self,col:&i32,v:&Vector3){
        self._rows[0].set_element(col,v.get_x().borrow());
        self._rows[1].set_element(col,v.get_y().borrow());
        self._rows[2].set_element(col,v.get_z().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_columns(self,col0:&*mut  Vector3,col1:&*mut  Vector3,col2:&*mut Vector3){
        col0.set_simd_value(self.get_column(0.borrow()).get_simd_value());
        col1.set_simd_value(self.get_column(1.borrow()).get_simd_value());
        col2.set_simd_value(self.get_column(2.borrow()).get_simd_value());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_columns(&mut self,col0:&Vector3,col1:&Vector3,col2:&Vector3){
        self.set_row_vec3(0.borrow(),col0);
        self.set_row_vec3(1.borrow(),col1);
        self.set_row_vec3(2.borrow(),col2);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_basis_x(self)->Vector3{
        return self.get_column(0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_x(&mut self,x:&f32,y:&f32,z:&f32) {
        self.set_column(0.borrow(),x,y,z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_x_vec3(&mut self,v:&Vector3){
        self.set_column_vec3(0.borrow(),v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_basis_y(self)->Vector3{
        return self.get_column(1.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_y(&mut self,x:&f32,y:&f32,z:&f32) {
        self.set_column(1.borrow(),x,y,z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_y_vec3(&mut self,v:&Vector3){
        self.set_column_vec3(1.borrow(),v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_basis_z(self)->Vector3{
        return self.get_column(2.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_z(&mut self,x:&f32,y:&f32,z:&f32) {
        self.set_column(2.borrow(),x,y,z);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis_z_vec3(&mut self,v:&Vector3){
        self.set_column_vec3(2.borrow(),v);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_basis(self, basis_x:&*mut Vector3, basis_y:&*mut Vector3, basis_z:&*mut Vector3){
        self.get_columns(basis_x, basis_y, basis_z)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_basis(&mut self,basis_x:&Vector3, basis_y:&Vector3, basis_z:&Vector3){
        self.set_columns(basis_x, basis_y, basis_z)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn transposed_multiply(self,rhs:&Matrix3x3)->Matrix3x3{
        let mut result = Matrix3x3::new();
        Vec3::mat3x3transpose_multiply(self.get_simd_values().borrow(),rhs.get_simd_values().borrow(),result._rows.borrow_mut());
        result
    }



    //! Transpose calculation, flips the rows and columns.
    //! @{
    Matrix3x3 GetTranspose() const;
    void Transpose();
    //! @}

    //! Gets the inverse of the matrix.
    //! Use GetInverseFast instead of this if the matrix is orthogonal.
    //! @{
    Matrix3x3 GetInverseFull() const;
    void InvertFull();
    //! @}

    //! Fast inversion assumes the matrix is orthogonal.
    //! @{
    Matrix3x3 GetInverseFast() const;
    void InvertFast();
    //! @}

    //! Gets the scale part of the transformation, i.e. the length of the scale components.
    [[nodiscard]] Vector3 RetrieveScale() const;

    //! Gets the squared scale part of the transformation (the squared length of the basis vectors).
    [[nodiscard]] Vector3 RetrieveScaleSq() const;

    //! Gets the scale part of the transformation as in RetrieveScale, and also removes this scaling from the matrix.
    Vector3 ExtractScale();

    //! Quick multiplication by a scale matrix, equivalent to m*=Matrix3x3::CreateScale(scale).
    void MultiplyByScale(const Vector3& scale);

    //! Returns a matrix with the reciprocal scale, keeping the same rotation and translation.
    [[nodiscard]] Matrix3x3 GetReciprocalScaled() const;

    //! Polar decomposition, M=U*H, U is orthogonal (unitary) and H is symmetric (hermitian).
    //! This function returns the orthogonal part only
    Matrix3x3 GetPolarDecomposition() const;

    //! Polar decomposition, M=U*H, U is orthogonal (unitary) and H is symmetric (hermitian).
    void GetPolarDecomposition(Matrix3x3* orthogonalOut, Matrix3x3* symmetricOut) const;

    bool IsOrthogonal(float tolerance = Constants::Tolerance) const;

    //! Adjusts an almost orthogonal matrix to be orthogonal.
    Matrix3x3 GetOrthogonalized() const;

    //! Adjusts an almost orthogonal matrix to be orthogonal.
    void Orthogonalize();

    bool IsClose(const Matrix3x3& rhs, float tolerance = Constants::Tolerance) const;

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_rotation_part_from_quaternion(&mut self, q:&Quaternion){
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

        self.SetElement(0, 0, 1.0 - (tyy + tzz)); // 1.0-2yy-2zz    2xy-2wz    2xz+2wy
        SetElement(0, 1, txy - twz);
        SetElement(0, 2, txz + twy);

        SetElement(1, 0, txy + twz); // 2xy+2wz    1.0-2xx-2zz    2yz-2wx
        SetElement(1, 1, 1.0 - (txx + tzz));
        SetElement(1, 2, tyz - twx);

        SetElement(2, 0, txz - twy); // 2xz-2wy    2yz+2wx    1.0-2xx-2yy
        SetElement(2, 1, tyz + twx);
        SetElement(2, 2, 1.0f - (txx + tyy));
    }
    void Matrix3x3::SetRotationPartFromQuaternion(const Quaternion& q)
    {
    float tx = q.GetX() * 2.0f;
    float ty = q.GetY() * 2.0f;
    float tz = q.GetZ() * 2.0f;
    float twx = q.GetW() * tx;
    float twy = q.GetW() * ty;
    float twz = q.GetW() * tz;
    float txx = q.GetX() * tx;
    float txy = q.GetX() * ty;
    float txz = q.GetX() * tz;
    float tyy = q.GetY() * ty;
    float tyz = q.GetY() * tz;
    float tzz = q.GetZ() * tz;

    SetElement(0, 0, 1.0f - (tyy + tzz)); // 1.0-2yy-2zz    2xy-2wz    2xz+2wy
    SetElement(0, 1, txy - twz);
    SetElement(0, 2, txz + twy);

    SetElement(1, 0, txy + twz); // 2xy+2wz    1.0-2xx-2zz    2yz-2wx
    SetElement(1, 1, 1.0f - (txx + tzz));
    SetElement(1, 2, tyz - twx);

    SetElement(2, 0, txz - twy); // 2xz-2wy    2yz+2wx    1.0-2xx-2yy
    SetElement(2, 1, tyz + twx);
    SetElement(2, 2, 1.0f - (txx + tyy));
    }

    Vector3 GetDiagonal() const;

    float GetDeterminant() const;

    //! This is the transpose of the matrix of cofactors.
    //! Also known as the adjoint, adjugate is the modern name which avoids confusion with the adjoint conjugate transpose.
    Matrix3x3 GetAdjugate() const;

    bool IsFinite() const;

    const Simd::Vec3::FloatType* GetSimdValues() const;

    Simd::Vec3::FloatType* GetSimdValues();
}