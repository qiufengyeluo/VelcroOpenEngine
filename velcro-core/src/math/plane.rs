#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

#[allow(dead_code)]
pub enum IntersectResult{
    Interior = 0,
    Overlaps = 1,
    Exterior = 2,
}


// PartialEq 是否相等
#[derive(Debug,Copy, Clone)]
pub struct Plane {
    _plane: Vector4
}


impl Plane {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Plane {
            _plane: Vector4::new(),
        }
    }

    
}