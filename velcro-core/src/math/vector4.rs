#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector4 {
    _x: f32;
    _y: f32;
    _z: f32;
    _w: f32;
}

impl Vector4 {
    #[allow(dead_code)]
    pub fn new(x f32, y f32, z f32, w f32) -> Vector4 {
        Vector4 {
           x,
           y,
           z,
           w,
        }
    }

    pub fn new_zero()->Vector4{
        Vector4{
            x: 0.0,             
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    
}