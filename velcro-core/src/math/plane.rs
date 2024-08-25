#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::fmt::Debug;
use std::ops::Add;
use crate::math::common_sse::{VecFourthType, VecType};
use crate::math::math_utils::constants;
use crate::math::simd_math_vec1_sse::Vec1;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::{FloatArgType, FloatType};

#[allow(dead_code)]
pub enum IntersectResult{
    Interior = 0,
    Overlaps = 1,
    Exterior = 2,
}


// PartialEq 是否相等
#[derive(Debug,Copy, Clone)]
pub struct Plane {
    _plane: Vector4
}
impl PartialEq<Self> for Plane {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return self._plane.is_close_default(other._plane.borrow()); }
    }
    fn ne(&self, other: &Self) -> bool {
        return !(self._plane == other._plane);
    }
}

impl Plane {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Plane {
            _plane: Vector4::new(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_float_type(plane:&FloatArgType)->Plane{
        unsafe {
            Plane {
                _plane: Vector4::new_float_type(plane),
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_normal_and_point(normal:&Vector3,point:&Vector3)->Plane{
        unsafe {
            Plane {
                _plane:  Vector4::new_float_type(Vec4::construct_plane(normal.get_simd_value().borrow(), point.get_simd_value().borrow()).borrow())
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_normal_and_distance(normal:&Vector3,dist:&f32)->Plane{
        let mut result = Plane::new();
        result.set_normal_and_distance(normal, dist);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_coefficients(a:&f32,b:&f32,c:&f32,d:&f32)->Plane{
        let mut result = Plane::new();
        result.set_coefficients(a,b,c,d);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_triangle(v0:&Vector3,v1:&Vector3,v2:&Vector3)->Plane{
        let mut result = Plane::new();
        let normal = unsafe { ((v1 - v0).cross((v2 - v0).borrow())).get_normalized() };
        let dist = -(unsafe { normal.dot3(v0) });
        result.set_normal_and_distance(normal.borrow(), dist.borrow());
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_vector_coefficients(coefficients:&Vector4)->Plane{
        let mut result = Plane::new();
        result.set_vec4(coefficients);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_vec4(&mut self,plane:&Vector4){
        self._plane = plane.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_normal_and_distance(&mut self,normal:&Vector3,d:&f32){
        unsafe { self._plane.set_vec3_f32(normal, d); }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_coefficients(&mut self,a:&f32,b:&f32,c:&f32,d:&f32){
        unsafe { self._plane.set_x_y_z_w(a, b, c, d) }
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_normal(&mut self,normal:&Vector3){
        self._plane.set_x(normal.get_x().borrow());
        self._plane.set_y(normal.get_y().borrow());
        self._plane.set_z(normal.get_z().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_distance(&mut self,d:&f32){
        self._plane.set_w(d);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_transform(self,tm:&Transform)->Plane{
        let mut new_dist = - self.get_distance();
        new_dist +=  self._plane.get_x() * (tm.get_basis_x().dot3(tm.get_translation().borrow()));
        new_dist += self._plane.get_y() * (tm.get_basis_y().dot3(tm.get_translation().borrow()));
        new_dist +=  self._plane.get_z() * (tm.get_basis_z().dot3(tm.get_translation().borrow()));
        let mut normal = self.get_normal();
        normal = tm.transform_vector(normal.borrow());
        return Plane::create_from_normal_and_distance(normal.borrow(), (-new_dist).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn apply_transform(&mut self,tm:&Transform){
        self._plane = self.get_transform(tm)._plane;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_plane_equation_coefficients(self)->Vector4{
        return self._plane;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_normal(self)->Vector3{
        return self._plane.get_as_vector3();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_distance(self)->f32{
        return self._plane.get_w();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_point_dist(self,pos:&Vector3)->f32{
        return  Vec1::select_index0(Vec4::plane_distance(self._plane.get_simd_value().borrow(),pos.get_simd_value().borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_projected(self,v:&Vector3)->Vector3{
        let n = self._plane.get_as_vector3();
        return v - (n * v.dot3(n.borrow()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn cast_ray_3vec3(&mut self, start:&Vector3, dir:&Vector3, mut hit_point:&Vector3) ->bool{
        let mut t :f32 = 0f32;
        if !self.cast_ray_2vec3_f32(start,dir,t.borrow_mut()){
            return  false
        }
        hit_point = (start + dir * t).borrow_mut();
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn cast_ray_2vec3_f32(self, start:&Vector3, dir:&Vector3, mut hit_time:&f32) ->bool{
        let n_dot_dir =self._plane.dot3(dir);
        if ( constants::is_close_f32(n_dot_dir.borrow(), 0.0.borrow(), constants::FLOAT_EPSILON.borrow()))
        {
            return false;
        }
        hit_time = (-(self._plane.get_w() + self._plane.dot3(start)) / n_dot_dir).borrow_mut();
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn intersect_segment_3vec3(self, start:&Vector3, end:&Vector3, mut hit_point:&Vector3) ->bool{
        let dir = end - start;
        let hit_time:f32 = 0f32;
        if (!self.cast_ray_2vec3_f32(start, dir.borrow(), hit_time.borrow()))
        {
            return false;
        }
        if (hit_time >= 0.0 && hit_time <= 1.0)
        {
            hit_point = (start + dir * hit_time).borrow_mut();
            return true;
        }
        return false;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn intersect_segment_2vec3_f32(self, start:&Vector3, end:&Vector3, mut hit_time:&f32) ->bool{
        let dir = end - start;
        if (!self.cast_ray_2vec3_f32(start, dir.borrow(), hit_time))
        {
            return false;
        }
        if (hit_time.to_owned() >= 0.0 && hit_time.to_owned() <= 1.0)
        {
            return true;
        }
        return false;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn is_finite(self)->bool{
        return self._plane.is_finite();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_simd_value(self)->FloatType{
        return self._plane.get_simd_value();
    }




}