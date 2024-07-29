#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::ops::*;
#[cfg(target_arch = "arm")]
#[allow(dead_code)]
use vsimd::neon::*;

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
use vsimd::sse::*;

use crate::math::vector::*;
use crate::math::*;
use crate::math::constants::*;
use crate::math::simd_math::*;
use crate::math::math_utils::*;
// PartialEq 是否相等
#[derive(Debug,Eq, Copy, Clone)]
pub struct Vector3 {
    _value: FloatType,
}

impl Vector3 {
    #[allow(dead_code)]
    pub fn new() -> Vector3 {
        unsafe {
            Vector3 {
                _value: zero_float(),
            }
        }
    }

    pub unsafe fn new_x(x:&f32) ->Vector3{
       Vector3{
           _value:splat(x.to_owned()),
       }
    }

    pub unsafe fn new_xyz(x:&f32, y:&f32, z:&f32) ->Vector3{
        Vector3{
            _value:load_immediate(x.to_owned(),y.to_owned(),z.to_owned(),0.0),
        }
    }
    pub fn new_float_type(v :  &FloatType)->Vector3{
        Vector3{
            _value:v.to_owned(),
        }
    }
    pub unsafe fn create_zero() ->Vector3{
        let result:Vector3 = Self.new_float_type(zero_float());
        result
    }
    
