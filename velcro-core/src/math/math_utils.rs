#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use num_traits::Float;

pub mod constants {
    use crate::math::simd_math::simd;

    pub const FLT_MAX:f32 = 3.402823466e+38;
    pub const FLT_EPSILON:f32 =  1.192092896e-07;
    pub(crate) const PI:f32 = 3.14159265358979323846;
    pub(crate) const TWO_PI:f32 = 6.28318530717958647692;
    pub(crate) const HALF_PI:f32 = 1.57079632679489661923;
    pub(crate) const QUARTER_PI:f32 = 0.78539816339744830962;
    pub(crate) const TWO_OVER_PI:f32 = 0.63661977236758134308;
    pub const MAX_FLOAT_BEFORE_PRECISION_LOSS:f32 = 100000.0;
    pub(crate) const TOLERANCE:f32 = 0.001;
    pub const FLOAT_MAX:f32 = FLT_MAX;
    pub(crate) const FLOAT_EPSILON:f32 = FLT_EPSILON;
    pub(crate) enum Axis
    {
        XPositive,
        XNegative,
        YPositive,
        YNegative,
        ZPositive,
        ZNegative
    }
    pub fn is_power_of_two(test_value:&u32 ) ->bool
    {
        return (test_value & (test_value - 1)) == 0;
    }

    pub fn log2(mut max_value: u64 ) ->u32
    {
        let mut  bits:u32 = 0;
        loop {
            max_value >>= 1;
            bits += 1;
            if(max_value <= 0){
                break
            }
        }
        return bits;
    }

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
        return (a - b).abs() <= tolerance.to_owned()
    }

    pub fn is_close_f32_default(a:&f32,b:&f32)->bool{
        return (a - b).abs() <= TOLERANCE
    }

    pub fn is_close_f64(a:&f64,b:&f64,tolerance:&f64)->bool{
        return (a - b).abs() <= tolerance.to_owned()
    }

    pub fn is_close_f64_default(a:&f64,b:&f64)->bool{
        return (a - b).abs() <= (TOLERANCE as f64).to_owned()
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
        unsafe { return simd::mod_calculate(a, b) }
    }

    pub fn get_mod_f64(a :&f64,b:&f64)->f64{
        return a % b;
    }
}










