#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::aabb::Aabb;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    _center:Vector3 ,
    _radius:f32
}

impl PartialEq<Self> for Sphere {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { return  (self._center == rhs._center) && (self._radius == rhs._radius); }
    }
    fn ne(&self, rhs: &Self) -> bool {
        unsafe { return !(self == rhs); }
    }
}

impl Sphere{
    #[inline]
    #[allow(dead_code)]
    pub fn new()->Sphere{
        Sphere{
            _radius:0f32,
            _center:Vector3::new(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_vec3_f32(center:&Vector3,radius:&f32)->Sphere{
        Sphere{
            _radius:radius.to_owned(),
            _center:center.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_unit_sphere()->Sphere{
        unsafe { return Sphere::new_vec3_f32(Vector3::create_zero().borrow(), 1.0.borrow()); }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_aabb(aabb:&Aabb)->Sphere{
        let half_extent = (aabb.get_max() - aabb.get_min()) *  0.5;
        let center = aabb.get_min() + half_extent;
        let radius = half_extent.get_max_element();
        return  Sphere::new_vec3_f32(center.borrow(), radius.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_center(self)->Vector3{
        return self._center;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_radius(self)->f32{
        return self._radius;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_center(&mut self,center:&Vector3){
        self._center = center.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_radius(&mut self,radius:&f32){
        self._radius = radius.to_owned();
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set(&mut self,sphere:&Sphere){
        self._radius = sphere._radius.to_owned();
        self._center = sphere._center.to_owned();
    }

}