    pub fn create_one()->Vector3{
        let result:Vector3 = Self.new_splat(1.0);
        result
    }
    pub fn create_axis_x(length:f32)->Vector3{
        let result:Vector3 = Self.new_load_immediate(length, 0.0, 0.0);
        result
    }
    pub fn create_axis_y(length:f32)->Vector3{
        let result:Vector3 = Self.new_load_immediate(0.0, length, 0.0);
        result
    }
    pub fn create_axis_z(length:f32)->Vector3{
        let result:Vector3 = Self.new_load_immediate(0.0, 0.0, length);
        result
    }
    pub fn create_from_float_3(ptr :*const f32)->Vector3{
        let val =ptr as *[f32;3];
        let result:Vector3 = Self.new_load_immediate(val[0], val[1], val[2]);
        result
    }
    pub unsafe fn create_select_cmp_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = cmp_eq(cmp1._value, cmp2._value);
        let result = Self.new_float_type( select(va._value,vb._value,mask));
        result
    }
    pub unsafe fn create_select_cmp_greater_equal(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = cmp_gt_eq(cmp1._value, cmp2._value);
        let result = Self.new_float_type( select(va._value,vb._value,mask));
        result
    }
    pub unsafe fn create_select_cmp_greater(cmp1:&Vector3, cmp2:&Vector3, va :&Vector3, vb :&Vector3) ->Vector3{
        let mask = cmp_gt(cmp1._value, cmp2._value);
        let result = Self.new_float_type(select(va._value,vb._value,mask));
        result
    }
    pub fn store_to_float_3(self, &mut  ptr :*const f32){
        let mut result = ptr as *[f32;3];
        let values:*const [f32;3] = (&self._value) as *const [f32;3];

        *result[0] = values[0];
        *result[1] = values[1];
        *result[2] = values[2];
    }
    pub unsafe fn store_to_float_4(self, &mut value :*const f32){
        store_unaligned(value, self._value);
    }
    pub fn get_x(self)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[0]
    }
    pub fn get_y(self)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[1]
    }
    pub fn get_z(self)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[2]
    }
    pub fn get_element(self,index:i32)->f32{
        let values:*const [f32;3] = (&self._value) as *const [f32;3];
        values[index]
    }
    pub fn get_simd_value(&self)->FloatType{
        self._value
    }

    pub fn set_x(mut self, x :f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[0] = x
    }
    pub fn set_y(mut self, y:f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[1] = y
    }
    pub fn set_z(mut self, z:f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[2] = z
    }
    pub unsafe fn set_value_x(mut self, x :f32){
        self._value = splat(x);
    }
    pub fn set_element(mut self,index:i32,v:f32){
        let mut values:*const [f32;3] = (&self._value) as *const [f32;3];
        *values[index] = v
    }
    pub unsafe fn set_value_xyz(mut self, x:f32, y:f32, z:f32){
        self._value = load_immediate(x, y, z, 0.0);
    }
    pub unsafe fn set_value_ptr(mut self, ptr:*const f32){
        let val= ptr as [f32;3];
        self._value = load_immediate(val[0],val[1],val[2], 0.0);
    }
    pub unsafe fn get_length_sq(&self) ->f32{
        let result =  dot_to_f32(self,&Vector3{_value:self._value});
        result
    }

    pub unsafe fn get_length(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_estimate(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt_estimate(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_reciprocal(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt_inv(length);
        let result = select_first(length_sqrt);
        result
    }
    pub unsafe fn get_length_reciprocal_estimate(self) ->f32{
        let length = dot_to_f32_type(self._value,self._value);
        let length_sqrt =  sqrt_inv_estimate(length);
        let result = select_first(length_sqrt);
        result
    }

    pub unsafe fn get_normalized(self) ->Vector3{
        let result = Vector3::new_float_type(normalize(self._value).borrow());
        result
    }
    pub unsafe fn get_normalized_estimate(self)->Vector3{
        let result = Vector3::new_float_type( normalize_estimate(self._value).borrow());
        result
    }
    pub unsafe fn normalize(mut self){
        self = self.get_normalized();
    }
    pub unsafe fn normalize_estimate(mut self){
        self = self.get_normalized_estimate();
    }
    pub unsafe fn normalize_with_length(mut self)->f32{
        let dot_val = dot_to_f32_type(self._value,self._value);
        let sqrt_val = sqrt(dot_val);
        let length = select_first(sqrt_val);
        let splat_val = splat(length);
        self._value = div(self._value,splat_val);
        length
    }
    pub unsafe fn normalize_with_length_estimate(mut self)->f32{
        let dot_val = dot_to_f32_type(self._value,self._value);
        let sqrt_val = sqrt_estimate(dot_val);
        let length = select_first(sqrt_val);
        let splat_val = splat(length);
        self._value = div(self._value,splat_val);
        length
    }
    pub unsafe fn get_normalized_safe( self,tolerance:f32)->Vector3{
        let result = Vector3::new_float_type(normalize_safe(self._value,tolerance).borrow());
        result
    }
    pub unsafe fn get_normalized_safe_estimate(self,tolerance:f32)->Vector3{
        let result = Vector3::new_float_type(normalize_safe_estimate(self._value,tolerance).borrow());
        result
    }
    pub unsafe fn normalize_safe(mut self, tolerance:f32){
        self._value = normalize_safe(self._value,tolerance)
    }
    pub unsafe fn normalize_safe_estimate(mut self, tolerance:f32){
        self._value = normalize_safe_estimate(self._value,tolerance);
    }
    pub unsafe fn normalize_safe_with_length(mut self, tolerance:f32)->f32{
        let length = sqrt( dot_to_f32_type(self._value,self._value));
        if select_first(length) < tolerance{
            self._value = zero_float();
        }else {
            self._value = div(self._value,splat_first(from_vec_first(length)));
        }
        let result = select_first(length);
        result
    }
    pub unsafe fn normalize_safe_with_length_estimate(mut self, tolerance:f32) ->f32{
        let length = sqrt_estimate(dot_to_f32_type(self._value,self._value));
        if select_first(length) < tolerance{
            self._value = zero_float();
        }else {
            self._value = div(self._value,splat_first(from_vec_first(length)));
        }
        let result = select_first(length);
        result
    }
    pub unsafe fn is_normalized(self,tolerance:f32)->bool{


        let splat_val = splat(self.get_length_sq()-1.0);
        let abs_val = abs(splat_val);
        let val = select_first(abs_val);
        return val <= tolerance;
    }
    pub unsafe fn set_length(mut self, length:f32){
        let scale =   self.get_length_reciprocal() * length;
        self *= splat(scale) ;
    }
    pub unsafe fn set_length_estimate(mut self, length:f32){
        let scale = length* self.get_length_reciprocal_estimate();
        self *= splat(scale) ;
    }
    pub unsafe fn get_distance_sq(&mut self, v :&Vector3)->f32{
        self -= v;
        let result = self.get_length_sq();
        result
    }
    pub unsafe fn get_distance(&mut self, v :&Vector3)->f32{
        self -= v;
        let result = self.get_length();
        result
    }
    pub unsafe fn get_distance_estimate(&mut self, v :&Vector3)->f32{
        self -= v;
        let result = self.get_length_estimate();
        result
    }
    pub unsafe fn lerp(self,dest :&Vector3,t :&f32)->Vector3{
        let sub_val = sub(dest._value,self._value);
        let splat_val = splat(t.to_owned());

        Vector3{
            _value : madd(sub_val,splat_val,self._value)
        }
    }
    pub unsafe fn slerp(self,dest :&Vector3,t :f32)->Vector3{
        let dot_val = clamp(dot_to_f32_type(self._value,dest._value),splat(-1.0),splat(1.0));
        let theta = mul(acos(dot_val.borrow()),splat(t));
        let relative_vec = sub(dest.get_simd_value(), mul(self.get_simd_value(), from_vec_first(dot_val)));
        let rel_vec_norm = normalize_safe(relative_vec, TOLERANCE);
        let sin_cos = from_vec_second(sin_cos_float_type(theta));
        let rel_vec_sin_theta = mul(rel_vec_norm, splat_first(sin_cos));
        let result = Vector3::new_float_type(madd(self.get_simd_value(), splat_first(sin_cos), rel_vec_sin_theta).borrow());
        result
    }
    pub unsafe fn nlerp(self, dest :&Vector3,t:&f32)->Vector3{
        let result = self.lerp(dest.borrow(),t);
        return  result.get_normalized_safe(TOLERANCE);
    }
    pub unsafe fn dot_f32(self,rhs:&Vector3)->f32{
        return select_first(dot_to_f32_type(self.get_simd_value(),rhs.get_simd_value()));
    }

    pub unsafe fn cross(self,rhs :&Vector3)->Vector3{
        let result = Vector3::new_float_type(cross_f32_type(self.get_simd_value().borrow(),rhs.get_simd_value().borrow()).borrow());
        result
    }
    pub unsafe fn cross_x_axis(self)->Vector3{
        return Vector3::new_load_immediate(0.0,self.get_z(),-self.get_y());
    }
    pub unsafe fn cross_y_axis(self)->Vector3{
        return  Vector3::new_load_immediate(-self.get_z(),0.0,self.get_z());
    }
    pub unsafe fn cross_z_axis(self)->Vector3{
        return  Vector3::new_load_immediate(self.get_y(),-self.get_x(),0.0);
    }
    pub unsafe fn x_axis_cross(self)->Vector3{
        return Vector3::new_load_immediate(0.0,-self.get_z(),self.get_y());
    }
    pub unsafe fn y_axis_cross(self)->Vector3{
        return Vector3::new_load_immediate(self.get_z(),0.0,-self.get_x());
    }
    pub unsafe fn z_axis_cross(self)->Vector3{
        return  Vector3::new_load_immediate(-self.get_y(),self.get_x(),0.0);
    }

    pub fn is_close(&self, v:&Vector3, tolerance :&f32) ->bool
    {
        let dist:Vector3 = (v - (*self)).get_abs();
        return dist.is_less_equal_than(Self.new_splat(tolerance));
    }
    pub fn is_close_with_default(&self, v:&Vector3)->bool{
        let dist:Vector3 = (v - (*self)).get_abs();
        return dist.is_less_equal_than(Self.new_splat(TOLERANCE));
    }
    pub unsafe fn is_zero(self, tolerance:&f32) ->bool{
        let dist = self.get_abs();
        return  dist.is_less_equal_than(Self.new_splat(tolerance));
    }
    pub unsafe fn is_zero_with_default(self)->bool{
        let dist = self.get_abs();
        return  dist.is_less_equal_than(Self.new_splat(FLOAT_EPSILON));
    }
    pub unsafe fn is_less_than(self, rhs :&Vector3)->bool{
        return cmp_all_lt(self.get_simd_value(),rhs.get_simd_value(),0b0111);
    }
    pub unsafe fn is_less_equal_than(rhs:&Vector3) ->bool
    {
        return cmp_all_lt_eq(Self._value, rhs._value, 0b0111);
    }
    pub unsafe fn is_greater_than(self,rhs:&Vector3)->bool{
        return  cmp_all_gt(self.get_simd_value(),rhs.get_simd_value(),0b0111);
    }
    pub unsafe fn is_greater_equal_than(self,rhs:&Vector3)->bool{
        return  cmp_all_gt_eq(self.get_simd_value(),rhs.get_simd_value(),0b0111);
    }
    pub unsafe fn get_floor(self)->Vector3{
        return Vector3::new_float_type(floor(self.get_simd_value()).borrow()) ;
    }
    pub unsafe fn get_ceil(self)->Vector3{
        return Vector3::new_float_type(ceil(self.get_simd_value()).borrow()) ;
    }
    pub unsafe fn get_round(self)->Vector3{
        return  Vector3::new_float_type(round(self.get_simd_value()).borrow()) ;
    }
    pub unsafe fn get_min(self,v :&Vector3)->Vector3{
        return  Vector3::new_float_type(min(self.get_simd_value(),v.get_simd_value()).borrow()) ;
    }
    pub unsafe fn get_max(self,v :&Vector3)->Vector3{
        return  Vector3::new_float_type(max(self.get_simd_value(),v.get_simd_value()).borrow()) ;
    }
    pub unsafe fn get_clamp(self, min:&Vector3,max:&Vector3)->Vector3{
        let min_val = self.get_min(max);
        return min_val.get_max(min);
    }
    pub unsafe fn get_max_element(self)->f32{
       return  max_f32(self.get_x().borrow(),max_f32(self.get_y().borrow(),self.get_z().borrow()).borrow());
    }
    pub unsafe fn get_min_element(self)->f32{
        return min_f32(self.get_x().borrow(),min_f32(self.get_y().borrow(),self.get_z().borrow()).borrow());
    }
    pub  unsafe fn get_sin(self)->Vector3{
        return Vector3::new_float_type(sin(self.get_simd_value()).borrow());
    }
    pub unsafe fn get_cos(self)->Vector3{
        return  Vector3::new_float_type(cos(self.get_simd_value()).borrow())
    }

    pub unsafe fn get_sin_cos(self,sin:&Vector3, cos :&Vector3){
        sin_cos(self.get_simd_value(),sin.get_simd_value().borrow(),cos.get_simd_value().borrow());
    }
    pub unsafe fn get_acos(self)->Vector3{
        return Vector3::new_float_type(acos(self.get_simd_value().borrow()).borrow())
    }
    pub unsafe fn get_atan(self)->Vector3{
        return Vector3::new_float_type(atan(self.get_simd_value().borrow()).borrow())
    }
    pub unsafe fn get_angle_mod(self)->Vector3{
        return  Vector3::new_float_type(angle_mod(self.get_simd_value().borrow()).borrow());
    }
    pub unsafe fn angle(self, v:&Vector3)->f32{
        let cos =self.dot_f32(v.borrow())*simd_inv_sqrt((self.get_length_sq()*v.get_length_sq()).borrow());
        let res = simd_acos(get_clamp(cos.borrow(), (-1.0) .borrow(), 1.0.borrow()));
        res
    }
    pub unsafe fn angle_deg(self,v:&Vector3)->f32{
        return rad_to_deg(self.angle(v).borrow())
    }

    pub unsafe fn angle_safe(self, v:&Vector3)->f32{
       return   if !self.is_zero_with_default()&&!v.is_zero_with_default(){
            let result = self.angle(v.borrow());
           result
        }else {
            0.0
         }
    }

    pub unsafe fn angle_safe_deg(self,v:&Vector3)->f32{
        return if !self.is_zero_with_default() && !v.is_zero_with_default() {
            let result = self.angle_deg(v);
            result
        }else{
            0.0
        }
    }

    pub unsafe fn get_abs(self)->Vector3{
        return  Vector3::new_float_type(abs(self.get_simd_value()).borrow());
    }

    pub unsafe fn get_reciprocal(self)->Vector3{
        return  Vector3::new_float_type(reciprocal(self.get_simd_value()).borrow());
    }

    pub unsafe fn get_reciprocal_estimate(self)->Vector3{
        return Vector3::new_float_type(reciprocal_estimate(self.get_simd_value()).borrow());
    }

    pub unsafe fn get_m_add(self,mul :&Vector3,add :&Vector3)->Vector3{
        return  Vector3::new_float_type(madd(self.get_simd_value(),mul.get_simd_value(),add.get_simd_value()).borrow());
    }

    pub unsafe fn m_add(mut self,mul :&Vector3,add :&Vector3){
        self._value = self.get_m_add(mul,add)._value;
    }

    pub unsafe fn is_perpendicular(self,v:&Vector3,tolerance:&f32)->bool{
        let abs_length_sq = abs(dot_to_f32_type(self.get_simd_value(), v.get_simd_value()));
        return  select_first(abs_length_sq)< tolerance.to_owned();
    }

    pub unsafe fn get_orthogonal_vector(self)->Vector3{
        let mut axis:Vector3 = Vector3::new();
        let val = (self.get_x() * self.get_x());
        if val < 0.5 * self.get_langth_sq(){
            axis = Vector3::create_axis_x(1.0);
        }else{
            axis = Vector3::create_axis_y(1.0);
        }
        return self.cross(axis.borrow());
    }
    pub unsafe fn project(&mut self, rhs: &mut Vector3){
        let div_val = self.dot_f32(rhs.borrow())/ rhs.dot_f32(rhs.borrow());
        let tmp_val = rhs.mul(Vector3::new_splat(div_val).borrow());
        self._value = tmp_val._value;
    }
    pub unsafe fn project_on_normal(mut self,mut normal :&Vector3){
        normal.mul_assign(self.dot_f32(normal));
        self._value = normal._value;
    }
    pub  unsafe fn get_projected(self,mut rhs:&Vector3)->Vector3{
        let result = rhs.mul (Vector3::new_splat(self.dot_f32(rhs)/rhs.dot_f32(rhs)).borrow());
        result
    }
    pub unsafe fn get_projected_on_normal(self,normal:&Vector3)->Vector3{
        let result = normal.mul(Vector3::new_splat(self.dot_f32(normal)).borrow());
        result
    }
    pub unsafe fn is_finite(self)->bool{
        return is_finite_float(self.get_x().borrow())&&is_finite_float(self.get_y().borrow())&&is_finite_float(self.get_z().borrow());
    }
    pub fn set_simd_value(mut self, value :FloatArgType ){
        self._value = value;
    }
    pub unsafe fn vector3rad_to_deg(mut self,radians:&Vector3)->Vector3{
        return Vector3::new_float_type(mul(radians.get_simd_value(),splat(180.0/PI)).borrow());
    }
    pub unsafe fn vector3deg_to_rad(self,degrees:&Vector3)->Vector3{
        return  Vector3::new_float_type(mul(degrees.get_simd_value(),splat(PI/180.0)).borrow());
    }
}

impl PartialEq<Self> for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return cmp_all_eq(self._value, other._value, 0b0111); }
    }
    fn ne(&self, other: &Self) -> bool {
        unsafe { return !cmp_all_eq(self._value, other._value, 0b0111); }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self { _value: add(self._value, rhs._value) } }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self { _value: sub(self._value, rhs._value) } }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        unsafe { Self { _value: mul(self._value, rhs._value) } }
    }

}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Self { _value: div(self._value, rhs._value) } }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self = self + rhs;
    }
    /* */
}

impl SubAssign<Vector3> for  Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self = self - rhs;
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self = self * rhs;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self = Vector3::new_float_type(mul(self.get_simd_value(), splat(rhs)).borrow()).to_owned().borrow_mut(); }
    }
}
impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        self = self / rhs;
    }
}
impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self = Vector3::new_float_type(div(self.get_simd_value(), splat(rhs)).borrow()).to_owned().borrow_mut(); }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ve3_get_length() {
        let vec3 = Vector3::new();
        unsafe { println!("crc32 from string:{}", vec3.get_length()); }
    }
}