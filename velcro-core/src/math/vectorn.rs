#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::math::common_sse::VecType;
use crate::math::math_utils::constants::FLOAT_EPSILON;
use crate::math::simd_math::simd;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::vector4::Vector4;
use crate::SimpleLcgRandomVec4;

#[derive(Debug)]
pub struct VectorN {
    _size:i32,
    _values:Vec<Vector4>,
}

impl Clone for VectorN {
    fn clone(&self) -> Self {
        let mut result= unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len(){
            result._values.get_mut(i).unwrap().set_simd_value(self._values.get(i).unwrap().get_simd_value());
        }
        result
    }
}

impl Copy for VectorN {
}
impl AddAssign<&VectorN> for VectorN{
    fn add_assign(&mut self, rhs: &VectorN) {
        for i in self._values.len()
        {
            self._values[i] += rhs._values[i];
        }
        self.fix_last_vector_element();
    }
}

impl SubAssign<&VectorN> for VectorN{
    fn sub_assign(&mut self, rhs: &VectorN) {
        for i in self._values.len()
        {
            self._values[i] -= rhs._values[i];
        }
        self.fix_last_vector_element();
    }
}

impl MulAssign<&VectorN> for VectorN{
    fn mul_assign(&mut self, rhs: &VectorN) {
        for i in self._values.len()
        {
            self._values[i] *= rhs._values[i];
        }
    }
}

impl DivAssign<&VectorN> for VectorN{
    fn div_assign(&mut self, rhs: &VectorN) {
        for i in self._values.len()
        {
            self._values[i] /= rhs._values[i];
        }
    }
}

impl AddAssign<f32> for VectorN{
    fn add_assign(&mut self, sum: f32) {
        let sum_vec = unsafe { Vector4::new_x(sum) };
        for i in self._values.len()
        {
            self._values[i] += sum_vec;
        }
        self.fix_last_vector_element();
    }
}

impl SubAssign<f32> for VectorN{
    fn sub_assign(&mut self, difference: f32) {
        let diff_vec =  unsafe { Vector4::new_x(difference) };
        for i in self._values.len()
        {
            self._values[i] -= diff_vec;
        }
        self.fix_last_vector_element();
    }
}

impl MulAssign<f32> for VectorN{
    fn mul_assign(&mut self, multiplier: f32) {
        for i in self._values.len()
        {
            self._values[i] *= multiplier;
        }
    }
}

impl DivAssign<f32> for VectorN{
    fn div_assign(&mut self, divisor: f32) {
        for i in self._values.len()
        {
            self._values[i] /= divisor;
        }
        self.fix_last_vector_element();
    }
}

impl Sub for VectorN {
    type Output = VectorN;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut return_value = unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len()
        {
            return_value._values[i] = -self._values[i];
        }
        return return_value;
    }
}

impl Add<&VectorN> for VectorN {
    type Output = VectorN;
    fn add(self, rhs: &VectorN) -> Self::Output {
        let mut return_value = unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len()
        {
            return_value._values[i] = self._values[i] + rhs._values[i];
        }
        return_value.fix_last_vector_element();
        return return_value;
    }
}

impl Sub<&VectorN> for VectorN{
    type Output = VectorN;

    fn sub(self, rhs: &VectorN) -> Self::Output {
        let mut return_value = unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len()
        {
            return_value._values[i] = self._values[i] - rhs._values[i];
        }
        return_value.fix_last_vector_element();
        return return_value;
    }
}

impl Mul<&VectorN> for VectorN{
    type Output = VectorN;

    fn mul(self, rhs: &VectorN) -> Self::Output {
        let mut return_value = unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len()
        {
            return_value._values[i] = self._values[i] * rhs._values[i];
        }
        return return_value;
    }
}

impl Div<&VectorN> for VectorN{
    type Output = VectorN;

    fn div(self, rhs: &VectorN) -> Self::Output {
        let mut return_value = unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len()
        {
            return_value._values[i] = self._values[i] / rhs._values[i];
        }
        return_value.fix_last_vector_element();
        return return_value;
    }
}

impl Mul<f32> for VectorN{
    type Output = VectorN;

    fn mul(self, multiplier: f32) -> Self::Output {
        let mut return_value = unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len()
        {
            return_value._values[i] = self._values[i] * multiplier;
        }
        return return_value;
    }
}

impl Div<f32> for VectorN{
    type Output = VectorN;

    fn div(self, divisor: f32) -> Self::Output {
        let mut return_value = unsafe { VectorN::new_i32(self._size) };
        for i in self._values.len()
        {
            return_value._values[i] = self._values[i] / divisor;
        }
        return_value.fix_last_vector_element();
        return return_value;
    }
}


impl VectorN {

