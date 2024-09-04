
#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]



pub mod simd{
    use crate::math::common_sse::{Vec2Type, VecTwoType};
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    use crate::math::common_sse::VecType;
    use crate::math::math_utils::constants;
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    use crate::math::simd_math_vec1_sse::Vec1;
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    use crate::math::simd_math_vec2_sse::Vec2;

    pub const G_VEC1111:[f32;4]         = [1.0, 1.0, 1.0, 1.0];
    pub const G_VEC1000:[f32;4]  = [1.0, 0.0, 0.0, 0.0 ];
    pub const G_VEC0100:[f32;4]  = [0.0, 1.0, 0.0, 0.0 ];
    pub const G_VEC0010:[f32;4]  = [0.0, 0.0, 1.0, 0.0 ];
    pub const G_VEC0001:[f32;4]  = [0.0, 0.0, 0.0, 1.0 ];
    pub const G_PI:[f32;4]       = [constants::PI, constants::PI, constants::PI, constants::PI ];
    pub const G_TWO_PI:[f32;4]    = [constants::TWO_PI, constants::TWO_PI, constants::TWO_PI, constants::TWO_PI ];
    pub const G_HALF_PI:[f32;4]   = [constants::HALF_PI, constants::HALF_PI, constants::HALF_PI, constants::HALF_PI ];
    pub const G_QUARTER_PI:[f32;4]      = [constants::QUARTER_PI, constants::QUARTER_PI, constants::QUARTER_PI, constants::QUARTER_PI ];
    pub const G_TWO_OVER_PI:[f32;4]       = [constants::TWO_OVER_PI, constants::TWO_OVER_PI, constants::TWO_OVER_PI, constants::TWO_OVER_PI ];
    pub const G_ABS_MASK:[i32;4]       = [0x7fffffff, 0x7fffffff,0x7fffffff, 0x7fffffff ];
    pub const G_NEGATE_MASK:[i32;4]    = [0x80000000, 0x80000000, 0x80000000, 0x80000000 ];
    pub const G_NEGATE_XMASK:[i32;4]   = [0x80000000, 0x00000000, 0x00000000, 0x00000000 ];
    pub const G_NEGATE_YMASK:[i32;4]   = [0x00000000, 0x80000000, 0x00000000, 0x00000000 ];
    pub const G_NEGATE_ZMASK:[i32;4]   = [0x00000000, 0x00000000, 0x80000000, 0x00000000 ];
    pub const G_NEGATE_WMASK:[i32;4]   = [0x00000000, 0x00000000, 0x00000000, 0x80000000 ];
    pub const G_NEGATE_XYZMASK:[i32;4] = [0x80000000, 0x80000000, 0x80000000, 0x00000000 ];
    pub const G_W_MASK:[i32;4]         = [0xffffffff, 0xffffffff, 0xffffffff, 0x00000000 ];

