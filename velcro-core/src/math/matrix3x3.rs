#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Matrix3x3 {
    _rows:[Vector3;3]
}