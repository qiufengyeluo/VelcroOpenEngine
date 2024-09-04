#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::math_utils::constants::*;
use crate::math::simd_math::simd::{G_ACOS_COEF1, G_ACOS_COEF2, G_ACOS_HI_COEF1, G_ACOS_HI_COEF2, G_ACOS_HI_COEF3, G_ACOS_HI_COEF4, G_ACOS_LO_COEF1, G_ACOS_LO_COEF2, G_ACOS_LO_COEF3, G_ACOS_LO_COEF4, G_ATAN_COEF1, G_ATAN_COEF2, G_ATAN_COEF3, G_ATAN_COEF4, G_ATAN_HI_RANGE, G_ATAN_LO_RANGE, G_COS_COEF1, G_COS_COEF2, G_COS_COEF3, G_EXP_COEF1, G_EXP_COEF2, G_EXP_COEF3, G_EXP_COEF4, G_EXP_COEF5, G_EXP_COEF6, G_HALF_PI, G_NEGATE_MASK, G_PI, G_QUARTER_PI, G_SIN_COEF1, G_SIN_COEF2, G_SIN_COEF3, G_TWO_OVER_PI, G_VEC1111};
use crate::math::simd_math_vec1_sse::*;
use crate::math::vsimd::*;

pub trait VecType{
    fn load_aligned(addr :*f32)->FloatType;
    fn load_aligned_i128(addr :*const Int32Type)->Int32Type;
    fn load_unaligned(addr:&f32)->FloatType;
    fn load_unaligned_i128(addr:*const Int32Type)->Int32Type;
    fn store_aligned( addr:*mut f32,value:FloatArgType);
    fn store_aligned_i128(addr :*mut Int32Type,value:Int32ArgType);
    fn store_unaligned(addr :*mut f32,value:FloatArgType);
    fn store_unaligned_i128(addr:*mut Int32Type,value:Int32ArgType);
    fn stream_aligned(addr :*mut f32,value:FloatArgType);
    fn stream_aligned_i128(addr:*mut Int32Type,value:Int32ArgType);
    fn select_index0(value:FloatArgType)->f32;
    fn splat(value:f32)->FloatType;
    fn splat_i32(value:i32)->Int32Type;

    fn add(arg1:FloatArgType,arg2:FloatArgType)->FloatType;
    fn sub(arg1:FloatArgType,arg2:FloatArgType)->FloatType;
    fn mul(arg1:FloatArgType,arg2:FloatArgType)->FloatType;
    fn madd(mul1:FloatArgType,mul2:FloatArgType,add:FloatArgType)->FloatType;
    fn div(arg1:FloatType, arg2: &mut FloatType) ->FloatType;
    fn abs(value:FloatArgType)->FloatType;
    fn add_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type;
    fn sub_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type;
    fn mul_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type;
    fn madd_i32(mul1:Int32ArgType,mul2:Int32ArgType,add:Int32ArgType)->Int32Type;
    fn abs_i32(value:Int32ArgType)->Int32Type;
    fn not(value:FloatArgType)->FloatType;
    fn and(arg1:FloatArgType,arg2:FloatArgType)->FloatType;
    fn and_not(arg1:FloatArgType,arg2:FloatArgType)->FloatType;
    fn or(arg1:FloatArgType,arg2:FloatArgType)->FloatType;
    fn xor(arg1:FloatArgType,arg2:FloatArgType)->FloatType;
    fn not_i32(value:Int32ArgType)->Int32Type;
    fn and_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type;
    fn or_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type;
    fn xor_i32(arg1:&Int32ArgType,arg2:Int32ArgType)->Int32Type;
    fn and_not_i32(arg1:Int32ArgType,arg2:Int32ArgType)->Int32Type;
    fn floor(value:FloatArgType)->FloatType;
    fn ceil(value:FloatArgType)->FloatType;
    fn round(value:FloatArgType)->FloatType;
    fn truncate(value:FloatArgType) ->FloatType;
    fn min(arg1:FloatArgType,arg2:FloatArgType) ->FloatType;
    fn max(arg1:FloatArgType,arg2:FloatArgType) ->FloatType;
    fn clamp(value: FloatArgType,min:FloatArgType,max:FloatArgType) ->FloatType;
    fn min_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn max_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn clamp_i32(value:Int32ArgType,min:Int32ArgType,max:Int32ArgType) ->Int32Type;
    fn cmp_eq(arg1: FloatArgType,arg2: FloatArgType) ->FloatType;
    fn cmp_neq(arg1: FloatArgType,arg2: FloatArgType) ->FloatType;
    fn cmp_gt(arg1: FloatArgType,arg2: FloatArgType) ->FloatType;
    fn cmp_gt_eq(arg1:FloatArgType,arg2:FloatArgType) ->FloatType;
    fn cmp_lt(arg1:FloatArgType,arg2:FloatArgType) ->FloatType;
    fn cmp_lt_eq(arg1: FloatArgType,arg2: FloatArgType) ->FloatType;
    fn cmp_all_eq(arg1: FloatArgType,arg2: FloatArgType) ->bool;
    fn cmp_all_lt(arg1: FloatArgType,arg2: FloatArgType) ->bool;
    fn cmp_all_lt_eq(arg1: FloatArgType,arg2: FloatArgType) -> bool;
    fn cmp_all_gt(arg1: FloatArgType,arg2: FloatArgType) ->bool;
    fn cmp_all_gt_eq(arg1: FloatArgType,arg2: FloatArgType) ->bool;
    fn cmp_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn cmp_neq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn cmp_gt_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn cmp_gt_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn cmp_lt_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn cmp_lt_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->Int32Type;
    fn cmp_all_eq_i32(arg1:Int32ArgType,arg2:Int32ArgType) ->bool;
    fn select(arg1:FloatArgType,arg2:FloatArgType,mask:FloatArgType)->FloatType;
    fn select_i32(arg1:Int32ArgType,arg2:Int32ArgType,mask:Int32ArgType)->Int32Type;
    fn reciprocal(value: FloatArgType)->FloatType;
    fn reciprocal_estimate(value: FloatArgType)->FloatType;
    fn mod_calculate(value:FloatArgType,divisor:FloatArgType)->FloatType;
    fn  wrap(value: FloatArgType, min_value: FloatArgType, max_value: FloatArgType) ->FloatType;
    fn angle_mod(value: FloatArgType) ->FloatType;
    fn sqrt(value: FloatArgType)->FloatType;
    fn sqrt_estimate(value: FloatArgType)->FloatType;
    fn sqrt_inv(value: FloatArgType)->FloatType;
    fn sqrt_inv_estimate(value: FloatArgType) ->FloatType;
    fn sin(value: FloatArgType)->FloatType;
    fn cos(value: FloatArgType)->FloatType;
    fn sin_cos(value: FloatArgType,sin:&FloatType,cos:&FloatType);
    fn acos(value: FloatArgType)->FloatType;
    fn atan(value: FloatArgType) ->FloatType;
    fn atan2(y: FloatArgType,x: FloatArgType) ->FloatType;
    fn exp_estimate(x: FloatArgType)->FloatType;
    fn convert_to_float(value:Int32ArgType)->FloatType;
    fn convert_to_int(value: FloatArgType)->Int32Type;
    fn convert_to_int_nearest(value: FloatArgType)->Int32Type;
    fn cast_to_float(value:Int32ArgType)->FloatType;
    fn cast_to_int(value: FloatArgType)->Int32Type;
    fn zero_float() ->FloatType;
    fn zero_int() ->Int32Type;
}

