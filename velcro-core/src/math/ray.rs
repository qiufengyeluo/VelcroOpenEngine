#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::line_segment::LineSegment;
use crate::math::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray{
    _origin:Vector3,
    _direction:Vector3
}
impl Ray{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Ray{
        Ray{
            _origin:Vector3::new(),
            _direction:Vector3::new()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_2_vec3(origin:&Vector3,direction:&Vector3)->Ray{
        Ray{
            _origin:origin.to_owned(),
            _direction:direction.to_owned()
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_line_segment(segment : &LineSegment)->Ray{
        unsafe { return Ray::new_2_vec3(segment.get_start().borrow(), segment.get_difference().get_normalized().borrow()) }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_origin(self) ->Vector3{
        return self._origin;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_direction(self) ->Vector3{
        return self._direction;
    }

}