    pub const G_SIN_COEF1:[f32;4]    = [ -0.0001950727, -0.0001950727, -0.0001950727, -0.0001950727 ];
    pub const G_SIN_COEF2:[f32;4]    = [  0.0083320758,  0.0083320758,  0.0083320758,  0.0083320758 ];
    pub const G_SIN_COEF3:[f32;4]    = [ -0.1666665247, -0.1666665247, -0.1666665247, -0.1666665247 ];
    pub const G_COS_COEF1:[f32;4]    = [ -0.0013602249, -0.0013602249, -0.0013602249, -0.0013602249 ];
    pub const G_COS_COEF2:[f32;4]    = [  0.0416566950,  0.0416566950,  0.0416566950,  0.0416566950 ];
    pub const G_COS_COEF3:[f32;4]    = [ -0.4999990225, -0.4999990225, -0.4999990225, -0.4999990225 ];
    pub const G_ACOS_HI_COEF1:[f32;4] = [ -0.0012624911, -0.0012624911, -0.0012624911, -0.0012624911 ];
    pub const G_ACOS_HI_COEF2:[f32;4] = [  0.0066700901,  0.0066700901,  0.0066700901,  0.0066700901 ];
    pub const G_ACOS_HI_COEF3:[f32;4] = [ -0.0170881256, -0.0170881256, -0.0170881256, -0.0170881256 ];
    pub const G_ACOS_HI_COEF4:[f32;4] = [  0.0308918810,  0.0308918810,  0.0308918810,  0.0308918810 ];
    pub const G_ACOS_LO_COEF1:[f32;4] = [ -0.0501743046, -0.0501743046, -0.0501743046, -0.0501743046 ];
    pub const G_ACOS_LO_COEF2:[f32;4] = [  0.0889789874,  0.0889789874,  0.0889789874,  0.0889789874 ];
    pub const G_ACOS_LO_COEF3:[f32;4] = [ -0.2145988016, -0.2145988016, -0.2145988016, -0.2145988016 ];
    pub const G_ACOS_LO_COEF4:[f32;4] = [  1.5707963050,  1.5707963050,  1.5707963050,  1.5707963050 ];
    pub const G_ACOS_COEF1:[f32;4]   = [ -0.0200752200, -0.0200752200, -0.0200752200, -0.0200752200 ];
    pub const G_ACOS_COEF2:[f32;4]   = [  0.0759031500,  0.0759031500,  0.0759031500,  0.0759031500 ];
    pub const G_ACOS_COEF3:[f32;4]   = [ -0.2126757000, -0.2126757000, -0.2126757000, -0.2126757000 ];
    pub const G_ATAN_HI_RANGE:[f32;4] = [  2.4142135624,  2.4142135624,  2.4142135624,  2.4142135624 ];
    pub const G_ATAN_LO_RANGE:[f32;4] = [  0.4142135624,  0.4142135624,  0.4142135624,  0.4142135624 ];
    pub const G_ATAN_COEF1:[f32;4]   = [  8.05374449538e-2,  8.05374449538e-2,  8.05374449538e-2,  8.05374449538e-2 ];
    pub const G_ATAN_COEF2:[f32;4]   = [ -1.38776856032e-1, -1.38776856032e-1, -1.38776856032e-1, -1.38776856032e-1 ];
    pub const G_ATAN_COEF3:[f32;4]   = [  1.99777106478e-1,  1.99777106478e-1,  1.99777106478e-1,  1.99777106478e-1 ];
    pub const G_ATAN_COEF4:[f32;4]   = [ -3.33329491539e-1, -3.33329491539e-1, -3.33329491539e-1, -3.33329491539e-1 ];
    pub const G_EXP_COEF1:[f32;4]    = [  1.2102203e7, 1.2102203e7, 1.2102203e7, 1.2102203e7 ];
    pub const G_EXP_COEF2:[i32;4]  = [ -8388608, -8388608, -8388608, -8388608 ];
    pub const G_EXP_COEF3:[f32;4]    = [  1.1920929e-7, 1.1920929e-7, 1.1920929e-7, 1.1920929e-7 ];
    pub const G_EXP_COEF4:[f32;4]    = [  3.371894346e-1, 3.371894346e-1, 3.371894346e-1, 3.371894346e-1 ];
    pub const G_EXP_COEF5:[f32;4]    = [  6.57636276e-1, 6.57636276e-1, 6.57636276e-1, 6.57636276e-1 ];
    pub const G_EXP_COEF6:[f32;4]    = [  1.00172476, 1.00172476, 1.00172476, 1.00172476 ];
    
    #[inline]
    pub unsafe  fn abs(value:f32)->f32{
        return Vec1::select_index0(Vec1::abs(Vec1::splat(value)))
    }

    #[inline]
    pub unsafe  fn mod_calculate(value:f32,divisor:f32)->f32{
        return Vec1::select_index0(Vec1::mod_calculate(Vec1::splat(value),Vec1::splat(divisor)));
    }

    #[inline]
    pub unsafe  fn wrap2(value:f32, max_value:f32) ->f32{
        return Vec1::select_index0(Vec1::wrap(Vec1::splat(value),Vec1::zero_float(),Vec1::splat(max_value)));
    }

    #[inline]
    pub unsafe  fn wrap3(value:f32, min_value:f32, max_value:f32) ->f32{
        return Vec1::select_index0(Vec1::wrap(Vec1::splat(value), Vec1::splat(min_value), Vec1::splat(max_value)));
    }

    #[inline]
    pub unsafe  fn angle_mod(value:f32)->f32{
        return Vec1::select_index0(Vec1::angle_mod(Vec1::splat(value)));
    }

    #[inline]
    pub unsafe  fn sin_cos(angle:f32,mut sin:&f32,mut cos:&f32){
        let values = Vec2::sin_cos_to_float_type(Vec1::splat(angle));
        sin = Vec2::select_index0(values).borrow_mut();
        cos = Vec2::select_index1(values).borrow_mut();
    }

    #[inline]
    pub unsafe  fn sin(angle:f32)->f32{
        return Vec1::select_index0(Vec1::sin(Vec1::splat(angle)));
    }

    #[inline]
    pub unsafe  fn cos(angle:f32)->f32{
        return Vec1::select_index0(Vec1::cos(Vec1::splat(angle)));
    }

    #[inline]
    pub unsafe  fn acos(value:f32)->f32{
        return Vec1::select_index0(Vec1::acos(Vec1::splat(value)))
    }

    #[inline]
    pub unsafe  fn atan(value:f32)->f32{
        return Vec1::select_index0(Vec1::atan(Vec1::splat(value)));
    }

    #[inline]
    pub unsafe  fn atan2(y:f32,x:f32)->f32{
        return Vec1::select_index0(Vec1::atan2(Vec1::splat(y),Vec1::splat(x)));
    }

    #[inline]
    pub unsafe  fn sqrt(value:f32)->f32{
        return Vec1::select_index0(Vec1::sqrt(Vec1::splat(value)));
    }

    #[inline]
    pub unsafe  fn inv_sqrt(value:f32)->f32{
        return Vec1::select_index0(Vec1::sqrt_inv(Vec1::splat(value)));
    }
}