    #[inline]
    #[allow(dead_code)]
    pub fn new()->VectorN{
        VectorN{
            _size:0,
            _values:Vec::new(),
        }
    }
    #[inline]
    #[allow(dead_code)]
    pub fn new_i32(num_elements:i32) -> VectorN {
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
    pub  fn fix_last_vector_element(&mut self){
        if self._values.size() == 0{
            return;
        }
        let masks:[u32;16] = [0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
            0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000,
            0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x00000000,
            0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000];
        let last_element = self._values.size() -1;
        let trailing_zero_elements = 4*(self._size%4);
        let mask = unsafe { Vec4::load_aligned(masks[trailing_zero_elements]) };
        unsafe { self._values[last_element].set_simd_value(Vec4::and(self._values[last_element].get_simd_value(), mask.borrow())); }
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
        return_value

        // return_value.fix_last_vector_element();
    }

    #[inline]
    #[allow(dead_code)]
    pub  fn get_dimensionality(self)->i32{
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
        return  self._values.get(element) .unwrap().get_element(index % 4);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_element(&mut self,index:i32,value:f32){
        let element = index/4;
        self._values.get_mut(element).unwrap().set_element(index % 4, value);
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
        assert_eq!(self._size, v._size);
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
        assert_eq!(self._size, v._size);
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
        assert_eq!(self._size, v._size);
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
        assert_eq!(self._size, v._size);
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
        assert_eq!(self._size, v._size);
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
        assert_eq!(self._size, v._size);
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0..self._values.len()
        {
            return_value._values[i] = self._values.get(i).unwrap().get_max(v._values.get(i).unwrap());
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_clamp(self,min:&VectorN,max:&VectorN)->VectorN{
        assert!(self._size == min._size);
        assert!(self._size == max._size);
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0..self._values.len()
        {
            return_value._values[i] = self._values.get(i).unwrap().get_clamp(min._values.get(i).unwrap(), max._values.get(i).unwrap());
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn l1norm(self)->f32{
        let mut partial_lengths = Vector4::create_zero();
        for i in 0.. self._values.len()
        {
            partial_lengths += self._values.get(i).unwrap().get_abs();
        }
        return partial_lengths.dot4(Vector4::create_one().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn l2norm(self)->f32{
        return simd::sqrt(Self.dotn(*self));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_normalized(self)->VectorN{
        let mut return_value = VectorN::new_n(*self);
        return_value.normalize();
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn normalize(self){
        let length = self.l2norm();
        *self /= length;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_abs(self)->VectorN{
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0.. self._values.len()
        {
            return_value._values.get_mut(i).unwrap().set_simd_value(self._values.get(i).unwrap().get_abs().get_simd_value());
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn absolute(&mut self){
        for i in 0.. self._values.len()
        {
            self._values.get_mut(i).unwrap().set_simd_value(self._values.get(i).unwrap().get_abs().get_simd_value());
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_square(self)->VectorN{
        let mut return_value = VectorN::new_i32(self._size);
        for i in 0.. self._values.len()
        {
            return_value._values.get_mut(i).unwrap().set_simd_value((self._values[i] * self._values[i]).get_simd_value());
        }
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn square(&mut self){
        for i in 0.. self._values.len()
        {
            self._values.get_mut(i).unwrap().mul_assign(self._values.get(i).unwrap());
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn dotn(self,rhs:&VectorN)->f32{
        assert_eq!(self._size, rhs._size, "Dimensionality must be equal");
        let mut partial_sums = Vector4::create_zero();
        for i in 0.. self._values.len()
        {
            partial_sums.set_simd_value(Vec4::madd(self._values.get(i).unwrap().get_simd_value(), rhs._values.get(i).unwrap().get_simd_value(), partial_sums.get_simd_value()));
        }
        return partial_sums.dot4(Vector4::create_one().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_zero(&mut self){
        for i in self._values.len() {
            self._values.get_mut(i).unwrap().set_simd_value(Vec4::zero_float())
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub  fn get_vector_values(self) ->Vec<Vector4>{
        let mut result:Vec<Vector4> = Vec::new();
        for i in 0.. self._values.len()
        {
            result.push(self._values.get(i).unwrap().to_owned())
        }
        result
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn on_size_changed(&mut self){
        self._values.resize(((self._size + 3) / 4) as usize,Vector4::create_zero());
        self.fix_last_vector_element();
    }

    #[inline]
    #[allow(dead_code)]
    pub  fn add_f32_vecn(self,lhs:f32,rhs:&VectorN)->VectorN{
        let mut return_value = VectorN::new_i32(rhs.get_dimensionality());
        let lhs_vec = Vector4::new_x(lhs);
        for i in 0..rhs.get_vector_values().len()
        {
            return_value._values.get_mut(i).unwrap().set_simd_value((rhs._values.get(i).unwrap().to_owned() + lhs_vec).get_simd_value());
        }
        return_value.fix_last_vector_element();
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub  fn sub_f32_vecn(self,lhs:f32,rhs:&VectorN)->VectorN{
        let mut return_value = VectorN::new_i32(rhs.get_dimensionality());
        let lhs_vec = Vector4::new_x(lhs);
        for i in 0..rhs.get_vector_values().len()
        {
            return_value._values.get_mut(i).unwrap().set_simd_value((rhs._values.get(i).unwrap().to_owned() - lhs_vec).get_simd_value());
        }
        return_value.fix_last_vector_element();
        return return_value;
    }

    #[inline]
    #[allow(dead_code)]
    pub  fn mul_f32_vecn(self,lhs:f32,rhs:&VectorN)->VectorN{
        let mut return_value = VectorN::new_i32(rhs.get_dimensionality());
        let lhs_vec = Vector4::new_x(lhs);
        for i in 0..rhs.get_vector_values().len()
        {
            return_value._values.get_mut(i).unwrap().set_simd_value((rhs._values.get(i).unwrap().to_owned() * lhs_vec).get_simd_value());
        }
        return_value.fix_last_vector_element();
        return return_value;
    }

}

