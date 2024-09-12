#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::common_sse::VecType;
use crate::math::math_utils::constants;
use crate::math::matrix3x4::Matrix3x4;
use crate::math::obb::Obb;
use crate::math::quaternion::Quaternion;
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
    pub fn create_null() -> Self {
        Aabb {
            _min: Vector3::new_splat(constants:: FLOAT_MAX),
            _max: Vector3::new_splat(-constants::FLOAT_MAX),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_point(p: &Vector3) -> Aabb {
        let mut aabb: Aabb = Aabb::new();
        aabb._max = *p;
        aabb._min = *p;
        aabb
    }

    #[inline]
    #[allow(dead_code)]
    pub fn create_from_min_max(min: &Vector3, max: &Vector3) -> Aabb
    {
        Aabb {
            _max: max.to_owned(),
            _min: min.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_min_max_values(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Aabb {
        Aabb {
            _min: Vector3::new_xyz(min_x, min_y, min_z),
            _max: Vector3::new_xyz(max_x, max_y, max_z),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_center_half_extents(center: &Vector3, half_extents: &Vector3) -> Aabb {
        Aabb {
            _min: center.to_owned() - half_extents.to_owned(),
            _max: center.to_owned() + half_extents.to_owned(),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_center_radius(center: &Vector3, radius: f32) -> Aabb {
        return Aabb::create_center_half_extents(center, Vector3::new_x(radius).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_points(points: &*const Vector3, point_count: &i32) -> Aabb {
        let mut aabb = Aabb::create_null();
        for i in point_count {
            aabb.add_point((*points[i]).borrow());
        }
        aabb
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_points_list(points: &Vec<Vector3>) -> Aabb {
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
    pub unsafe fn set(mut self, min: &Vector3, max: &Vector3) {
        self._min = min.to_owned();
        self._max = max.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_min(mut self, min: &Vector3) {
        self._min = min.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_max(mut self, max: &Vector3) {
        self._max = max.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_xextent(self) -> f32 {
        return self._max.get_x() - self._min.get_x()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_yextent(self) -> f32 {
        return self._max.get_y() - self._min.get_y()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_zextent(self) -> f32 {
        return self._max.get_z() - self._min.get_z()
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_extents(self) -> Vector3 {
        return self._max - self._min
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_center(self) -> Vector3 {
        return (self._min + self._max) * 0.5f32;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_support(self, normal: &Vector3) -> Vector3 {
        let select_max_mask = Vec3::cmp_lt(normal.get_simd_value(), Vec3::zero_float());
        return Vector3::new_float_type(Vec3::select(self._max.get_simd_value(), self._min.get_simd_value(), select_max_mask));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_as_sphere(self, mut center: &Vector3, mut radius: &f32) {
        center = self.get_center().borrow_mut();
        radius = (self._max - center.to_owned()).get_length().borrow_mut();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains(self, v: &Vector3) -> bool {
        return v.is_greater_equal_than(self._min.borrow()) && v.is_less_equal_than(self._max.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_aabb(self, aabb: &Aabb) -> bool {
        return aabb.get_min().is_greater_equal_than(self._min.borrow()) && aabb.get_max().is_less_equal_than(self._max.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps(self, aabb: &Aabb) -> bool {
        return self._min.is_less_equal_than(aabb._max.borrow()) && self._max.is_greater_equal_than(aabb._min.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn disjoint(self, aabb: &Aabb) -> bool {
        return !(self._max.is_greater_equal_than(aabb._min.borrow()) && self._min.is_less_equal_than(aabb._max.borrow()));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn expand(mut self, delta: &Vector3) {
        self._min -= delta.to_owned();
        self._max += delta.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_expanded(self, delta: &Vector3) -> Aabb {
        return Aabb::create_from_min_max((self._min - delta.to_owned()).borrow(), (self._max - delta.to_owned()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn add_point(mut self, p: &Vector3) {
        self._min = self._min.get_min(p);
        self._max = self._max.get_max(p);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn add_aabb(mut self, val: &Aabb) {
        self._min = self._min.get_min(val.get_min().borrow());
        self._max = self._max.get_max(val.get_max().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance(self, p: &Vector3) -> f32 {
        let closest = p.get_clamp(self._min.borrow(), self._max.borrow());
        return p.get_distance(closest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_distance_sq(self, p: &Vector3) -> f32 {
        let closest = p.get_clamp(self._min.borrow(), self._max.borrow());
        return p.get_distance_sq(closest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_max_distance(self, p: &Vector3) -> f32 {
        let farthest = Vector3::create_select_cmp_greater_equal(p, self.get_center().borrow(), self._min.borrow(), self._max.borrow());
        return p.get_distance(farthest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_max_distance_sq(self, p: &Vector3) -> f32 {
        let farthest = Vector3::create_select_cmp_greater_equal(p, self.get_center().borrow(), self._min.borrow(), self._max.borrow());
        return p.get_distance_sq(farthest.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_clamped(self, clamp: &Aabb) -> Aabb {
        let mut clamped_aabb = Aabb::create_from_min_max(self._min.borrow(), self._max.borrow());
        clamped_aabb.clamp(clamp.borrow());
        return clamped_aabb;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn clamp(mut self, clamp: &Aabb) {
        self._min = self._min.get_clamp(clamp._min.borrow(), clamp._max.borrow());
        self._max = self._max.get_clamp(clamp._min.borrow(), clamp._max.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn set_null(mut self) {
        self._min = Vector3::new_x(constants::FLOAT_MAX);
        self._max = Vector3::new_x((-constants::FLOAT_MAX));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn translate(mut self, offset: &Vector3) {
        self._min += offset.to_owned();
        self._max += offset.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_translated(mut self, offset: &Vector3) -> Aabb {
        return Aabb::create_from_min_max((self._min + offset.to_owned()).borrow(), (self._max + offset.to_owned()).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_surface_area(self) -> f32 {
        let dx = self.get_xextent();
        let dy = self.get_yextent();
        let dz = self.get_zextent();
        return 2.0f32 * (dx * dy + dy * dz + dz * dx);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn multiply_by_scale(mut self, scale: &Vector3) {
        self._min *= scale;
        self._max *= scale;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transformed_aabb(self, transform: &Transform) -> Aabb {
        let mut aabb = Aabb::create_from_min_max(self._min.borrow(), self._max.borrow());
        aabb.apply_transform(transform);
        aabb
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transformed_aabb_matrix3x4(self, matrix3x4: &Matrix3x4) -> Aabb {
        let mut aabb = Aabb::create_from_min_max(self._min.borrow(), self._max.borrow());
        aabb.apply_matrix3x4(matrix3x4);
        aabb
    }

    #[allow(dead_code)]
    pub unsafe fn apply_matrix3x4(&mut self, matrix3x4: &Matrix3x4) {
        let mut new_min = matrix3x4.get_translation();
        let mut new_max = new_min;
        for axis_index in 3 {
            let axis_coeffs = matrix3x4.get_row_as_vector3(axis_index);
            let projected_contributions_from_min = axis_coeffs * self._min;
            let projected_contributions_from_max = axis_coeffs * self._max;
            new_min.set_element(axis_index, new_min.get_element(axis_index) + projected_contributions_from_min.get_min(projected_contributions_from_max.borrow()).dot3(Vector3::create_one().borrow()));
            new_max.set_element(axis_index, new_max.get_element(axis_index) + projected_contributions_from_min.get_max(projected_contributions_from_max.borrow()).dot3(Vector3::create_one().borrow()));
        }
        self._min = new_min;
        self._max = new_max;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(self, rhs: &Aabb, tolerance: f32) -> bool
    {
        return self._min.is_close(rhs._min.borrow(), tolerance) && self._max.is_close(rhs._max.borrow(), tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_valid(self) -> bool
    {
        return self._min.is_less(self._max);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_finite(self) -> bool
    {
        return self._min.is_finite() && self._max.is_finite();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn create_from_obb(obb: &Obb) -> Aabb {
        let mut tmp: [Vector3; 8] = [Vector3::new(), Vector3::new(), Vector3::new(), Vector3::new(), Vector3::new(), Vector3::new(), Vector3::new(), Vector3::new()];
        tmp[0] = obb.get_position() + obb.get_axis_x() * obb.get_half_length_x()
            + obb.get_axis_y() * obb.get_half_length_y()
            + obb.get_axis_z() * obb.get_half_length_z();
        tmp[1] = tmp[0] - obb.get_axis_z() * (2.0 * obb.get_half_length_z());
        tmp[2] = tmp[0] - obb.get_axis_x() * (2.0 * obb.get_half_length_x());
        tmp[3] = tmp[1] - obb.get_axis_x() * (2.0 * obb.get_half_length_x());
        tmp[4] = tmp[0] - obb.get_axis_y() * (2.0 * obb.get_half_length_y());
        tmp[5] = tmp[1] - obb.get_axis_y() * (2.0 * obb.get_half_length_y());
        tmp[6] = tmp[2] - obb.get_axis_y() * (2.0 * obb.get_half_length_y());
        tmp[7] = tmp[3] - obb.get_axis_y() * (2.0 * obb.get_half_length_y());
        let mut min = tmp[0].to_owned();
        let mut max = tmp[0].to_owned();
        for i in 0..8 {
            min = min.get_min(tmp[i].borrow());
            max = max.get_max(tmp[i].borrow());
        }

        return Aabb::create_from_min_max(min.borrow(), max.borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transformed_obb_transform(self, transform: &Transform) -> Obb {
        let temp = Obb::create_from_aabb(self.borrow());
        return temp * transform;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_transformed_obb_matrix3x4(self, matrix3x4: &Matrix3x4)->Obb {
        let mut matrix_no_scale = matrix3x4.to_owned();
        let scale = matrix_no_scale.extract_scale();
        let rotation = Quaternion::create_from_matrix3x4(matrix_no_scale.borrow());

        return Obb::create_from_position_rotation_and_half_lengths(
            (matrix3x4 *self.get_center()).borrow(),
            rotation.borrow(),
            ((scale * self.get_extents()) * 0.5).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn apply_transform(&mut self, transform: &Transform) {
        let mut new_min = transform.get_translation();
        let mut new_max = new_min.to_owned();
        for axisIndex in 3 {
            let mut axis = Vector3::create_zero();
            axis.set_element(axisIndex, 1.0);
            let axis_coeffs = (transform.get_rotation().get_conjugate().transform_vector(axis.borrow())) * transform.get_uniform_scale();
            let projected_contributions_from_min = axis_coeffs * self._min;
            let projected_contributions_from_max = axis_coeffs * self._max;

            new_min.set_element(
                axisIndex,
                (new_min.get_element(axisIndex) +
                    projected_contributions_from_min.get_min(projected_contributions_from_max.borrow()).dot3(Vector3::create_one().borrow())));
            new_max.set_element(
                axisIndex,
                (new_max.get_element(axisIndex) +
                    projected_contributions_from_min.get_max(projected_contributions_from_max.borrow()).dot3(Vector3::create_one().borrow())));
        }

        self._min = new_min;
        self._max = new_max;
    }
}



