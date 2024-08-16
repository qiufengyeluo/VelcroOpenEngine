#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    _min: Vector3,
    _max: Vector3
}