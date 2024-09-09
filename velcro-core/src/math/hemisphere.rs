#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::sphere::Sphere;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;

#[derive(Debug,Copy, Clone)]
pub struct Hemisphere{
     _center_radius:Vector4,
     _direction:Vector3
}

impl PartialEq<Self> for Hemisphere {
    fn eq(&self, other: &Self) -> bool {
        unsafe {  return (self._center_radius == other._center_radius) && (self._direction == other._direction); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !(self == other); }
    }
}

impl Hemisphere{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Hemisphere{
        Hemisphere{
            _direction:Vector3::new(),
            _center_radius:Vector4::new()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec3_f32_vec3(center:&Vector3, radius:f32, normalized_direction:&Vector3) ->Hemisphere{
        Hemisphere{
            _direction: normalized_direction.to_owned(),
            _center_radius:Vector4::new_vec3_w(center, radius)
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_sphere_and_direction(sphere:&Sphere, normalized_direction:&Vector3) ->Hemisphere{
        return Hemisphere::new_vec3_f32_vec3(sphere.get_center().borrow(), sphere.get_radius(), normalized_direction);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_center(self) ->Vector3{
        return Vector3::new_vec4(self._center_radius.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_radius(self)->f32{
        return self._center_radius.get_w();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_direction(self)->Vector3{
        return self._direction;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_center(&mut self,center:&Vector3){
        self._center_radius.set_vec3_f32(center, self.get_radius());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_radius(&mut self,radius:f32){
        self._center_radius.set_w(radius);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_direction(&mut self,direction:&Vector3){
        self._direction = direction.to_owned();
    }
}