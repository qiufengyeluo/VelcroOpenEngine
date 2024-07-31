#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

mod vec1{
    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[allow(dead_code)]
    use std::arch::x86_64::*;

    use crate::math::vsimd::*;

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_aligned_f32(addr :*f32)->FloatType{
        return  _mm_load_ps1(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_aligned_i128(addr :*const Int32Type)->Int32Type{
        return sse::load_aligned_i128(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_unaligned(addr:&f32)->FloatType{
        return  _mm_load_ps1(addr);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_unaligned_i128(addr:*const Int32Type)->Int32Type{
        return sse::load_aligned_i128(addr as *const Int32Type);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_aligned( addr:*mut f32,value:&FloatArgType){
        _mm_store_ss(addr, value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_aligned_i128(addr :*mut Int32Type,value:&Int32ArgType){
        sse::store_aligned_i128(addr as *mut Int32ArgType,value.to_owned())
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_unaligned(addr :*mut f32,value:&FloatArgType){
        _mm_store_ss(addr, value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_unaligned_i128(addr:*mut Int32Type,value:&Int32ArgType){
        sse::store_unaligned_i128(addr as *mut Int32Type,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn stream_aligned(addr :*mut f32,value:&FloatArgType){
        sse::stream_aligned(addr,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn stream_aligned_i128(addr:*mut Int32Type,value:&Int32ArgType){
        sse::stream_aligned_i128(addr as *mut Int32Type,value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn select_index0(value:&FloatArgType)->f32{
        return sse::select_first(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat(value:&f32)->FloatType{
        return _mm_set_ps1(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn splat_i32(value:&i32)->Int32Type{
        return sse::splat_i32(value.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_immediate(x:&f32)->FloatType{
        return sse::load_immediate(x.to_owned(),0.0,0.0,0.0);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn load_immediate_i32(x:&i32)->Int32Type{
        return sse::load_immediate_i32(x.to_owned(),0,0,0);
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn add(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::add(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn sub(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::sub(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn mul(arg1:&FloatArgType,arg2:&FloatArgType)->FloatType{
        return sse::mul(arg1.to_owned(),arg2.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn madd(mul1:&FloatArgType,mul2:&FloatArgType,add:&FloatArgType)->FloatType{
        return sse::madd(mul1.to_owned(),mul2.to_owned(),add.to_owned());
    }

    #[cfg(any(target_arch = "x86_64", target_arch="x86"))]
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn div(arg1:&FloatType, arg2: &mut FloatType) ->FloatType{
        let ones = sse::splat(1.0);
        *arg2 = sse::replace_first(ones.to_owned(),arg2.to_owned());
        return sse::div(arg1.to_owned(),arg2.to_owned())
    }
}


    AZ_MATH_INLINE Vec1::FloatType Vec1::Div(FloatArgType arg1, FloatArgType arg2)
    {
    // In Vec1 the last 3 elements can be zero, avoid doing division by zero
    const FloatType ones = Sse::Splat(1.0f);
    arg2 = Sse::ReplaceIndex0(ones, arg2);
    return Sse::Div(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Abs(FloatArgType value)
    {
    return Sse::Abs(value);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Add(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::Add(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Sub(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::Sub(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Mul(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::Mul(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Madd(Int32ArgType mul1, Int32ArgType mul2, Int32ArgType add)
    {
    return Sse::Madd(mul1, mul2, add);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Abs(Int32ArgType value)
    {
    return Sse::Abs(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Not(FloatArgType value)
    {
    return Sse::Not(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::And(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::And(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::AndNot(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::AndNot(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Or(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::Or(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Xor(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::Xor(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Not(Int32ArgType value)
    {
    return Sse::Not(value);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::And(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::And(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::AndNot(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::AndNot(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Or(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::Or(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Xor(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::Xor(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Floor(FloatArgType value)
    {
    return Sse::Floor(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Ceil(FloatArgType value)
    {
    return Sse::Ceil(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Round(FloatArgType value)
    {
    return Sse::Round(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Truncate(FloatArgType value)
    {
    return Sse::Truncate(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Min(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::Min(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Max(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::Max(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Clamp(FloatArgType value, FloatArgType min, FloatArgType max)
    {
    return Sse::Clamp(value, min, max);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Min(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::Min(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Max(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::Max(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Clamp(Int32ArgType value, Int32ArgType min, Int32ArgType max)
    {
    return Sse::Clamp(value, min, max);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::CmpEq(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpEq(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::CmpNeq(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpNeq(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::CmpGt(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpGt(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::CmpGtEq(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpGtEq(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::CmpLt(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpLt(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::CmpLtEq(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpLtEq(arg1, arg2);
    }

    AZ_MATH_INLINE bool Vec1::CmpAllEq(FloatArgType arg1, FloatArgType arg2)
    {
    // Only check the first bit for Vector1
    return Sse::CmpAllEq(arg1, arg2, 0b0001);
    }

    AZ_MATH_INLINE bool Vec1::CmpAllLt(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpAllLt(arg1, arg2, 0b0001);
    }

    AZ_MATH_INLINE bool Vec1::CmpAllLtEq(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpAllLtEq(arg1, arg2, 0b0001);
    }

    AZ_MATH_INLINE bool Vec1::CmpAllGt(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpAllGt(arg1, arg2, 0b0001);
    }

    AZ_MATH_INLINE bool Vec1::CmpAllGtEq(FloatArgType arg1, FloatArgType arg2)
    {
    return Sse::CmpAllGtEq(arg1, arg2, 0b0001);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::CmpEq(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::CmpEq(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::CmpNeq(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::CmpNeq(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::CmpGt(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::CmpGt(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::CmpGtEq(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::CmpGtEq(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::CmpLt(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::CmpLt(arg1, arg2);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::CmpLtEq(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::CmpLtEq(arg1, arg2);
    }

    AZ_MATH_INLINE bool Vec1::CmpAllEq(Int32ArgType arg1, Int32ArgType arg2)
    {
    return Sse::CmpAllEq(arg1, arg2, 0b0001);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Select(FloatArgType arg1, FloatArgType arg2, FloatArgType mask)
    {
    return Sse::Select(arg1, arg2, mask);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::Select(Int32ArgType arg1, Int32ArgType arg2, Int32ArgType mask)
    {
    return Sse::Select(arg1, arg2, mask);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Reciprocal(FloatArgType value)
    {
    // In Vec1 all the elements except the first one can be garbage or 0
    // Using (value.x, 1, 1, 1) to avoid divisions by 0.
    const FloatType ones = Sse::Splat(1.0f);
    return Sse::Reciprocal(
    Sse::ReplaceIndex0(ones, value));
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::ReciprocalEstimate(FloatArgType value)
    {
    // In Vec1 all the elements except the first one can be garbage or 0
    // Using (value.x, 1, 1, 1) to avoid divisions by 0.
    const FloatType ones = Sse::Splat(1.0f);
    return Sse::ReciprocalEstimate(
    Sse::ReplaceIndex0(ones, value));
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Mod(FloatArgType value, FloatArgType divisor)
    {
    return Sse::Mod(value, divisor);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Wrap(FloatArgType value, FloatArgType minValue, FloatArgType maxValue)
    {
    return Common::Wrap<Vec1>(value, minValue, maxValue);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::AngleMod(FloatArgType value)
    {
    return Common::AngleMod<Vec1>(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Sqrt(FloatArgType value)
    {
    return Sse::Sqrt(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::SqrtEstimate(FloatArgType value)
    {
    return Sse::SqrtEstimate(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::SqrtInv(FloatArgType value)
    {
    return Sse::SqrtInv(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::SqrtInvEstimate(FloatArgType value)
    {
    return Sse::SqrtInvEstimate(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Sin(FloatArgType value)
    {
    return Common::Sin<Vec1>(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Cos(FloatArgType value)
    {
    return Common::Cos<Vec1>(value);
    }

    AZ_MATH_INLINE void Vec1::SinCos(FloatArgType value, FloatType& sin, FloatType& cos)
    {
    Common::SinCos<Vec1>(value, sin, cos);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Acos(FloatArgType value)
    {
    return Common::Acos<Vec1>(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Atan(FloatArgType value)
    {
    return Common::Atan<Vec1>(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::Atan2(FloatArgType y, FloatArgType x)
    {
    return Common::Atan2<Vec1>(y, x);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::ExpEstimate(FloatArgType x)
    {
    return Common::ExpEstimate<Vec1>(x);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::ConvertToFloat(Int32ArgType value)
    {
    return Sse::ConvertToFloat(value);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::ConvertToInt(FloatArgType value)
    {
    return Sse::ConvertToInt(value);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::ConvertToIntNearest(FloatArgType value)
    {
    return Sse::ConvertToIntNearest(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::CastToFloat(Int32ArgType value)
    {
    return Sse::CastToFloat(value);
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::CastToInt(FloatArgType value)
    {
    return Sse::CastToInt(value);
    }

    AZ_MATH_INLINE Vec1::FloatType Vec1::ZeroFloat()
    {
    return Sse::ZeroFloat();
    }

    AZ_MATH_INLINE Vec1::Int32Type Vec1::ZeroInt()
    {
    return Sse::ZeroInt();
    }
