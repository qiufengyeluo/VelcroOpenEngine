#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::common_sse::VecType;
use crate::math::math_utils::constants::FLOAT_EPSILON;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::vector4::Vector4;
use crate::SimpleLcgRandomVec4;

#[derive(Debug)]
pub struct VectorN {
    _size:i32,
    _values:Vec<Vector4>,
}

impl VectorN {

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn new()->VectorN{
        VectorN{
            _size:0,
            _values:Vec::new(),
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_i32(num_elements:i32) -> VectorN {
        let mut result = VectorN::new();
        result._size = num_elements;
        for i in 0..num_elements{
            result._values.push(Vector4::new())
        }
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_i32_f32(num_elements:i32,x:f32) -> VectorN {
        let mut result = VectorN::new();
        result._size = num_elements;
        for i in 0..num_elements{
            result._values.push(Vector4::new_x(x))
        }
        result
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_n(v:&VectorN)->VectorN{
        let mut result = VectorN::new();
        result._size = v._size;
        for i in 0..v._size{
            result._values.push(v._values[i])
        }
        result
    }
    #[inline]
    #[allow(dead_code)]
    pub unsafe fn fix_last_vector_element(mut self){
        if self._values.size() == 0{
            return;
        }
        let masks:[u32;16] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
            0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000,
            0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x00000000,
            0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000];
        let last_element = self._values.size() -1;
        let trailing_zero_elements = 4*(self._size%4);
        let mask = Vec4::load_aligned(masks[trailing_zero_elements]);
        self._values[last_element].set_simd_value(Vec4::and(self._values[last_element].get_simd_value(), mask.borrow()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_zero(num_elements:i32) ->VectorN{
        return VectorN::new_i32(num_elements)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_one(num_elements:i32)->VectorN{
        return VectorN::new_i32_f32(num_elements,1.0)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_floats(numElements:i32,inputs:*const *f32)->VectorN{
        let mut result = VectorN::new_i32(numElements);
        for i in 0.. numElements{
            result.set_element(i, inputs[i])
        }
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_random(num_elements:i32) ->VectorN{
        let mut rand_gen = SimpleLcgRandomVec4::new();
        let mut return_value = VectorN::new_i32(num_elements);
        for  element in &mut return_value._values
        {
            element.set_simd_value(rand_gen.get_random_float4());
        }
        return_value.fix_last_vector_element();
        return_value
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_dimensionality(self)->i32{
        return self._size
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn resize(&mut self,size :i32){
        self._size = size;
        self._values = Vec::new();
        for i in 0.. size{
            self._values.push(Vector4::new());
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_element(self,index:i32)->f32{
        let element = index/4;
        return  (self._values.get(element) as Vector4).get_element(index % 4);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(&mut self,index:i32,value:f32){
        let element = index/4;
        (self._values.get_mut(element) as Vector4).set_element(index % 4, value);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close_default(self, v:&VectorN)->bool{
        return self.is_close(v,0.001)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(self,v:&VectorN,tolerance:f32)->bool{
        let vec_tolerance = Vec4::splat(tolerance);
        for i in 0.. self._values.size()
        {
            let dist = Vec4::abs(Vec4::Sub(self._values.get(i).unwrap().get_simd_value(), v._values.get(i).unwrap().get_simd_value()));
            if (!Vec4::cmp_all_lt_eq(dist, vec_tolerance))
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_zero_default(self)->bool{
        return  self.is_zero(FLOAT_EPSILON)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_zero(self,tolerance:f32)->bool{
        let vec_tolerance = Vec4::splat(tolerance);
        for element in self._values
        {
            if (!Vec4::cmp_all_lt_eq(Vec4::abs(element.get_simd_value()), vec_tolerance))
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_less_than(self,v :&VectorN)->bool{
        for i in 0..self._values.len()
        {
            if (!Vec4::cmp_all_lt(self._values[i].get_simd_value(), v._values[i].get_simd_value()))
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_less_equal_than(self,v:&VectorN)->bool{
        for i in 0..self._values.len()
        {
            if (!Vec4::cmp_all_lt_eq(self._values[i].get_simd_value(), v._values[i].get_simd_value()))
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_greater_than(self,v:&VectorN)->bool{
        for i in 0..self._values.len()
        {
            if (!Vec4::cmp_all_gt(self._values[i].get_simd_value(), v._values[i].get_simd_value()))
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_greater_equal_than(self,v:&VectorN)->bool{
        for i in 0..self._values.len()
        {
            if (!Vec4::cmp_all_gt_eq(self._values[i].get_simd_value(), v._values[i].get_simd_value()))
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_floor(self)->VectorN{
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0..self._values.len()
        {
            return_value._values[i] = self._values.get(i).unwrap().get_floor();
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_ceil(self)->VectorN{
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0..self._values.len()
        {
            return_value._values[i] =(self._values.get(i)).unwrap().get_ceil();
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_round(self)->VectorN{
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0..self._values.len()
        {
            return_value._values[i] =self._values.get(i).unwrap().get_round();
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_min(self,v:&VectorN)->VectorN{
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0..self._values.len()
        {
            return_value._values[i] = self._values.get(i).unwrap().get_min(v._values.get(i).unwrap());
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_max(self, v:&VectorN) ->VectorN{
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0..self._values.len()
        {
            return_value._values[i] = self._values.get(i).unwrap().get_max(v._values.get(i).unwrap());
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn
    AZ_MATH_INLINE VectorN VectorN::GetClamp(const VectorN& min, const VectorN& max) const
{
AZ_Assert(m_numElements == min.m_numElements, "Dimensionality must be equal");
AZ_Assert(m_numElements == max.m_numElements, "Dimensionality must be equal");
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i].GetClamp(min.m_values[i], max.m_values[i]);
}
return returnValue;
}

AZ_MATH_INLINE float VectorN::L1Norm() const
{
AZ::Vector4 partialLengths = AZ::Vector4::CreateZero();
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
partialLengths += m_values[i].GetAbs();
}
return partialLengths.Dot(AZ::Vector4::CreateOne());
}

AZ_MATH_INLINE float VectorN::L2Norm() const
{
return AZ::Sqrt(Dot(*this));
}

AZ_MATH_INLINE VectorN VectorN::GetNormalized() const
{
VectorN returnValue(*this);
returnValue.Normalize();
return returnValue;
}

AZ_MATH_INLINE void VectorN::Normalize()
{
const float length = L2Norm();
*this /= length;
}

AZ_MATH_INLINE VectorN VectorN::GetAbs() const
{
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i].GetAbs();
}
return returnValue;
}

AZ_MATH_INLINE void VectorN::Absolute()
{
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] = m_values[i].GetAbs();
}
}

AZ_MATH_INLINE VectorN VectorN::GetSquare() const
{
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i] * m_values[i];
}
return returnValue;
}

AZ_MATH_INLINE void VectorN::Square()
{
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] *= m_values[i];
}
}

AZ_MATH_INLINE float VectorN::Dot(const VectorN& rhs) const
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
AZ::Vector4 partialSums = AZ::Vector4::CreateZero();
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
partialSums.SetSimdValue(Simd::Vec4::Madd(m_values[i].GetSimdValue(), rhs.m_values[i].GetSimdValue(), partialSums.GetSimdValue()));
}
return partialSums.Dot(AZ::Vector4::CreateOne());
}

AZ_MATH_INLINE void VectorN::SetZero()
{
AZ::Vector4* data = m_values.data();
memset(data, 0, sizeof(AZ::Vector4) * m_values.size());
}

AZ_MATH_INLINE VectorN& VectorN::operator+=(const VectorN& rhs)
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] += rhs.m_values[i];
}
FixLastVectorElement();
return *this;
}

AZ_MATH_INLINE VectorN& VectorN::operator-=(const VectorN& rhs)
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] -= rhs.m_values[i];
}
FixLastVectorElement();
return *this;
}

AZ_MATH_INLINE VectorN& VectorN::operator*=(const VectorN& rhs)
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] *= rhs.m_values[i];
}
return *this;
}

AZ_MATH_INLINE VectorN& VectorN::operator/=(const VectorN& rhs)
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] /= rhs.m_values[i];
}
FixLastVectorElement();
return *this;
}

AZ_MATH_INLINE VectorN& VectorN::operator+=(float sum)
{
Vector4 sumVec = Vector4(sum);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] += sumVec;
}
FixLastVectorElement();
return *this;
}

AZ_MATH_INLINE VectorN& VectorN::operator-=(float difference)
{
Vector4 diffVec = Vector4(difference);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] -= diffVec;
}
FixLastVectorElement();
return *this;
}

AZ_MATH_INLINE VectorN& VectorN::operator*=(float multiplier)
{
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] *= multiplier;
}
return *this;
}

AZ_MATH_INLINE VectorN& VectorN::operator/=(float divisor)
{
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
m_values[i] /= divisor;
}
FixLastVectorElement();
return *this;
}

AZ_MATH_INLINE VectorN VectorN::operator-() const
{
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = -m_values[i];
}
return returnValue;
}

AZ_MATH_INLINE VectorN VectorN::operator+(const VectorN& rhs) const
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i] + rhs.m_values[i];
}
returnValue.FixLastVectorElement();
return returnValue;
}

AZ_MATH_INLINE VectorN VectorN::operator-(const VectorN& rhs) const
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i] - rhs.m_values[i];
}
returnValue.FixLastVectorElement();
return returnValue;
}

AZ_MATH_INLINE VectorN VectorN::operator*(const VectorN& rhs) const
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i] * rhs.m_values[i];
}
return returnValue;
}

