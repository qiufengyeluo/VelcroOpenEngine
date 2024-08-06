#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::constants::*;
use crate::math::simd_math_vec1_sse::*;
use crate::math::vsimd::*;

pub trait VecType{
    fn load_aligned(addr :*f32)->FloatType;
    fn load_aligned_i128(addr :*const Int32Type)->Int32Type;
    fn load_unaligned(addr:&f32)->FloatType;
    fn load_unaligned_i128(addr:*const Int32Type)->Int32Type;
    fn store_aligned( addr:*mut f32,value:&FloatArgType);
    fn store_aligned_i128(addr :*mut Int32Type,value:&Int32ArgType);
    fn store_unaligned(addr :*mut f32,value:&FloatArgType);
    fn store_unaligned_i128(addr:*mut Int32Type,value:&Int32ArgType);
    fn stream_aligned(addr :*mut f32,value:&FloatArgType);
    fn stream_aligned_i128(addr:*mut Int32Type,value:&Int32ArgType);
    fn select_index0(value:&FloatArgType)->f32;
    fn splat(value:&f32)->FloatType;
    fn splat_i32(value:&i32)->Int32Type;

    fn add(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn sub(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn mul(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn madd(mul1:&FloatArgType,mul2:&FloatArgType,add:&FloatArgType)->FloatType;
    fn div(arg1:&FloatType, arg2: &mut FloatType) ->FloatType;
    fn abs(value:&FloatArgType)->FloatType;
    fn add_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn sub_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn mul_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn madd_i32(mul1:&Int32ArgType,mul2:Int32ArgType,add:&Int32ArgType)->Int32Type;
    fn abs_i32(value:&Int32ArgType)->Int32Type;
    fn not(value:&FloatArgType)->FloatType;
    fn and(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn and_not(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn or(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn xor(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn not_i32(value:&Int32ArgType)->Int32Type;
    fn and_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn or_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn xor_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn and_not_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn floor(value:&FloatArgType)->FloatType;
    fn ceil(value:&FloatArgType)->FloatType;
    fn round(value:&FloatArgType)->FloatType;
    fn truncate(value:&FloatArgType) ->FloatType;
    fn min(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn max(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn clamp(value:&FloatArgType,min:&FloatArgType,max:&FloatArgType) ->FloatType;
    fn min_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn max_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn clamp_i32(value:&Int32ArgType,min:&Int32ArgType,max:&Int32ArgType) ->Int32Type;
    fn cmp_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_neq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_all_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool;
    fn cmp_all_lt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool;
    fn cmp_all_lt_eq(arg1:&FloatArgType,arg2:&FloatArgType) -> bool;
    fn cmp_all_gt(arg1:&FloatArgType,arg2:&FloatArgType) ->bool;
    fn cmp_all_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->bool;
    fn cmp_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn cmp_neq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn cmp_gt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn cmp_gt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn cmp_lt_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn cmp_lt_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn cmp_all_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->bool;
    fn select(arg1:&FloatArgType,arg2:&FloatArgType,mask:&FloatArgType)->FloatType;
    fn select_i32(arg1:&Int32ArgType,arg2:&Int32ArgType,mask:&Int32ArgType)->Int32Type;
    fn reciprocal(value:&FloatArgType)->FloatType;
    fn reciprocal_estimate(value:&FloatArgType)->FloatType;
    fn mod_calculate(value:&FloatArgType,divisor:&FloatArgType)->FloatType;
    fn  wrap(value:&FloatArgType, min_value:&FloatArgType, max_value:&FloatArgType) ->FloatType;
    fn angle_mod(value:&FloatArgType) ->FloatType;
    fn sqrt(value:&FloatArgType)->FloatType;
    fn sqrt_estimate(value:&FloatArgType)->FloatType;
    fn sqrt_inv(value:&FloatArgType)->FloatType;
    fn sqrt_inv_estimate(value:&FloatArgType) ->FloatType;
    fn sin(value:&FloatArgType)->FloatType;
    fn cos(value:&FloatArgType)->FloatType;
    fn sin_cos(value:&FloatArgType,sin:&FloatType,cos:&FloatType);
    fn acos(value:&FloatArgType)->FloatType;
    fn atan(value:&FloatArgType) ->FloatType;
    fn atan2(y:&FloatArgType,x:&FloatArgType) ->FloatType;
    fn exp_estimate(x:&FloatArgType)->FloatType;
    fn convert_to_float(value:&Int32ArgType)->FloatType;
    fn convert_to_int(value:&FloatArgType)->Int32Type;
    fn convert_to_int_nearest(value:&FloatArgType)->Int32Type;
    fn cast_to_float(value:&Int32ArgType)->FloatType;
    fn cast_to_int(value:&FloatArgType)->Int32Type;
    fn zero_float() ->FloatType;
    fn zero_int() ->Int32Type;
}

pub trait VecTwoType:VecType{
    fn value_to_vec1(value:&FloatArgType) ->FloatType;
    fn from_vec1(value:&FloatArgType) ->FloatType;
    fn select_index1(value:&FloatArgType)->f32;
    fn splat_index0(value:&FloatArgType)->FloatType;
    fn splat_index1(value:&FloatArgType)->FloatType;
    fn replace_index0(a:&FloatArgType,b:&FloatArgType)->FloatType;
    fn replace_index0_f32(value:&FloatArgType,b:&f32)->FloatType;
    fn replace_index1_f32(a:&FloatArgType,b:&f32)->FloatType;
    fn replace_index1(a:&FloatArgType,b:&FloatArgType)->FloatType;
    fn dot(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn normalize(value:&FloatArgType)->FloatType;
    fn normalize_estimate(value:&FloatArgType)->FloatType;
    fn normalize_safe(value:&FloatArgType,tolerance:&f32)->FloatType;
    fn normalize_safe_estimate(value:&FloatArgType,tolerance:&f32)->FloatType;
}

pub trait VecThirdType:VecTwoType{
    fn value_to_vec2(value:&FloatArgType)->FloatType;
    fn from_vec2(value:&FloatArgType)->FloatType;
    fn select_index2(value:&FloatArgType)->f32;
    fn splat_index2(value:&FloatArgType)->FloatType;
    fn replace_index2_f32(a:&FloatArgType,b:&f32)->FloatType;
    fn replace_index2(a:&FloatArgType,b:&FloatArgType)->FloatType;

}
pub trait VecFourthType:VecThirdType{
    fn value_to_vec3(value:&FloatArgType)->FloatType;
    fn from_vec3(value:&FloatArgType)->FloatType;
    fn select_index3(value:&FloatArgType)->f32;
    fn splat_index3(value:&FloatArgType)->FloatType;
    fn replace_index3_f32(a:&FloatArgType,b:&f32)->FloatType;
    fn replace_index3(a:&FloatArgType,b:&FloatArgType)->FloatType;
}

pub trait Vec1Type:VecType{
    fn load_immediate(x:&f32)->FloatType;
    fn load_immediate_i32(x:&i32)->Int32Type;
}
pub trait Vec2Type :VecTwoType{
    fn load_immediate(x:&f32,y:&f32)->FloatType;
    fn load_immediate_i32(x:&i32,y:&i32)->Int32Type;
    fn atan2_float_type(value:&FloatArgType)->FloatType;
    fn sin_cos_to_float_type(angle:&FloatArgType)->FloatType;

}
pub trait Vec3Type :VecThirdType{
    fn load_immediate(x:&f32,y:&f32,z:&f32)->FloatType;
    fn load_immediate_i32(x:&i32,y:&i32,z:&i32)->Int32Type;
    fn cross(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType;
    fn mat3x3inverse(rows:*const FloatType,out :*const FloatType);
}
pub trait Vec4Type :VecFourthType{
    fn load_immediate(x:&f32,y:&f32,z:&f32,w:&f32)->FloatType;
    fn load_immediate_i32(x:&i32,y:&i32,z:&i32,w:&i32)->Int32Type;
}
pub struct Common{
}
impl  Common{


    pub fn fast_load_constant<T:VecType>(values:*const f32)->FloatType{
        unsafe { return *(values as * FloatType); }
    }

    pub fn fast_load_constant_i32<T:VecType>(values:*const i32)->Int32Type{
        unsafe { return *(values as * Int32Type); }
    }
    pub fn wrap<T: VecType>(value :&FloatArgType, min_value:&FloatArgType, max_value:&FloatArgType ) ->FloatType
    {
        let value_adjust:FloatType = T::sub(value, min_value);
        let max_adjust:FloatType = T::sub(max_value, min_value);
        let value_offset = T::select(max_value, T::zero_float().borrow(), T::cmp_lt(value_adjust.borrow(), T::zero_float().borrow()).borrow());
        return  T::add(min_value,T::add(value_offset.borrow(), T::mod_calculate(value_adjust.borrow(), max_adjust.borrow()).borrow()).borrow());
    }

    pub fn angle_mod<T:VecType>(value:&FloatArgType)->FloatType{
        let vec_pi:FloatType = T::splat(PI.borrow());
        let vec_two_pi = T::splat(TWO_PI.borrow());
        let positive_angles = T::sub(T::mod_calculate(T::add(value, vec_pi.borrow()).borrow(), vec_two_pi.borrow()).borrow(), vec_pi.borrow());
        let negative_angles = T::add(T::mod_calculate(T::sub(value, vec_pi.borrow()).borrow(), vec_two_pi.borrow()).borrow(), vec_pi.borrow());
        let mask = T::cmp_gt_eq(value,T::zero_float().borrow());
        return T::select(positive_angles.borrow(), negative_angles.borrow(), mask.borrow());
    }

    pub fn compute_sinx_cosx<T:VecType>(x:&FloatArgType,mut sinx: &FloatArgType,mut cosx: &FloatArgType){
        let x2 = T::mul(x,x);
        let x3 = T::mul(x2.borrow(),x);
        sinx = T::madd(x3.borrow(),
                       T::madd(x2.borrow(),
                               T::madd(x2.borrow(),
                                       Self::fast_load_constant(G_SIN_COEF1.as_ptr()).borrow(),
                                       Self::fast_load_constant(G_SIN_COEF2.as_ptr()).borrow()).borrow(),
                               Self::fast_load_constant(G_SIN_COEF3.as_ptr()).borrow()
                       ).borrow(),x).borrow_mut();
        cosx = T::madd(x2.borrow(),
                       T::madd(x2.borrow(),
                               T::madd(x2.borrow(),
                                       Self::fast_load_constant(G_COS_COEF1.as_ptr()).borrow(),
                                       Self::fast_load_constant(G_COS_COEF2.as_ptr()).borrow()).borrow(),
                               Self::fast_load_constant(G_COS_COEF3.as_ptr()).borrow()
                       ).borrow(),T::splat(1.0.borrow()).borrow()).borrow_mut();
    }

    pub unsafe fn sin<T:VecType>(value:&FloatArgType)->FloatType{
        let mut x = T::mul(value,Self::fast_load_constant(G_TWO_OVER_PI.as_ptr()).borrow());
        let intx =T::convert_to_int_nearest(x.borrow());
        let offset = T::and_i32(intx.borrow(), T::splat_i32(3.borrow()).borrow());
        let intx_float = T::convert_to_float(intx.borrow());
        x = T::sub(value,T::mul(intx_float.borrow(), Self::fast_load_constant(G_HALF_PI.as_ptr()).borrow()).borrow());
        let mut sinx:FloatType;
        let mut cosx:FloatType;
        Self::compute_sinx_cosx(x.borrow(),sinx.borrow_mut(),cosx.borrow_mut());
        let mut mask =T::cmp_eq_i32(T::and_i32(offset.borrow(),T::splat_i32(1.borrow()).borrow()).borrow(),T::zero_int().borrow());
        let mut result = T::select(sinx.borrow(),cosx.borrow(),T::cast_to_float(mask.borrow()).borrow());
        mask = T::cmp_eq_i32(T::and_i32(offset.borrow(),T::splat_i32(2.borrow()).borrow()).borrow(),T::zero_int().borrow());
        result = T::select(result.borrow(),T::xor(result.borrow(),T::splat(-0.0.borrow()).borrow()).borrow(),T::cast_to_float(mask.borrow()).borrow());
        result
    }
    pub unsafe fn cos<T:VecType>(value:&FloatArgType)->FloatType{
        let mut x = T::mul(value,Self::fast_load_constant(G_TWO_OVER_PI.as_ptr()).borrow());
        let intx = T::convert_to_int_nearest(x.borrow());
        let offset = T::and_i32(T::add_i32(intx.borrow(), T::splat_i32(1.borrow()).borrow()).borrow(), T::splat_i32(3.borrow()).borrow());
        let intx_float = T::convert_to_float(intx.borrow());
        x = T::sub(value,T::mul(intx_float.borrow(), Self::fast_load_constant(G_HALF_PI.as_ptr()).borrow()).borrow());
        let mut sinx:FloatType;
        let mut cosx:FloatType;
        Self::compute_sinx_cosx(x.borrow(),sinx.borrow_mut(),cosx.borrow_mut());
        let mut mask = T::cmp_eq_i32(T::and_i32(offset.borrow(),T::splat_i32(1.borrow()).borrow()).borrow(),T::zero_int().borrow());
        let mut result = T::select(sinx.borrow(),cosx.borrow(),T::cast_to_float(mask.borrow()).borrow());
        mask =T::cmp_eq_i32(T::and_i32(offset.borrow(),T::splat_i32(2.borrow()).borrow()).borrow(),T::zero_int().borrow());
        result = T::select(result.borrow(),T::xor(result.borrow(),T::splat(-0.0.borrow()).borrow()).borrow(),T::cast_to_float(mask.borrow()).borrow());
        result
    }

    pub fn sin_cos<T:VecType>(value:&FloatArgType,mut sin:&FloatArgType,mut cos:&FloatArgType){
        let mut x = T::mul(value,Self::fast_load_constant(G_TWO_OVER_PI.as_ptr()).borrow());
        let intx = T::convert_to_int_nearest(x.borrow());
        let offset_sin = T::and_i32(intx.borrow(), T::splat_i32(3.borrow()).borrow());
        let offset_cos = T::and_i32(T::add_i32(intx.borrow(), T::splat_i32(1.borrow()).borrow()).borrow(), T::splat_i32(3.borrow()).borrow());
        let intx_float = T::convert_to_float(intx.borrow());
        x = T::sub(value,T::mul(intx_float.borrow(), Self::fast_load_constant(G_HALF_PI.as_ptr()).borrow()).borrow());
        let mut sinx:FloatType;
        let mut cosx:FloatType;
        Self::compute_sinx_cosx(x.borrow(),sinx.borrow_mut(),cosx.borrow_mut());
        let mut sin_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_sin.borrow(), T::splat_i32(1.borrow()).borrow()).borrow(), T::zero_int().borrow()).borrow());
        let mut cos_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_cos.borrow(), T::splat_i32(1.borrow()).borrow()).borrow(), T::zero_int().borrow()).borrow());
        sin = T::select(sinx.borrow(), cosx.borrow(), sin_mask.borrow()).borrow_mut();
        cos = T::select(sinx.borrow(), cosx.borrow(), cos_mask.borrow()).borrow_mut();
        sin_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_sin.borrow(), T::splat_i32(2.borrow()).borrow()).borrow(), T::zero_int().borrow()).borrow());
        cos_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_cos.borrow(), T::splat_i32(2.borrow()).borrow()).borrow(), T::zero_int().borrow()).borrow());
        sin = T::select(sin.borrow(),T::xor(sin.borrow(),Self::fast_load_constant(G_NEGATE_MASK.as_ptr() as *const f32).borrow()).borrow(),sin_mask.borrow()).borrow_mut();
        cos = T::select(cos.borrow(),T::xor(cos.borrow(),Self::fast_load_constant(G_NEGATE_MASK.as_ptr() as *const f32).borrow()).borrow(),cos_mask.borrow()).borrow_mut();
    }

    pub fn sin_cos_to_float_type<T:VecType>(angles:&FloatArgType)->FloatType{
        let angle_offset = T::load_immediate_fourth_f32(0.0.borrow(), HALF_PI.borrow(), 0.0.borrow(), HALF_PI.borrow());
        let sin_angles = T::add(angles, angle_offset.borrow());
        return  T::sin(sin_angles.borrow());
    }

    pub fn acos<T:VecType>(value:&FloatArgType)->FloatType{
        let xabs = T::abs(value);
        let xabs2 = T::mul(xabs.borrow(),xabs.borrow());
        let xabs4 = T::mul(xabs2.borrow(),xabs2.borrow());
        let t1 = T::sqrt(T::sub(T::splat(1.0.borrow()).borrow(),xabs.borrow()).borrow());
        let select = T::cmp_lt(value.to_owned().borrow(),T::zero_float().borrow());

        let hi = T::madd(xabs.borrow(),
                                    T::madd(xabs.borrow(),
                                            T::madd(xabs.borrow(),
                                                    Self::fast_load_constant(G_ACOS_HI_COEF1.as_ptr()).borrow(),
                                                    Self::fast_load_constant(G_ACOS_HI_COEF2.as_ptr()).borrow()).borrow(),
                                            Self::fast_load_constant(G_ACOS_HI_COEF3.as_ptr()).borrow()).borrow(),
                                    Self::fast_load_constant(G_ACOS_HI_COEF4.as_ptr()).borrow());

        let lo = T::madd(xabs.borrow(),
                                    T::madd(xabs.borrow(),
                                            T::madd(xabs.borrow(),
                                                    Self::fast_load_constant_f32(G_ACOS_LO_COEF1.as_ptr()).borrow(),
                                                    Self::fast_load_constant_f32(G_ACOS_LO_COEF2.as_ptr()).borrow()).borrow(),
                                            Self::fast_load_constant_f32(G_ACOS_LO_COEF3.as_ptr()).borrow()).borrow(),
                                    Self::fast_load_constant_f32(G_ACOS_LO_COEF4.as_ptr()).borrow());

        let result = T::madd(hi.borrow(),xabs4.borrow(),lo.borrow());
        let positive = T::mul(t1,result.borrow());
        let negative = T::sub(T::splat(PI.borrow()).borrow(),positive.borrow());
        return T::select(negative.borrow(),positive.borrow(),select.borrow());

    }

    pub fn acos_estimate<T:VecType>(value:&FloatArgType)->FloatType{
        let xabs = T::abs(value);
        let t1 = T::sqrt_estimate(T::sub(T::splat(1.0.borrow()).borrow(),xabs.borrow()).borrow());
        let select = T::cmp_lt(value,T::zero_float().borrow());
        let result = T::madd(xabs.borrow(),
                                        T::madd(xabs.borrow(),
                                                T::madd(xabs.borrow(),
                                                        Self::fast_load_constant(G_ACOS_COEF1.as_ptr()).borrow(),
                                                        Self::fast_load_constant(G_ACOS_COEF2.as_ptr()).borrow()).borrow(),
                                                Self::fast_load_constant(G_COS_COEF3.as_ptr()).borrow()).borrow(),
                                        Self::fast_load_constant(G_HALF_PI.as_ptr()).borrow());
        let positive = T::mul(t1.borrow(),result.borrow());
        let negative = T::sub(T::splat(PI.borrow()).borrow(),positive.borrow());
        return T::select(negative.borrow(),positive.borrow(),select.borrow());
    }

    pub fn atan<T:VecType>(value:&FloatArgType)->FloatType
    {
        let mut x = value.to_owned();
        let signbit = T::and(x.borrow(), T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.borrow()).borrow()).borrow());

        let xabs = T::abs(x.borrow());
        let cmp0 = T::cmp_gt(xabs.borrow(),Self::fast_load_constant(G_ATAN_HI_RANGE.as_ptr()).borrow());
        let mut cmp1 = T::cmp_gt(xabs.borrow(),Self::fast_load_constant(G_ATAN_LO_RANGE.as_ptr()).borrow());
        let cmp2 = T::and_not(cmp0.borrow(),cmp1.borrow());

        let mut xabs_safe = T::add(xabs.borrow(), T::and(T::cmp_eq(xabs.borrow(), T::zero_float().borrow()).borrow(), Self::fast_load_constant(G_VEC1111.as_ptr()).borrow()).borrow());
        let y0 = T::and(cmp0.borrow(),Self::fast_load_constant(G_HALF_PI.as_ptr()).borrow());
        let mut x0 = T::div(Self::fast_load_constant(G_VEC1111.as_ptr()).borrow(), xabs_safe.borrow_mut());
        x0 = T::xor(x0.borrow(),T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.borrow()).borrow()).borrow());
        let y1 = T::and(cmp2.borrow(),Self::fast_load_constant(G_QUARTER_PI.borrow()).borrow());
        let x1_numer = T::sub(xabs.borrow(),Self::fast_load_constant(G_VEC1111.borrow()).borrow());
        let mut x1_denom = T::add(xabs.borrow(),Self::fast_load_constant(G_VEC1111.borrow()).borrow());
        let x1 = T::div(x1_numer.borrow(),x1_denom.borrow_mut());
        let mut x2 = T::and(cmp2.borrow(),x1.borrow());
        x0 = T::and(cmp0.borrow(),x0.borrow());
        x2 = T::or(x2.borrow(),x0.borrow());
        cmp1 = T::or(cmp0.borrow(),cmp2.borrow());
        x2 = T::and(cmp1.borrow(),x2.borrow());
        x = T::and_not(cmp1.borrow(), xabs.borrow());
        x = T::or(x2.borrow(),x.borrow());
        let mut y = T::or(y0.borrow(),y1.borrow());
        let x_sqr = T::mul(x.borrow(),x.borrow());
        let x_cub = T::mul(x_sqr.borrow(),x.borrow());


        let result = T::madd(x_cub.borrow(),
                                        T::madd(x_sqr.borrow(),
                                                T::madd(x_sqr.borrow(),
                                                        T::madd(x_sqr.borrow(),
                                                                Self::fast_load_constant(G_ATAN_COEF1.borrow()).borrow(),
                                                                Self::fast_load_constant(G_ATAN_COEF2.borrow()).borrow()).borrow(),
                                                        Self::fast_load_constant(G_ATAN_COEF3.borrow()).borrow()).borrow(),
                                                Self::fast_load_constant(G_ATAN_COEF4.borrow()).borrow()).borrow(),
                                        x.borrow());
        y = T::add(y.borrow(),result.borrow());
        y = T::xor(y.borrow(), signbit.borrow());
        y
    }

    pub fn atan2<T:VecType>(y:&FloatArgType,x:&FloatArgType)->FloatType
    {
        let x_eq_0 = T::cmp_eq(x,T::zero_float().borrow());
        let x_ge_0 = T::cmp_gt_eq(x,T::zero_float().borrow());
        let x_lt_0 = T::cmp_lt(x,T::zero_float().borrow());

        let y_eq_0 = T::cmp_eq(y,T::zero_float().borrow());
        let y_lt_0 = T::cmp_lt(y,T::zero_float().borrow());

        let zero_mask = T::and(x_ge_0.borrow(),y_eq_0.borrow());
        let pio2_mask = T::and_not(y_eq_0.borrow(),x_eq_0.borrow());
        let pio2_mask_sign = T::and(y_lt_0.borrow(),T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.borrow()).borrow()).borrow());
        let mut pio2_result = Self::fast_load_constant(G_HALF_PI.borrow());
        pio2_result = T::xor(pio2_result.borrow(),pio2_mask_sign.borrow());
        pio2_result = T::and(pio2_mask.borrow(), pio2_result.borrow());

        let pi_mask = T::and(y_eq_0.borrow(),x_lt_0.borrow());
        let mut pi_result = Self::fast_load_constant(G_PI.borrow());
        pi_result = T::and(pi_mask.borrow(),pi_result.borrow());
        let mut swap_sign_mask_offset = T::and(x_lt_0.borrow(),y_lt_0.borrow());
        swap_sign_mask_offset = T::and(swap_sign_mask_offset.borrow(),T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.borrow()).borrow()).borrow());

        let mut offset1 = Self::fast_load_constant(G_PI.borrow());
        offset1 = T::xor(offset1.borrow(),swap_sign_mask_offset.borrow());

        let offset = T::and(x_lt_0.borrow(),offset1.borrow());

        let mut x_safe = T::add(x, T::and(x_eq_0.borrow(), Self::fast_load_constant(G_VEC1111.borrow()).borrow()).borrow());
        let atan_mask = T::not(T::or(x_eq_0.borrow(),y_eq_0.borrow()).borrow());
        let atan_arg = T::div(y.borrow(), x_safe.borrow_mut());
        let mut atan_result = T::atan(atan_arg.borrow());
        atan_result = T::add(atan_result.borrow(),offset.borrow());
        atan_result = T::and_not(pio2_mask.borrow(),atan_result.borrow());
        atan_result = T::and(atan_mask.borrow(),atan_result.borrow());

        let mut result = T::and_not(zero_mask.borrow(),pio2_result.borrow());
        result = T::or(result.borrow(),pio2_result.borrow());
        result = T::or(result.borrow(),pi_result.borrow());
        result = T::or(result.borrow(),atan_result.borrow());

        result
    }

    pub fn exp_estimate<T:VecType>(x:&FloatArgType)->FloatType{
        let a = T::convert_to_int_nearest(T::mul(Self::fast_load_constant(G_EXP_COEF1.borrow()).borrow(),x).borrow());
        let b = T::and_i32(a.borrow(),Self::fast_load_constant_i32(G_EXP_COEF2.borrow()).borrow());
        let c = T::sub_i32(a.borrow(),b.borrow());
        let f = T::mul(Self::fast_load_constant(G_EXP_COEF3.borrow()).borrow(),T::convert_to_float(c.borrow()).borrow());
        let i = T::madd(f.borrow(),Self::fast_load_constant(G_EXP_COEF4.borrow()).borrow(),Self::fast_load_constant(G_EXP_COEF5.borrow()).borrow());
        let j = T::madd(i.borrow(),f.borrow(),Self::fast_load_constant(G_EXP_COEF6.borrow()).borrow());
        return T::cast_to_float(T::add_i32(b.borrow(),T::cast_to_int(j.borrow()).borrow()).borrow());
    }

    pub fn normalize<T:VecType>(value:&FloatArgType)->FloatType{
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value).borrow()).borrow());
        let mut length = T::sqrt(length_squared.borrow());
        return  T::div(value,length.borrow_mut());
    }

    pub fn normalize_estimate<T:VecType>(value:&FloatArgType)->FloatType{
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value).borrow()).borrow());
        let inv_length = T::sqrt_inv_estimate(length_squared.borrow());
        return  T::mul(inv_length.borrow(), value);
    }

    pub fn normalize_safe<T:VecType>(value:&FloatArgType,tolerance:&f32)->FloatType{
        let float_epsilon = T::splat((tolerance*tolerance).borrow());
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value).borrow()).borrow());
        if T::cmp_all_lt(length_squared.borrow(), float_epsilon.borrow()){
            return T::zero_float();
        }else {
            return T::div(value,T::sqrt(length_squared.borrow()).borrow_mut());
        }
    }

    pub fn normalize_safe_estimate<T:VecType>(value:&FloatArgType,tolerance:&f32) ->FloatType{
        let float_epsilon = T::splat((tolerance*tolerance).borrow());
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value).borrow()).borrow());
        if T::cmp_all_lt(length_squared.borrow(), float_epsilon.borrow()){
            return T::zero_float();
        }else {
            return T::mul(value,T::sqrt_inv_estimate(length_squared.borrow()).borrow());
        }
    }

    pub fn quaternion_transform<T:VecType>(quat:&FloatArgType,vec3:&FloatArgType) ->FloatType{
        let two = T::splat(2.0.borrow());
        let scalar = unsafe { T::splat_index3(quat.borrow()) };
        let partial1 = T::splat_index0(T::from_vec1(T::dot(quat,vec3).borrow()).borrow());
        let partial2 = T::mul(quat,partial1.borrow());
        let sum1 = T::mul(partial2.borrow(),two.borrow());
        let partial3 = T::splat_index0(T::from_vec1(T::dot(quat,quat).borrow()).borrow());
        let partial4 = T::mul(scalar.borrow(),scalar.borrow());
        let partial5 = T::sub(partial4.borrow(),partial3.borrow());
        let sum2 = T::mul(partial5.borrow(),vec3);
        let partial6 = T::mul(scalar.borrow(),two.borrow());
        let partial7 = T::cross(quat,vec3);
        let sum3 = T::mul(partial6.borrow(),partial7.borrow());
        return T::add(T::add(sum1.borrow(),sum2.borrow()).borrow(),sum3.borrow());
    }

    pub fn construct_plane<T:VecType>(normal:&FloatArgType,point:&FloatArgType)->FloatType{
        let distance = unsafe { Vec1::sub(Vec1::zero_float().borrow(), T::dot(normal.borrow(),point.borrow()).borrow()) };
        unsafe { return T::replace_index3(normal.borrow(), T::splat_index0(T::from_vec1(distance.borrow()).borrow()).borrow()); }
    }

    pub fn plane_distance<T:VecType>(plane:&FloatArgType, point:&FloatArgType) ->FloatType{
        let reference_point = unsafe { T::replace_index3_f32(point.borrow(), 1.0.borrow()) };
        return T::dot(reference_point.borrow(), plane);
    }

    pub unsafe fn mat3x3multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        *out[0] = T::madd(T::splat_index2(*rows_a[0]).borrow(), *rows_b[2], T::madd(T::splat_index1(rows_a[0]).borrow(), rows_b[1], T::mul(T::splat_index0(*rows_a[0]).borrow(), *rows_b[0]).borrow()).borrow() );
        *out[0] = T::madd(T::splat_index2(*rows_a[1]).borrow(), *rows_b[2], T::madd(T::splat_index1(rows_a[1]).borrow(), rows_b[1], T::mul(T::splat_index0(*rows_a[1]).borrow(), *rows_b[0]).borrow()).borrow());
        *out[0] = T::madd(T::splat_index2(*rows_a[2]).borrow(), *rows_b[2], T::madd(T::splat_index1(rows_a[2]).borrow(), rows_b[1], T::mul(T::splat_index0(*rows_a[2]).borrow(), *rows_b[0]).borrow()).borrow());
    }

    pub unsafe fn mat3x3transpose_multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:*const FloatType){
        *out[0] = T::madd(T::splat_index0(*rows_a[0]).borrow(), *rows_b[0], T::madd(T::splat_index0(rows_a[1]).borrow(), rows_b[1], T::mul(T::splat_index0(*rows_a[2]).borrow(), *rows_b[2]).borrow()).borrow() );
        *out[0] = T::madd(T::splat_index1(*rows_a[0]).borrow(), *rows_b[0], T::madd(T::splat_index2(rows_a[1]).borrow(), rows_b[1], T::mul(T::splat_index0(*rows_a[2]).borrow(), *rows_b[2]).borrow()).borrow());
        *out[0] = T::madd(T::splat_index2(*rows_a[0]).borrow(), *rows_b[0], T::madd(T::splat_index3(rows_a[1]).borrow(), rows_b[1], T::mul(T::splat_index0(*rows_a[2]).borrow(), *rows_b[2]).borrow()).borrow());

    }

    pub unsafe fn mat3x3transform_vector<T:VecType>(rows:*const FloatType,vector:&FloatArgType)->FloatType{
        let mut transposed:[FloatType;3];
        VecType::mat3x3transpose(rows,transposed.borrow_mut());
        return VecType::mat3x3transpose_transform_vector(transposed.borrow(),vector);
    }

    pub unsafe fn mat3x3transpose_transform_vector<T:VecType>(rows:*const FloatType,vector:&FloatArgType)->FloatType{
        return  T::madd(T::splat_index2(vector).borrow(),*rows[2],T::madd(T::splat_index1(vector).borrow(),*rows[1],T::mul(T::splat_index0(vector).borrow(),*rows[0]).borrow()).borrow());
    }

    pub unsafe fn mat3x4multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:*const FloatType){
        let fourth = Self::fast_load_constant(G_VEC1111.borrow());
        *out[0] = T::madd(T::splat_index3(*rows_a[0]).borrow(), fourth.borrow(), T::madd(T::splat_index2(*rows_a[0]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[0]).borrow(), *rows_b[1], T::mul(T::splat_index0(*rows_a[0]).borrow(), *rows_b[0]).borrow()).borrow()).borrow());
        *out[1] = T::madd(T::splat_index3(*rows_a[0]).borrow(), fourth.borrow(), T::madd(T::splat_index2(*rows_a[1]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[1]).borrow(), *rows_b[1], T::mul(T::splat_index0(*rows_a[1]).borrow(), *rows_b[0]).borrow()).borrow()).borrow());
        *out[2] = T::madd(T::splat_index3(*rows_a[0]).borrow(), fourth.borrow(), T::madd(T::splat_index2(*rows_a[2]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[2]).borrow(), *rows_b[1], T::mul(T::splat_index0(*rows_a[2]).borrow(), *rows_b[0]).borrow()).borrow()).borrow());
    }

    pub unsafe fn mat4x4inverse_fast<T:VecType>(rows:*const FloatType,mut out :*const FloatType){
        let pos = T::madd(T::splat_index3(*rows[2]).borrow(),
                          *rows[2], T::madd(T::splat_index3(*rows[1]).borrow(),*rows[1],
                                            T::mul(T::splat_index3(*rows[0]).borrow(),*rows[0]).borrow()).borrow());
        let mut transposed : [FloatType;4] = [*rows[0],*rows[1],*rows[2],T::xor(pos.borrow(),Self::fast_load_constant(G_NEGATE_MASK.borrow()).borrow())];
        T::mat3x3transpose(transposed.borrow_mut(),out);
        *out[3] = Self::fast_load_constant(G_VEC1111.borrow());
    }

    pub unsafe fn mat4x4multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:*const FloatType){
        *out[0] = T::madd(T::splat_index3(*rows_a[0]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[0]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[0]).borrow(), *rows_b[1], T::mul(T::splat_index0(rows_a[0]).borrow(), rows_b[0]).borrow()).borrow()).borrow());
        *out[1] = T::madd(T::splat_index3(*rows_a[1]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[1]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[1]).borrow(), *rows_b[1], T::mul(T::splat_index0(rows_a[1]).borrow(), rows_b[0]).borrow()).borrow()).borrow());
        *out[2] = T::madd(T::splat_index3(*rows_a[2]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[2]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[2]).borrow(), *rows_b[1], T::mul(T::splat_index0(rows_a[2]).borrow(), rows_b[0]).borrow()).borrow()).borrow());
        *out[3] = T::madd(T::splat_index3(*rows_a[3]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[3]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[3]).borrow(), *rows_b[1], T::mul(T::splat_index0(rows_a[3]).borrow(), rows_b[0]).borrow()).borrow()).borrow());
    }

    pub unsafe fn mat4x4multiply_add<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, add:*const FloatType, mut out:*const FloatType){
        *out[0] = T::madd(T::splat_index3(*rows_a[0]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[0]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[0]).borrow(), *rows_b[1], T::madd(T::splat_index0(*rows_a[0]).borrow(), *rows_b[0], *add[0]).borrow()).borrow()).borrow());
        *out[1] = T::madd(T::splat_index3(*rows_a[1]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[1]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[1]).borrow(), *rows_b[1], T::madd(T::splat_index0(*rows_a[1]).borrow(), *rows_b[0], *add[1]).borrow()).borrow()).borrow());
        *out[2] = T::madd(T::splat_index3(*rows_a[2]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[2]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[2]).borrow(), *rows_b[1], T::madd(T::splat_index0(*rows_a[2]).borrow(), *rows_b[0], *add[2]).borrow()).borrow()).borrow());
        *out[3] = T::madd(T::splat_index3(*rows_a[3]).borrow(), *rows_b[3], T::madd(T::splat_index2(*rows_a[3]).borrow(), *rows_b[2], T::madd(T::splat_index1(*rows_a[3]).borrow(), *rows_b[1], T::madd(T::splat_index0(*rows_a[3]).borrow(), *rows_b[0], *add[3]).borrow()).borrow()).borrow());
    }

    pub unsafe fn mat4x4transpose_multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:*const FloatType){
        *out[0] = T::madd(T::splat_index0(*rows_a[0]).borrow(), *rows_b[0], T::madd(T::splat_index0(*rows_a[1]).borrow(), *rows_b[1], T::madd(T::splat_index0(*rows_a[2]).borrow(), *rows_b[2], T::mul(T::splat_index0(*rows_a[3]).borrow(), *rows_b[3]).borrow()).borrow()).borrow());
        *out[1] = T::madd(T::splat_index1(*rows_a[0]).borrow(), *rows_b[0], T::madd(T::splat_index1(*rows_a[1]).borrow(), *rows_b[1], T::madd(T::splat_index1(*rows_a[2]).borrow(), *rows_b[2], T::mul(T::splat_index1(*rows_a[3]).borrow(), *rows_b[3]).borrow()).borrow()).borrow());
        *out[2] = T::madd(T::splat_index2(*rows_a[0]).borrow(), *rows_b[0], T::madd(T::splat_index2(*rows_a[1]).borrow(), *rows_b[1], T::madd(T::splat_index2(*rows_a[2]).borrow(), *rows_b[2], T::mul(T::splat_index2(*rows_a[3]).borrow(), *rows_b[3]).borrow()).borrow()).borrow());
        *out[3] = T::madd(T::splat_index3(*rows_a[0]).borrow(), *rows_b[0], T::madd(T::splat_index3(*rows_a[1]).borrow(), *rows_b[1], T::madd(T::splat_index3(*rows_a[2]).borrow(), *rows_b[2], T::mul(T::splat_index3(*rows_a[3]).borrow(), *rows_b[3]).borrow()).borrow()).borrow());
    }

    pub unsafe fn mat4x4transpose_transform_vector<T:VecType>(rows:*const FloatType,vector:&FloatArgType)->FloatType{
        return T::madd(T::splat_index3(vector).borrow(), *rows[3], T::madd(T::splat_index2(vector).borrow(), *rows[2], T::madd(T::splat_index1(vector).borrow(), *rows[1], T::mul(T::splat_index0(vector).borrow(), *rows[0]).borrow()).borrow()).borrow());
    }
}
