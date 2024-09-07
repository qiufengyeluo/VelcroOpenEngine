#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::cmp::{Ordering, PartialEq, PartialOrd};
use crate::math::aabb::Aabb;
use crate::math::common_sse::{VecFourthType, VecTwoType, VecType};
use crate::math::frustum::CornerIndices::{FarBottomLeft, FarBottomRight, FarTopLeft, FarTopRight, NearBottomLeft, NearBottomRight, NearTopLeft, NearTopRight};
use crate::math::frustum::PlaneId::{Bottom, Far, Left, Near, Right, Top};
use crate::math::math_utils::constants::TOLERANCE;
use crate::math::matrix3x3::Matrix3x3;
use crate::math::matrix4x4::Matrix4x4;
use crate::math::plane::{IntersectResult, Plane};
use crate::math::shape_intersection::ShapeIntersection;
use crate::math::simd_math_vec1_sse::Vec1;
use crate::math::simd_math_vec3_sse::Vec3;
use crate::math::simd_math_vec4_sse::Vec4;
use crate::math::sphere::Sphere;
use crate::math::transform::Transform;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;
use crate::math::vsimd::FloatType;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub struct ViewFrustumAttributes
{
    _world_transform:Transform,
     _aspect_ratio: f32,
     _vertical_fov_radians:f32,
     _near_clip:f32,
     _far_clip:f32
}
impl ViewFrustumAttributes{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->ViewFrustumAttributes{
        unsafe {
            ViewFrustumAttributes {
                _world_transform: Transform::create_identity(),
                _aspect_ratio: 0.0,
                _vertical_fov_radians: 0.0,
                _near_clip: 0.0,
                _far_clip: 0.0,
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_transform_and4f32(world_transform:&Transform, aspect_ratio:f32, vertical_fov_radians:f32, near_clip:f32, far_clip:f32) ->ViewFrustumAttributes{
        ViewFrustumAttributes{
            _world_transform: world_transform.to_owned(),
            _aspect_ratio: aspect_ratio,
            _vertical_fov_radians: vertical_fov_radians,
            _near_clip: near_clip,
            _far_clip: far_clip,

        }
    }
}
#[derive(Clone, Copy)]
enum PlaneId
{
    Near,
    Far,
    Left,
    Right,
    Top,
    Bottom,
    MAX,
}

#[derive(TryFromPrimitive)]
#[repr(u32)]
enum ReverseDepth
{
    True,
    False,
}
impl PartialEq<ReverseDepth> for &ReverseDepth {
    fn eq(&self, other: &ReverseDepth) -> bool {
        ReverseDepth::try_from(self) == ReverseDepth::try_from(other)
    }
}
enum CornerIndices
{
    NearTopLeft,
    NearTopRight,
    NearBottomLeft,
    NearBottomRight,
    FarTopLeft,
    FarTopRight,
    FarBottomLeft,
    FarBottomRight,
    Count,
}

#[derive(Debug, Copy, Clone)]
pub struct Frustum {
    _planes:[FloatType;PlaneId::MAX as usize],
    _serialized_planes: [Plane;PlaneId::MAX as usize],
}



impl Frustum{

    #[inline]
    #[allow(dead_code)]
    pub fn new()->Frustum{
        unsafe {
            Frustum {
                _planes: [Vec1::zero_float(), Vec1::zero_float(), Vec1::zero_float(), Vec1::zero_float()],
                _serialized_planes: [Plane; PlaneId::MAX as usize],
            }
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn new_view_frustum_attributes(view_frustum_attributes:&ViewFrustumAttributes) ->Frustum{
        let mut result = Frustum::new();
        result.construct_planes(view_frustum_attributes);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn new_plane(near_plane:&Plane, far_plane:&Plane, left_plane:&Plane, right_plane:&Plane, top_plane:&Plane, bottom_plane:&Plane) ->Frustum{
        let mut result = Frustum::new();
        result.set_plane(PlaneId::Near, near_plane);
        result.set_plane(PlaneId::Far, far_plane);
        result.set_plane(PlaneId::Left, left_plane);
        result.set_plane(PlaneId::Right, right_plane);
        result.set_plane(PlaneId::Top, top_plane);
        result.set_plane(PlaneId::Bottom, bottom_plane);
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix_row_major(matrix:&Matrix4x4, reverse_depth:&ReverseDepth)->Frustum{
        let mut near_plane_id = PlaneId::Far;
        if reverse_depth == ReverseDepth::True{
            near_plane_id = PlaneId::Near;
        }
        let mut far_plane_id =PlaneId::Near;
        if reverse_depth == ReverseDepth::True{
            far_plane_id = PlaneId::Far;
        }

        let mut frustum = Frustum::new();
        frustum.set_plane(near_plane_id, Plane::create_from_vector_coefficients(matrix.get_column(2).borrow()).borrow());
        frustum.set_plane(far_plane_id, Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(2)).borrow()).borrow());
        frustum.set_plane(PlaneId::Left,   Plane::create_from_vector_coefficients((matrix.get_column(3) + matrix.get_column(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Right,  Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Top,    Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(1)).borrow()).borrow());
        frustum.set_plane(PlaneId::Bottom, Plane::create_from_vector_coefficients((matrix.get_column(3) + matrix.get_column(1)).borrow()).borrow());
        frustum
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix_column_major(matrix:&Matrix4x4, reverse_depth:&ReverseDepth)->Frustum{
        let mut near_plane_id = PlaneId::Far;
        if reverse_depth == ReverseDepth::True{
            near_plane_id = PlaneId::Near;
        }
        let mut far_plane_id =PlaneId::Near;
        if reverse_depth == ReverseDepth::True{
            far_plane_id = PlaneId::Far;
        }


        let mut frustum = Frustum::new();
        frustum.set_plane(near_plane_id,     Plane::create_from_vector_coefficients(matrix.get_row(2).borrow()).borrow());
        frustum.set_plane(far_plane_id,      Plane::create_from_vector_coefficients((matrix.get_row(3) - matrix.get_row(2)).borrow()).borrow());
        frustum.set_plane(PlaneId::Left,   Plane::create_from_vector_coefficients((matrix.get_row(3) + matrix.get_row(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Right,  Plane::create_from_vector_coefficients((matrix.get_row(3) - matrix.get_row(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Top,    Plane::create_from_vector_coefficients((matrix.get_row(3) - matrix.get_row(1)).borrow()).borrow());
        frustum.set_plane(PlaneId::Bottom, Plane::create_from_vector_coefficients((matrix.get_row(3) + matrix.get_row(1)).borrow()).borrow());
        frustum
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix_row_major_symmetric_z(matrix:&Matrix4x4,reverse_depth:&ReverseDepth)->Frustum{
        let mut near_plane_id = PlaneId::Far;
        if reverse_depth == ReverseDepth::True{
            near_plane_id = PlaneId::Near;
        }
        let mut far_plane_id =PlaneId::Near;
        if reverse_depth == ReverseDepth::True{
            far_plane_id = PlaneId::Far;
        }

        let mut frustum = Frustum::new();
        frustum.set_plane(near_plane_id,     Plane::create_from_vector_coefficients((matrix.get_column(3) + matrix.get_column(2)).borrow()).borrow());
        frustum.set_plane(far_plane_id,      Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(2)).borrow()).borrow());
        frustum.set_plane(PlaneId::Left,   Plane::create_from_vector_coefficients((matrix.get_column(3) + matrix.get_column(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Right,  Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Top,    Plane::create_from_vector_coefficients((matrix.get_column(3) - matrix.get_column(1)).borrow()).borrow());
        frustum.set_plane(PlaneId::Bottom, Plane::create_from_vector_coefficients((matrix.get_column(3) + matrix.get_column(1)).borrow()).borrow());
        frustum
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn create_from_matrix_column_major_symmetric_z(matrix:&Matrix4x4,reverse_depth:&ReverseDepth)->Frustum{
        let mut near_plane_id = PlaneId::Far;
        if reverse_depth == ReverseDepth::True{
            near_plane_id = PlaneId::Near;
        }
        let mut far_plane_id =PlaneId::Near;
        if reverse_depth == ReverseDepth::True{
            far_plane_id = PlaneId::Far;
        }

        let mut frustum = Frustum::new();
        frustum.set_plane(near_plane_id,     Plane::create_from_vector_coefficients((matrix.get_row(3) + matrix.get_row(2)).borrow()).borrow());
        frustum.set_plane(far_plane_id,      Plane::create_from_vector_coefficients((matrix.get_row(3) - matrix.get_row(2)).borrow()).borrow());
        frustum.set_plane(PlaneId::Left,   Plane::create_from_vector_coefficients((matrix.get_row(3) + matrix.get_row(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Right,  Plane::create_from_vector_coefficients((matrix.get_row(3) - matrix.get_row(0)).borrow()).borrow());
        frustum.set_plane(PlaneId::Top,    Plane::create_from_vector_coefficients((matrix.get_row(3) - matrix.get_row(1)).borrow()).borrow());
        frustum.set_plane(PlaneId::Bottom, Plane::create_from_vector_coefficients((matrix.get_row(3) - matrix.get_row(1)).borrow()).borrow());
        return frustum;
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn get_plane(self, plane_id:PlaneId) ->Plane{
        return Plane::new_float_type(self._planes[plane_id])
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn set_plane(&mut self, plane_id: PlaneId, plane:&Plane){
        self._serialized_planes[plane_id] = plane;

        let length_squared = Vec4::from_vec1(Vec3::dot(plane.get_normal().get_simd_value(), plane.get_normal().get_simd_value()));
        let length = Vec4::sqrt(length_squared);
        self._planes[plane_id] = Vec4::div(plane.get_simd_value(), length);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_sphere_vec3_f32(self,center:&Vector3,radius:f32)->IntersectResult{
        let mut intersect = false;
        for i in PlaneId::Near .. PlaneId::MAX{
            let distance = Vec1::select_index0(Vec4::plane_distance(self._planes[i], center.get_simd_value()));

            if (distance < -radius)
            {
                return IntersectResult::Exterior;
            }

            intersect |= (f32::fabsf(distance) < radius);
        }
        if intersect{
            IntersectResult::Overlaps
        }else {
            IntersectResult::Interior
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_sphere(self,sphere:&Sphere)->IntersectResult{
        return self.intersect_sphere_vec3_f32(sphere.get_center().borrow(),sphere.get_radius());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_aabb_2vec3(self,minimum:&Vector3,maximum:&Vector3)->IntersectResult{
        return  self.intersect_aabb(Aabb::create_from_min_max(minimum, maximum).borrow())
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_aabb(self,aabb:&Aabb)->IntersectResult{
        let mut num_interior = 0u32;

        for  i in PlaneId::Near .. PlaneId::MAX
        {
            let disjoint_support = aabb.get_support(-Vector3::new_float_type(Vec4::value_to_vec3(self._planes[i])));
            let disjoint_distance = Vec1::select_index0(Vec4::plane_distance(self._planes[i], disjoint_support.get_simd_value()));

            if (disjoint_distance < 0.0)
            {
                return IntersectResult::Exterior;
            }

            let intersect_support = aabb.get_support(Vector3::new_float_type(Vec4::value_to_vec3(self._planes[i])).borrow());
            let intersect_distance = Vec1::select_index0(Vec4::plane_distance(self._planes[i], intersect_support.get_simd_value()));

            if (intersect_distance >= 0.)
            {
                // If the whole AABB passes the plane check, increment the number of planes the AABB is 'interior' to
                num_interior += 1;
            }
        }

        if num_interior < PlaneId::MAX as u32{
            IntersectResult::Overlaps
        }else {
            IntersectResult::Interior
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close_default(self,rhs:&Frustum)->bool{
        return self.is_close(rhs,TOLERANCE)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn is_close(self,rhs:&Frustum,tolerance:f32)->bool{
        return Vector4::new_float_type(self._planes[PlaneId::Near  ]).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Near  ]).borrow(), tolerance)
            && Vector4::new_float_type(self._planes[PlaneId::Far   ]).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Far   ]).borrow(), tolerance)
            && Vector4::new_float_type(self._planes[PlaneId::Left  ]).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Left  ]).borrow(), tolerance)
            && Vector4::new_float_type(self._planes[PlaneId::Right ]).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Right ]).borrow(), tolerance)
            && Vector4::new_float_type(self._planes[PlaneId::Top   ]).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Top   ]).borrow(), tolerance)
            && Vector4::new_float_type(self._planes[PlaneId::Bottom]).is_close(Vector4::new_float_type(rhs._planes[PlaneId::Bottom]).borrow(), tolerance);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set(&mut self,frustum:&Frustum){
        for plane_id in PlaneId::Near .. PlaneId::MAX {
            unsafe { self.set_plane(plane_id, frustum._serialized_planes[plane_id]); }
        }
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn construct_planes(&mut self, view_frustum_attributes:&ViewFrustumAttributes){
        let tan_half_fov = f32::tan(view_frustum_attributes._vertical_fov_radians * 0.5);
        let near_plane_half_height = tan_half_fov * view_frustum_attributes._near_clip;
        let near_plane_half_width = near_plane_half_height * view_frustum_attributes._aspect_ratio;

        let translation = unsafe { view_frustum_attributes._world_transform.get_translation() };
        let forward = unsafe { view_frustum_attributes._world_transform.get_basis_y() };
        let right = unsafe { view_frustum_attributes._world_transform.get_basis_x() };
        let up = unsafe { view_frustum_attributes._world_transform.get_basis_z() };

        unsafe {
            self.set_plane(
                PlaneId::Near,
                Plane::create_from_normal_and_point(forward.borrow(), (translation + (forward * view_frustum_attributes._near_clip)).borrow()).borrow());
        }
        unsafe {
            self.set_plane(
                PlaneId::Far,
                Plane::create_from_normal_and_point((-forward).borrow(), (translation + (forward * view_frustum_attributes._far_clip)).borrow()).borrow());
        }

        let left_normal =
            unsafe { (right + forward * (near_plane_half_width / view_frustum_attributes._near_clip)).get_normalized() };
        let right_normal =
            (-right + forward * (near_plane_half_width / view_frustum_attributes._near_clip)).get_normalized();

        self.set_plane(PlaneId::Left, Plane::create_from_normal_and_point(left_normal, translation));
        self.set_plane(PlaneId::Right, Plane::create_from_normal_and_point(right_normal, translation));

        let top_normal =
            (-up + forward * (near_plane_half_height / view_frustum_attributes._near_clip)).get_normalized();
        let bottom_normal =
            unsafe { (up + forward * (near_plane_half_height / view_frustum_attributes._near_clip)).get_normalized() };

        self.set_plane(PlaneId::Top, Plane::CreateFromNormalAndPoint(top_normal, translation));
        self.set_plane(PlaneId::Bottom, Plane::CreateFromNormalAndPoint(bottom_normal, translation));
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn calculate_view_frustum_attributes(self)->ViewFrustumAttributes{

        let mut view_frustum_attributes = ViewFrustumAttributes::new();

        let forward =  Vector4::new_float_type(self._planes[PlaneId::Near]).get_as_vector3().get_normalized();
        let right =
            Vector4::new_float_type(Vec4::sub(self._planes[PlaneId::Left], self._planes[PlaneId::Right])).get_as_vector3().get_normalized();
        let up =
            Vector4::new_float_type(Vec4::sub(self._planes[PlaneId::Bottom], self._planes[PlaneId::Top])).get_as_vector3().get_normalized();

        let orientation = Matrix3x3::create_from_columns(right.borrow(), forward.borrow(), up.borrow());
        let bottom =Plane::new_float_type(self._planes[PlaneId::Bottom]) ;
        let top = Plane::new_float_type(self._planes[PlaneId::Top]);
        let left = Plane::new_float_type(self._planes[PlaneId::Left]);
        let origin = (-Matrix3x3::create_from_rows(bottom.get_normal().borrow(), top.get_normal().borrow(), left.get_normal().borrow()).get_inverse_full()) * Vector3::new_xyz(bottom.get_distance(), top.get_distance(), left.get_distance()).borrow();


        view_frustum_attributes._world_transform = Transform::create_from_matrix3x3and_translation(orientation.borrow(), origin.borrow());

        let origin_dot_forward = origin.dot3(forward.borrow());
        let near_clip = -Vec4::select_index3(self._planes[PlaneId::Near]) - origin_dot_forward;

        view_frustum_attributes._near_clip = near_clip;
        view_frustum_attributes._far_clip = Vec4::SelectIndex3(self._planes[PlaneId::Far]) - origin_dot_forward;

        let left_normal_dot_forward = left.get_normal().dot3(forward.borrow());
        let frustum_near_height =
            2.0 * near_clip * left_normal_dot_forward / f32::sqrt(1.0 - left_normal_dot_forward * left_normal_dot_forward);
        let bottom_normal_dot_forward = bottom.get_normal().dot3(forward);
        let tan_half_fov =
            bottom_normal_dot_forward / f32::sqrt(1.0 - bottom_normal_dot_forward * bottom_normal_dot_forward);
        let frustum_near_width = 2.0 * near_clip * tan_half_fov;

        view_frustum_attributes._aspect_ratio = frustum_near_height / frustum_near_width;
        view_frustum_attributes._vertical_fov_radians = 2.0 * f32::atan(tan_half_fov);

        return view_frustum_attributes;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_corners(self,corners:&CornerVertexArray)->bool{
        return
        ShapeIntersection::intersect_three_planes(self.get_plane(Near).borrow(), self.get_plane(Top).borrow(), self.get_plane(Left).borrow(), corners[NearTopLeft]) &&
            ShapeIntersection::intersect_three_planes(self.get_plane(Near).borrow(), self.get_plane(Top).borrow(), self.get_plane(Right).borrow(), corners[NearTopRight]) &&
            ShapeIntersection::intersect_three_planes(self.get_plane(Near).borrow(), self.get_plane(Bottom).borrow(), self.get_plane(Left).borrow(), corners[NearBottomLeft]) &&
            ShapeIntersection::intersect_three_planes(self.get_plane(Near).borrow(), self.get_plane(Bottom).borrow(), self.get_plane(Right).borrow(), corners[NearBottomRight]) &&
            ShapeIntersection::intersect_three_planes(self.get_plane(Far).borrow(), self.get_plane(Top).borrow(), self.get_plane(Left).borrow(), corners[FarTopLeft]) &&
            ShapeIntersection::intersect_three_planes(self.get_plane(Far).borrow(), self.get_plane(Top).borrow(), self.get_plane(Right).borrow(), corners[FarTopRight]) &&
            ShapeIntersection::intersect_three_planes(self.get_plane(Far).borrow(), self.get_plane(Bottom).borrow(), self.get_plane(Left).borrow(), corners[FarBottomLeft]) &&
            ShapeIntersection::intersect_three_planes(self.get_plane(Far).borrow(), self.get_plane(Bottom).borrow(), self.get_plane(Right).borrow(), corners[FarBottomRight])
        ;
    }
}






