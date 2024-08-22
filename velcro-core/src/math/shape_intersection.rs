#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

use crate::math::math_utils::get_abs_f32;
use crate::math::plane::Plane;
use crate::math::vector4::Vector4;

#[derive(Debug,Copy, Clone)]
pub struct ShapeIntersection {
    _plane: Vector4
}

impl ShapeIntersection{

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_three_planes(p1:&Plane,p2:&Plane,p3:&Plane,mut outP:&Plane)->bool{
        let n1CrossN2 = p1.get_normal().cross(p2.GetNormal());
        let det = n1CrossN2.dot3(p3.get_normal().borrow());
        if (get_abs_f32(det) >f32::max_value()  FLT_MIN)
        {
            Vector3 n3CrossN2 = p3.GetNormal().Cross(p2.GetNormal());
            Vector3 n1CrossN3 = p1.GetNormal().Cross(p3.GetNormal());

            outP = (n3CrossN2 * p1.GetDistance() + n1CrossN3 * p2.GetDistance() - n1CrossN2 * p3.GetDistance()) / det;
            return true;
        }

        return false;
    }
    bool IntersectThreePlanes(const Plane& p1, const Plane& p2, const Plane& p3, Vector3& outP);

    //! Tests to see which halfspace Arg2 is in relative to Arg1.
    IntersectResult Classify(const Plane& plane, const Sphere& sphere);
    IntersectResult Classify(const Plane& plane, const Obb& obb);

    //! Tests to see how Arg2 relates to frustum Arg1.
    IntersectResult Classify(const Frustum& frustum, const Sphere& sphere);

    //! Tests to see if Arg1 overlaps Arg2. Symmetric.
    //! @{
    bool Overlaps(const Aabb& aabb1, const Aabb& aabb2);
    bool Overlaps(const Aabb& aabb, const Sphere& sphere);
    bool Overlaps(const Sphere& sphere, const Aabb& aabb);
    bool Overlaps(const Sphere& sphere, const Frustum& frustum);
    bool Overlaps(const Sphere& sphere, const Plane& plane);
    bool Overlaps(const Sphere& sphere1, const Sphere& sphere2);
    bool Overlaps(const Sphere& sphere, const Obb& obb);
    bool Overlaps(const Sphere& sphere, const Capsule& capsule);
    bool Overlaps(const Hemisphere& hemisphere, const Sphere& sphere);
    bool Overlaps(const Hemisphere& hemisphere, const Aabb& aabb); // Can have false positives for near intersections.
    bool Overlaps(const Frustum& frustum, const Sphere& sphere);
    bool Overlaps(const Frustum& frustum, const Obb& obb);
    bool Overlaps(const Frustum& frustum, const Aabb& aabb);
    bool Overlaps(const Capsule& capsule1, const Capsule& capsule2);
    bool Overlaps(const Capsule& capsule, const Obb& obb);
    bool Overlaps(const Capsule& capsule, const Sphere& sphere);
    bool Overlaps(const Capsule& capsule, const Aabb& aabb);
    bool Overlaps(const Aabb& aabb, const Capsule& capsule);
    bool Overlaps(const Obb& obb1, const Obb& obb2);
    bool Overlaps(const Obb& obb, const Capsule& capsule);
    bool Overlaps(const Obb& obb, const Sphere& sphere);
    //! @}

    //! Tests to see if Arg1 contains Arg2. Non Symmetric.
    //! @{
    bool Contains(const Aabb& aabb1, const Aabb& aabb2);
    bool Contains(const Aabb& aabb, const Sphere& sphere);
    bool Contains(const Sphere& sphere,  const Aabb& aabb);
    bool Contains(const Sphere& sphere,  const Vector3& point);
    bool Contains(const Sphere& sphere1, const Sphere& sphere2);
    bool Contains(const Hemisphere& hemisphere, const Aabb& aabb);
    bool Contains(const Capsule& capsule, const Sphere& sphere);
    bool Contains(const Capsule& capsule, const Aabb& aabb);
    bool Contains(const Frustum& frustum,  const Aabb& aabb);
    bool Contains(const Frustum& frustum,  const Sphere& sphere);
    bool Contains(const Frustum& frustum,  const Vector3& point);
}