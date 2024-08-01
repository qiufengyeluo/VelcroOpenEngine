#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::constants::*;
use crate::math::vsimd::{FloatArgType, FloatType, Int32ArgType, Int32Type};

trait Vec {
    fn add(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType;
    fn sub(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType;
    fn mul(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType;
    fn madd(mul1:&FloatArgType,mul2:&FloatArgType,add:&FloatArgType)->FloatType;
    fn add_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type
    fn and_i32(arg1:&Int32ArgType,arg2:&Int32ArgType)->Int32Type;
    fn splat_i32(value:&i32)->Int32Type;
    fn select(arg1:&FloatArgType,arg2:&FloatArgType,mask:&FloatArgType)->FloatType;
    fn splat(value:&f32)->FloatType;

    fn mod_calculate(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType;
    fn cmp_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_lt(arg1: &FloatArgType, arg2: &FloatArgType) -> FloatType;
    fn cmp_gt_eq(arg1:&FloatArgType,arg2:&FloatArgType) ->FloatType;
    fn cmp_eq_i32(arg1:&Int32ArgType,arg2:&Int32ArgType) ->Int32Type;
    fn convert_to_float(value:&Int32ArgType)->FloatType;
    fn convert_to_int(value:&FloatArgType)->Int32Type;
    fn convert_to_int_nearest(value:&FloatArgType)->Int32Type;
    fn cast_to_float(value:&Int32ArgType)->FloatType;
    fn zero_float()->FloatType;
    fn zero_int() ->Int32Type;
}
pub struct Common{
}
impl  Common{

    pub fn fast_load_constant<T:Vec>(values:*const f32)->FloatType{
        unsafe { return *(values as * FloatType); }
    }

    pub fn fast_load_constant_i32<T:Vec>(values:*const f32)->Int32Type{
        unsafe { return *(values as * Int32Type); }
    }
    pub fn wrap<T: Vec>(value :&FloatArgType, min_value:&FloatArgType, max_value:&FloatArgType ) ->FloatType
    {
        let value_adjust:FloatType = T::sub(value, min_value);
        let max_adjust:FloatType = T::sub(max_value, min_value);
        let value_offset = T::select(max_value, T::zero_float().borrow(), T::cmp_lt(value_adjust.borrow(), T::zero_float().borrow()).borrow());
        return  T::add(min_value,T::add(value_offset.borrow(), T::mod_calculate(value_adjust.borrow(), max_adjust.borrow()).borrow()).borrow());
    }

    pub fn angle_mod<T:Vec>(value:&FloatArgType)->FloatType{
        let vec_pi:FloatType = T::splat(PI.borrow());
        let vec_two_pi = T::splat(TWO_PI.borrow());
        let positive_angles = T::sub(T::mod_calculate(T::add(value, vec_pi.borrow()).borrow(), vec_two_pi.borrow()).borrow(), vec_pi.borrow());
        let negative_angles = T::add(T::mod_calculate(T::sub(value, vec_pi.borrow()).borrow(), vec_two_pi.borrow()).borrow(), vec_pi.borrow());
        let mask = T::cmp_gt_eq(value,T::zero_float().borrow());
        return T::select(positive_angles.borrow(), negative_angles.borrow(), mask.borrow());
    }

    pub fn compute_sinx_cosx<T:Vec>(x:&FloatArgType,mut sinx: &FloatArgType,mut cosx: &FloatArgType){
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

    pub unsafe fn sin<T:Vec>(value:&FloatArgType)->FloatType{
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
        result = T::select(result.borrow(),T::xor(result.borrow(),T::splat(-0.0.borrow()).borrow()),T::cast_to_float(mask.borrow()).borrow());
        result
    }
    pub unsafe fn cos<T:Vec>(value:&FloatArgType)->FloatType{
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
        result = T::select(result.borrow(),T::xor(result.borrow(),T::splat(-0.0.borrow()).borrow()),T::cast_to_float(mask.borrow()).borrow());
        result
    }


    template <typename VecType>
    AZ_MATH_INLINE void SinCos(typename VecType::FloatArgType value, typename VecType::FloatArgType& sin, typename VecType::FloatArgType& cos)
    {
    // Range Reduction
    typename VecType::FloatType x = VecType::Mul(value, FastLoadConstant<VecType>(Simd::g_TwoOverPi));

    // Find offset mod 4
    typename VecType::Int32Type intx = VecType::ConvertToIntNearest(x);
    typename VecType::Int32Type offsetSin = VecType::And(intx, VecType::Splat(3));
    typename VecType::Int32Type offsetCos = VecType::And(VecType::Add(intx, VecType::Splat(1)), VecType::Splat(3));

    typename VecType::FloatType intxFloat = VecType::ConvertToFloat(intx);
    x = VecType::Sub(value, VecType::Mul(intxFloat, FastLoadConstant<VecType>(Simd::g_HalfPi)));

    typename VecType::FloatType sinx, cosx;
    ComputeSinxCosx<VecType>(x, sinx, cosx);

    // Choose sin for even offset, cos for odd
    typename VecType::FloatType sinMask = VecType::CastToFloat(VecType::CmpEq(VecType::And(offsetSin, VecType::Splat(1)), VecType::ZeroInt()));
    typename VecType::FloatType cosMask = VecType::CastToFloat(VecType::CmpEq(VecType::And(offsetCos, VecType::Splat(1)), VecType::ZeroInt()));

    sin = VecType::Select(sinx, cosx, sinMask);
    cos = VecType::Select(sinx, cosx, cosMask);

    // Change sign for result if offset puts it in quadrant 1 or 2
    sinMask = VecType::CastToFloat(VecType::CmpEq(VecType::And(offsetSin, VecType::Splat(2)), VecType::ZeroInt()));
    cosMask = VecType::CastToFloat(VecType::CmpEq(VecType::And(offsetCos, VecType::Splat(2)), VecType::ZeroInt()));

    sin = VecType::Select(sin, VecType::Xor(sin, FastLoadConstant<VecType>(reinterpret_cast<const float*>(Simd::g_negateMask))), sinMask);
    cos = VecType::Select(cos, VecType::Xor(cos, FastLoadConstant<VecType>(reinterpret_cast<const float*>(Simd::g_negateMask))), cosMask);
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType SinCos(typename VecType::FloatArgType angles)
    {
    const typename VecType::FloatType angleOffset = VecType::LoadImmediate(0.0f, Constants::HalfPi, 0.0f, Constants::HalfPi);
    const typename VecType::FloatType sinAngles = VecType::Add(angles, angleOffset);
    return VecType::Sin(sinAngles);
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType Acos(typename VecType::FloatArgType value)
    {
    const typename VecType::FloatType xabs = VecType::Abs(value);
    const typename VecType::FloatType xabs2 = VecType::Mul(xabs, xabs);
    const typename VecType::FloatType xabs4 = VecType::Mul(xabs2, xabs2);
    const typename VecType::FloatType t1 = VecType::Sqrt(VecType::Sub(VecType::Splat(1.0f), xabs));

    const typename VecType::FloatType select = VecType::CmpLt(value, VecType::ZeroFloat());

    const typename VecType::FloatType hi = VecType::Madd(
    xabs,
    VecType::Madd(
    xabs,
    Vec1::Madd(
    xabs, FastLoadConstant<VecType>(g_acosHiCoef1), FastLoadConstant<VecType>(g_acosHiCoef2)
    ),
    FastLoadConstant<VecType>(g_acosHiCoef3)
    ),
    FastLoadConstant<VecType>(g_acosHiCoef4)
    );

    const typename VecType::FloatType lo = VecType::Madd(
    xabs,
    VecType::Madd(
    xabs,
    VecType::Madd(
    xabs, FastLoadConstant<VecType>(g_acosLoCoef1), FastLoadConstant<VecType>(g_acosLoCoef2)
    ),
    FastLoadConstant<VecType>(g_acosLoCoef3)
    ),
    FastLoadConstant<VecType>(g_acosLoCoef4)
    );

    const typename VecType::FloatType result = VecType::Madd(hi, xabs4, lo);

    const typename VecType::FloatType positive = VecType::Mul(t1, result);
    const typename VecType::FloatType negative = VecType::Sub(VecType::Splat(Constants::Pi), positive);

    return VecType::Select(negative, positive, select);
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType AcosEstimate(typename VecType::FloatArgType value)
    {
    const typename VecType::FloatType xabs = VecType::Abs(value);
    const typename VecType::FloatType t1 = VecType::SqrtEstimate(VecType::Sub(VecType::Splat(1.0f), xabs));

    const typename VecType::FloatType select = VecType::CmpLt(value, VecType::ZeroFloat());

    const typename VecType::FloatType result = VecType::Madd(
    xabs,
    VecType::Madd(
    xabs,
    VecType::Madd(
    xabs, FastLoadConstant<VecType>(g_acosCoef1), FastLoadConstant<VecType>(g_acosCoef2)
    ),
    FastLoadConstant<VecType>(g_acosCoef3)
    ),
    FastLoadConstant<VecType>(g_HalfPi)
    );

    const typename VecType::FloatType positive = VecType::Mul(t1, result);
    const typename VecType::FloatType negative = VecType::Sub(VecType::Splat(Constants::Pi), positive);

    return VecType::Select(negative, positive, select);
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType Atan(typename VecType::FloatArgType value)
    {
    typename VecType::FloatType x = value;
    const typename VecType::FloatType signbit = VecType::And(x, VecType::CastToFloat(FastLoadConstant<VecType>(Simd::g_negateMask)));

    const typename VecType::FloatType xabs = VecType::Abs(x);

    // Test for x > Sqrt(2) + 1
    const typename VecType::FloatType cmp0 = VecType::CmpGt(xabs, FastLoadConstant<VecType>(Simd::g_atanHiRange));
    // Test for x > Sqrt(2) - 1
    typename VecType::FloatType cmp1 = VecType::CmpGt(xabs, FastLoadConstant<VecType>(Simd::g_atanLoRange));
    // Test for Sqrt(2) + 1 >= x > Sqrt(2) - 1
    const typename VecType::FloatType cmp2 = VecType::AndNot(cmp0, cmp1);

    // -1/x
    // this step is calculated for all values of x, but only used if x > Sqrt(2) + 1
    // in order to avoid a division by zero, detect if xabs is zero here and replace it with an arbitrary value
    // if xabs does equal zero, the value here doesn't matter because the result will be thrown away
    typename VecType::FloatType xabsSafe =
    VecType::Add(xabs, VecType::And(VecType::CmpEq(xabs, VecType::ZeroFloat()), FastLoadConstant<VecType>(Simd::g_vec1111)));
    const typename VecType::FloatType y0 = VecType::And(cmp0, FastLoadConstant<VecType>(Simd::g_HalfPi));
    typename VecType::FloatType x0 = VecType::Div(FastLoadConstant<VecType>(Simd::g_vec1111), xabsSafe);
    x0 = VecType::Xor(x0, VecType::CastToFloat(FastLoadConstant<VecType>(Simd::g_negateMask)));

    const typename VecType::FloatType y1 = VecType::And(cmp2, FastLoadConstant<VecType>(Simd::g_QuarterPi));
    // (x-1)/(x+1)
    const typename VecType::FloatType x1_numer = VecType::Sub(xabs, FastLoadConstant<VecType>(Simd::g_vec1111));
    const typename VecType::FloatType x1_denom = VecType::Add(xabs, FastLoadConstant<VecType>(Simd::g_vec1111));
    const typename VecType::FloatType x1 = VecType::Div(x1_numer, x1_denom);

    typename VecType::FloatType x2 = VecType::And(cmp2, x1);
    x0 = VecType::And(cmp0, x0);
    x2 = VecType::Or(x2, x0);
    cmp1 = VecType::Or(cmp0, cmp2);
    x2 = VecType::And(cmp1, x2);
    x = VecType::AndNot(cmp1, xabs);
    x = VecType::Or(x2, x);

    typename VecType::FloatType y = VecType::Or(y0, y1);

    typename VecType::FloatType x_sqr = VecType::Mul(x, x);
    typename VecType::FloatType x_cub = VecType::Mul(x_sqr, x);

    typename VecType::FloatType result = VecType::Madd(x_cub,
    VecType::Madd(x_sqr,
    VecType::Madd(x_sqr,
    VecType::Madd(x_sqr,
    FastLoadConstant<VecType>(Simd::g_atanCoef1),
    FastLoadConstant<VecType>(Simd::g_atanCoef2)),
    FastLoadConstant<VecType>(Simd::g_atanCoef3)),
    FastLoadConstant<VecType>(Simd::g_atanCoef4)),
    x);

    y = VecType::Add(y, result);

    y = VecType::Xor(y, signbit);

    return y;
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType Atan2(typename VecType::FloatArgType y, typename VecType::FloatArgType x)
    {
    const typename VecType::FloatType x_eq_0 = VecType::CmpEq(x, VecType::ZeroFloat());
    const typename VecType::FloatType x_ge_0 = VecType::CmpGtEq(x, VecType::ZeroFloat());
    const typename VecType::FloatType x_lt_0 = VecType::CmpLt(x, VecType::ZeroFloat());

    const typename VecType::FloatType y_eq_0 = VecType::CmpEq(y, VecType::ZeroFloat());
    const typename VecType::FloatType y_lt_0 = VecType::CmpLt(y, VecType::ZeroFloat());

    // returns 0 if x == y == 0 (degenerate case) or x >= 0, y == 0
    const typename VecType::FloatType zero_mask = VecType::And(x_ge_0, y_eq_0);

    // returns +/- pi/2 if x == 0, y != 0
    const typename VecType::FloatType pio2_mask = VecType::AndNot(y_eq_0, x_eq_0);
    const typename VecType::FloatType pio2_mask_sign = VecType::And(y_lt_0, VecType::CastToFloat(FastLoadConstant<VecType>(Simd::g_negateMask)));
    typename VecType::FloatType pio2_result = FastLoadConstant<VecType>(g_HalfPi);
    pio2_result = VecType::Xor(pio2_result, pio2_mask_sign);
    pio2_result = VecType::And(pio2_mask, pio2_result);

    // pi when y == 0 and x < 0
    const typename VecType::FloatType pi_mask = VecType::And(y_eq_0, x_lt_0);
    typename VecType::FloatType pi_result = FastLoadConstant<VecType>(g_Pi);
    pi_result = VecType::And(pi_mask, pi_result);

    typename VecType::FloatType swap_sign_mask_offset = VecType::And(x_lt_0, y_lt_0);
    swap_sign_mask_offset = VecType::And(swap_sign_mask_offset, VecType::CastToFloat(FastLoadConstant<VecType>(Simd::g_negateMask)));

    typename VecType::FloatType offset1 = FastLoadConstant<VecType>(g_Pi);
    offset1 = VecType::Xor(offset1, swap_sign_mask_offset);

    typename VecType::FloatType offset = VecType::And(x_lt_0, offset1);

    // the result of this part of the computation is thrown away if x equals 0,
    // but if x does equal 0, it will cause a division by zero
    // so replace zero by an arbitrary value here in that case
    typename VecType::FloatType xSafe = VecType::Add(x, VecType::And(x_eq_0, FastLoadConstant<VecType>(Simd::g_vec1111)));
    const typename VecType::FloatType atan_mask = VecType::Not(VecType::Or(x_eq_0, y_eq_0));
    const typename VecType::FloatType atan_arg = VecType::Div(y, xSafe);
    typename VecType::FloatType atan_result = VecType::Atan(atan_arg);
    atan_result = VecType::Add(atan_result, offset);
    atan_result = VecType::AndNot(pio2_mask, atan_result);
    atan_result = VecType::And(atan_mask, atan_result);

    // Select between zero, pio2, pi, and atan
    typename VecType::FloatType result = VecType::AndNot(zero_mask, pio2_result);
    result = VecType::Or(result, pio2_result);
    result = VecType::Or(result, pi_result);
    result = VecType::Or(result, atan_result);

    return result;
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType ExpEstimate(typename VecType::FloatArgType x)
    {
    // N. N. Schraudolph, 'A Fast, Compact Approximation of the Exponential Function'
    // This method exploits the logrithmic nature of IEEE-754 floating point to quickly estimate exp(x)
    // While the concept is based on that paper, this specific implementation is based on a selection from several variants
    // of that algorithm to choose the fastest of the variants that had the highest accuracy.
    typename VecType::Int32Type a = VecType::ConvertToIntNearest(VecType::Mul(FastLoadConstant<VecType>(Simd::g_expCoef1), x));
    typename VecType::Int32Type b = VecType::And(a, FastLoadConstant<VecType>(Simd::g_expCoef2));
    typename VecType::Int32Type c = VecType::Sub(a, b);
    typename VecType::FloatType f = VecType::Mul(FastLoadConstant<VecType>(Simd::g_expCoef3), VecType::ConvertToFloat(c)); // Approximately (x/log(2)) - floor(x/log(2))
    typename VecType::FloatType i = VecType::Madd(f, FastLoadConstant<VecType>(Simd::g_expCoef4), FastLoadConstant<VecType>(Simd::g_expCoef5));
    typename VecType::FloatType j = VecType::Madd(i, f, FastLoadConstant<VecType>(Simd::g_expCoef6)); // Approximately 2^f
    return VecType::CastToFloat(VecType::Add(b, VecType::CastToInt(j)));
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType Normalize(typename VecType::FloatArgType value)
    {
    const typename VecType::FloatType lengthSquared = VecType::SplatIndex0(VecType::FromVec1(VecType::Dot(value, value)));
    const typename VecType::FloatType length = VecType::Sqrt(lengthSquared);
    return VecType::Div(value, length);
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType NormalizeEstimate(typename VecType::FloatArgType value)
    {
    const typename VecType::FloatType lengthSquared = VecType::SplatIndex0(VecType::FromVec1(VecType::Dot(value, value)));
    const typename VecType::FloatType invLength = VecType::SqrtInvEstimate(lengthSquared);
    return VecType::Mul(invLength, value);
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType NormalizeSafe(typename VecType::FloatArgType value, float tolerance)
    {
    const typename VecType::FloatType floatEpsilon = VecType::Splat(tolerance * tolerance);
    const typename VecType::FloatType lengthSquared = VecType::SplatIndex0(VecType::FromVec1(VecType::Dot(value, value)));
    if (VecType::CmpAllLt(lengthSquared, floatEpsilon))
    {
    return VecType::ZeroFloat();
    }
    return VecType::Div(value, VecType::Sqrt(lengthSquared));
    }

    template <typename VecType>
    AZ_MATH_INLINE typename VecType::FloatType NormalizeSafeEstimate(typename VecType::FloatArgType value, float tolerance)
    {
    const typename VecType::FloatType floatEpsilon = VecType::Splat(tolerance * tolerance);
    const typename VecType::FloatType lengthSquared = VecType::SplatIndex0(VecType::FromVec1(VecType::Dot(value, value)));
    if (VecType::CmpAllLt(lengthSquared, floatEpsilon))
    {
    return VecType::ZeroFloat();
    }
    return VecType::Mul(value, VecType::SqrtInvEstimate(lengthSquared));
    }

    template <typename Vec4Type, typename Vec3Type>
    AZ_MATH_INLINE typename Vec4Type::FloatType QuaternionTransform(typename Vec4Type::FloatArgType quat, typename Vec3Type::FloatArgType vec3)
    {
    const typename Vec4Type::FloatType Two = Vec4Type::Splat(2.0f);
    const typename Vec4Type::FloatType scalar = Vec4Type::SplatIndex3(quat); // Scalar portion of quat (W, W, W)

    const typename Vec4Type::FloatType partial1 = Vec4Type::SplatIndex0(Vec4Type::FromVec1(Vec3Type::Dot(quat, vec3)));
    const typename Vec4Type::FloatType partial2 = Vec4Type::Mul(quat, partial1);
    const typename Vec4Type::FloatType sum1 = Vec4Type::Mul(partial2, Two); // quat.Dot(vec3) * vec3 * 2.0f

    const typename Vec4Type::FloatType partial3 = Vec4Type::SplatIndex0(Vec4Type::FromVec1(Vec3Type::Dot(quat, quat)));
    const typename Vec4Type::FloatType partial4 = Vec4Type::Mul(scalar, scalar);
    const typename Vec4Type::FloatType partial5 = Vec4Type::Sub(partial4, partial3);
    const typename Vec4Type::FloatType sum2 = Vec4Type::Mul(partial5, vec3); // vec3 * (scalar * scalar - quat.Dot(quat))

    const typename Vec4Type::FloatType partial6 = Vec4Type::Mul(scalar, Two);
    const typename Vec4Type::FloatType partial7 = Vec3Type::Cross(quat, vec3);
    const typename Vec4Type::FloatType sum3 = Vec4Type::Mul(partial6, partial7); // scalar * 2.0f * quat.Cross(vec3)

    return Vec4Type::Add(Vec4Type::Add(sum1, sum2), sum3);
    }

    template <typename Vec4Type, typename Vec3Type>
    AZ_MATH_INLINE typename Vec4Type::FloatType ConstructPlane(typename Vec3Type::FloatArgType normal, typename Vec3Type::FloatArgType point)
    {
    const Vec1::FloatType distance = Vec1::Sub(Vec1::ZeroFloat(), Vec3Type::Dot(normal, point));
    return Vec4Type::ReplaceIndex3(normal, Vec4Type::SplatIndex0(Vec4Type::FromVec1(distance))); // replace 'w' coordinate with distance
    }

    template <typename Vec4Type, typename Vec3Type>
    AZ_MATH_INLINE Vec1::FloatType PlaneDistance(typename Vec4Type::FloatArgType plane, typename Vec3Type::FloatArgType point)
    {
    const typename Vec4Type::FloatType referencePoint = Vec4Type::ReplaceIndex3(point, 1.0f); // replace 'w' coordinate with 1.0
    return Vec4Type::Dot(referencePoint, plane);
    }


}
