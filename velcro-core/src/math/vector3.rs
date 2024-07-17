#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector3 {
    _x: f32;
    _y: f32;
    _z: f32;
}

impl Vector3 {
    #[allow(dead_code)]
    pub fn new(x f32, y f32, z f32) -> Vector3 {
        Vector3 {
           x,
           y,
           z,
           w,
        }
    }

    pub fn new_zero()->Vector3{
        Vector3{
            x: 0.0,             
            y: 0.0,
            z: 0.0,
        }
    }
    
    pub fn new_one()->Vector3{
        Vector3
    }
}