pub trait VecTwoType:VecType{
    fn value_to_vec1(value: FloatArgType) ->FloatType;
    fn from_vec1(value: FloatArgType) ->FloatType;
    fn select_index1(value: FloatArgType)->f32;
    fn splat_index0(value: FloatArgType)->FloatType;
    fn splat_index1(value: FloatArgType)->FloatType;
    fn replace_index0(a: FloatArgType,b: FloatArgType)->FloatType;
    fn replace_index0_f32(value: FloatArgType,b:&f32)->FloatType;
    fn replace_index1_f32(a: FloatArgType,b:&f32)->FloatType;
    fn replace_index1(a: FloatArgType,b: FloatArgType)->FloatType;
    fn dot(arg1: FloatArgType,arg2: FloatArgType)->FloatType;
    fn normalize(value: FloatArgType)->FloatType;
    fn normalize_estimate(value: FloatArgType)->FloatType;
    fn normalize_safe(value: FloatArgType,tolerance:&f32)->FloatType;
    fn normalize_safe_estimate(value: FloatArgType,tolerance:&f32)->FloatType;
}

pub trait VecThirdType:VecTwoType{
    fn value_to_vec2(value: FloatArgType)->FloatType;
    fn from_vec2(value: FloatArgType)->FloatType;
    fn select_index2(value: FloatArgType)->f32;
    fn splat_index2(value: FloatArgType)->FloatType;
    fn replace_index2_f32(a: FloatArgType,b:&f32)->FloatType;
    fn replace_index2(a: FloatArgType,b: FloatArgType)->FloatType;

}
pub trait VecFourthType:VecThirdType{
    fn value_to_vec3(value: FloatArgType)->FloatType;
    fn from_vec3(value: FloatArgType)->FloatType;
    fn select_index3(value: FloatArgType)->f32;
    fn splat_index3(value: FloatArgType)->FloatType;
    fn replace_index3_f32(a: FloatArgType,b:&f32)->FloatType;
    fn replace_index3(a: FloatArgType,b: FloatArgType)->FloatType;
    fn quaternion_multiply(arg1: FloatArgType,arg2: FloatArgType)->FloatType;
    fn quaternion_transform(quat: FloatArgType,vec3: FloatArgType)->FloatType;
    fn construct_plane(normal: FloatArgType,point: FloatArgType)->FloatType;
    fn plane_distance(plane: FloatArgType,point: FloatArgType)->FloatType;
}

