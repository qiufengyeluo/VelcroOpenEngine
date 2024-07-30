#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::constants::FLOAT_MAX;
use crate::math::vector3::Vector3;

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Aabb {
    _min: Vector3,
    _max: Vector3
}

impl Aabb {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Aabb {
            _min: Vector3::new(),
            _max: Vector3::new(),
        }
    }

    pub fn create_null()->Self{
        Aabb{
            _min: Vector3::new_splat(FLOAT_MAX),
            _max: Vector3::new_splat(-FLOAT_MAX),
        }
    }

    pub fn create_from_point(p: &Vector3)->Aabb{
        let mut aabb :Aabb = Aabb::new();
        aabb._max = *p;
        aabb._min = *p;
        aabb
    }

    pub fn create_from_min_max(min:&Vector3,max :&Vector3)->Aabb
    {
        let mut aabb :Aabb = Aabb::new();
        aabb.m_min = min;
        aabb.m_max = max;
        assert!(aabb.is_v(), "Min must be less than Max");
        return aabb;
    }


    pub fn is_close(rhs :&Aabb, tolerance:&f32 ) ->bool
    {
        return Self._min.is_close(rhs._min.borrow(), tolerance) && Self._max.is_close(rhs._max.borrow(), tolerance);
    }

    pub fn is_valid(self) ->bool
    {
        return self.m_min.is_less(self.m_max);
    }

    pub fn is_finite(self) ->bool
    {
        return self.m_min.IsFinite() && self.m_max.IsFinite();
    }

}