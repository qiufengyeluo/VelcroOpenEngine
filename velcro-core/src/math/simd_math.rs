
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

    #[inline]
    pub unsafe  fn abs(value:&f32)->f32{
        return Vec1::select_index0(Vec1::abs(Vec1::splat(value).borrow()).borrow())
    }

    #[inline]
    pub unsafe  fn mod_calculate(value:&f32,divisor:&f32)->f32{
        return Vec1::select_index0(Vec1::mod_calculate(Vec1::splat(value.borrow()).borrow(),Vec1::splat(divisor).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn wrap2(value:&f32, max_value:&f32) ->f32{
        return Vec1::select_index0(Vec1::wrap(Vec1::splat(value).borrow(),Vec1::zero_float().borrow(),Vec1::splat(max_value).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn wrap3(value:&f32, min_value:&f32, max_value:&f32) ->f32{
        return Vec1::select_index0(Vec1::wrap(Vec1::splat(value).borrow(), Vec1::splat(min_value).borrow(), Vec1::splat(max_value).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn angle_mod(value:&f32)->f32{
        return Vec1::select_index0(Vec1::angle_mod(Vec1::splat(value).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn sin_cos(angle:&f32,mut sin:&f32,mut cos:&f32){
        let values = Vec2::sin_cos_to_float_type(Vec1::splat(angle).borrow());
        sin = Vec2::select_index0(values.borrow()).borrow_mut();
        cos = Vec2::select_index1(values.borrow()).borrow_mut();
    }

    #[inline]
    pub unsafe  fn sin(angle:&f32)->f32{
        return Vec1::select_index0(Vec1::sin(Vec1::splat(angle).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn cos(angle:&f32)->f32{
        return Vec1::select_index0(Vec1::cos(Vec1::splat(angle).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn acos(value:&f32)->f32{
        return Vec1::select_index0(Vec1::acos(Vec1::splat(value).borrow()).borrow())
    }

    #[inline]
    pub unsafe  fn atan(value:&f32)->f32{
        return Vec1::select_index0(Vec1::atan(Vec1::splat(value).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn atan2(y:&f32,x:&f32)->f32{
        return Vec1::select_index0(Vec1::atan2(Vec1::splat(y).borrow(),Vec1::splat(x).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn sqrt(value:&f32)->f32{
        return Vec1::select_index0(Vec1::sqrt(Vec1::splat(value).borrow()).borrow());
    }

    #[inline]
    pub unsafe  fn inv_sqrt(value:&f32)->f32{
        return Vec1::select_index0(Vec1::sqrt_inv(Vec1::splat(value).borrow()).borrow());
    }
}