pub trait Vec1Type:VecType{
    fn load_immediate(x:&f32)->FloatType;
    fn load_immediate_i32(x:&i32)->Int32Type;
}
pub trait Vec2Type :VecTwoType{
    fn load_immediate(x:&f32,y:&f32)->FloatType;
    fn load_immediate_i32(x:&i32,y:&i32)->Int32Type;
    fn atan2_float_type(value: FloatArgType)->FloatType;
    fn sin_cos_to_float_type(angle: FloatArgType)->FloatType;

}
pub trait Vec3Type :VecThirdType{
    fn load_immediate(x:&f32,y:&f32,z:&f32)->FloatType;
    fn load_immediate_i32(x:&i32,y:&i32,z:&i32)->Int32Type;
    fn cross(arg1: FloatArgType,arg2: FloatArgType)->FloatType;
    fn mat3x3inverse(rows:*const FloatType, out :*const FloatType);
    fn mat3x3adjugate(rows:*const FloatType, out :*const FloatType);
    fn mat3x3transpose(rows:*const FloatType, out :*const FloatType);
    fn mat3x3multiply(rows_a:*const FloatType, rows_b:*const FloatType,  out:&*const FloatType);
    fn mat3x3transpose_multiply(rows_a:*const FloatType, rows_b:*const FloatType, out:&*const FloatType);
    fn mat3x3transform_vector(rows:*const FloatType,vector: FloatArgType)->FloatType;
    fn mat3x3transpose_transform_vector(rows:*const FloatType,vector: FloatArgType)->FloatType;
}
pub trait Vec4Type :VecFourthType{
    fn load_immediate(x:&f32,y:&f32,z:&f32,w:&f32)->FloatType;
    fn load_immediate_i32(x:&i32,y:&i32,z:&i32,w:&i32)->Int32Type;
    fn sin_cos_to_float_type(angles: FloatArgType)->FloatType;
    fn mat3x4inverse_fast(rows:*const FloatType, out:&*const FloatType);
    fn mat3x4transpose(rows:*const FloatType, out:&*const FloatType);
    fn mat3x4multiply(rows_a:*const FloatType, rows_b:*const FloatType,  out:&*const FloatType);
    fn mat4x4inverse_fast(rows:*const FloatType, out :&*const FloatType);
    fn mat4x4transpose(rows:*const FloatType, out :&*const FloatType);
    fn mat4x4multiply(rows_a:*const FloatType, rows_b:*const FloatType,  out:&*const FloatType);
    fn mat4x4multiply_add(rows_a:*const FloatType, rows_b:*const FloatType, add:*const FloatType, out:&*const FloatType);
    fn mat4x4transpose_multiply(rows_a:*const FloatType, rows_b:*const FloatType, out:&*const FloatType);
    fn mat4x4transform_vector(rows:*const FloatType,vector: FloatArgType)->FloatType;
    fn mat4x4_transpose_transform_vector(rows:*const FloatType,vector: FloatArgType)->FloatType;
    fn mat4x4transform_point3(rows:*const FloatType,vector: FloatArgType)->FloatType;
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
    pub fn wrap<T: VecType>(value :FloatArgType, min_value:FloatArgType, max_value:FloatArgType ) ->FloatType
    {
        let value_adjust:FloatType = T::sub(value, min_value);
        let max_adjust:FloatType = T::sub(max_value, min_value);
        let value_offset = T::select(max_value, T::zero_float(), T::cmp_lt(value_adjust, T::zero_float()));
        return  T::add(min_value,T::add(value_offset, T::mod_calculate(value_adjust, max_adjust)));
    }

    pub fn angle_mod<T:VecType>(value:FloatArgType)->FloatType{
        let vec_pi:FloatType = T::splat(PI);
        let vec_two_pi = T::splat(TWO_PI);
        let positive_angles = T::sub(T::mod_calculate(T::add(value, vec_pi), vec_two_pi), vec_pi);
        let negative_angles = T::add(T::mod_calculate(T::sub(value, vec_pi), vec_two_pi), vec_pi);
        let mask = T::cmp_gt_eq(value,T::zero_float());
        return T::select(positive_angles, negative_angles, mask);
    }

    pub fn compute_sinx_cosx<T:VecType>(x:FloatArgType,mut sinx: &FloatArgType,mut cosx:&FloatArgType){
        let x2 = T::mul(x,x);
        let x3 = T::mul(x2,x);
        sinx = T::madd(x3,
                       T::madd(x2,
                               T::madd(x2,
                                       Self::fast_load_constant(G_SIN_COEF1.as_ptr()),
                                       Self::fast_load_constant(G_SIN_COEF2.as_ptr())),
                               Self::fast_load_constant(G_SIN_COEF3.as_ptr())
                       ) ,x).borrow_mut();
        cosx = T::madd(x2 ,
                       T::madd(x2 ,
                               T::madd(x2 ,
                                       Self::fast_load_constant(G_COS_COEF1.as_ptr()) ,
                                       Self::fast_load_constant(G_COS_COEF2.as_ptr()) ) ,
                               Self::fast_load_constant(G_COS_COEF3.as_ptr())
                       ) ,T::splat(1.0 ) ).borrow_mut();
    }

