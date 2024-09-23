#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use std::borrow::Borrow;
use std::cmp::PartialEq;

use crate::math::aabb::Aabb;
use crate::math::capsule::Capsule;
use crate::math::frustum::Frustum;
use crate::math::hemisphere::Hemisphere;
use crate::math::intersect::intersect_point::Intersect;
use crate::math::math_utils::constants;
use crate::math::math_utils::constants::FLOAT_EPSILON;
use crate::math::obb::Obb;
use crate::math::plane::{IntersectResult, Plane};
use crate::math::simd_math::simd;
use crate::math::sphere::Sphere;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;

#[derive(Debug,Copy, Clone)]
pub struct ShapeIntersection {
    _plane: Vector4
}
const FLT_MIN_F32: f32 = 1.175494351e-38f32;

impl PartialEq for IntersectResult {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl ShapeIntersection{

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_three_planes(p1:&Plane, p2:&Plane, p3:&Plane, mut out_p: &Vector3) ->bool{
        let n1cross_n2 = p1.get_normal().cross(p2.get_normal().borrow());
        let det = n1cross_n2.dot3(p3.get_normal().borrow());
        if (constants::get_abs_f32(det) >  FLT_MIN_F32)
        {
            let n3cross_n2 = p3.get_normal().cross(p2.get_normal().borrow());
            let n1cross_n3 = p1.get_normal().cross(p3.get_normal().borrow());
            out_p = ((n3cross_n2 * p1.get_distance() + n1cross_n3 * p2.get_distance() - n1cross_n2 * p3.get_distance()) / det).borrow_mut();
            return true;
        }

        return false;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn classify_plane_and_sphere(plane:&Plane,sphere:&Sphere)->IntersectResult{
        let distance = plane.get_point_dist(sphere.get_center().borrow());
        let radius = sphere.get_radius();
        if (distance < -radius)
        {
            return IntersectResult::Exterior;
        }
        else if (distance > radius)
        {
            return IntersectResult::Interior;
        }
        else
        {
            return IntersectResult::Overlaps;
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn classify_plane_and_obb(plane:&Plane,obb:&Obb)->IntersectResult{
        let d = plane.get_point_dist(obb.get_position().borrow());
        let r = obb.get_half_length_x() * constants::get_abs_f32(plane.get_normal().dot3(obb.get_axis_x().borrow()))
            + obb.get_half_length_y() * constants::get_abs_f32(plane.get_normal().dot3(obb.get_axis_y().borrow()))
            + obb.get_half_length_z() * constants::get_abs_f32(plane.get_normal().dot3(obb.get_axis_z().borrow()));
        if (d < -r)
        {
            return IntersectResult::Exterior;
        }
        else if (d > r)
        {
            return IntersectResult::Interior;
        }
        else
        {
            return IntersectResult::Overlaps;
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn classify_frustum_and_sphere(frustum:&Frustum,sphere:&Sphere)->IntersectResult{
        return frustum.intersect_sphere(sphere);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_aabb_and_aabb(aabb1:&Aabb,aabb2:&Aabb)->bool{
        return aabb1.overlaps(aabb2)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_aabb_and_sphere(aabb:&Aabb, sphere:&Sphere) ->bool{
        return ShapeIntersection::overlaps_sphere_and_aabb(sphere,aabb);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_aabb(sphere:&Sphere, aabb:&Aabb) ->bool{
        let dist_sq = aabb.get_distance_sq(sphere.get_center().borrow());
        let radius_sq = sphere.get_radius() * sphere.get_radius();
        return dist_sq <= radius_sq;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_frustum(sphere:&Sphere,frustum:&Frustum)->bool{
        return ShapeIntersection::overlaps_frustum_and_sphere(frustum,sphere);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_plane(sphere:&Sphere,plane:&Plane)->bool{
        let dist = plane.get_point_dist(sphere.get_center().borrow());
        return dist * dist <= sphere.get_radius() * sphere.get_radius();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_sphere(sphere1:&Sphere,sphere2:&Sphere)->bool{
        let radius_sum = sphere1.get_radius() + sphere2.get_radius();
        return sphere1.get_center().get_distance_sq(sphere2.get_center().borrow()) <= (radius_sum * radius_sum);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_obb(sphere:&Sphere,obb:&Obb)->bool{
        let radius = sphere.get_radius();
        return obb.get_distance_sq(sphere.get_center().borrow()) < radius * radius;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_capsule(sphere:&Sphere,capsule:&Capsule)->bool{
        return ShapeIntersection::overlaps_capsule_and_sphere(capsule,sphere);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_hemisphere_and_sphere(hemisphere:&Hemisphere,sphere:&Sphere)->bool{
        let sphere_distance_to_plane = hemisphere.get_direction().dot3((sphere.get_center() - hemisphere.get_center()).borrow());

        if (sphere_distance_to_plane >= 0.0)
        {
            // Sphere is in front of hemisphere, so treat the hemisphere as a sphere
            return ShapeIntersection::overlaps_sphere_and_sphere(Sphere::new_vec3_f32(hemisphere.get_center().borrow(), hemisphere.get_radius()).borrow(), sphere);
        }
        else if (sphere_distance_to_plane > -sphere.get_radius())
        {
            // Sphere is behind hemisphere, project the sphere onto the plane, then check radius of circle.
            let projected_sphere_center = sphere.get_center() + hemisphere.get_direction() * -sphere_distance_to_plane;
            let circle_radius = simd::sqrt(sphere.get_radius() * sphere.get_radius() - sphere_distance_to_plane * sphere_distance_to_plane);
            let radius_sum = hemisphere.get_radius() + circle_radius;
            return hemisphere.get_center().get_distance_sq(projected_sphere_center.borrow()) < (radius_sum * radius_sum);
        }
        return false; // too far behind hemisphere to intersect
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_hemisphere_and_aabb(hemisphere:&Hemisphere,aabb:&Aabb)->bool{
        let dist_sq = aabb.get_distance_sq(hemisphere.get_center().borrow());
        let radius_sq = hemisphere.get_radius() * hemisphere.get_radius();
        if (dist_sq > radius_sq)
        {
            return false;
        }

        if (aabb.Contains(hemisphere.get_center()))
        {
            return true;
        }

        let nearest_point_to_plane = aabb.get_support(-hemisphere.get_direction());
        let above_plane = hemisphere.get_direction().dot3((hemisphere.get_center() - nearest_point_to_plane).borrow()) > 0.0;
        return !above_plane; // This has false positives but is reasonably tight.
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_frustum_and_sphere(frustum:&Frustum,sphere:&Sphere)->bool{
        for  plane_id in Frustum::PlaneId::Near.. Frustum::PlaneId::MAX
        {
            if (frustum.get_plane(plane_id).get_point_dist(sphere.get_center().borrow()) + sphere.get_radius() < 0.0)
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_frustum_and_aabb(frustum:&Frustum,aabb:&Aabb)->bool{
        let center = aabb.get_center();

        let extents = (0.5 * aabb.GetMax()) - (0.5 * aabb.GetMin());

        for  plane_id in Frustum::PlaneId::Near.. Frustum::PlaneId::MAX
        {
            let plane = frustum.get_plane(plane_id);
            if (plane.get_point_dist(center.borrow()) + extents.Dot(plane.get_normal().get_abs()) <= 0.0)
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_frustum_and_obb(self,frustum:&Frustum,obb:&Obb)->bool{
        for  plane_id in Frustum::PlaneId::Near.. Frustum::PlaneId::MAX
        {
            if (Self::classify_plane_and_obb(frustum.get_plane(plane_id).borrow(), obb) == IntersectResult::Exterior)
            {
                return false;
            }
        }

        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_capsule_and_capsule(capsule1:&Capsule, capsule2:&Capsule) ->bool{
        let closest_point_segment1:Vector3 = Vector3::new();
        let closest_point_segment2:Vector3 = Vector3::new();;
        let segment1proportion:f32 = 0f32;
        let segment2proportion:f32 = 0f32;
        Intersect::closest_segment_segment(
            capsule1.GetFirstHemisphereCenter(), capsule1.GetSecondHemisphereCenter(), capsule2.GetFirstHemisphereCenter(),
            capsule2.GetSecondHemisphereCenter(), segment1proportion, segment2proportion, closest_point_segment1, closest_point_segment2);
        let radius_sum = capsule1.get_radius() + capsule2.get_radius();
        return closest_point_segment1.get_distance_sq(closest_point_segment2.borrow()) <= radius_sum * radius_sum;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_capsule_and_obb(capsule:&Capsule,obb:&Obb)->bool{
        if (capsule.get_cylinder_height() < FLOAT_EPSILON * capsule.get_radius())
        {
            return ShapeIntersection::overlaps_sphere_and_obb(Sphere::new_vec3_f32(capsule.get_center().borrow(), capsule.get_radius()).borrow(), obb);
        }

        let capsule_point1 =
            obb.get_rotation().get_inverse_fast().transform_vector((capsule.get_first_hemisphere_center() - obb.get_position()).borrow());
        let capsule_point2 =
            obb.get_rotation().get_inverse_fast().transform_vector((capsule.get_second_hemisphere_center() - obb.get_position()).borrow());
        let radius = capsule.get_radius();
        let half_lengths = obb.get_half_lengths();

        for  capsule_point in capsule_point1..capsule_point2
        {
            let closest = capsule_point.get_clamp(-half_lengths, half_lengths.borrow());
            if (capsule_point.get_distance_sq(closest.borrow()) < radius * radius)
            {
                return true;
            }
        }

        let capsule_point_min_values = capsule_point1.get_min(capsule_point2.borrow()) - Vector3::new_x(radius);
        let capsule_point_max_values = capsule_point1.get_max(capsule_point2.borrow()) + Vector3::new_x(radius);
        let overlaps_xyz = capsule_point_max_values.is_greater_equal_than(-half_lengths) &&
            capsule_point_min_values.is_less_equal_than(half_lengths.borrow());
        if (!overlaps_xyz)
        {
            return false;
        }

        let capsule_axis = (capsule_point2 - capsule_point1).get_normalized();
        let overlaps_axis = |axis:&Vector3 |{
            let capsule_point1projected = capsule_point1.dot3(axis);
            let capsule_point2projected = capsule_point2.dot3(axis);
            let capsule_projected_min = constants::get_min(capsule_point1projected, capsule_point2projected) - radius;
            let capsule_projected_max = constants::get_max(capsule_point1projected, capsule_point2projected) + radius;
            let obb_projected_half_extent = half_lengths.dot3(axis.get_abs().borrow());
            return capsule_projected_max > -obb_projected_half_extent && capsule_projected_min < obb_projected_half_extent;
        };
        for test_axis in   &[capsule_axis.cross_x_axis(), capsule_axis.cross_y_axis(), capsule_axis.cross_z_axis() ]{
            if (!overlaps_axis(test_axis))
            {
                return false;
            }
        }

        for point in &[capsule_point1, capsule_point2]{
            let closest_point_in_obb = point.get_clamp(-half_lengths, half_lengths.borrow());
            let test_axis = (point - closest_point_in_obb).get_normalized();
            if (!overlaps_axis(test_axis.borrow()))
            {
                return false;
            }
        }

        let get_half_length = |ib:i32,a:f32,b:f32|->f32{
            if ib{
                a
            }else {
                b
            }
        };

        for vertex_index in 0..8 {
            let vertex = Vector3::new_xyz(get_half_length(vertex_index & 4,obb.get_half_length_x(),-obb.get_half_length_x()),
                                          get_half_length(vertex_index & 2,obb.get_half_length_y(),-obb.get_half_length_y()),
                                          get_half_length(vertex_index & 1,obb.get_half_length_z(),-obb.get_half_length_z())
            );

            let vertex_relative = vertex - capsule_point1;
            let test_axis = (vertex_relative - capsule_axis* vertex_relative.dot3(capsule_axis.borrow())).get_normalized();
            if (!overlaps_axis(test_axis.borrow()))
            {
                return false;
            }
        }

        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_capsule_and_sphere(capsule:&Capsule,sphere:&Sphere)->bool{
        let proportion :f32 = 0.0;
        let closest_point_on_capsule_axis = Vector3::new();
        Intersect::closest_point_segment(sphere.get_center(), capsule.get_first_hemisphere_center(), capsule.get_second_hemisphere_center(), proportion, closest_point_on_capsule_axis);
        let radius_sum = sphere.get_radius() + capsule.get_radius();
        return closest_point_on_capsule_axis.get_distance_sq(sphere.get_center().borrow()) <= radius_sum * radius_sum;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_capsule_and_aabb(capsule:&Capsule,aabb:&Aabb)->bool{
        let mut aabb_sphere_center = Vector3::new();
        let mut aabb_sphere_radius:f32 = 0.0;
        aabb.get_as_sphere(aabb_sphere_center.borrow_mut(), aabb_sphere_radius.borrow_mut());
        let aabb_sphere = Sphere::new_vec3_f32(aabb_sphere_center.borrow(), aabb_sphere_radius);
        if !Self::overlaps_capsule_and_sphere(capsule, aabb_sphere.borrow())
        {
            return false;
        }

        let capsule_radius_sq = capsule.get_radius() * capsule.get_radius();
        let capsule_start = capsule.get_first_hemisphere_center();
        let capsule_end = capsule.get_second_hemisphere_center();
        let capsule_segment = capsule_end - capsule_start;
        if capsule_segment.is_close_default(Vector3::create_zero().borrow())
        {
            return false;
        }
        let segment_length_squared = capsule_segment.dot3(capsule_segment.borrow());
        let rcp_segment_length_squared = 1.0 / segment_length_squared;

        let mut clamped_point1 = capsule_start.get_clamp(aabb.get_min().borrow(), aabb.get_max().borrow());
        let mut clamped_point2 = capsule_end.get_clamp(aabb.get_min().borrow(), aabb.get_max().borrow());

        let get_closest_point_on_capsule = |point:&Vector3| ->Vector3{
            let proportion = (point - capsule_start).dot3(capsule_segment.borrow());
            if (proportion <= 0.0)
            {
                return capsule_start;
            }
            if (proportion >= segment_length_squared)
            {
                return capsule_end;
            }
            return capsule_start + (capsule_segment * rcp_segment_length_squared * proportion);
        };


        let max_iterations:u32 = 16;
        for  i in 0..max_iterations
        {
            let closest_point_on_capsule_axis1 = get_closest_point_on_capsule(clamped_point1.borrow());
            if (clamped_point1.get_distance_sq(closest_point_on_capsule_axis1.borrow()) < capsule_radius_sq)
            {
                return true;
            }

            let closest_point_on_capsule_axis2 = get_closest_point_on_capsule(clamped_point2.borrow());
            if (clamped_point2.get_distance_sq(closest_point_on_capsule_axis2.borrow()) < capsule_radius_sq)
            {
                return true;
            }

            if (clamped_point1.is_close_default(clamped_point2.borrow()))
            {
                return false;
            }

            let newclamped_point1 = closest_point_on_capsule_axis1.get_clamp(aabb.get_min().borrow(), aabb.get_max().borrow());
            let newclamped_point2 = closest_point_on_capsule_axis2.get_clamp(aabb.get_min().borrow(), aabb.get_max().borrow());

            if (newclamped_point1.is_close_default(clamped_point1.borrow()) || newclamped_point2.is_close_default(clamped_point2.borrow()))
            {
                return false;
            }

            clamped_point1 = newclamped_point1;
            clamped_point2 = newclamped_point2;
        }

        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_obb_and_obb(obb1:&Obb,obb2:&Obb)->bool{
        let overlaps_axis = |axis:&Vector3| ->bool{
            let transformed_axis1 = obb1.GetRotation().GetInverseFast().TransformVector(axis);
            let obb1projected_position = obb1.GetPosition().Dot(axis);
            let obb1projected_half_extent = obb1.GetHalfLengths().Dot(transformed_axis1.GetAbs());
            let obb1projected_min = obb1projected_position - obb1projected_half_extent;
            let obb1projected_max = obb1projected_position + obb1projected_half_extent;
            let transformed_axis2 = obb2.GetRotation().GetInverseFast().TransformVector(axis);
            let obb2projected_position = obb2.GetPosition().Dot(axis);
            let obb2projected_half_extent = obb2.GetHalfLengths().Dot(transformed_axis2.GetAbs());
            let obb2projected_min = obb2projected_position - obb2projected_half_extent;
            let obb2projected_max = obb2projected_position + obb2projected_half_extent;
            return obb1projected_max >= obb2projected_min && obb1projected_min <= obb2projected_max;
        };

        let x_axis1 = obb1.get_rotation().transform_vector(Vector3::create_axis_x(1.0).borrow());
        let y_axis1 = obb1.get_rotation().transform_vector(Vector3::create_axis_y(1.0).borrow());
        let z_axis1 = obb1.get_rotation().transform_vector(Vector3::create_axis_z(1.0).borrow());
        let x_axis2 = obb2.get_rotation().transform_vector(Vector3::create_axis_x(1.0).borrow());
        let y_axis2 = obb2.get_rotation().transform_vector(Vector3::create_axis_y(1.0).borrow());
        let z_axis2 = obb2.get_rotation().transform_vector(Vector3::create_axis_z(1.0).borrow());

        return
        overlaps_axis(x_axis1.borrow()) &&
            overlaps_axis(y_axis1.borrow()) &&
            overlaps_axis(z_axis1.borrow()) &&
            overlaps_axis(x_axis2.borrow()) &&
            overlaps_axis(y_axis2.borrow()) &&
            overlaps_axis(z_axis2.borrow()) &&
            overlaps_axis(x_axis1.cross(x_axis2.borrow()).borrow()) &&
            overlaps_axis(x_axis1.cross(y_axis2.borrow()).borrow()) &&
            overlaps_axis(x_axis1.cross(z_axis2.borrow()).borrow()) &&
            overlaps_axis(y_axis1.cross(x_axis2.borrow()).borrow()) &&
            overlaps_axis(y_axis1.cross(y_axis2.borrow()).borrow()) &&
            overlaps_axis(y_axis1.cross(z_axis2.borrow()).borrow()) &&
            overlaps_axis(z_axis1.cross(x_axis2.borrow()).borrow()) &&
            overlaps_axis(z_axis1.cross(y_axis2.borrow()).borrow()) &&
            overlaps_axis(z_axis1.cross(z_axis2.borrow()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_obb_and_capsule(obb:&Obb,capsule:&Capsule)->bool{
        return Self::overlaps_capsule_and_obb(capsule, obb);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_obb_and_sphere(obb:&Obb,sphere:&Sphere)->bool{
        return Self::overlaps_sphere_and_obb(sphere, obb);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_aabb_and_capsule(aabb:&Aabb,capsule:&Capsule)->bool{
        return Self::overlaps_capsule_and_aabb(capsule, aabb);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_aabb_and_aabb(aabb1:&Aabb,aabb2:&Aabb)->bool{
        return aabb1.contains_aabb(aabb2);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_aabb_and_sphere(aabb:&Aabb,sphere:&Sphere)->bool{
        return Self::contains_aabb_and_aabb(aabb, Aabb::create_center_radius(sphere.get_center().borrow(), sphere.get_radius()).borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_sphere_and_aabb(sphere:&Sphere,aabb:&Aabb)->bool{
        let radius_sq = sphere.get_radius() * sphere.get_radius();
        return aabb.get_max_distance_sq(sphere.get_center().borrow()) <= radius_sq;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_sphere_and_vec3(sphere:&Sphere,point:&Vector3)->bool{
        let dist_sq = sphere.get_center().get_distance_sq(point);
        let radius_sq = sphere.get_radius() * sphere.get_radius();
        return dist_sq <= radius_sq;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_sphere_and_sphere(sphere1:&Sphere,sphere2:&Sphere)->bool{
        let radius_diff = sphere1.get_radius() - sphere2.get_radius();
        return sphere1.get_center().get_distance_sq(sphere2.get_center().borrow()) <= (radius_diff * radius_diff) * constants::get_sign(radius_diff);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_hemisphere_and_aabb(hemisphere:&Hemisphere,aabb:&Aabb)->bool{
        let radius_sq = hemisphere.get_radius() * hemisphere.get_radius();
        if (aabb.GetMaxDistanceSq(hemisphere.get_center()) <= radius_sq)
        {
            let nearest_point_to_plane = aabb.get_support(hemisphere.get_direction().borrow());
            return hemisphere.get_direction().dot3((nearest_point_to_plane - hemisphere.get_center()).borrow()) >= 0.0;
        }
        return false;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_frustum_and_aabb(frustum:&Frustum,aabb:&Aabb)->bool{
        let center = aabb.get_center();
        let extents = 0.5 * aabb.GetExtents();

        for plane_id in Frustum::PlaneId::Near.. Frustum::PlaneId::MAX
        {
            let plane = frustum.get_plane(plane_id);
            if (plane.get_point_dist(center.borrow()) - extents.Dot(plane.get_normal().get_abs()) < 0.0)
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_frustum_and_sphere(frustum:&Frustum,sphere:&Sphere)->bool{
        for plane_id in Frustum::PlaneId::Near.. Frustum::PlaneId::MAX
        {
            if frustum.get_plane(plane_id).get_point_dist(sphere.get_center().borrow()) - sphere.get_radius() < 0.0
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_frustum_and_vec3(frustum:&Frustum,point:&Vector3)->bool{
        for plane_id in Frustum::PlaneId::Near.. Frustum::PlaneId::MAX
        {
            if frustum.get_plane(plane_id).get_point_dist(point) < 0.0
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_capsule_and_sphere(capsule:&Capsule,sphere:&Sphere)->bool{
        let proportion:f32 = 0.0;
        let closest_point_on_capsule_axis = Vector3::new();
        Intersect::closest_point_segment(sphere.get_center(), capsule.get_first_hemisphere_center(), capsule.get_second_hemisphere_center(), proportion, closest_point_on_capsule_axis);
        return Self::contains_sphere_and_sphere(Sphere::new_vec3_f32(closest_point_on_capsule_axis.borrow(), capsule.get_radius()).borrow(), sphere);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn contains_capsule_and_aabb(capsule:&Capsule,aabb:&Aabb)->bool{
        let mut aabb_sphere_center =Vector3::new();
        let mut aabb_sphere_radius:f32 = 0.0;
        aabb.get_as_sphere(aabb_sphere_center.borrow_mut(), aabb_sphere_radius.borrow_mut());
        let aabb_sphere = Sphere::new_vec3_f32(aabb_sphere_center.borrow(), aabb_sphere_radius);

        if (Self::contains_capsule_and_sphere(capsule, aabb_sphere.borrow()))
        {
            return true;
        }
        else if (!Self::overlaps_capsule_and_sphere(capsule, aabb_sphere.borrow()))
        {
            return false;
        }
        for  aabb_point in
        &[
            aabb.get_min(),
            aabb.get_max(),
            Vector3::new_xyz(aabb.get_min().get_x(), aabb.get_min().get_y(), aabb.get_max().get_z()),
            Vector3::new_xyz(aabb.get_min().get_x(), aabb.get_max().get_y(), aabb.get_min().get_z()),
            Vector3::new_xyz(aabb.get_min().get_x(), aabb.get_max().get_y(), aabb.get_max().get_z()),
            Vector3::new_xyz(aabb.get_max().get_x(), aabb.get_min().get_y(), aabb.get_min().get_z()),
            Vector3::new_xyz(aabb.get_max().get_x(), aabb.get_min().get_y(), aabb.get_max().get_z()),
            Vector3::new_xyz(aabb.get_max().get_x(), aabb.get_max().get_y(), aabb.get_min().get_z()),
        ]
        {
            if (!capsule.contains(aabb_point))
            {
                return false;
            }
        }
        return true;
    }

}