AZ_MATH_INLINE VectorN VectorN::operator/(const VectorN& rhs) const
{
AZ_Assert(m_numElements == rhs.m_numElements, "Dimensionality must be equal");
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i] / rhs.m_values[i];
}
returnValue.FixLastVectorElement();
return returnValue;
}

AZ_MATH_INLINE VectorN VectorN::operator*(float multiplier) const
{
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i] * multiplier;
}
return returnValue;
}

AZ_MATH_INLINE VectorN VectorN::operator/(float divisor) const
{
VectorN returnValue(m_numElements);
for (AZStd::size_t i = 0; i < m_values.size(); ++i)
{
returnValue.m_values[i] = m_values[i] / divisor;
}
returnValue.FixLastVectorElement();
return returnValue;
}

AZ_MATH_INLINE const AZStd::vector<Vector4>& VectorN::GetVectorValues() const
{
return m_values;
}

AZ_MATH_INLINE AZStd::vector<Vector4>& VectorN::GetVectorValues()
{
return m_values;
}

AZ_MATH_INLINE void VectorN::FixLastVectorElement()
{
if (m_values.empty())
{
return;
}

const uint32_t masks[] =
{
0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000,
0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x00000000,
0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000
};

const AZStd::size_t lastElement = m_values.size() - 1;
const AZStd::size_t trailingZeroElements = 4 * (m_numElements % 4);
const Simd::Vec4::FloatType mask = Simd::Vec4::LoadAligned(reinterpret_cast<const float*>(&masks[trailingZeroElements]));

m_values[lastElement].SetSimdValue(Simd::Vec4::And(m_values[lastElement].GetSimdValue(), mask));
}
#[inline]
#[allow(dead_code)]
pub unsafe fn fix_last_vector_element(mut self){
    if self._value.size() == 0{
        return;
    }
   let masks:[u32] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
       0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000,
       0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x00000000,
       0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000];
    let lastElement = self._value.size() -1;
    let trailingZeroElements = 4*(self._elements%4);
    let mask = Vec4::load_aligned(masks[trailingZeroElements]);

}
AZ_MATH_INLINE void VectorN::OnSizeChanged()
{
m_values.resize((m_numElements + 3) / 4);
FixLastVectorElement();
}
#[inline]
#[allow(dead_code)]
pub unsafe fn on_size_changed(mut self){
    self._value.resize();
    self.fix_last_vector_element();
}
AZ_MATH_INLINE VectorN operator+(float lhs, const VectorN& rhs)
{
VectorN returnValue(rhs.GetDimensionality());
const AZ::Vector4 lhsVec = AZ::Vector4(lhs);
for (AZStd::size_t i = 0; i < rhs.GetVectorValues().size(); ++i)
{
returnValue.GetVectorValues()[i] = lhsVec + rhs.GetVectorValues()[i];
}
returnValue.FixLastVectorElement();
return returnValue;
}

AZ_MATH_INLINE VectorN operator-(float lhs, const VectorN& rhs)
{
VectorN returnValue(rhs.GetDimensionality());
const AZ::Vector4 lhsVec = AZ::Vector4(lhs);
for (AZStd::size_t i = 0; i < rhs.GetVectorValues().size(); ++i)
{
returnValue.GetVectorValues()[i] = lhsVec - rhs.GetVectorValues()[i];
}
returnValue.FixLastVectorElement();
return returnValue;
}

AZ_MATH_INLINE VectorN operator*(float lhs, const VectorN& rhs)
{
VectorN returnValue(rhs.GetDimensionality());
const AZ::Vector4 lhsVec = AZ::Vector4(lhs);
for (AZStd::size_t i = 0; i < rhs.GetVectorValues().size(); ++i)
{
returnValue.GetVectorValues()[i] = lhsVec * rhs.GetVectorValues()[i];
}
return returnValue;
}
}