    pub unsafe fn sin<T:VecType>(value:FloatArgType)->FloatType{
        let mut x = T::mul(value,Self::fast_load_constant(G_TWO_OVER_PI.as_ptr()));
        let intx =T::convert_to_int_nearest(x);
        let offset = T::and_i32(intx, T::splat_i32(3));
        let intx_float = T::convert_to_float(intx);
        x = T::sub(value,T::mul(intx_float, Self::fast_load_constant(G_HALF_PI.as_ptr())));
        let mut sinx:FloatType = Vec1::zero_float();
        let mut cosx:FloatType = Vec1::zero_float();
        Self::compute_sinx_cosx(x,sinx.borrow_mut(),cosx.borrow_mut());
        let mut mask =T::cmp_eq_i32(T::and_i32(offset,T::splat_i32(1)),T::zero_int());
        let mut result = T::select(sinx,cosx,T::cast_to_float(mask));
        mask = T::cmp_eq_i32(T::and_i32(offset,T::splat_i32(2)),T::zero_int());
        result = T::select(result,T::xor(result,T::splat(-0.0)),T::cast_to_float(mask));
        result
    }
    pub unsafe fn cos<T:VecType>(value:FloatArgType)->FloatType{
        let mut x = T::mul(value,Self::fast_load_constant(G_TWO_OVER_PI.as_ptr()));
        let intx = T::convert_to_int_nearest(x);
        let offset = T::and_i32(T::add_i32(intx, T::splat_i32(1)), T::splat_i32(3));
        let intx_float = T::convert_to_float(intx);
        x = T::sub(value,T::mul(intx_float, Self::fast_load_constant(G_HALF_PI.as_ptr())));
        let mut sinx:FloatType = Vec1::zero_float();
        let mut cosx:FloatType = Vec1::zero_float();
        Self::compute_sinx_cosx(x,sinx.borrow_mut(),cosx.borrow_mut());
        let mut mask = T::cmp_eq_i32(T::and_i32(offset,T::splat_i32(1)),T::zero_int());
        let mut result = T::select(sinx,cosx,T::cast_to_float(mask));
        mask =T::cmp_eq_i32(T::and_i32(offset,T::splat_i32(2)),T::zero_int());
        result = T::select(result,T::xor(result,T::splat(-0.0)),T::cast_to_float(mask));
        result
    }

