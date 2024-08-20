#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::common_sse::VecType;
use crate::math::constants::FLOAT_MAX;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;

// PartialEq 是否相等
#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    _min: Vector3,
    _max: Vector3
}
impl PartialEq<Self> for Aabb {
    fn eq(&self, other: &Self) -> bool {
        unsafe { return self.get_min() == other.get_min()&&self.get_max() == other.get_max() }
    }

    fn ne(&self, other: &Self) -> bool {
        unsafe { return self.get_min() != other.get_min() || self.get_max() != other.get_max()}
    }
}



impl Aabb {

    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Aabb {
            _min: Vector3::new(),
            _max: Vector3::new(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_null()->Self{
        Aabb{
            _min: Vector3::new_splat(FLOAT_MAX),
            _max: Vector3::new_splat(-FLOAT_MAX),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_point(p: &Vector3)->Aabb{
        let mut aabb :Aabb = Aabb::new();
        aabb._max = *p;
        aabb._min = *p;
        aabb
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_min_max(min:&Vector3,max :&Vector3)->Aabb
    {
        Aabb{
            _max:max.to_owned(),
            _min:min.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_min_max_values(min_x:&f32, min_y:&f32, min_z:&f32, max_x:&f32, max_y:&f32, max_z:&f32) ->Aabb{
        Aabb{
            _min:Vector3::new_xyz(min_x, min_y, min_z),
            _max:Vector3::new_xyz(max_x, max_y, max_z),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_center_half_extents(center:&Vector3, half_extents:&Vector3) ->Aabb{
        Aabb{
            _min:center.to_owned() - half_extents.to_owned(),
            _max:center.to_owned() + half_extents.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_center_radius(center:&Vector3,radius:&f32)->Aabb{
        return Aabb::create_center_half_extents(center,Vector3::new_x(radius).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_points(points:&*const Vector3, point_count:&i32) ->Aabb{
        let mut aabb = Aabb::create_null();
        for i in point_count {
            aabb.add_point((*points[i]).borrow());
        }
        aabb
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_points_list( points:&Vec<Vector3>)->Aabb{
        let mut aabb = Aabb::create_null();
        for auto in points {
            aabb.add_point(auto);
        }
        aabb
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_min(self) -> Vector3 {
        return self._min;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_max(self) -> Vector3 {
        return self._max;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set(mut self,min:&Vector3,max:&Vector3){
        self._min = min.to_owned();
        self._max = max.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_min(mut self,min:&Vector3){
        self._min = min.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_max(mut self,max:&Vector3){
        self._max = max.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_xextent(self)->f32{
        return  self._max.get_x() - self._min.get_x()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_yextent(self)->f32{
        return  self._max.get_y() - self._min.get_y()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_zextent(self)->f32{
        return  self._max.get_z() - self._min.get_z()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_extents(self)->Vector3{
        return  self._max - self._min
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_center(self)->Vector3{
        return  (self._min + self._max) * 0.5f32;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_support(self,normal:&Vector3)->Vector3{
        let select_max_mask = Vec3::cmp_lt(normal.get_simd_value().borrow(), Vec3::zero_float().borrow());
        return Vector3::new_float_type(Vec3::select(self._max.get_simd_value().borrow(), self._min.get_simd_value().borrow(), select_max_mask.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_as_sphere(self,mut center:&Vector3,mut radius:&f32){
        center = self.get_center().borrow_mut();
        radius = (self._max-center.to_owned()).get_length().borrow_mut();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains(self,v:&Vector3)->bool{
        return v.is_greater_equal_than(self._min.borrow())&&v.is_less_equal_than(self._max.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_aabb(self,aabb:&Aabb)->bool{
        return aabb.get_min().is_greater_equal_than(self._min.borrow()) && aabb.get_max().is_less_equal_than(self._max.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps(self,aabb:&Aabb)->bool{
        return self._min.is_less_equal_than(aabb._max.borrow())&& self._max.is_greater_equal_than(aabb._min.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn disjoint(self,aabb: &Aabb)->bool{
        return  !(self._max.is_greater_equal_than(aabb._min.borrow()) &&self._min.is_less_equal_than(aabb._max.borrow()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn expand(mut self,delta:&Vector3){
        self._min -= delta.to_owned();
        self._max += delta.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_expanded(self,delta:&Vector3)->Aabb{
        return Aabb::create_from_min_max((self._min-delta.to_owned()).borrow(),(self._max-delta.to_owned()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn add_point(mut self,p:&Vector3){
        self._min = self._min.get_min(p);
        self._max = self._max.get_max(p);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn add_aabb(mut self,val:&Aabb){
        self._min = self._min.get_min(val.get_min().borrow());
        self._max = self._max.get_max(val.get_max().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance(self,p:&Vector3)->f32{
        let closest = p.get_clamp(self._min.borrow(),self._max.borrow());
        return  p.get_distance(closest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance_sq(self, p:&Vector3) ->f32{
        let closest = p.get_clamp(self._min.borrow(),self._max.borrow());
        return  p.get_distance_sq(closest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_max_distance(self, p:&Vector3)->f32{
        let farthest = Vector3::create_select_cmp_greater_equal(p,self.get_center().borrow(),self._min.borrow(),self._max.borrow());
        return p.get_distance(farthest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_max_distance_sq(self, p:&Vector3) ->f32{
        let farthest = Vector3::create_select_cmp_greater_equal(p,self.get_center().borrow(),self._min.borrow(),self._max.borrow());
        return p.get_distance_sq(farthest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_clamped(self,clamp:&Aabb)->Aabb{
        let mut clamped_aabb = Aabb::create_from_min_max(self._min.borrow(), self._max.borrow());
        clamped_aabb.clamp(clamp.borrow());
        return clamped_aabb;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn clamp(mut self,clamp:&Aabb){
        self._min = self._min.get_clamp(clamp._min.borrow(),clamp._max.borrow());
        self._max = self._max.get_clamp(clamp._min.borrow(),clamp._max.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_null(mut self){
        self._min = Vector3::new_x(FLOAT_MAX.borrow());
        self._max = Vector3::new_x((-FLOAT_MAX).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn translate(mut self,offset:&Vector3){
        self._min += offset.to_owned();
        self._max += offset.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_translated(mut self,offset:&Vector3)->Aabb{
        return  Aabb::create_from_min_max((self._min + offset.to_owned()).borrow(),(self._max+offset.to_owned()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_surface_area(self)->f32{
        let dx = self.get_xextent();
        let dy = self.get_yextent();
        let dz = self.get_zextent();
        return 2.0f32 * (dx * dy + dy * dz + dz * dx);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn multiply_by_scale(mut self,scale:&Vector3){
        self._min *= scale;
        self._max *= scale;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transformed_aabb(self,transform:&Transform)->Aabb{
        let mut aabb = Aabb::create_from_min_max(self._min.borrow(),self._max.borrow());
        aabb.apply_transform(transform);
        aabb
    }



#[inline]
#[allow(dead_code)]
pub unsafe fn get_transformed_aabb_matrix3x4(self,matrix3x4:&Matrix3x4)->Aabb{
    let mut aabb = Aabb::create_from_min_max(self._min.borrow(),self._max.borrow());
    aabb.apply_matrix3x4(matrix3x4);
    aabb
}

#[allow(dead_code)]
pub unsafe fn apply_matrix3x4(&mut self, matrix3x4:&Matrix3x4){
    let new_min = matrix3x4;
    let new_max = new_min;
    let mut axis_index = 0;
    while axis_index < 3 {
        let axis_coeffs = matrix3x4.get_row_as_vector3(axis_index);
        let projected_contributions_from_min = axis_coeffs * self._min;
        let projected_contributions_from_max = axis_coeffs * self._max;
        new_min.set_element(axis_index, new_min.get_element(axis_index)+ projected_contributions_from_min.get_min(projected_contributions_from_max).dot(Vector3::create_one().borrow()));
        new_max.set_element(axis_index, new_max.get_element(axis_index)+ projected_contributions_from_min.get_max(projected_contributions_from_max).dot(Vector3::create_one().borrow()));
        axis_index +=1;
    }
    self._min = new_min;
    self._max = new_max;
}

#[inline]
#[allow(dead_code)]
pub unsafe fn is_close(self, rhs :&Aabb, tolerance:&f32 ) ->bool
    {
        return self._min.is_close(rhs._min.borrow(), tolerance) && self._max.is_close(rhs._max.borrow(), tolerance);
    }

#[inline]
#[allow(dead_code)]
pub fn is_valid(self) ->bool
    {
        return self._min.is_less(self._max);
    }

#[inline]
#[allow(dead_code)]
pub unsafe fn is_finite(self) ->bool
    {
        return self._min.is_finite() && self._max.is_finite();
    }
}