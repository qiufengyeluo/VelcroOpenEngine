#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ops::Mul;

use crate::math::aabb::Aabb;
use crate::math::quaternion::Quaternion;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Obb {
    _position:Vector3,
    _rotation:Quaternion,
    _half_lengths:Vector3,
}

impl Obb {

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new()->Obb{
        Obb{
            _position:Vector3::new(),
            _rotation:Quaternion::new(),
            _half_lengths:Vector3::new()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_position(self) ->Vector3{
        return  self._position;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_rotation(self)->Quaternion{
        return self._rotation;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_rotation(&mut self,rotation:&Quaternion){
        self._rotation = rotation.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_half_lengths(self)->Vector3{
        return self._half_lengths;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_half_lengths(&mut self, half_lengths:&Vector3){
        self._half_lengths = half_lengths.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_axis_x(self)->Vector3{
        return self._rotation.transform_vector(Vector3::create_axis_x(1.0).borrow())
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_axis_y(self)->Vector3{
        return self._rotation.transform_vector(Vector3::create_axis_y(1.0).borrow())
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_axis_z(self)->Vector3{
        return self._rotation.transform_vector(Vector3::create_axis_z(1.0).borrow())
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_axis(self,index:i32) ->Vector3{
        let mut axis = Vector3::create_zero();
        axis.set_element(index,1.0);
        return self._rotation.transform_vector(axis.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_half_length_x(self)->f32{
        return self._half_lengths.get_x();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_half_length_y(self)->f32{
        return self._half_lengths.get_y();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_half_length_z(self)->f32{
        return self._half_lengths.get_z();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_half_length(self,index:i32) ->f32{
        return self._half_lengths.get_element(index);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_position(&mut self,position:&Vector3){
        self._position = position.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_half_length_x(&mut self, half_length:f32){
        self._half_lengths.set_x(half_length);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_half_length_y(&mut self, half_length:f32){
        self._half_lengths.set_y(half_length);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_half_length_z(&mut self, half_length:f32){
        self._half_lengths.set_z(half_length);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_half_length(&mut self, index:i32, half_length:f32){
        self._half_lengths.set_element(index, half_length)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains(self,point:&Vector3)->bool{
        let local = self._rotation.get_inverse_fast().transform_vector((point  - self._position).borrow());
        return local.is_greater_equal_than((-self._half_lengths).borrow()) && local.is_less_equal_than(self._half_lengths.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance(self,point:&Vector3)->f32{
        let local = self._rotation.get_inverse_fast().transform_vector((point  - self._position).borrow());
        let closest = local.get_clamp((-self._half_lengths).borrow(),self._half_lengths.borrow());
        return local.get_distance(closest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance_sq(self, point:&Vector3) ->f32{
        let local = self._rotation.get_inverse_fast().transform_vector((point  - self._position).borrow());
        let closest = local.get_clamp((-self._half_lengths).borrow(),self._half_lengths.borrow());
        return local.get_distance_sq(closest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_position_rotation_and_half_lengths(position:&Vector3, rotation:&Quaternion, half_lengths:&Vector3) ->Obb{
        Obb{
            _position:position.to_owned(),
            _rotation:rotation.to_owned(),
            _half_lengths: half_lengths.to_owned()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_aabb(aabb:&Aabb)->Obb{
        return Obb::create_from_position_rotation_and_half_lengths(aabb.get_center().borrow(), Quaternion::create_identity().borrow(), (aabb.get_extents() * 0.5).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_finite(self)->bool{
        return self._position.is_finite() && self._rotation.is_finite() && self._half_lengths.is_finite();
    }

}
impl PartialEq<Self> for Obb {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { return self._position.is_close_default(rhs._position.borrow())
            && self._rotation.is_close_default(rhs._rotation.borrow())
            && self._half_lengths.is_close_default(rhs._half_lengths.borrow()); }
    }
    fn ne(&self, rhs: &Self) -> bool {
        unsafe { return !(self.to_owned() == rhs.to_owned()); }
    }
}
impl Mul<&Transform> for Obb {
    type Output = Obb;

    fn mul(self, transform: &Transform) -> Self::Output {
        unsafe { return Obb::create_from_position_rotation_and_half_lengths(transform.transform_point_vec3(self.get_position().borrow()).borrow(),
                                                                            (transform.get_rotation() * self.get_rotation()).borrow(),
                                                                            (transform.get_uniform_scale()*self.get_half_lengths()).borrow()) }
    }
}
