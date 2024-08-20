#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ops::{Mul, MulAssign};

use crate::math::math_utils::{is_close_f32, is_finite_float};
use crate::math::quaternion::Quaternion;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    _rotation:Quaternion,
    _scale:f32,
    _translation:Vector3
}

impl PartialEq<Self> for Transform {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return self._rotation == other._rotation && self._scale == other._scale && self._translation == other._translation; }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !(self == other); }
    }
}

impl Mul<Transform> for Transform{
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        unsafe {
            Transform {
                _rotation: self._rotation * rhs._rotation,
                _scale: self._scale * rhs._scale,
                _translation: self.transform_point_vec3(rhs._translation.borrow()),
            }
        }
    }
}
impl MulAssign<Transform> for Transform{
    fn mul_assign(&mut self, rhs: Transform) {
        *self = *self * rhs
    }
}

impl Transform {

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new()->Transform{
        Transform{
            _rotation:Quaternion::new(),
            _scale:0f32,
            _translation:Vector3::new()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_all(translation:&Vector3,rotation:&Quaternion,scale:&f32)->Transform{
        Transform{
            _rotation:rotation.to_owned(),
            _scale:scale.to_owned(),
            _translation:translation.to_owned()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_identity() ->Transform{
        Transform{
            _rotation:Quaternion::create_identity(),
            _scale:1.0f32,
            _translation:Vector3::create_zero(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_x(angle:&f32)->Transform{
        return Transform::create_from_quaternion(Quaternion::create_rotation_x(angle).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_y(angle:&f32)->Transform{
        return Transform::create_from_quaternion(Quaternion::create_rotation_y(angle).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_rotation_z(angle:&f32)->Transform{
        return Transform::create_from_quaternion(Quaternion::create_rotation_z(angle).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_quaternion(q:&Quaternion)->Transform{
        Transform{
            _rotation:q.to_owned(),
            _scale:1.0,
            _translation:Vector3::create_zero()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_quaternion_and_translation(q:&Quaternion,p:&Vector3) ->Transform{
        Transform{
            _rotation:q.to_owned(),
            _scale:1.0,
            _translation:p.to_owned()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_uniform_scale(scale:&f32) ->Transform{
        Transform{
            _rotation:Quaternion::create_identity(),
            _scale:scale.to_owned(),
            _translation:Vector3::create_zero()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_translation(translation:&Vector3) ->Transform{
        Transform{
            _rotation:Quaternion::create_identity(),
            _scale:1.0,
            _translation:translation.to_owned()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis(self, index:&i32) ->Vector3{
        match index {
            0 =>{
                return self.get_basis_x();
            }
            1 =>{
                return self.get_basis_y();
            }
            2 =>{
                return self.get_basis_z();
            }
            &_ => {
                return Vector3::create_zero();
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_x(self)->Vector3{
        return self._rotation.transform_vector(Vector3::create_axis_x(self._scale.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_y(self)->Vector3{
        return self._rotation.transform_vector(Vector3::create_axis_y(self._scale.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_z(self)->Vector3{
        return self._rotation.transform_vector(Vector3::create_axis_z(self._scale.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_basis_and_translation(self,mut basisX:*const Vector3,mut basisY:*const Vector3,mut basisZ:*const Vector3,mut pos:*const Vector3){
        basisX = self.get_basis_x().borrow_mut();
        basisY = self.get_basis_y().borrow_mut();
        basisZ = self.get_basis_z().borrow_mut();
        pos = self.get_translation().borrow_mut();

    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_translation(self) ->Vector3{
        return  self._translation
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_translation(&mut self, x:&f32,y:&f32,z:&f32){
        self.set_translation_vec3(Vector3::new_xyz(x,y,z).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_translation_vec3(&mut self,v:&Vector3){
        self._translation =v.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_rotation(self)->Quaternion{
        return self._rotation;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_rotation(&mut self,rotation:&Quaternion){
        self._rotation =rotation.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_uniform_scale(self)->f32{
        return self._scale;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_uniform_scale(&mut self,scale:&f32){
        self._scale =scale.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn extract_uniform_scale(&mut self)->f32{
        let scale = self._scale;
        self._scale = 1.0;
        scale
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn multiply_by_uniform_scale(&mut self,scale:&f32){
        self._scale *= scale;
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transform_point_vec3(self,rhs:&Vector3)->Vector3{
        return self._rotation.transform_vector((self._scale * rhs).borrow()) + self._translation
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transform_point_vec4(self,rhs:&Vector4)->Vector4{
        return Vector4::create_from_vector3_and_float((self._rotation.transform_vector((self._scale * rhs.get_as_vector3()).borrow()) + self._translation * rhs.get_element(3)).borrow(),rhs.get_element(3))
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn transform_vector(self,rhs:&Vector3)->Vector3{
        return self._rotation.transform_vector((self._scale * rhs).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_inverse(self)->Transform{
        let mut result = Transform::new();
        result._rotation = self._rotation.get_conjugate();
        result._scale = 1.0 * self._scale;
        result._translation = (result._rotation.transform_vector(self._translation.borrow())) * -result._scale;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn invert(&mut self){
        *self = self.get_inverse()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_orthogonal(self,tolerance:&f32)->bool{
        return is_close_f32(self._scale.borrow(),1.0.borrow(),tolerance)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_orthogonalized(self)->Transform{
        Transform{
            _rotation:self._rotation,
            _scale:1.0,
            _translation :self._translation
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn orthogonalize(&mut self){
        self._scale = 1.0;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(self,rhs:&Transform,tolerance:&f32)->bool{
        return self._rotation.is_close(rhs._rotation.borrow(),tolerance)
            && is_close_f32(self._scale.borrow(),rhs._scale.borrow(),tolerance)
            && self._translation.is_close(rhs._translation.borrow(),tolerance)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_euler_degrees(self)->Vector3{
        return self._rotation.get_euler_degrees();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_euler_radians(self) ->Vector3{
        return self._rotation.get_euler_radians();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_from_euler_degrees(&mut self, euler_degrees:&Vector3){
        self._translation = Vector3::create_zero();
        self._scale = 1.0;
        self._rotation.set_from_euler_degrees(euler_degrees);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_from_euler_radians(&mut self, euler_radians:&Vector3){
        self._translation = Vector3::create_zero();
        self._scale = 1.0;
        self._rotation.set_from_euler_radians(euler_radians);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_finite(self)->bool{
        return self._rotation.is_finite() && is_finite_float(self._scale.borrow()) && self._translation.is_finite()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn convert_transform_to_euler_degrees(transform:&Transform)->Vector3{
        return transform.get_euler_degrees();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn convert_transform_to_euler_radians(transform:&Transform) ->Vector3{
        return transform.get_euler_radians();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn convert_euler_degrees_to_transform(euler_degrees:&Vector3) ->Transform{
        let mut final_rotation:Transform = Transform::new();
        final_rotation.set_from_euler_degrees(euler_degrees);
        final_rotation
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn convert_euler_radians_to_transform(euler_radians:&Vector3) ->Transform{
        let mut final_rotation:Transform = Transform::new();
        final_rotation.set_from_euler_radians(euler_radians);
        final_rotation
    }

}