#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::aabb::Aabb;
use crate::math::capsule::Capsule;
use crate::math::constants::FLOAT_EPSILON;
use crate::math::math_utils::get_abs_f32;
use crate::math::obb::Obb;
use crate::math::plane::{IntersectResult, Plane};
use crate::math::sphere::Sphere;
use crate::math::vector3::Vector3;
use crate::math::vector4::Vector4;

#[derive(Debug,Copy, Clone)]
pub struct ShapeIntersection {
    _plane: Vector4
}
const FLT_MIN_F32: f32 = 1.175494351e-38f32;
impl ShapeIntersection{

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_three_planes(p1:&Plane, p2:&Plane, p3:&Plane, mut out_p: &Vector3) ->bool{
        let n1cross_n2 = p1.get_normal().cross(p2.get_normal().borrow());
        let det = n1cross_n2.dot3(p3.get_normal().borrow());
        if (get_abs_f32(det.borrow()) >  FLT_MIN_F32)
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
        let r = obb.get_half_length_x() * get_abs_f32(plane.get_normal().dot3(obb.get_axis_x().borrow()).borrow())
            + obb.get_half_length_y() * get_abs_f32(plane.get_normal().dot3(obb.get_axis_y().borrow()).borrow())
            + obb.get_half_length_z() * get_abs_f32(plane.get_normal().dot3(obb.get_axis_z().borrow()).borrow());
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
        let dist_sq = aabb.get_distance_sq(sphere.get_center());
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
        let dist = plane.get_point_dist(sphere.get_center());
        return dist * dist <= sphere.get_radius() * sphere.get_radius();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_sphere(sphere1:&Sphere,sphere2:&Sphere)->bool{
        let radius_sum = sphere1.get_radius() + sphere2.get_radius();
        return sphere1.get_center().get_distance_sq(sphere2.get_center()) <= (radius_sum * radius_sum);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_obb(sphere:&Sphere,obb:&Obb)->bool{
        let radius = sphere.get_radius();
        return obb.get_distance_sq(sphere.get_center()) < radius * radius;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_sphere_and_capsule(sphere:&Sphere,capsule:&Capsule)->bool{
        return ShapeIntersection::overlaps_capsule_and_sphere(capsule,sphere);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_hemisphere_and_sphere(hemisphere:&Hemisphere,sphere:&Sphere)->bool{
        let sphere_distance_to_plane = hemisphere.get_direction().Dot(sphere.get_center() - hemisphere.get_center());

        if (sphere_distance_to_plane >= 0.0)
        {
            // Sphere is in front of hemisphere, so treat the hemisphere as a sphere
            return ShapeIntersection::overlaps_sphere_and_sphere(Sphere(hemisphere.get_center(), hemisphere.get_radius()), sphere);
        }
        else if (sphere_distance_to_plane > -sphere.get_radius())
        {
            // Sphere is behind hemisphere, project the sphere onto the plane, then check radius of circle.
            let projected_sphere_center = sphere.get_center() + hemisphere.get_direction() * -sphere_distance_to_plane;
            let circle_radius = AZStd::sqrt(sphere.get_radius() * sphere.get_radius() - sphere_distance_to_plane * sphere_distance_to_plane);
            let radius_sum = hemisphere.get_radius() + circle_radius;
            return hemisphere.get_center().get_distance_sq(projected_sphere_center) < (radius_sum * radius_sum);
        }
        return false; // too far behind hemisphere to intersect
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_hemisphere_and_aabb(hemisphere:&Hemisphere,aabb:&Aabb)->bool{
        let dist_sq = aabb.get_distance_sq(hemisphere.get_center());
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
        let above_plane = hemisphere.get_direction().Dot(hemisphere.get_center() - nearest_point_to_plane) > 0.0;
        return !above_plane; // This has false positives but is reasonably tight.
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_frustum_and_sphere(frustum:&Frustum,sphere:&Sphere)->bool{
        for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
        {
            if (frustum.get_plane(planeId).get_point_dist(sphere.get_center()) + sphere.get_radius() < 0.0f)
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

        for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
        {
            let plane = frustum.get_plane(planeId);
            if (plane.get_point_dist(center) + extents.Dot(plane.GetNormal().GetAbs()) <= 0.0)
            {
                return false;
            }
        }
        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_frustum_and_obb(frustum:&Frustum,obb:&Obb)->bool{
        for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
        {
            if (Classify(frustum.get_plane(planeId), obb) == IntersectResult::Exterior)
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
        Intersect::ClosestSegmentSegment(
            capsule1.GetFirstHemisphereCenter(), capsule1.GetSecondHemisphereCenter(), capsule2.GetFirstHemisphereCenter(),
            capsule2.GetSecondHemisphereCenter(), segment1proportion, segment2proportion, closest_point_segment1, closest_point_segment2);
        let radius_sum = capsule1.get_radius() + capsule2.get_radius();
        return closest_point_segment1.get_distance_sq(closest_point_segment2) <= radius_sum * radius_sum;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn overlaps_capsule_and_obb(capsule1:&Capsule,obb:&Obb)->bool{
        // if the capsule is sufficiently close to being a sphere, just test a sphere against the OBB
        if (capsule.GetCylinderHeight() < FLOAT_EPSILON * capsule.get_radius())
        {
            return Overlaps(Sphere(capsule.GetCenter(), capsule.get_radius()), obb);
        }

        // transform capsule points into a space where the obb is centred at the origin with identity rotation
        const Vector3 capsulePoint1 =
            obb.GetRotation().GetInverseFast().TransformVector(capsule.GetFirstHemisphereCenter() - obb.GetPosition());
        const Vector3 capsulePoint2 =
            obb.GetRotation().GetInverseFast().TransformVector(capsule.GetSecondHemisphereCenter() - obb.GetPosition());
        const float radius = capsule.get_radius();
        const Vector3& halfLengths = obb.GetHalfLengths();

        // early out if the distance from either of the capsule hemisphere centers is less than radius
        for (const Vector3& capsulePoint : { capsulePoint1, capsulePoint2 })
        {
            const Vector3 closest = capsulePoint.GetClamp(-halfLengths, halfLengths);
            if (capsulePoint.GetDistanceSq(closest) < radius * radius)
            {
                return true;
            }
        }

        // use separating axis theorem
        // up to 16 axes need to be tested:
        // - the 3 face normals of the box (x, y, and z)
        // - the 3 cross products of those directions with the capsule axis
        // - the 2 directions from the two capsule hemispheres to their closest points on the box
        // - the 8 directions from the capsule axis to the box vertices, orthogonal to the capsule axis
        // if the projections of the two shapes onto any of those axes do not overlap then the shapes do not overlap

        // test the x, y and z axes
        const Vector3 capsulePointMinValues = capsulePoint1.GetMin(capsulePoint2) - Vector3(radius);
        const Vector3 capsulePointMaxValues = capsulePoint1.GetMax(capsulePoint2) + Vector3(radius);
        const bool overlapsXYZ = capsulePointMaxValues.IsGreaterEqualThan(-halfLengths) &&
            capsulePointMinValues.IsLessEqualThan(halfLengths);
        if (!overlapsXYZ)
        {
            return false;
        }

        // test the axes formed by the cross product of the capsule axis with x, y and z
        const Vector3 capsuleAxis = (capsulePoint2 - capsulePoint1).GetNormalized();
        auto overlapsAxis = [&capsulePoint1, &capsulePoint2, &radius, &halfLengths](const Vector3& axis)
        {
            const float capsulePoint1Projected = capsulePoint1.Dot(axis);
            const float capsulePoint2Projected = capsulePoint2.Dot(axis);
            const float capsuleProjectedMin = AZ::GetMin(capsulePoint1Projected, capsulePoint2Projected) - radius;
            const float capsuleProjectedMax = AZ::GetMax(capsulePoint1Projected, capsulePoint2Projected) + radius;
            const float obbProjectedHalfExtent = halfLengths.Dot(axis.GetAbs());
            return capsuleProjectedMax > -obbProjectedHalfExtent && capsuleProjectedMin < obbProjectedHalfExtent;
        };

        for (const Vector3& testAxis : { capsuleAxis.CrossXAxis(), capsuleAxis.CrossYAxis(), capsuleAxis.CrossZAxis() })
        {
            if (!overlapsAxis(testAxis))
            {
                return false;
            }
        }

        // test the directions from the two capsule hemispheres to their closest points on the box
        for (const Vector3& point : { capsulePoint1, capsulePoint2 })
        {
            const Vector3 closestPointInObb = point.GetClamp(-halfLengths, halfLengths);
            const Vector3 testAxis = (point - closestPointInObb).GetNormalized();
            if (!overlapsAxis(testAxis))
            {
                return false;
            }
        }

        // test the 8 directions from the capsule axis to the box vertices, orthogonal to the capsule axis
        for (int vertexIndex = 0; vertexIndex < 8; vertexIndex++)
        {
            const Vector3 vertex(
            (vertexIndex & 4) ? obb.GetHalfLengthX() : -obb.GetHalfLengthX(),
            (vertexIndex & 2) ? obb.GetHalfLengthY() : -obb.GetHalfLengthY(),
            (vertexIndex & 1) ? obb.GetHalfLengthZ() : -obb.GetHalfLengthZ());
            // get a vector from the capsule axis to the vertex
            const Vector3 vertexRelative = vertex - capsulePoint1;
            // subtract the part parallel to the axis to get an axis orthogonal to the axis
            const Vector3 testAxis = (vertexRelative - vertexRelative.Dot(capsuleAxis) * capsuleAxis).GetNormalized();

            if (!overlapsAxis(testAxis))
            {
                return false;
            }
        }

        // none of the tested axes were separating axes, the shapes must overlap
        return true;
    }
    bool Overlaps(const Capsule& capsule, const Obb& obb)
    {
    // if the capsule is sufficiently close to being a sphere, just test a sphere against the OBB
    if (capsule.GetCylinderHeight() < AZ::Constants::FloatEpsilon * capsule.get_radius())
    {
    return Overlaps(Sphere(capsule.get_center(), capsule.get_radius()), obb);
    }

    // transform capsule points into a space where the obb is centred at the origin with identity rotation
    const Vector3 capsulePoint1 =
    obb.GetRotation().GetInverseFast().TransformVector(capsule.GetFirstHemisphereCenter() - obb.GetPosition());
    const Vector3 capsulePoint2 =
    obb.GetRotation().GetInverseFast().TransformVector(capsule.GetSecondHemisphereCenter() - obb.GetPosition());
    const float radius = capsule.get_radius();
    const Vector3& halfLengths = obb.GetHalfLengths();

    // early out if the distance from either of the capsule hemisphere centers is less than radius
    for (const Vector3& capsulePoint : { capsulePoint1, capsulePoint2 })
    {
    const Vector3 closest = capsulePoint.GetClamp(-halfLengths, halfLengths);
    if (capsulePoint.get_distance_sq(closest) < radius * radius)
    {
    return true;
    }
    }

    // use separating axis theorem
    // up to 16 axes need to be tested:
    // - the 3 face normals of the box (x, y, and z)
    // - the 3 cross products of those directions with the capsule axis
    // - the 2 directions from the two capsule hemispheres to their closest points on the box
    // - the 8 directions from the capsule axis to the box vertices, orthogonal to the capsule axis
    // if the projections of the two shapes onto any of those axes do not overlap then the shapes do not overlap

    // test the x, y and z axes
    const Vector3 capsulePointMinValues = capsulePoint1.GetMin(capsulePoint2) - Vector3(radius);
    const Vector3 capsulePointMaxValues = capsulePoint1.GetMax(capsulePoint2) + Vector3(radius);
    const bool overlapsXYZ = capsulePointMaxValues.IsGreaterEqualThan(-halfLengths) &&
    capsulePointMinValues.IsLessEqualThan(halfLengths);
    if (!overlapsXYZ)
    {
    return false;
    }

    // test the axes formed by the cross product of the capsule axis with x, y and z
    const Vector3 capsuleAxis = (capsulePoint2 - capsulePoint1).GetNormalized();
    auto overlapsAxis = [&capsulePoint1, &capsulePoint2, &radius, &halfLengths](const Vector3& axis)
    {
    const float capsulePoint1Projected = capsulePoint1.Dot(axis);
    const float capsulePoint2Projected = capsulePoint2.Dot(axis);
    const float capsuleProjectedMin = AZ::GetMin(capsulePoint1Projected, capsulePoint2Projected) - radius;
    const float capsuleProjectedMax = AZ::GetMax(capsulePoint1Projected, capsulePoint2Projected) + radius;
    const float obbProjectedHalfExtent = halfLengths.Dot(axis.GetAbs());
    return capsuleProjectedMax > -obbProjectedHalfExtent && capsuleProjectedMin < obbProjectedHalfExtent;
    };

    for (const Vector3& testAxis : { capsuleAxis.CrossXAxis(), capsuleAxis.CrossYAxis(), capsuleAxis.CrossZAxis() })
    {
    if (!overlapsAxis(testAxis))
    {
    return false;
    }
    }

    // test the directions from the two capsule hemispheres to their closest points on the box
    for (const Vector3& point : { capsulePoint1, capsulePoint2 })
    {
    const Vector3 closestPointInObb = point.GetClamp(-halfLengths, halfLengths);
    const Vector3 testAxis = (point - closestPointInObb).GetNormalized();
    if (!overlapsAxis(testAxis))
    {
    return false;
    }
    }

    // test the 8 directions from the capsule axis to the box vertices, orthogonal to the capsule axis
    for (int vertexIndex = 0; vertexIndex < 8; vertexIndex++)
    {
    const Vector3 vertex(
    (vertexIndex & 4) ? obb.GetHalfLengthX() : -obb.GetHalfLengthX(),
    (vertexIndex & 2) ? obb.GetHalfLengthY() : -obb.GetHalfLengthY(),
    (vertexIndex & 1) ? obb.GetHalfLengthZ() : -obb.GetHalfLengthZ());
    // get a vector from the capsule axis to the vertex
    const Vector3 vertexRelative = vertex - capsulePoint1;
    // subtract the part parallel to the axis to get an axis orthogonal to the axis
    const Vector3 testAxis = (vertexRelative - vertexRelative.Dot(capsuleAxis) * capsuleAxis).GetNormalized();

    if (!overlapsAxis(testAxis))
    {
    return false;
    }
    }

    // none of the tested axes were separating axes, the shapes must overlap
    return true;
    }
    AZ_MATH_INLINE bool Overlaps(const Capsule& capsule, const Sphere& sphere)
    {
    float proportion;
    Vector3 closestPointOnCapsuleAxis;
    Intersect::ClosestPointSegment(sphere.get_center(), capsule.GetFirstHemisphereCenter(), capsule.GetSecondHemisphereCenter(), proportion, closestPointOnCapsuleAxis);
    const float radiusSum = sphere.get_radius() + capsule.get_radius();
    return closestPointOnCapsuleAxis.get_distance_sq(sphere.get_center()) <= radiusSum * radiusSum;
    }
    AZ_MATH_INLINE bool Overlaps(const Capsule& capsule, const Aabb& aabb)
    {
    // First attempt a cheap rejection by comparing to the aabb's sphere.
    Vector3 aabbSphereCenter;
    float aabbSphereRadius;
    aabb.GetAsSphere(aabbSphereCenter, aabbSphereRadius);
    Sphere aabbSphere(aabbSphereCenter, aabbSphereRadius);
    if (!Overlaps(capsule, aabbSphere))
    {
    return false;
    }

    // Now do the more expensive test. The idea is to start with the end points of the capsule then
    // - Clamp the points with the aabb
    // - Find the closest points on the line segment to the clamped points.
    // - If the distance between the clamped point and the line segment point is less than the radius, then we know it intersects.
    // - Generate new clamped points from the points on the line segment for next iteration.
    // - If the two clamped points are equal to each other, or either of the new clamped points is equivalent to the previous clamped points,
    //   then we know we've already found the closest point possible on the aabb, so fail because previous distance check failed.
    // - Loop with new clamped points.

    float capsuleRadiusSq = capsule.get_radius() * capsule.get_radius();
    const Vector3& capsuleStart = capsule.GetFirstHemisphereCenter();
    const Vector3& capsuleEnd = capsule.GetSecondHemisphereCenter();
    const Vector3 capsuleSegment = capsuleEnd - capsuleStart;
    if (capsuleSegment.IsClose(AZ::Vector3::CreateZero()))
    {
    // capsule is nearly a sphere, and already failed sphere check above.
    return false;
    }
    const float segmentLengthSquared = capsuleSegment.Dot(capsuleSegment);
    const float rcpSegmentLengthSquared = 1.0f / segmentLengthSquared;

    Vector3 clampedPoint1 = capsuleStart.GetClamp(aabb.GetMin(), aabb.GetMax());
    Vector3 clampedPoint2 = capsuleEnd.GetClamp(aabb.GetMin(), aabb.GetMax());

    // Simplified from Intersect::ClosestPointSegment with certain parts pre-calculated, no need to return proportion.
    auto getClosestPointOnCapsule = [&](const Vector3& point) -> Vector3
    {
    float proportion = (point - capsuleStart).Dot(capsuleSegment);
    if (proportion <= 0.0f)
    {
    return capsuleStart;
    }
    if (proportion >= segmentLengthSquared)
    {
    return capsuleEnd;
    }
    return capsuleStart + (proportion * capsuleSegment * rcpSegmentLengthSquared);
    };

    constexpr uint32_t MaxIterations = 16;
    for (uint32_t i = 0; i < MaxIterations; ++i)
    {
    // Check point 1
    Vector3 closestPointOnCapsuleAxis1 = getClosestPointOnCapsule(clampedPoint1);
    if (clampedPoint1.get_distance_sq(closestPointOnCapsuleAxis1) < capsuleRadiusSq)
    {
    return true;
    }

    // Check point 2
    Vector3 closestPointOnCapsuleAxis2 = getClosestPointOnCapsule(clampedPoint2);
    if (clampedPoint2.get_distance_sq(closestPointOnCapsuleAxis2) < capsuleRadiusSq)
    {
    return true;
    }

    // If the points are the same, and previous tests failed, then this is the best point, but it's too far away.
    if (clampedPoint1.IsClose(clampedPoint2))
    {
    return false;
    }

    // Choose better points.
    Vector3 newclampedPoint1 = closestPointOnCapsuleAxis1.GetClamp(aabb.GetMin(), aabb.GetMax());
    Vector3 newclampedPoint2 = closestPointOnCapsuleAxis2.GetClamp(aabb.GetMin(), aabb.GetMax());

    if (newclampedPoint1.IsClose(clampedPoint1) || newclampedPoint2.IsClose(clampedPoint2))
    {
    // Capsule is parallel to AABB or beyond the end points and failing above tests, so it must be outside the capsule.
    return false;
    }

    clampedPoint1 = newclampedPoint1;
    clampedPoint2 = newclampedPoint2;
    }

    return true; // prefer false positive
    }
    bool Overlaps(const Capsule& capsule, const Obb& obb)
    {
    // if the capsule is sufficiently close to being a sphere, just test a sphere against the OBB
    if (capsule.GetCylinderHeight() < AZ::Constants::FloatEpsilon * capsule.get_radius())
    {
    return Overlaps(Sphere(capsule.get_center(), capsule.get_radius()), obb);
    }

    // transform capsule points into a space where the obb is centred at the origin with identity rotation
    const Vector3 capsulePoint1 =
    obb.GetRotation().GetInverseFast().TransformVector(capsule.GetFirstHemisphereCenter() - obb.GetPosition());
    const Vector3 capsulePoint2 =
    obb.GetRotation().GetInverseFast().TransformVector(capsule.GetSecondHemisphereCenter() - obb.GetPosition());
    const float radius = capsule.get_radius();
    const Vector3& halfLengths = obb.GetHalfLengths();

    // early out if the distance from either of the capsule hemisphere centers is less than radius
    for (const Vector3& capsulePoint : { capsulePoint1, capsulePoint2 })
    {
    const Vector3 closest = capsulePoint.GetClamp(-halfLengths, halfLengths);
    if (capsulePoint.get_distance_sq(closest) < radius * radius)
    {
    return true;
    }
    }

    // use separating axis theorem
    // up to 16 axes need to be tested:
    // - the 3 face normals of the box (x, y, and z)
    // - the 3 cross products of those directions with the capsule axis
    // - the 2 directions from the two capsule hemispheres to their closest points on the box
    // - the 8 directions from the capsule axis to the box vertices, orthogonal to the capsule axis
    // if the projections of the two shapes onto any of those axes do not overlap then the shapes do not overlap

    // test the x, y and z axes
    const Vector3 capsulePointMinValues = capsulePoint1.GetMin(capsulePoint2) - Vector3(radius);
    const Vector3 capsulePointMaxValues = capsulePoint1.GetMax(capsulePoint2) + Vector3(radius);
    const bool overlapsXYZ = capsulePointMaxValues.IsGreaterEqualThan(-halfLengths) &&
    capsulePointMinValues.IsLessEqualThan(halfLengths);
    if (!overlapsXYZ)
    {
    return false;
    }

    // test the axes formed by the cross product of the capsule axis with x, y and z
    const Vector3 capsuleAxis = (capsulePoint2 - capsulePoint1).GetNormalized();
    auto overlapsAxis = [&capsulePoint1, &capsulePoint2, &radius, &halfLengths](const Vector3& axis)
    {
    const float capsulePoint1Projected = capsulePoint1.Dot(axis);
    const float capsulePoint2Projected = capsulePoint2.Dot(axis);
    const float capsuleProjectedMin = AZ::GetMin(capsulePoint1Projected, capsulePoint2Projected) - radius;
    const float capsuleProjectedMax = AZ::GetMax(capsulePoint1Projected, capsulePoint2Projected) + radius;
    const float obbProjectedHalfExtent = halfLengths.Dot(axis.GetAbs());
    return capsuleProjectedMax > -obbProjectedHalfExtent && capsuleProjectedMin < obbProjectedHalfExtent;
    };

    for (const Vector3& testAxis : { capsuleAxis.CrossXAxis(), capsuleAxis.CrossYAxis(), capsuleAxis.CrossZAxis() })
    {
    if (!overlapsAxis(testAxis))
    {
    return false;
    }
    }

    // test the directions from the two capsule hemispheres to their closest points on the box
    for (const Vector3& point : { capsulePoint1, capsulePoint2 })
    {
    const Vector3 closestPointInObb = point.GetClamp(-halfLengths, halfLengths);
    const Vector3 testAxis = (point - closestPointInObb).GetNormalized();
    if (!overlapsAxis(testAxis))
    {
    return false;
    }
    }

    // test the 8 directions from the capsule axis to the box vertices, orthogonal to the capsule axis
    for (int vertexIndex = 0; vertexIndex < 8; vertexIndex++)
    {
    const Vector3 vertex(
    (vertexIndex & 4) ? obb.GetHalfLengthX() : -obb.GetHalfLengthX(),
    (vertexIndex & 2) ? obb.GetHalfLengthY() : -obb.GetHalfLengthY(),
    (vertexIndex & 1) ? obb.GetHalfLengthZ() : -obb.GetHalfLengthZ());
    // get a vector from the capsule axis to the vertex
    const Vector3 vertexRelative = vertex - capsulePoint1;
    // subtract the part parallel to the axis to get an axis orthogonal to the axis
    const Vector3 testAxis = (vertexRelative - vertexRelative.Dot(capsuleAxis) * capsuleAxis).GetNormalized();

    if (!overlapsAxis(testAxis))
    {
    return false;
    }
    }

    // none of the tested axes were separating axes, the shapes must overlap
    return true;
    }

    bool Overlaps(const Obb& obb1, const Obb& obb2)
    {
    // the separating axis theorem
    // there are up to 15 axes to test:
    // - the 6 axes of the 2 OBBs
    // - the 9 directions formed by taking cross products of the 3 axes of the first OBB with the 3 axes of the second OBB
    // if the projections of the two shapes onto any of those axes do not overlap then the shapes do not overlap

    auto overlapsAxis = [&obb1, &obb2](const Vector3& axis)
    {
    const Vector3 transformedAxis1 = obb1.GetRotation().GetInverseFast().TransformVector(axis);
    const float obb1ProjectedPosition = obb1.GetPosition().Dot(axis);
    const float obb1ProjectedHalfExtent = obb1.GetHalfLengths().Dot(transformedAxis1.GetAbs());
    const float obb1ProjectedMin = obb1ProjectedPosition - obb1ProjectedHalfExtent;
    const float obb1ProjectedMax = obb1ProjectedPosition + obb1ProjectedHalfExtent;
    const Vector3 transformedAxis2 = obb2.GetRotation().GetInverseFast().TransformVector(axis);
    const float obb2ProjectedPosition = obb2.GetPosition().Dot(axis);
    const float obb2ProjectedHalfExtent = obb2.GetHalfLengths().Dot(transformedAxis2.GetAbs());
    const float obb2ProjectedMin = obb2ProjectedPosition - obb2ProjectedHalfExtent;
    const float obb2ProjectedMax = obb2ProjectedPosition + obb2ProjectedHalfExtent;
    return obb1ProjectedMax >= obb2ProjectedMin && obb1ProjectedMin <= obb2ProjectedMax;
    };

    const Vector3 xAxis1 = obb1.GetRotation().TransformVector(AZ::Vector3::CreateAxisX());
    const Vector3 yAxis1 = obb1.GetRotation().TransformVector(AZ::Vector3::CreateAxisY());
    const Vector3 zAxis1 = obb1.GetRotation().TransformVector(AZ::Vector3::CreateAxisZ());
    const Vector3 xAxis2 = obb2.GetRotation().TransformVector(AZ::Vector3::CreateAxisX());
    const Vector3 yAxis2 = obb2.GetRotation().TransformVector(AZ::Vector3::CreateAxisY());
    const Vector3 zAxis2 = obb2.GetRotation().TransformVector(AZ::Vector3::CreateAxisZ());

    return
    overlapsAxis(xAxis1) &&
    overlapsAxis(yAxis1) &&
    overlapsAxis(zAxis1) &&
    overlapsAxis(xAxis2) &&
    overlapsAxis(yAxis2) &&
    overlapsAxis(zAxis2) &&
    overlapsAxis(xAxis1.Cross(xAxis2)) &&
    overlapsAxis(xAxis1.Cross(yAxis2)) &&
    overlapsAxis(xAxis1.Cross(zAxis2)) &&
    overlapsAxis(yAxis1.Cross(xAxis2)) &&
    overlapsAxis(yAxis1.Cross(yAxis2)) &&
    overlapsAxis(yAxis1.Cross(zAxis2)) &&
    overlapsAxis(zAxis1.Cross(xAxis2)) &&
    overlapsAxis(zAxis1.Cross(yAxis2)) &&
    overlapsAxis(zAxis1.Cross(zAxis2));
    }

    bool Overlaps(const Obb& obb, const Capsule& capsule)
    {
    return Overlaps(capsule, obb);
    }

    bool Overlaps(const Obb& obb, const Sphere& sphere)
    {
    return Overlaps(sphere, obb);
    }
    AZ_MATH_INLINE bool Overlaps(const Aabb& aabb1, const Aabb& aabb2)
    {
    return aabb1.Overlaps(aabb2);
    }

    AZ_MATH_INLINE bool Overlaps(const Aabb& aabb, const Sphere& sphere)
    {
    return Overlaps(sphere, aabb);
    }

    AZ_MATH_INLINE bool Overlaps(const Sphere& sphere, const Aabb& aabb)
    {
    float distSq = aabb.get_distance_sq(sphere.get_center());
    float radiusSq = sphere.get_radius() * sphere.get_radius();
    return distSq <= radiusSq;
    }

    AZ_MATH_INLINE bool Overlaps(const Sphere& sphere, const Frustum& frustum)
    {
    return Overlaps(frustum, sphere);
    }

    AZ_MATH_INLINE bool Overlaps(const Sphere& sphere, const Plane& plane)
    {
    const float dist = plane.get_point_dist(sphere.get_center());
    return dist * dist <= sphere.get_radius() * sphere.get_radius();
    }

    AZ_MATH_INLINE bool Overlaps(const Sphere& sphere1, const Sphere& sphere2)
    {
    const float radiusSum = sphere1.get_radius() + sphere2.get_radius();
    return sphere1.get_center().get_distance_sq(sphere2.get_center()) <= (radiusSum * radiusSum);
    }

    AZ_MATH_INLINE bool Overlaps(const Sphere& sphere, const Obb& obb)
    {
    const float radius = sphere.get_radius();
    return obb.get_distance_sq(sphere.get_center()) < radius * radius;
    }

    AZ_MATH_INLINE bool Overlaps(const Sphere& sphere, const Capsule& capsule)
    {
    return Overlaps(capsule, sphere);
    }

    AZ_MATH_INLINE bool Overlaps(const Hemisphere& hemisphere, const Sphere& sphere)
    {
    float sphereDistanceToPlane = hemisphere.get_direction().Dot(sphere.get_center() - hemisphere.get_center());

    if (sphereDistanceToPlane >= 0.0f)
    {
    // Sphere is in front of hemisphere, so treat the hemisphere as a sphere
    return Overlaps(Sphere(hemisphere.get_center(), hemisphere.get_radius()), sphere);
    }
    else if (sphereDistanceToPlane > -sphere.get_radius())
    {
    // Sphere is behind hemisphere, project the sphere onto the plane, then check radius of circle.
    Vector3 projectedSphereCenter = sphere.get_center() + hemisphere.get_direction() * -sphereDistanceToPlane;
    float circleRadius = AZStd::sqrt(sphere.get_radius() * sphere.get_radius() - sphereDistanceToPlane * sphereDistanceToPlane);
    const float radiusSum = hemisphere.get_radius() + circleRadius;
    return hemisphere.get_center().get_distance_sq(projectedSphereCenter) < (radiusSum * radiusSum);
    }
    return false; // too far behind hemisphere to intersect
    }

    AZ_MATH_INLINE bool Overlaps(const Hemisphere& hemisphere, const Aabb& aabb)
    {
    float distSq = aabb.get_distance_sq(hemisphere.get_center());
    float radiusSq = hemisphere.get_radius() * hemisphere.get_radius();
    if (distSq > radiusSq)
    {
    return false;
    }

    if (aabb.Contains(hemisphere.get_center()))
    {
    return true;
    }

    Vector3 nearestPointToPlane = aabb.get_support(-hemisphere.get_direction());
    bool abovePlane = hemisphere.get_direction().Dot(hemisphere.get_center() - nearestPointToPlane) > 0.0f;
    return !abovePlane; // This has false positives but is reasonably tight.
    }

    AZ_MATH_INLINE bool Overlaps(const Frustum& frustum, const Sphere& sphere)
    {
    for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
    {
    if (frustum.get_plane(planeId).get_point_dist(sphere.get_center()) + sphere.get_radius() < 0.0f)
    {
    return false;
    }
    }
    return true;
    }

    AZ_MATH_INLINE bool Overlaps(const Frustum& frustum, const Aabb& aabb)
    {
    //For an AABB, extents.Dot(planeAbs) computes the projection interval radius of the AABB onto the plane normal.
    //So for each plane, we can test compare the center-to-plane distance to this interval to see which side of the plane the AABB is on.
    //The AABB is not overlapping if it is fully behind any of the planes, otherwise it is overlapping.
    const Vector3 center = aabb.get_center();

    //If the AABB contains FLT_MAX at either (or both) extremes, it would be easy to overflow here by using "0.5f * GetExtents()"
    //or "0.5f * (GetMax() - GetMin())".  By separating into two separate multiplies before the subtraction, we can ensure
    //that we don't overflow.
    const Vector3 extents = (0.5f * aabb.GetMax()) - (0.5f * aabb.GetMin());

    for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
    {
    const Plane plane = frustum.get_plane(planeId);
    if (plane.get_point_dist(center) + extents.Dot(plane.GetNormal().GetAbs()) <= 0.0f)
    {
    return false;
    }
    }
    return true;
    }

    AZ_MATH_INLINE bool Overlaps(const Frustum& frustum, const Obb& obb)
    {
    for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
    {
    if (Classify(frustum.get_plane(planeId), obb) == IntersectResult::Exterior)
    {
    return false;
    }
    }

    return true;
    }

    AZ_MATH_INLINE bool Overlaps(const Capsule& capsule1, const Capsule& capsule2)
    {
    Vector3 closestPointSegment1;
    Vector3 closestPointSegment2;
    float segment1Proportion;
    float segment2Proportion;
    Intersect::ClosestSegmentSegment(
    capsule1.GetFirstHemisphereCenter(), capsule1.GetSecondHemisphereCenter(), capsule2.GetFirstHemisphereCenter(),
    capsule2.GetSecondHemisphereCenter(), segment1Proportion, segment2Proportion, closestPointSegment1, closestPointSegment2);
    const float radiusSum = capsule1.get_radius() + capsule2.get_radius();
    return closestPointSegment1.get_distance_sq(closestPointSegment2) <= radiusSum * radiusSum;
    }

    AZ_MATH_INLINE bool Overlaps(const Capsule& capsule, const Sphere& sphere)
    {
    float proportion;
    Vector3 closestPointOnCapsuleAxis;
    Intersect::ClosestPointSegment(sphere.get_center(), capsule.GetFirstHemisphereCenter(), capsule.GetSecondHemisphereCenter(), proportion, closestPointOnCapsuleAxis);
    const float radiusSum = sphere.get_radius() + capsule.get_radius();
    return closestPointOnCapsuleAxis.get_distance_sq(sphere.get_center()) <= radiusSum * radiusSum;
    }

    AZ_MATH_INLINE bool Overlaps(const Aabb& aabb, const Capsule& capsule)
    {
    return Overlaps(capsule, aabb);
    }

    AZ_MATH_INLINE bool Overlaps(const Capsule& capsule, const Aabb& aabb)
    {
    // First attempt a cheap rejection by comparing to the aabb's sphere.
    Vector3 aabbSphereCenter;
    float aabbSphereRadius;
    aabb.GetAsSphere(aabbSphereCenter, aabbSphereRadius);
    Sphere aabbSphere(aabbSphereCenter, aabbSphereRadius);
    if (!Overlaps(capsule, aabbSphere))
    {
    return false;
    }

    // Now do the more expensive test. The idea is to start with the end points of the capsule then
    // - Clamp the points with the aabb
    // - Find the closest points on the line segment to the clamped points.
    // - If the distance between the clamped point and the line segment point is less than the radius, then we know it intersects.
    // - Generate new clamped points from the points on the line segment for next iteration.
    // - If the two clamped points are equal to each other, or either of the new clamped points is equivalent to the previous clamped points,
    //   then we know we've already found the closest point possible on the aabb, so fail because previous distance check failed.
    // - Loop with new clamped points.

    float capsuleRadiusSq = capsule.get_radius() * capsule.get_radius();
    const Vector3& capsuleStart = capsule.GetFirstHemisphereCenter();
    const Vector3& capsuleEnd = capsule.GetSecondHemisphereCenter();
    const Vector3 capsuleSegment = capsuleEnd - capsuleStart;
    if (capsuleSegment.IsClose(AZ::Vector3::CreateZero()))
    {
    // capsule is nearly a sphere, and already failed sphere check above.
    return false;
    }
    const float segmentLengthSquared = capsuleSegment.Dot(capsuleSegment);
    const float rcpSegmentLengthSquared = 1.0f / segmentLengthSquared;

    Vector3 clampedPoint1 = capsuleStart.GetClamp(aabb.GetMin(), aabb.GetMax());
    Vector3 clampedPoint2 = capsuleEnd.GetClamp(aabb.GetMin(), aabb.GetMax());

    // Simplified from Intersect::ClosestPointSegment with certain parts pre-calculated, no need to return proportion.
    auto getClosestPointOnCapsule = [&](const Vector3& point) -> Vector3
    {
    float proportion = (point - capsuleStart).Dot(capsuleSegment);
    if (proportion <= 0.0f)
    {
    return capsuleStart;
    }
    if (proportion >= segmentLengthSquared)
    {
    return capsuleEnd;
    }
    return capsuleStart + (proportion * capsuleSegment * rcpSegmentLengthSquared);
    };

    constexpr uint32_t MaxIterations = 16;
    for (uint32_t i = 0; i < MaxIterations; ++i)
    {
    // Check point 1
    Vector3 closestPointOnCapsuleAxis1 = getClosestPointOnCapsule(clampedPoint1);
    if (clampedPoint1.get_distance_sq(closestPointOnCapsuleAxis1) < capsuleRadiusSq)
    {
    return true;
    }

    // Check point 2
    Vector3 closestPointOnCapsuleAxis2 = getClosestPointOnCapsule(clampedPoint2);
    if (clampedPoint2.get_distance_sq(closestPointOnCapsuleAxis2) < capsuleRadiusSq)
    {
    return true;
    }

    // If the points are the same, and previous tests failed, then this is the best point, but it's too far away.
    if (clampedPoint1.IsClose(clampedPoint2))
    {
    return false;
    }

    // Choose better points.
    Vector3 newclampedPoint1 = closestPointOnCapsuleAxis1.GetClamp(aabb.GetMin(), aabb.GetMax());
    Vector3 newclampedPoint2 = closestPointOnCapsuleAxis2.GetClamp(aabb.GetMin(), aabb.GetMax());

    if (newclampedPoint1.IsClose(clampedPoint1) || newclampedPoint2.IsClose(clampedPoint2))
    {
    // Capsule is parallel to AABB or beyond the end points and failing above tests, so it must be outside the capsule.
    return false;
    }

    clampedPoint1 = newclampedPoint1;
    clampedPoint2 = newclampedPoint2;
    }

    return true; // prefer false positive
    }

    AZ_MATH_INLINE bool Contains(const Aabb& aabb1, const Aabb& aabb2)
    {
    return aabb1.Contains(aabb2);
    }

    AZ_MATH_INLINE bool Contains(const Aabb& aabb, const Sphere& sphere)
    {
    // Convert the sphere to an aabb
    return Contains(aabb, AZ::Aabb::CreateCenterRadius(sphere.get_center(), sphere.get_radius()));
    }

    AZ_MATH_INLINE bool Contains(const Sphere& sphere, const Aabb& aabb)
    {
    const float radiusSq = sphere.get_radius() * sphere.get_radius();
    return aabb.GetMaxDistanceSq(sphere.get_center()) <= radiusSq;
    }

    AZ_MATH_INLINE bool Contains(const Sphere& sphere, const Vector3& point)
    {
    const float distSq = sphere.get_center().get_distance_sq(point);
    const float radiusSq = sphere.get_radius() * sphere.get_radius();
    return distSq <= radiusSq;
    }

    AZ_MATH_INLINE bool Contains(const Sphere& sphere1, const Sphere& sphere2)
    {
    const float radiusDiff = sphere1.get_radius() - sphere2.get_radius();
    return sphere1.get_center().get_distance_sq(sphere2.get_center()) <= (radiusDiff * radiusDiff) * AZ::GetSign(radiusDiff);
    }

    AZ_MATH_INLINE bool Contains(const Hemisphere& hemisphere, const Aabb& aabb)
    {
    const float radiusSq = hemisphere.get_radius() * hemisphere.get_radius();
    if (aabb.GetMaxDistanceSq(hemisphere.get_center()) <= radiusSq)
    {
    // points are inside sphere, check to make sure it's on the right side of the hemisphere plane
    Vector3 nearestPointToPlane = aabb.get_support(hemisphere.get_direction());
    return hemisphere.get_direction().Dot(nearestPointToPlane - hemisphere.get_center()) >= 0.0f;
    }
    return false;
    }

    AZ_MATH_INLINE bool Contains(const Frustum& frustum, const Aabb& aabb)
    {
    // For an AABB, extents.Dot(planeAbs) computes the projection interval radius of the AABB onto the plane normal.
    // So for each plane, we can test compare the center-to-plane distance to this interval to see which side of the plane the AABB is on.
    // The AABB is contained if it is fully in front of all of the planes.
    const Vector3 center = aabb.get_center();
    const Vector3 extents = 0.5f * aabb.GetExtents();

    for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
    {
    const Plane plane = frustum.get_plane(planeId);
    if (plane.get_point_dist(center) - extents.Dot(plane.GetNormal().GetAbs()) < 0.0f)
    {
    return false;
    }
    }
    return true;
    }

    AZ_MATH_INLINE bool Contains(const Frustum& frustum, const Sphere& sphere)
    {
    for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
    {
    if (frustum.get_plane(planeId).get_point_dist(sphere.get_center()) - sphere.get_radius() < 0.0f)
    {
    return false;
    }
    }
    return true;
    }

    AZ_MATH_INLINE bool Contains(const Frustum& frustum, const Vector3& point)
    {
    for (Frustum::PlaneId planeId = Frustum::PlaneId::Near; planeId < Frustum::PlaneId::MAX; ++planeId)
    {
    if (frustum.get_plane(planeId).get_point_dist(point) < 0.0f)
    {
    return false;
    }
    }
    return true;
    }

    AZ_MATH_INLINE bool Contains(const Capsule& capsule, const Sphere& sphere)
    {
    float proportion;
    Vector3 closestPointOnCapsuleAxis;
    Intersect::ClosestPointSegment(sphere.get_center(), capsule.GetFirstHemisphereCenter(), capsule.GetSecondHemisphereCenter(), proportion, closestPointOnCapsuleAxis);
    return Contains(Sphere(closestPointOnCapsuleAxis, capsule.get_radius()), sphere);
    }

    AZ_MATH_INLINE bool Contains(const Capsule& capsule, const Aabb& aabb)
    {
    AZ::Vector3 aabbSphereCenter;
    float aabbSphereRadius;
    aabb.GetAsSphere(aabbSphereCenter, aabbSphereRadius);
    AZ::Sphere aabbSphere(aabbSphereCenter, aabbSphereRadius);

    if (Contains(capsule, aabbSphere))
    {
    return true;
    }
    else if (!Overlaps(capsule, aabbSphere))
    {
    return false;
    }

    // Unable to determine with fast sphere based checks, so check each point in the aabb.
    for (const AZ::Vector3& aabbPoint :
    {
    aabb.GetMin(),
    aabb.GetMax(),
    AZ::Vector3(aabb.GetMin().GetX(), aabb.GetMin().GetY(), aabb.GetMax().GetZ()),
    AZ::Vector3(aabb.GetMin().GetX(), aabb.GetMax().GetY(), aabb.GetMin().GetZ()),
    AZ::Vector3(aabb.GetMin().GetX(), aabb.GetMax().GetY(), aabb.GetMax().GetZ()),
    AZ::Vector3(aabb.GetMax().GetX(), aabb.GetMin().GetY(), aabb.GetMin().GetZ()),
    AZ::Vector3(aabb.GetMax().GetX(), aabb.GetMin().GetY(), aabb.GetMax().GetZ()),
    AZ::Vector3(aabb.GetMax().GetX(), aabb.GetMax().GetY(), aabb.GetMin().GetZ()),
    })
    {
    if (!capsule.Contains(aabbPoint))
    {
    return false;
    }
    }
    return true;
    }
}