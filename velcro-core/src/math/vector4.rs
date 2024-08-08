#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

#[cfg(target_arch = "arm")]
#[allow(dead_code)]
use vsimd::neon::*;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use vsimd::sse::{FloatArgType, FloatType};

use crate::math::*;
use crate::math::common_sse::{Vec4Type, VecThirdType, VecType};
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::vector2::Vector2;
use crate::math::vector3::Vector3;

// PartialEq 是否相等
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    _value: FloatType,
}

impl Vector4 {
    #[inline]
    #[allow(dead_code)]
    pub fn new()->Vector4{
        unsafe {
            Vector4 {
                _value:Vec4::zero_float(),
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_x(x:&f32)->Vector4{
        Vector4{
            _value:Vec4::splat(x),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_x_y_z_w(x:&f32,y:&f32,z:&f32,w:&f32)->Vector4{
        Vector4{
            _value:Vec4::load_immediate(x,y,z,w),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_float_type(value:&FloatArgType)->Vector4{
        Vector4{
            _value:value.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2(source:&Vector2)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec2(source.get_simd_value().borrow())};
        let mut tmp = *result._value as *const f32;
        *tmp[2] = 0.0;
        *tmp[3] = 1.0;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2_z(source:&Vector2,z:&f32)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec2(source.get_simd_value().borrow())};
        let mut tmp = *result._value as *const f32;
        *tmp[2] = z;
        *tmp[3] = 1.0;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec2_z_w(source:&Vector2,z:&f32,w:&f32)->Vector4{
        let result =  Vector4{ _value:Vec4::from_vec2(source.get_simd_value().borrow())};
        let mut tmp = *result._value as *const f32;
        *tmp[2] = z;
        *tmp[3] = w;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec3(source:&Vector3)->Vector4{
        let result =  Vector4{ _value:replace_third_f32(source.get_simd_value(),0.0)};
        let mut tmp = *result._value as [f32;4];
        *tmp[3] = 1.0;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_vec3_w(source:&Vector3,w:&f32)->Vector4{
        let result =  Vector4{ _value:replace_third_f32(source.get_simd_value(),0.0)};
        let mut tmp = *result._value as [f32;4];
        *tmp[3] = w;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_zero()->Vector4{
        unsafe {
            Vector4 {
                _value: zero_float(),
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_one()->Vector4{
        return Vector4::new_x(1.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_x(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(length,0.0.borrow(),0.0.borrow(),0.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_y(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(0.0.borrow(),length,0.0.borrow(),0.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_z(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(0.0.borrow(),0.0.borrow(),length,0.0.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_axis_w(length:&f32)->Vector4{
        return Vector4::new_x_y_z_w(0.0.borrow(),0.0.borrow(),0.0.borrow(),length);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_float4(values:*const f32)->Vector4{
        let result = Vector4::new();
        result.set(values);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3(v:&Vector3)->Vector4{
        let mut result = Vector4::new();
        result.set_vec3(v);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_vector3_and_float(v:&Vector3,w:&f32)->Vector4{
        let mut result = Vector4::new();
        result.set_vec3_f32(v,w);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_select_cmp_equal(cmp1:&Vector4,cmp2:&Vector4,va:&Vector4,vb:&Vector4)->Vector4{
        let mask = cmp_eq(cmp1._value,cmp2._value);
        return Vector4::new_float_type(select(va._value,vb._value,mask).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_select_cmp_greater_equal(cmp1:&Vector4, cmp2:&Vector4, va:&Vector4, vb:&Vector4) ->Vector4{
        let mask = cmp_gt_eq(cmp1._value,cmp2._value);
        return Vector4::new_float_type(select(va._value,vb._value,mask).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_select_cmp_greater(cmp1:&Vector4, cmp2:&Vector4, va:&Vector4, vb:&Vector4) ->Vector4{
        let mask = cmp_gt(cmp1._value,cmp2._value);
        return Vector4::new_float_type(select(va._value,vb._value,mask).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn store_to_float_4(self, values:*mut f32){
        store_unaligned(values,self._value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_x(self)->f32{
        let values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[0]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_y(self)->f32{
        let values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[1]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_z(self)->f32{
        let values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[2]
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_w(self)->f32{
        let values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[3]
    }

    #[inline]
    #[allow(dead_code)]
    pub fn get_element(self,index:i32)->f32{
        let values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[index]
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_x(mut self, x :f32){
        let mut values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[0] = x
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_y(mut self, y:f32){
        let mut values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[1] = y
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_z(mut self, z:f32){
        let mut values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[2] = z
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_w(mut self, w:f32){
        let mut values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[3] = w
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_f32(mut self,x:&f32){
        self._value = splat(x.to_owned());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_x_y_z_w(mut self,x:&f32,y:&f32,z:&f32,w:&f32){
        self._value = load_immediate(x.to_owned(),y.to_owned(),z.to_owned(),w.to_owned());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set(mut self, values:*const f32){
        self._value = load_aligned(values);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_vec3(&mut self,v:&Vector3){
        self._value = Vector4::new_vec3_w(v,1.0.borrow())._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_vec3_f32(&mut self,v:&Vector3,w:&f32){
        self._value = Vector4::new_vec3_w(v,w)._value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_float_type(mut self,v:&FloatArgType){
        self._value = v.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(mut self,index:&i32,v:&f32){
        let mut values:*const [f32;4] = (*self._value) as *const [f32;4];
        *values[index] = v
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_as_vector3(self)->Vector3{
        return Vector3::new_float_type(self._value.borrow());
    }
AZ_MATH_INLINE float Vector4::GetLengthSq() const
{
return Dot(*this);
}
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_length_sq(self)->f32{
        return self.dot_f32(self)
    }
AZ_MATH_INLINE float Vector4::GetLength() const
{
const Simd::Vec1::FloatType lengthSq = Simd::Vec4::Dot(m_value, m_value);
return Simd::Vec1::SelectIndex0(Simd::Vec1::Sqrt(lengthSq));
}

AZ_MATH_INLINE float Vector4::GetLengthEstimate() const
{
const Simd::Vec1::FloatType lengthSq = Simd::Vec4::Dot(m_value, m_value);
return Simd::Vec1::SelectIndex0(Simd::Vec1::SqrtEstimate(lengthSq));
}

AZ_MATH_INLINE float Vector4::GetLengthReciprocal() const
{
const Simd::Vec1::FloatType lengthSq = Simd::Vec4::Dot(m_value, m_value);
return Simd::Vec1::SelectIndex0(Simd::Vec1::SqrtInv(lengthSq));
}

AZ_MATH_INLINE float Vector4::GetLengthReciprocalEstimate() const
{
const Simd::Vec1::FloatType lengthSq = Simd::Vec4::Dot(m_value, m_value);
return Simd::Vec1::SelectIndex0(Simd::Vec1::SqrtInvEstimate(lengthSq));
}

AZ_MATH_INLINE Vector4 Vector4::GetNormalized() const
{
return Vector4(Simd::Vec4::Normalize(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetNormalizedEstimate() const
{
return Vector4(Simd::Vec4::NormalizeEstimate(m_value));
}

AZ_MATH_INLINE void Vector4::Normalize()
{
*this = GetNormalized();
}

AZ_MATH_INLINE void Vector4::NormalizeEstimate()
{
*this = GetNormalizedEstimate();
}

AZ_MATH_INLINE Vector4 Vector4::GetNormalizedSafe(float tolerance) const
{
return Vector4(Simd::Vec4::NormalizeSafe(m_value, tolerance));
}

AZ_MATH_INLINE Vector4 Vector4::GetNormalizedSafeEstimate(float tolerance) const
{
return Vector4(Simd::Vec4::NormalizeSafeEstimate(m_value, tolerance));
}

AZ_MATH_INLINE void Vector4::NormalizeSafe(float tolerance)
{
*this = GetNormalizedSafe(tolerance);
}

AZ_MATH_INLINE void Vector4::NormalizeSafeEstimate(float tolerance)
{
*this = GetNormalizedSafeEstimate(tolerance);
}

AZ_MATH_INLINE float Vector4::NormalizeWithLength()
{
const float length = Simd::Vec1::SelectIndex0(
Simd::Vec1::Sqrt(Simd::Vec4::Dot(m_value, m_value)));
m_value = Simd::Vec4::Div(m_value, Simd::Vec4::Splat(length));
return length;
}

AZ_MATH_INLINE float Vector4::NormalizeWithLengthEstimate()
{
const float length = Simd::Vec1::SelectIndex0(
Simd::Vec1::SqrtEstimate(Simd::Vec4::Dot(m_value, m_value)));
m_value = Simd::Vec4::Div(m_value, Simd::Vec4::Splat(length));
return length;
}

AZ_MATH_INLINE float Vector4::NormalizeSafeWithLength(float tolerance)
{
const Simd::Vec1::FloatType length = Simd::Vec1::Sqrt(Simd::Vec4::Dot(m_value, m_value));
m_value = (Simd::Vec1::SelectIndex0(length) < tolerance) ? Simd::Vec4::ZeroFloat() : Simd::Vec4::Div(m_value, Simd::Vec4::SplatIndex0(Simd::Vec4::FromVec1(length)));
return Simd::Vec1::SelectIndex0(length);
}

AZ_MATH_INLINE float Vector4::NormalizeSafeWithLengthEstimate(float tolerance)
{
const Simd::Vec1::FloatType length = Simd::Vec1::SqrtEstimate(Simd::Vec4::Dot(m_value, m_value));
m_value = (Simd::Vec1::SelectIndex0(length) < tolerance) ? Simd::Vec4::ZeroFloat() : Simd::Vec4::Div(m_value, Simd::Vec4::SplatIndex0(Simd::Vec4::FromVec1(length)));
return Simd::Vec1::SelectIndex0(length);
}

AZ_MATH_INLINE bool Vector4::IsNormalized(float tolerance) const
{
return (Abs(GetLengthSq() - 1.0f) <= tolerance);
}

AZ_MATH_INLINE void Vector4::SetLength(float length)
{
float scale(length * GetLengthReciprocal());
(*this) *= scale;
}

AZ_MATH_INLINE void Vector4::SetLengthEstimate(float length)
{
float scale(length * GetLengthReciprocalEstimate());
(*this) *= scale;
}

AZ_MATH_INLINE float Vector4::GetDistanceSq(const Vector4& v) const
{
return ((*this) - v).GetLengthSq();
}

AZ_MATH_INLINE float Vector4::GetDistance(const Vector4& v) const
{
return ((*this) - v).GetLength();
}

AZ_MATH_INLINE float Vector4::GetDistanceEstimate(const Vector4& v) const
{
return ((*this) - v).GetLengthEstimate();
}

AZ_MATH_INLINE bool Vector4::IsClose(const Vector4& v, float tolerance) const
{
Vector4 dist = (v - (*this)).GetAbs();
return dist.IsLessEqualThan(Vector4(tolerance));
}

AZ_MATH_INLINE bool Vector4::IsZero(float tolerance) const
{
Vector4 dist = GetAbs();
return dist.IsLessEqualThan(Vector4(tolerance));
}

AZ_MATH_INLINE bool Vector4::operator==(const Vector4& rhs) const
{
return Simd::Vec4::CmpAllEq(m_value, rhs.m_value);
}

AZ_MATH_INLINE bool Vector4::operator!=(const Vector4& rhs) const
{
return !Simd::Vec4::CmpAllEq(m_value, rhs.m_value);
}

AZ_MATH_INLINE bool Vector4::IsLessThan(const Vector4& rhs) const
{
return Simd::Vec4::CmpAllLt(m_value, rhs.m_value);
}

AZ_MATH_INLINE bool Vector4::IsLessEqualThan(const Vector4& rhs) const
{
return Simd::Vec4::CmpAllLtEq(m_value, rhs.m_value);
}

AZ_MATH_INLINE bool Vector4::IsGreaterThan(const Vector4& rhs) const
{
return Simd::Vec4::CmpAllGt(m_value, rhs.m_value);
}

AZ_MATH_INLINE bool Vector4::IsGreaterEqualThan(const Vector4& rhs) const
{
return Simd::Vec4::CmpAllGtEq(m_value, rhs.m_value);
}

AZ_MATH_INLINE Vector4 Vector4::GetFloor() const
{
return Vector4(Simd::Vec4::Floor(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetCeil() const
{
return Vector4(Simd::Vec4::Ceil(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetRound() const
{
return Vector4(Simd::Vec4::Round(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetMin(const Vector4& v) const
{
#if AZ_TRAIT_USE_PLATFORM_SIMD_SCALAR
return Vector4(AZ::GetMin(m_x, v.m_x), AZ::GetMin(m_y, v.m_y), AZ::GetMin(m_z, v.m_z), AZ::GetMin(m_w, v.m_w));
#else
return Vector4(Simd::Vec4::Min(m_value, v.m_value));
#endif
}

AZ_MATH_INLINE Vector4 Vector4::GetMax(const Vector4& v) const
{
#if AZ_TRAIT_USE_PLATFORM_SIMD_SCALAR
return Vector4(AZ::GetMax(m_x, v.m_x), AZ::GetMax(m_y, v.m_y), AZ::GetMax(m_z, v.m_z), AZ::GetMax(m_w, v.m_w));
#else
return Vector4(Simd::Vec4::Max(m_value, v.m_value));
#endif
}

AZ_MATH_INLINE Vector4 Vector4::GetClamp(const Vector4& min, const Vector4& max) const
{
return GetMin(max).GetMax(min);
}

AZ_MATH_INLINE Vector4 Vector4::Lerp(const Vector4& dest, float t) const
{
return Vector4(Simd::Vec4::Madd(Simd::Vec4::Sub(dest.m_value, m_value), Simd::Vec4::Splat(t), m_value));
}

AZ_MATH_INLINE Vector4 Vector4::Slerp(const Vector4& dest, float t) const
{
// Dot product - the cosine of the angle between 2 vectors and clamp it to be in the range of Acos()
const Simd::Vec1::FloatType dot = Simd::Vec1::Clamp(Simd::Vec4::Dot(m_value, dest.m_value), Simd::Vec1::Splat(-1.0f), Simd::Vec1::Splat(1.0f));
// Acos(dot) returns the angle between start and end, and multiplying that by proportion returns the angle between start and the final result
const Simd::Vec1::FloatType theta = Simd::Vec1::Mul(Simd::Vec1::Acos(dot), Simd::Vec1::Splat(t));
const Simd::Vec4::FloatType relativeVec = Simd::Vec4::Sub(dest.GetSimdValue(), Simd::Vec4::Mul(GetSimdValue(), Simd::Vec4::FromVec1(dot)));
const Simd::Vec4::FloatType relVecNorm = Simd::Vec4::NormalizeSafe(relativeVec, Constants::Tolerance);
const Simd::Vec4::FloatType sinCos = Simd::Vec4::FromVec2(Simd::Vec2::SinCos(theta));
const Simd::Vec4::FloatType relVecSinTheta = Simd::Vec4::Mul(relVecNorm, Simd::Vec4::SplatIndex0(sinCos));
return Vector4(Simd::Vec4::Madd(GetSimdValue(), Simd::Vec4::SplatIndex1(sinCos), relVecSinTheta));
}

AZ_MATH_INLINE Vector4 Vector4::Nlerp(const Vector4& dest, float t) const
{
return Lerp(dest, t).GetNormalizedSafe(Constants::Tolerance);
}

AZ_MATH_INLINE float Vector4::Dot(const Vector4& rhs) const
{
#if AZ_TRAIT_USE_PLATFORM_SIMD_SCALAR
return (m_x * rhs.m_x + m_y * rhs.m_y + m_z * rhs.m_z + m_w * rhs.m_w);
#else
return Simd::Vec1::SelectIndex0(Simd::Vec4::Dot(m_value, rhs.m_value));
#endif
}
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn dot_f32(self,rhs:&Vector4)->f32{
        return select_first()
    }
AZ_MATH_INLINE float Vector4::Dot3(const Vector3& rhs) const
{
#if AZ_TRAIT_USE_PLATFORM_SIMD_SCALAR
return (m_x * rhs.GetX() + m_y * rhs.GetY() + m_z * rhs.GetZ());
#else
return Simd::Vec1::SelectIndex0(Simd::Vec3::Dot(Simd::Vec4::ToVec3(m_value), rhs.GetSimdValue()));
#endif
}

AZ_MATH_INLINE void Vector4::Homogenize()
{
const Simd::Vec4::FloatType divisor = Simd::Vec4::SplatIndex3(m_value);
m_value = Simd::Vec4::Div(m_value, divisor);
}

AZ_MATH_INLINE Vector3 Vector4::GetHomogenized() const
{
const Simd::Vec3::FloatType divisor = Simd::Vec4::ToVec3(Simd::Vec4::SplatIndex3(m_value));
return Vector3(Simd::Vec3::Div(Simd::Vec4::ToVec3(m_value), divisor));
}

AZ_MATH_INLINE Vector4 Vector4::operator-() const
{
return Vector4(Simd::Vec4::Sub(Simd::Vec4::ZeroFloat(), m_value));
}

AZ_MATH_INLINE Vector4 Vector4::operator+(const Vector4& rhs) const
{
return Vector4(Simd::Vec4::Add(m_value, rhs.m_value));
}

AZ_MATH_INLINE Vector4 Vector4::operator-(const Vector4& rhs) const
{
return Vector4(Simd::Vec4::Sub(m_value, rhs.m_value));
}

AZ_MATH_INLINE Vector4 Vector4::operator*(const Vector4& rhs) const
{
return Vector4(Simd::Vec4::Mul(m_value, rhs.m_value));
}

AZ_MATH_INLINE Vector4 Vector4::operator/(const Vector4& rhs) const
{
return Vector4(Simd::Vec4::Div(m_value, rhs.m_value));
}

AZ_MATH_INLINE Vector4 Vector4::operator*(float multiplier) const
{
return Vector4(Simd::Vec4::Mul(m_value, Simd::Vec4::Splat(multiplier)));
}

AZ_MATH_INLINE Vector4 Vector4::operator/(float divisor) const
{
return Vector4(Simd::Vec4::Div(m_value, Simd::Vec4::Splat(divisor)));
}

AZ_MATH_INLINE Vector4& Vector4::operator+=(const Vector4& rhs)
{
*this = (*this) + rhs;
return *this;
}

AZ_MATH_INLINE Vector4& Vector4::operator-=(const Vector4& rhs)
{
*this = (*this) - rhs;
return *this;
}

AZ_MATH_INLINE Vector4& Vector4::operator*=(const Vector4& rhs)
{
*this = (*this) * rhs;
return *this;
}

AZ_MATH_INLINE Vector4& Vector4::operator/=(const Vector4& rhs)
{
*this = (*this) / rhs;
return *this;
}

AZ_MATH_INLINE Vector4& Vector4::operator*=(float multiplier)
{
*this = (*this) * multiplier;
return *this;
}

AZ_MATH_INLINE Vector4& Vector4::operator/=(float divisor)
{
*this = (*this) / divisor;
return *this;
}

AZ_MATH_INLINE Vector4 Vector4::GetSin() const
{
return Vector4(Simd::Vec4::Sin(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetCos() const
{
return Vector4(Simd::Vec4::Cos(m_value));
}

AZ_MATH_INLINE void Vector4::GetSinCos(Vector4& sin, Vector4& cos) const
{
Simd::Vec4::FloatType sinValues, cosValues;
Simd::Vec4::SinCos(m_value, sinValues, cosValues);
sin = Vector4(sinValues);
cos = Vector4(cosValues);
}

AZ_MATH_INLINE Vector4 Vector4::GetAcos() const
{
return Vector4(Simd::Vec4::Acos(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetAtan() const
{
return Vector4(Simd::Vec4::Atan(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetExpEstimate() const
{
return Vector4(Simd::Vec4::ExpEstimate(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetAngleMod() const
{
return Vector4(Simd::Vec4::AngleMod(m_value));
}

AZ_MATH_INLINE float Vector4::Angle(const Vector4& v) const
{
const float cos = Dot(v) * InvSqrt(GetLengthSq() * v.GetLengthSq());
// secure against any float precision error, cosine must be between [-1, 1]
const float res = Acos(AZ::GetClamp(cos, -1.0f, 1.0f));
AZ_MATH_ASSERT(std::isfinite(res) && (res >= 0.0f) && (res <= Constants::Pi), "Calculated an invalid angle");
return res;
}

AZ_MATH_INLINE float Vector4::AngleDeg(const Vector4& v) const
{
return RadToDeg(Angle(v));
}

AZ_MATH_INLINE float Vector4::AngleSafe(const Vector4& v) const
{
return (!IsZero() && !v.IsZero()) ? Angle(v) : 0.0f;
}

AZ_MATH_INLINE float Vector4::AngleSafeDeg(const Vector4& v) const
{
return (!IsZero() && !v.IsZero()) ? AngleDeg(v) : 0.0f;
}

AZ_MATH_INLINE Vector4 Vector4::GetAbs() const
{
return Vector4(Simd::Vec4::Abs(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetReciprocal() const
{
return Vector4(Simd::Vec4::Reciprocal(m_value));
}

AZ_MATH_INLINE Vector4 Vector4::GetReciprocalEstimate() const
{
return Vector4(Simd::Vec4::ReciprocalEstimate(m_value));
}

AZ_MATH_INLINE bool Vector4::IsFinite() const
{
return IsFiniteFloat(GetX()) && IsFiniteFloat(GetY()) && IsFiniteFloat(GetZ()) && IsFiniteFloat(GetW());
}

AZ_MATH_INLINE Simd::Vec4::FloatType Vector4::GetSimdValue() const
{
return m_value;
}

AZ_MATH_INLINE void Vector4::SetSimdValue(Simd::Vec4::FloatArgType value)
{
m_value = value;
}

AZ_MATH_INLINE Vector4 operator*(float multiplier, const Vector4& rhs)
{
return rhs * multiplier;
}
    pub fn get_simd_value(self)->FloatType{
        self._value
    }
}