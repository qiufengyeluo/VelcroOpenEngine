#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use core::f64;
use num_traits::Float;
use crate::math::constants::{PI, TOLERANCE};
use crate::math::simd_math::simd_mod_calculate;

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
pub fn max<T>(left:&T,right:&T)->T{
    return if left > right{
        left
    }else {
        right
    }
}

pub fn min<T>(left:&T,right:&T)->T{
    return  if left < right{
        left
    }else {
        right
    }
}
pub fn is_close_f32(a:&f32,b:&f32,tolerance:&f32)->bool{
    return abs(a - b) <= tolerance
}

pub fn is_close_f32_default(a:&f32,b:&f32)->bool{
    return abs(a - b) <= TOLERANCE
}

pub fn is_close_f64(a:&f64,b:&f64,tolerance:&f64)->bool{
    return abs(a - b) <= tolerance
}

pub fn is_close_f64_default(a:&f64,b:&f64)->bool{
    return abs(a - b) <= TOLERANCE
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

pub fn get_abs_f32(a :&f32)->f32{
    return a.abs()
}
pub fn get_abs_f64(a :&f64)->f64{
    return a.abs()
}

pub fn get_mod_f32(a :&f32,b:&f32)->f32{
    unsafe { return simd_mod_calculate(a, b) }
}
pub fn get_mod_f64(a :&f64,b:&f64)->f64{
    return a % b;
}
