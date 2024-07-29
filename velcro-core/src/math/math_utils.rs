#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use core::f64;
use crate::math::constants::PI;

pub fn get_clamp<T>(value :T, min:T, max:T ) ->T
{
    return if value < min
    {
        min
    } else if value > max
    {
        max
    } else {
        value
    }
}

pub fn rad_to_deg(rad:&f32)->f32{
    return rad*180.0/PI
}

pub fn deg_to_rad(deg:&f32)->f32{
    return deg*PI /180.0;
}

fn is_normalized(x: &f64) -> bool {
    (x.abs() - 1.0).abs() < f64::EPSILON
}

pub fn is_normal_double(x:&f64)->bool{
    return is_normalized(x);
}
pub fn is_finite_float(x:&f32)->bool{
    return x.is_finite()
}