    pub unsafe  fn sin_cos<T:VecType>(value:FloatArgType,mut sin: &FloatArgType,mut cos: &FloatArgType){
        let mut x = T::mul(value,Self::fast_load_constant(G_TWO_OVER_PI.as_ptr()));
        let intx = T::convert_to_int_nearest(x);
        let offset_sin = T::and_i32(intx, T::splat_i32(3));
        let offset_cos = T::and_i32(T::add_i32(intx, T::splat_i32(1)), T::splat_i32(3));
        let intx_float = T::convert_to_float(intx);
        x = T::sub(value,T::mul(intx_float, Self::fast_load_constant(G_HALF_PI.as_ptr())));
        let mut sinx:FloatType = Vec1::zero_float();
        let mut cosx:FloatType = Vec1::zero_float();
        Self::compute_sinx_cosx(x,sinx.borrow_mut(),cosx.borrow_mut());
        let mut sin_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_sin, T::splat_i32(1)), T::zero_int()));
        let mut cos_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_cos, T::splat_i32(1)), T::zero_int()));
        sin = T::select(sinx, cosx, sin_mask).borrow_mut();
        cos = T::select(sinx, cosx, cos_mask).borrow_mut();
        sin_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_sin, T::splat_i32(2)), T::zero_int()));
        cos_mask = T::cast_to_float(T::cmp_eq_i32(T::and_i32(offset_cos, T::splat_i32(2)), T::zero_int()));
        sin = T::select(sin.to_owned(),T::xor(sin.to_owned(),Self::fast_load_constant(G_NEGATE_MASK.as_ptr() as *const f32)),sin_mask).borrow_mut();
        cos = T::select(cos.to_owned(),T::xor(cos.to_owned(),Self::fast_load_constant(G_NEGATE_MASK.as_ptr() as *const f32)),cos_mask).borrow_mut();
    }

    pub fn sin_cos_to_float_type<T:VecType>(angles:FloatArgType)->FloatType{
        let angle_offset = T::load_immediate_fourth_f32(0.0, HALF_PI, 0.0, HALF_PI);
        let sin_angles = T::add(angles, angle_offset);
        return  T::sin(sin_angles);
    }

    pub fn acos<T:VecType>(value:FloatArgType)->FloatType{
        let xabs = T::abs(value);
        let xabs2 = T::mul(xabs,xabs);
        let xabs4 = T::mul(xabs2,xabs2);
        let t1 = T::sqrt(T::sub(T::splat(1.0),xabs));
        let select = T::cmp_lt(value.to_owned(),T::zero_float());

        let hi = T::madd(xabs,
                                    T::madd(xabs,
                                            T::madd(xabs,
                                                    Self::fast_load_constant(G_ACOS_HI_COEF1.as_ptr()),
                                                    Self::fast_load_constant(G_ACOS_HI_COEF2.as_ptr())),
                                            Self::fast_load_constant(G_ACOS_HI_COEF3.as_ptr())),
                                    Self::fast_load_constant(G_ACOS_HI_COEF4.as_ptr()));

        let lo = T::madd(xabs,
                                    T::madd(xabs,
                                            T::madd(xabs,
                                                    Self::fast_load_constant_f32(G_ACOS_LO_COEF1.as_ptr()),
                                                    Self::fast_load_constant_f32(G_ACOS_LO_COEF2.as_ptr())),
                                            Self::fast_load_constant_f32(G_ACOS_LO_COEF3.as_ptr())),
                                    Self::fast_load_constant_f32(G_ACOS_LO_COEF4.as_ptr()));

        let result = T::madd(hi,xabs4,lo);
        let positive = T::mul(t1,result);
        let negative = T::sub(T::splat(PI),positive);
        return T::select(negative,positive,select);

    }

    pub fn acos_estimate<T:VecType>(value:FloatArgType)->FloatType{
        let xabs = T::abs(value);
        let t1 = T::sqrt_estimate(T::sub(T::splat(1.0),xabs));
        let select = T::cmp_lt(value,T::zero_float());
        let result = T::madd(xabs ,
                                        T::madd(xabs ,
                                                T::madd(xabs ,
                                                        Self::fast_load_constant(G_ACOS_COEF1.as_ptr()) ,
                                                        Self::fast_load_constant(G_ACOS_COEF2.as_ptr()) ) ,
                                                Self::fast_load_constant(G_COS_COEF3.as_ptr()) ) ,
                                        Self::fast_load_constant(G_HALF_PI.as_ptr()) );
        let positive = T::mul(t1 ,result );
        let negative = T::sub(T::splat(PI ) ,positive );
        return T::select(negative ,positive ,select );
    }

    pub fn atan<T:VecType>(value: FloatArgType)->FloatType
    {
        let mut x = value.to_owned();
        let signbit = T::and(x, T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.as_ptr())));

        let xabs = T::abs(x);
        let cmp0 = T::cmp_gt(xabs,Self::fast_load_constant(G_ATAN_HI_RANGE.as_ptr()));
        let mut cmp1 = T::cmp_gt(xabs,Self::fast_load_constant(G_ATAN_LO_RANGE.as_ptr()));
        let cmp2 = T::and_not(cmp0,cmp1);

        let mut xabs_safe = T::add(xabs, T::and(T::cmp_eq(xabs, T::zero_float()), Self::fast_load_constant(G_VEC1111.as_ptr())));
        let y0 = T::and(cmp0,Self::fast_load_constant(G_HALF_PI.as_ptr()));
        let mut x0 = T::div(Self::fast_load_constant(G_VEC1111.as_ptr()), xabs_safe.borrow_mut());
        x0 = T::xor(x0,T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.as_ptr())));
        let y1 = T::and(cmp2,Self::fast_load_constant(G_QUARTER_PI.as_ptr()));
        let x1_numer = T::sub(xabs,Self::fast_load_constant(G_VEC1111.as_ptr()));
        let mut x1_denom = T::add(xabs,Self::fast_load_constant(G_VEC1111.as_ptr()));
        let x1 = T::div(x1_numer,x1_denom.borrow_mut());
        let mut x2 = T::and(cmp2,x1);
        x0 = T::and(cmp0,x0);
        x2 = T::or(x2,x0);
        cmp1 = T::or(cmp0,cmp2);
        x2 = T::and(cmp1,x2);
        x = T::and_not(cmp1, xabs);
        x = T::or(x2,x);
        let mut y = T::or(y0,y1);
        let x_sqr = T::mul(x,x);
        let x_cub = T::mul(x_sqr,x);


        let result = T::madd(x_cub,
                                        T::madd(x_sqr,
                                                T::madd(x_sqr,
                                                        T::madd(x_sqr,
                                                                Self::fast_load_constant(G_ATAN_COEF1.as_ptr()),
                                                                Self::fast_load_constant(G_ATAN_COEF2.as_ptr())),
                                                        Self::fast_load_constant(G_ATAN_COEF3.as_ptr())),
                                                Self::fast_load_constant(G_ATAN_COEF4.as_ptr())),
                                        x);
        y = T::add(y,result);
        y = T::xor(y, signbit);
        y
    }

    pub fn atan2<T:VecType>(y: FloatArgType,x: FloatArgType)->FloatType
    {
        let x_eq_0 = T::cmp_eq(x,T::zero_float());
        let x_ge_0 = T::cmp_gt_eq(x,T::zero_float());
        let x_lt_0 = T::cmp_lt(x,T::zero_float());

        let y_eq_0 = T::cmp_eq(y,T::zero_float());
        let y_lt_0 = T::cmp_lt(y,T::zero_float());

        let zero_mask = T::and(x_ge_0,y_eq_0);
        let pio2_mask = T::and_not(y_eq_0,x_eq_0);
        let pio2_mask_sign = T::and(y_lt_0,T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.borrow())));
        let mut pio2_result = Self::fast_load_constant(G_HALF_PI.borrow());
        pio2_result = T::xor(pio2_result,pio2_mask_sign);
        pio2_result = T::and(pio2_mask, pio2_result);

        let pi_mask = T::and(y_eq_0,x_lt_0);
        let mut pi_result = Self::fast_load_constant(G_PI.borrow());
        pi_result = T::and(pi_mask,pi_result);
        let mut swap_sign_mask_offset = T::and(x_lt_0,y_lt_0);
        swap_sign_mask_offset = T::and(swap_sign_mask_offset,T::cast_to_float(Self::fast_load_constant_i32(G_NEGATE_MASK.borrow())));

        let mut offset1 = Self::fast_load_constant(G_PI.borrow());
        offset1 = T::xor(offset1,swap_sign_mask_offset);

        let offset = T::and(x_lt_0,offset1);

        let mut x_safe = T::add(x, T::and(x_eq_0, Self::fast_load_constant(G_VEC1111.borrow())));
        let atan_mask = T::not(T::or(x_eq_0,y_eq_0));
        let atan_arg = T::div(y, x_safe.borrow_mut());
        let mut atan_result = T::atan(atan_arg);
        atan_result = T::add(atan_result,offset);
        atan_result = T::and_not(pio2_mask,atan_result);
        atan_result = T::and(atan_mask,atan_result);

        let mut result = T::and_not(zero_mask,pio2_result);
        result = T::or(result,pio2_result);
        result = T::or(result,pi_result);
        result = T::or(result,atan_result);

        result
    }

    pub fn exp_estimate<T:VecType>(x: FloatArgType)->FloatType{
        let a = T::convert_to_int_nearest(T::mul(Self::fast_load_constant(G_EXP_COEF1.borrow()),x));
        let b = T::and_i32(a,Self::fast_load_constant_i32(G_EXP_COEF2.borrow()));
        let c = T::sub_i32(a,b);
        let f = T::mul(Self::fast_load_constant(G_EXP_COEF3.borrow()),T::convert_to_float(c));
        let i = T::madd(f,Self::fast_load_constant(G_EXP_COEF4.borrow()),Self::fast_load_constant(G_EXP_COEF5.borrow()));
        let j = T::madd(i,f,Self::fast_load_constant(G_EXP_COEF6.borrow()));
        return T::cast_to_float(T::add_i32(b,T::cast_to_int(j)));
    }

    pub fn normalize<T:VecType>(value: FloatArgType)->FloatType{
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value)));
        let mut length = T::sqrt(length_squared);
        return  T::div(value,length.borrow_mut());
    }

    pub fn normalize_estimate<T:VecType>(value: FloatArgType)->FloatType{
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value)));
        let inv_length = T::sqrt_inv_estimate(length_squared);
        return  T::mul(inv_length, value);
    }

    pub fn normalize_safe<T:VecType>(value: FloatArgType,tolerance:f32)->FloatType{
        let float_epsilon = T::splat((tolerance*tolerance));
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value)));
        if T::cmp_all_lt(length_squared, float_epsilon){
            return T::zero_float();
        }else {
            return T::div(value,T::sqrt(length_squared).borrow_mut());
        }
    }

    pub fn normalize_safe_estimate<T:VecType>(value: FloatArgType,tolerance:f32) ->FloatType{
        let float_epsilon = T::splat((tolerance*tolerance));
        let length_squared = T::splat_index0(T::from_vec1(T::dot(value, value)));
        if T::cmp_all_lt(length_squared, float_epsilon){
            return T::zero_float();
        }else {
            return T::mul(value,T::sqrt_inv_estimate(length_squared));
        }
    }

    pub fn quaternion_transform<T:VecType>(quat: FloatArgType,vec3: FloatArgType) ->FloatType{
        let two = T::splat(2.0);
        let scalar = unsafe { T::splat_index3(quat) };
        let partial1 = T::splat_index0(T::from_vec1(T::dot(quat,vec3)));
        let partial2 = T::mul(quat,partial1);
        let sum1 = T::mul(partial2,two);
        let partial3 = T::splat_index0(T::from_vec1(T::dot(quat,quat)));
        let partial4 = T::mul(scalar,scalar);
        let partial5 = T::sub(partial4,partial3);
        let sum2 = T::mul(partial5,vec3);
        let partial6 = T::mul(scalar,two);
        let partial7 = T::cross(quat,vec3);
        let sum3 = T::mul(partial6,partial7);
        return T::add(T::add(sum1,sum2),sum3);
    }

    pub fn construct_plane<T:VecType>(normal: FloatArgType,point: FloatArgType)->FloatType{
        let distance = unsafe { Vec1::sub(Vec1::zero_float(), T::dot(normal,point)) };
        unsafe { return T::replace_index3(normal, T::splat_index0(T::from_vec1(distance))); }
    }

    pub fn plane_distance<T:VecType>(plane: FloatArgType, point: FloatArgType) ->FloatType{
        let reference_point = unsafe { T::replace_index3_f32(point, 1.0) };
        return T::dot(reference_point, plane);
    }

    pub unsafe fn mat3x3multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        *out[0] = T::madd(T::splat_index2(*rows_a[0]), *rows_b[2], T::madd(T::splat_index1(rows_a[0]), rows_b[1], T::mul(T::splat_index0(*rows_a[0]), *rows_b[0])) );
        *out[0] = T::madd(T::splat_index2(*rows_a[1]), *rows_b[2], T::madd(T::splat_index1(rows_a[1]), rows_b[1], T::mul(T::splat_index0(*rows_a[1]), *rows_b[0])));
        *out[0] = T::madd(T::splat_index2(*rows_a[2]), *rows_b[2], T::madd(T::splat_index1(rows_a[2]), rows_b[1], T::mul(T::splat_index0(*rows_a[2]), *rows_b[0])));
    }

    pub unsafe fn mat3x3transpose_multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        *out[0] = T::madd(T::splat_index0(*rows_a[0]), *rows_b[0], T::madd(T::splat_index0(rows_a[1]), rows_b[1], T::mul(T::splat_index0(*rows_a[2]), *rows_b[2])) );
        *out[0] = T::madd(T::splat_index1(*rows_a[0]), *rows_b[0], T::madd(T::splat_index2(rows_a[1]), rows_b[1], T::mul(T::splat_index0(*rows_a[2]), *rows_b[2])));
        *out[0] = T::madd(T::splat_index2(*rows_a[0]), *rows_b[0], T::madd(T::splat_index3(rows_a[1]), rows_b[1], T::mul(T::splat_index0(*rows_a[2]), *rows_b[2])));

    }

    pub unsafe fn mat3x3transform_vector<T:VecType>(rows:*const FloatType,vector: FloatArgType)->FloatType{
        let mut transposed:[FloatType;3] = [Vec1Type::zero_float(),Vec1Type::zero_float(),Vec1Type::zero_float()];
        VecType::mat3x3transpose(rows,transposed.borrow_mut());
        return VecType::mat3x3transpose_transform_vector(transposed,vector);
    }

    pub unsafe fn mat3x3transpose_transform_vector<T:VecType>(rows:*const FloatType,vector: FloatArgType)->FloatType{
        return  T::madd(T::splat_index2(vector),*rows[2],T::madd(T::splat_index1(vector),*rows[1],T::mul(T::splat_index0(vector),*rows[0])));
    }

    pub unsafe fn mat3x4multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        let fourth = Self::fast_load_constant(G_VEC1111.borrow());
        *out[0] = T::madd(T::splat_index3(*rows_a[0]), fourth, T::madd(T::splat_index2(*rows_a[0]), *rows_b[2], T::madd(T::splat_index1(*rows_a[0]), *rows_b[1], T::mul(T::splat_index0(*rows_a[0]), *rows_b[0]))));
        *out[1] = T::madd(T::splat_index3(*rows_a[0]), fourth, T::madd(T::splat_index2(*rows_a[1]), *rows_b[2], T::madd(T::splat_index1(*rows_a[1]), *rows_b[1], T::mul(T::splat_index0(*rows_a[1]), *rows_b[0]))));
        *out[2] = T::madd(T::splat_index3(*rows_a[0]), fourth, T::madd(T::splat_index2(*rows_a[2]), *rows_b[2], T::madd(T::splat_index1(*rows_a[2]), *rows_b[1], T::mul(T::splat_index0(*rows_a[2]), *rows_b[0]))));
    }

    pub unsafe fn mat4x4inverse_fast<T:VecType>(rows:*const FloatType,mut out :&*const FloatType){
        let pos = T::madd(T::splat_index3(*rows[2]),
                          *rows[2], T::madd(T::splat_index3(*rows[1]),*rows[1],
                                            T::mul(T::splat_index3(*rows[0]),*rows[0])));
        let mut transposed : [FloatType;4] = [*rows[0],*rows[1],*rows[2],T::xor(pos,Self::fast_load_constant(G_NEGATE_MASK.borrow()))];
        T::mat3x3transpose(transposed.borrow_mut(),out);
        *out[3] = Self::fast_load_constant(G_VEC1111.borrow());
    }

    pub unsafe fn mat4x4multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        *out[0] = T::madd(T::splat_index3(*rows_a[0]), *rows_b[3], T::madd(T::splat_index2(*rows_a[0]), *rows_b[2], T::madd(T::splat_index1(*rows_a[0]), *rows_b[1], T::mul(T::splat_index0(rows_a[0]), rows_b[0]))));
        *out[1] = T::madd(T::splat_index3(*rows_a[1]), *rows_b[3], T::madd(T::splat_index2(*rows_a[1]), *rows_b[2], T::madd(T::splat_index1(*rows_a[1]), *rows_b[1], T::mul(T::splat_index0(rows_a[1]), rows_b[0]))));
        *out[2] = T::madd(T::splat_index3(*rows_a[2]), *rows_b[3], T::madd(T::splat_index2(*rows_a[2]), *rows_b[2], T::madd(T::splat_index1(*rows_a[2]), *rows_b[1], T::mul(T::splat_index0(rows_a[2]), rows_b[0]))));
        *out[3] = T::madd(T::splat_index3(*rows_a[3]), *rows_b[3], T::madd(T::splat_index2(*rows_a[3]), *rows_b[2], T::madd(T::splat_index1(*rows_a[3]), *rows_b[1], T::mul(T::splat_index0(rows_a[3]), rows_b[0]))));
    }

    pub unsafe fn mat4x4multiply_add<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, add:*const FloatType, mut out:&*const FloatType){
        *out[0] = T::madd(T::splat_index3(*rows_a[0]), *rows_b[3], T::madd(T::splat_index2(*rows_a[0]), *rows_b[2], T::madd(T::splat_index1(*rows_a[0]), *rows_b[1], T::madd(T::splat_index0(*rows_a[0]), *rows_b[0], *add[0]))));
        *out[1] = T::madd(T::splat_index3(*rows_a[1]), *rows_b[3], T::madd(T::splat_index2(*rows_a[1]), *rows_b[2], T::madd(T::splat_index1(*rows_a[1]), *rows_b[1], T::madd(T::splat_index0(*rows_a[1]), *rows_b[0], *add[1]))));
        *out[2] = T::madd(T::splat_index3(*rows_a[2]), *rows_b[3], T::madd(T::splat_index2(*rows_a[2]), *rows_b[2], T::madd(T::splat_index1(*rows_a[2]), *rows_b[1], T::madd(T::splat_index0(*rows_a[2]), *rows_b[0], *add[2]))));
        *out[3] = T::madd(T::splat_index3(*rows_a[3]), *rows_b[3], T::madd(T::splat_index2(*rows_a[3]), *rows_b[2], T::madd(T::splat_index1(*rows_a[3]), *rows_b[1], T::madd(T::splat_index0(*rows_a[3]), *rows_b[0], *add[3]))));
    }

    pub unsafe fn mat4x4transpose_multiply<T:VecType>(rows_a:*const FloatType, rows_b:*const FloatType, mut out:&*const FloatType){
        *out[0] = T::madd(T::splat_index0(*rows_a[0]), *rows_b[0], T::madd(T::splat_index0(*rows_a[1]), *rows_b[1], T::madd(T::splat_index0(*rows_a[2]), *rows_b[2], T::mul(T::splat_index0(*rows_a[3]), *rows_b[3]))));
        *out[1] = T::madd(T::splat_index1(*rows_a[0]), *rows_b[0], T::madd(T::splat_index1(*rows_a[1]), *rows_b[1], T::madd(T::splat_index1(*rows_a[2]), *rows_b[2], T::mul(T::splat_index1(*rows_a[3]), *rows_b[3]))));
        *out[2] = T::madd(T::splat_index2(*rows_a[0]), *rows_b[0], T::madd(T::splat_index2(*rows_a[1]), *rows_b[1], T::madd(T::splat_index2(*rows_a[2]), *rows_b[2], T::mul(T::splat_index2(*rows_a[3]), *rows_b[3]))));
        *out[3] = T::madd(T::splat_index3(*rows_a[0]), *rows_b[0], T::madd(T::splat_index3(*rows_a[1]), *rows_b[1], T::madd(T::splat_index3(*rows_a[2]), *rows_b[2], T::mul(T::splat_index3(*rows_a[3]), *rows_b[3]))));
    }

    pub unsafe fn mat4x4transpose_transform_vector<T:VecType>(rows:*const FloatType,vector:FloatArgType)->FloatType{
        return T::madd(T::splat_index3(vector), *rows[3], T::madd(T::splat_index2(vector), *rows[2], T::madd(T::splat_index1(vector), *rows[1], T::mul(T::splat_index0(vector), *rows[0]))));
    }
}
