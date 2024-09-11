#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::intersect::intersect_point::Intersect;
use crate::math::line_segment::LineSegment;
use crate::math::math_utils::constants;
use crate::math::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Capsule {
    _first_hemisphere_center:Vector3 ,
    _second_hemisphere_center:Vector3,
    _radius:f32
}

impl Capsule{

    #[inline]
    #[allow(dead_code)]
    pub fn new_vec3_vec3_f32(first_hemisphere_center:&Vector3, second_hemisphere_center:&Vector3, radius:&f32) ->Capsule{
        Capsule{
            _radius:radius.to_owned(),
            _first_hemisphere_center: first_hemisphere_center.to_owned(),
            _second_hemisphere_center: second_hemisphere_center.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_line_f32(line_segment:&LineSegment, radius:&f32) ->Capsule{
        Capsule{
            _radius:radius.to_owned(),
            _first_hemisphere_center: line_segment.get_start(),
            _second_hemisphere_center: line_segment.get_end(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_first_hemisphere_center(self)-> Vector3{
        return self._first_hemisphere_center;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_second_hemisphere_center(self) ->Vector3{
        return self._second_hemisphere_center
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_center(self)->Vector3{
        return  (self._first_hemisphere_center + self._second_hemisphere_center) * 0.5
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_radius(self)->f32{
        return  self._radius
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_cylinder_height(self) ->f32{
        return self._first_hemisphere_center.get_distance(self._second_hemisphere_center.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_total_height(self)->f32{
        return self.get_cylinder_height() + 2.0 * self._radius;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_first_hemisphere_center(&mut self, first_hemisphere_center:&Vector3){
        self._first_hemisphere_center = first_hemisphere_center.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_second_hemisphere_center(&mut self, second_hemisphere_center:&Vector3){
        self._second_hemisphere_center = second_hemisphere_center.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_radius(&mut self,radius:f32){
        self._radius = radius.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(self,rhs:&Capsule,tolerance:f32)->bool{
        return constants::is_close_f32(self._radius,rhs._radius,tolerance)
        && ( (self._first_hemisphere_center.is_close(rhs._first_hemisphere_center.borrow(),tolerance) && self._second_hemisphere_center.is_close(rhs._second_hemisphere_center.borrow(),tolerance))
            || (self._first_hemisphere_center.is_close(rhs._second_hemisphere_center.borrow(),tolerance)&& self._second_hemisphere_center.is_close(rhs._first_hemisphere_center.borrow(),tolerance)) )
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains(self,point:&Vector3)->bool{
        return  Intersect::point_segment_distance_sq(point,self._first_hemisphere_center.borrow(),self._second_hemisphere_center.borrow()) <= self._radius * self._radius;
    }
}