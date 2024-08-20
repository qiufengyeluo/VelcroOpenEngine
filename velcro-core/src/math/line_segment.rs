#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::ray::Ray;
use crate::math::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct LineSegment{
    _start:Vector3,
    _end:Vector3,
}
impl LineSegment{

    #[inline]
    #[allow(dead_code)]
    pub fn new(start:&Vector3,end:&Vector3)->LineSegment{
        LineSegment{
            _start:start.to_owned(),
            _end:end.to_owned()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_ray_and_length(ray:&Ray,length:&f32)->LineSegment{
        return LineSegment::new(ray.get_origin().borrow(),(ray.get_origin() + (ray.get_direction() * length.to_owned())).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_start(self)->Vector3{
        return self._start;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_end(self) ->Vector3{
        return self._end;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_difference(self) ->Vector3{
        return self._end - self._start;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_point(self, t:&f32) ->Vector3{
        unsafe { return self._start.lerp(self._end.borrow(), t); }
    }

}