#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::aabb::Aabb;
use crate::math::intersect::intersect_point::RayAABBIsectTypes::ISECT_RAY_AABB_NONE;
use crate::math::intersect::intersect_segment::SegmentTriangleHitTester;
use crate::math::math_utils::constants;
use crate::math::plane::Plane;
use crate::math::vector3::Vector3;

pub enum RayAABBIsectTypes
{
    ISECT_RAY_AABB_NONE = 0, ///< no intersection
    ISECT_RAY_AABB_SA_INSIDE, ///< the ray starts inside the aabb
    ISECT_RAY_AABB_ISECT, ///< intersects along the PQ segment
}

pub struct Intersect{

}

impl Intersect{

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn barycentric(a:&Vector3,b:&Vector3,c:&Vector3,p:&Vector3) ->Vector3{
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;
        let d00 = v0.dot3(v0.borrow());
        let d01 = v0.dot3(v1.borrow());
        let d11 = v1.dot3(v1.borrow());
        let d20 = v2.dot3(v0.borrow());
        let d21 = v2.dot3(v1.borrow());
        let denom = d00 * d11 - d01 * d01;
        let denom_rcp = 1.0 / denom;
        let v = (d11 * d20 - d01 * d21) * denom_rcp;
        let w = (d00 * d21 - d01 * d20) * denom_rcp;
        let u = 1.0 - v - w;
        return Vector3::new_xyz(u, v, w)
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn test_point_triangle(p:&Vector3, a:&Vector3, b:&Vector3, c:&Vector3) ->bool{
        let uvw =Intersect::barycentric(a,b,c,p);
        return uvw.is_greater_equal_than(Vector3::create_zero().borrow());
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn test_point_triangle_ccw(p:&Vector3, a:&Vector3, b:&Vector3, c:&Vector3)->bool{
        let p_a = a - p;
        let p_b = b - p;
        let p_c = c - p;
        let ab = p_a.dot3(p_b.borrow());
        let ac = p_a.dot3(p_c.borrow());
        let bc = p_b.dot3(p_c.borrow());
        let cc = p_c.dot3(p_c.borrow());

        if (bc * ac - cc * ab < 0.0)
        {
            return false;
        }

        let bb = p_b.dot3(p_b.borrow());

        if (ab * bc - ac * bb < 0.0)
        {
            return false;
        }

        return true;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn closest_point_plane(p:&Vector3, plane:&Plane, mut pt_on_plane:&Vector3) ->f32{
        let dist = plane.get_point_dist(p);
        pt_on_plane = &(p -   plane.get_normal() * dist);
        return dist;
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn closest_point_triangle(p:&Vector3, a:&Vector3, b:&Vector3, c:&Vector3)->Vector3{
        let ab = b - a;
        let ac = c - a;
        let ap = p - a;
        let d1 = ab.dot3(ap.borrow());
        let d2 = ac.dot3(ap.borrow());

        if (d1 <= 0.0 && d2 <= 0.0)
        {
            return a.to_owned();
        }
        let bp = p - b;
        let d3 = ab.dot3(bp.borrow());
        let d4 = ac.dot3(bp.borrow());
        if (d3 >= 0.0 && d4 <= d3)
        {
            return b.to_owned();
        }
        let vc = d1 * d4 - d3 * d2;
        if (vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0)
        {
            let v = d1 / (d1 - d3);
            return   ab * v + a.to_owned();
        }

        let cp = p - c;
        let d5 = ab.dot3(cp.borrow());
        let d6 = ac.dot3(cp.borrow());
        if (d6 >= 0.0 && d5 <= d6)
        {
            return c.to_owned();
        }

        let vb = d5 * d2 - d1 * d6;
        if (vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0)
        {
            let w = d2 / (d2 - d6);
            return ac * w +a.to_owned();
        }

        let va = d3 * d6 - d5 * d4;
        if (va <= 0.0 && d4 >= d3 && d5 >= d6)
        {
            let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
            return (c - b) *w +b.to_owned();
        }

        let denom_rcp = 1.0 / (va + vb + vc);
        let v = vb * denom_rcp;
        let w = vc * denom_rcp;
        return  ab * v + ac * w + a.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe  fn point_sphere(center_position:&Vector3, radius_squared:&f32, test_point:&Vector3) ->bool{
        return test_point.get_distance_sq(center_position) < radius_squared.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn point_cylinder(base_center_point:&Vector3, axis_vector:&Vector3, axis_length_squared:&f32, radius_squared:&f32, test_point:&Vector3) ->bool{
        if (axis_length_squared.to_owned() <= 0.0 || radius_squared.to_owned() <= 0.0)
        {
            return false;
        }


        let base_center_point_to_test_point = test_point - base_center_point;
        let dot_product = base_center_point_to_test_point.dot3(axis_vector);

        // If the dot is < 0, the point is below the base cap of the cylinder, if it's > lengthSquared then it's beyond the other cap.
        if (dot_product < 0.0 || dot_product > axis_length_squared.to_owned())
        {
            return false;
        }
        else
        {
            let distance_squared = (base_center_point_to_test_point.get_length_sq()) - (dot_product * dot_product / axis_length_squared);
            return distance_squared <= radius_squared.to_owned();
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn line_to_point_distance_time(s1:&Vector3, s21:&Vector3, p:&Vector3) ->f32{
        return s21.dot3((p - s1).borrow()) / s21.dot3(s21);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn line_to_point_distance(s1:&Vector3,s2:&Vector3,p:&Vector3,mut u:&f32)->Vector3{
        let s21 = s2 - s1;
        assert!(!s21.is_close(Vector3(0.0), 1e-4), "OK we agreed that we will pass valid segments! (s1 != s2)");

        u = Self::line_to_point_distance_time(s1, s21.borrow(), p).borrow_mut();

        return s1 +s21 * u.to_owned();
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_segment_triangle_ccw(p:&Vector3,q:&Vector3,a:&Vector3,b:&Vector3,c:&Vector3,normal:&Vector3,t:&f32)->bool{
        let hit_tester = SegmentTriangleHitTester::new_2vec3(p, q);
        return hit_tester.intersect_segment_triangle_ccw(a, b, c, normal, t);
    }


    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_segment_triangle(p:&Vector3,q:&Vector3,a:&Vector3,b:&Vector3,c:&Vector3,normal:&Vector3,t:&f32)->bool{
        let hit_tester = SegmentTriangleHitTester::new_2vec3(p, q);
        return hit_tester.intersect_segment_triangle(a, b, c, normal, t);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn IntersectRayAABB(rayStart:&Vector3,dir:&Vector3,dirRCP:&Vector3,aabb:&Aabb,tStart:&f32,tEnd:&f32,startNormal:&Vector3)->RayAABBIsectTypes{
        let eps = 0.0001f32;
        let mut tmin = 0.0f32;
        let mut tmax = f32::MAX;

        let aabbMin = aabb.get_min();
        let aabbMax = aabb.get_max();

        let time1 = (aabbMin - rayStart.to_owned()) * dirRCP;
        let time2 = (aabbMax - rayStart.to_owned()) * dirRCP;

        if (dir.get_x().abs() < eps)
        {
            if (rayStart.get_x() < aabbMin.get_x() || rayStart.get_x() > aabbMax.get_x())
            {
                return ISECT_RAY_AABB_NONE;
            }
        }
        else
        {
            let t1 = time1.get_x();
            let t2 = time2.get_x();
            let mut nSign = -1.0f32;

            // Make t1 be intersection with near plane, t2 with far plane
            if (t1 > t2)
            {

                AZStd::swap(t1, t2);
                nSign = 1.0;
            }

            // Compute the intersection of slab intersections intervals
            if (tmin < t1)
            {
                tmin = t1;

                startNormal.Set(nSign, 0.0, 0.0);
            }

            tmax = constants::get_min(tmax, t2);

            // Exit with no collision as soon as slab intersection becomes empty
            if (tmin > tmax)
            {
                return ISECT_RAY_AABB_NONE;
            }
        }

        if (dir.get_y().abs()< eps)
        {
            if (rayStart.get_y() < aabbMin.get_y() || rayStart.get_y() > aabbMax.get_y())
            {
                return ISECT_RAY_AABB_NONE;
            }
        }
        else
        {
            let t1 = time1.get_y();
            let t2 = time2.get_y();
            let mut nSign = -1.0f32;

            if (t1 > t2)
            {
                AZStd::swap(t1, t2);
                nSign = 1.0;
            }

            if (tmin < t1)
            {
                tmin = t1;

                startNormal.set_value_xyz(0.0, nSign, 0.0);
            }

            tmax = constants::get_min(tmax, t2);

            if (tmin > tmax)
            {
                return ISECT_RAY_AABB_NONE;
            }
        }

        // Z
        if (dir.get_z().abs() < eps)
        {
            if (rayStart.get_z() < aabbMin.get_z() || rayStart.get_z() > aabbMax.get_z())
            {
                return ISECT_RAY_AABB_NONE;
            }
        }
        else
        {
            // Compute intersection t value of ray with near and far plane of slab
            float t1 = time1.GetZ();
            float t2 = time2.GetZ();
            float nSign = -1.0;

            // Make t1 be intersection with near plane, t2 with far plane
            if (t1 > t2)
            {
                AZStd::swap(t1, t2);
                nSign = 1.0f;
            }

            // Compute the intersection of slab intersections intervals
            if (tmin < t1)
            {
                tmin = t1;

                startNormal.Set(0.0f, 0.0f, nSign);
            }

            tmax = AZ::GetMin(tmax, t2);

            // Exit with no collision as soon as slab intersection becomes empty
            if (tmin > tmax)
            {
                return ISECT_RAY_AABB_NONE;
            }
        }

        tStart = tmin;
        tEnd = tmax;

        if (tmin == 0.0f) // no intersect if the segments starts inside or coincident the aabb
        {
            return ISECT_RAY_AABB_SA_INSIDE;
        }

        // Ray intersects all 3 slabs. Return point (q) and intersection t value (tmin)
        // inter = rayStart + dir * tmin;
        return ISECT_RAY_AABB_ISECT;
    }

    RayAABBIsectTypes IntersectRayAABB(
const Vector3& rayStart,
const Vector3& dir,
const Vector3& dirRCP,
const Aabb& aabb,
float& tStart,
float& tEnd,
Vector3& startNormal);

//! Intersect ray against AABB.
//! @param rayStart Ray starting point.
//! @param dir Ray reciprocal direction.
//! @param aabb Axis aligned bounding box to intersect against.
//! @param start Length on ray of the first intersection.
//! @param end Length of the of the second intersection.
//! @return \ref RayAABBIsectTypes In this faster version than IntersectRayAABB we return only ISECT_RAY_AABB_NONE and
//! ISECT_RAY_AABB_ISECT. You can check yourself for that case.
RayAABBIsectTypes IntersectRayAABB2(const Vector3& rayStart, const Vector3& dirRCP, const Aabb& aabb, float& start, float& end);

//! Clip a ray to an aabb. return true if ray was clipped. The ray
//! can be inside so don't use the result if the ray intersect the box.
//! @param aabb Bounds to test against.
//! @param rayStart The start of the ray.
//! @param rayEnd The end of the ray.
//! @param[out] tClipStart The proportion where the ray enters the \ref Aabb.
//! @param[out] tClipEnd The proportion where the ray exits the \ref Aabb.
//! @return True if the ray was clipped, otherwise false.
bool ClipRayWithAabb(const Aabb& aabb, Vector3& rayStart, Vector3& rayEnd, float& tClipStart, float& tClipEnd);

//! Test segment and aabb where the segment is defined by midpoint
//! midPoint = (p1-p0) * 0.5f and half vector halfVector = p1 - midPoint.
//! the aabb is at the origin and defined by half extents only.
//! @param midPoint Midpoint of a line segment.
//! @param halfVector Half vector of an aabb.
//! @param aabbExtends The extends of a bounded box.
//! @return True if the segment and AABB intersect, otherwise false
bool TestSegmentAABBOrigin(const Vector3& midPoint, const Vector3& halfVector, const Vector3& aabbExtends);

//! Test if segment specified by points p0 and p1 intersects AABB. \ref TestSegmentAABBOrigin.
//! @param p0 Segment start point.
//! @param p1 Segment end point.
//! @param aabb Bounded box to test against.
//! @return True if the segment and AABB intersect, otherwise false.
bool TestSegmentAABB(const Vector3& p0, const Vector3& p1, const Aabb& aabb);

//! Ray sphere intersection result types.
enum SphereIsectTypes : AZ::s32
{
ISECT_RAY_SPHERE_SA_INSIDE = -1, //!< The ray starts inside the cylinder
ISECT_RAY_SPHERE_NONE, //!< No intersection
ISECT_RAY_SPHERE_ISECT, //!< Along the PQ segment
};

//! IntersectRaySphereOrigin
//! return time t>=0  but not limited, so if you check a segment make sure
//! t <= segmentLen.
//! @param rayStart ray start point.
//! @param rayDirNormalized ray direction normalized.
//! @param shereRadius Radius of sphere at origin.
//! @param time of closest intersection [0,+INF] in relation to the normalized direction.
//! @return \ref SphereIsectTypes.
SphereIsectTypes IntersectRaySphereOrigin(
const Vector3& rayStart, const Vector3& rayDirNormalized, const float sphereRadius, float& t);

//! Intersect ray (rayStart,rayDirNormalized) and sphere (sphereCenter,sphereRadius) \ref IntersectRaySphereOrigin
//! @param rayStart The start of the ray.
//! @param rayDirNormalized The direction of the ray normalized.
//! @param sphereCenter The center of the sphere.
//! @param sphereRadius Radius of the sphere.
//! @param[out] t Coefficient in the ray's explicit equation from which an
//! intersecting point is calculated as "rayOrigin + t1 * rayDir".
//! @return SphereIsectTypes
SphereIsectTypes IntersectRaySphere(
const Vector3& rayStart, const Vector3& rayDirNormalized, const Vector3& sphereCenter, const float sphereRadius, float& t);

//! Intersect ray (rayStarty, rayDirNormalized) and disk (center, radius, normal)
//! @param rayOrigin The origin of the ray to test.
//! @param rayDir The direction of the ray to test. It has to be unit length.
//! @param diskCenter Center point of the disk.
//! @param diskRadius Radius of the disk.
//! @param diskNormal A normal perpendicular to the disk.
//! @param[out] t If returning 1 (indicating a hit), this contains distance from rayOrigin along the normalized rayDir
//! that the hit occured at.
//! @return False if not interesecting and true if intersecting
bool IntersectRayDisk(
const Vector3& rayOrigin,
const Vector3& rayDir,
const Vector3& diskCenter,
const float diskRadius,
const AZ::Vector3& diskNormal,
float& t);

//! If there is only one intersecting point, the coefficient is stored in \ref t1.
//! @param rayOrigin The origin of the ray to test.
//! @param rayDir The direction of the ray to test. It has to be unit length.
//! @param cylinderEnd1 The center of the circle on one end of the cylinder.
//! @param cylinderDir The direction pointing from \ref cylinderEnd1 to the other end of the cylinder. It has to be unit length.
//! @param cylinderHeight The distance between two centers of the circles on two ends of the cylinder respectively.
//! @param[out] t1 A possible coefficient in the ray's explicit equation from which an intersecting point is calculated as "rayOrigin + t1 * rayDir".
//! @param[out] t2 A possible coefficient in the ray's explicit equation from which an intersecting point is calculated as "rayOrigin + t2 * rayDir".
//! @return The number of intersecting points.
int IntersectRayCappedCylinder(
const Vector3& rayOrigin,
const Vector3& rayDir,
const Vector3& cylinderEnd1,
const Vector3& cylinderDir,
float cylinderHeight,
float cylinderRadius,
float& t1,
float& t2);

//! If there is only one intersecting point, the coefficient is stored in \ref t1.
//! @param rayOrigin The origin of the ray to test.
//! @param rayDir The direction of the ray to test. It has to be unit length.
//! @param coneApex The apex of the cone.
//! @param coneDir The unit-length direction from the apex to the base.
//! @param coneHeight The height of the cone, from the apex to the base.
//! @param coneBaseRadius The radius of the cone base circle.
//! @param[out] t1 A possible coefficient in the ray's explicit equation from which an intersecting point is calculated as "rayOrigin + t1 * rayDir".
//! @param[out] t2 A possible coefficient in the ray's explicit equation from which an intersecting point is calculated as "rayOrigin + t2 * rayDir".
//! @return The number of intersecting points.
int IntersectRayCone(
const Vector3& rayOrigin,
const Vector3& rayDir,
const Vector3& coneApex,
const Vector3& coneDir,
float coneHeight,
float coneBaseRadius,
float& t1,
float& t2);

//! Test intersection between a ray and a plane in 3D.
//! @param rayOrigin The origin of the ray to test intersection with.
//! @param rayDir The direction of the ray to test intersection with.
//! @param planePos A point on the plane to test intersection with.
//! @param planeNormal The normal of the plane to test intersection with.
//! @param[out] t The coefficient in the ray's explicit equation from which the intersecting point is calculated as "rayOrigin + t * rayDirection".
//! @return The number of intersection point.
int IntersectRayPlane(
const Vector3& rayOrigin, const Vector3& rayDir, const Vector3& planePos, const Vector3& planeNormal, float& t);

//! Test intersection between a ray and a two-sided quadrilateral defined by four points in 3D.
//! The four points that define the quadrilateral could be passed in with either counter clock-wise
//! winding or clock-wise winding.
//! @param rayOrigin The origin of the ray to test intersection with.
//! @param rayDir The direction of the ray to test intersection with.
//! @param vertexA One of the four points that define the quadrilateral.
//! @param vertexB One of the four points that define the quadrilateral.
//! @param vertexC One of the four points that define the quadrilateral.
//! @param vertexD One of the four points that define the quadrilateral.
//! @param[out] t The coefficient in the ray's explicit equation from which the
//! intersecting point is calculated as "rayOrigin + t * rayDirection".
//! @return The number of intersection point.
int IntersectRayQuad(
const Vector3& rayOrigin,
const Vector3& rayDir,
const Vector3& vertexA,
const Vector3& vertexB,
const Vector3& vertexC,
const Vector3& vertexD,
float& t);

//!  Test intersection between a ray and an oriented box in 3D.
//! @param rayOrigin The origin of the ray to test intersection with.
//! @param rayDir The direction of the ray to test intersection with.
//! @param boxCenter The position of the center of the box.
//! @param boxAxis1 An axis along one dimension of the oriented box.
//! @param boxAxis2 An axis along one dimension of the oriented box.
//! @param boxAxis3 An axis along one dimension of the oriented box.
//! @param boxHalfExtent1 The half extent of the box on the dimension of \ref boxAxis1.
//! @param boxHalfExtent2 The half extent of the box on the dimension of \ref boxAxis2.
//! @param boxHalfExtent3 The half extent of the box on the dimension of \ref boxAxis3.
//! @param[out] t The coefficient in the ray's explicit equation from which the intersecting point is calculated as "rayOrigin + t * rayDirection".
//! @return true if there is an intersection, false otherwise.
bool IntersectRayBox(
const Vector3& rayOrigin,
const Vector3& rayDir,
const Vector3& boxCenter,
const Vector3& boxAxis1,
const Vector3& boxAxis2,
const Vector3& boxAxis3,
float boxHalfExtent1,
float boxHalfExtent2,
float boxHalfExtent3,
float& t);

//! Test intersection between a ray and an OBB.
//! @param rayOrigin The origin of the ray to test intersection with.
//! @param rayDir The direction of the ray to test intersection with.
//! @param obb The OBB to test for intersection with the ray.
//! @param[out] t The coefficient in the ray's explicit equation from which the intersecting point is calculated as "rayOrigin + t * rayDirection".
//! @return True if there is an intersection, false otherwise.
bool IntersectRayObb(const Vector3& rayOrigin, const Vector3& rayDir, const Obb& obb, float& t);

//! Ray cylinder intersection types.
enum CylinderIsectTypes : AZ::s32
{
RR_ISECT_RAY_CYL_SA_INSIDE = -1, //!< the ray starts inside the cylinder
RR_ISECT_RAY_CYL_NONE, //!< no intersection
RR_ISECT_RAY_CYL_PQ, //!< along the PQ segment
RR_ISECT_RAY_CYL_P_SIDE, //!< on the P side
RR_ISECT_RAY_CYL_Q_SIDE, //!< on the Q side
};

//! Reference: Real-Time Collision Detection - 5.3.7 Intersecting Ray or Segment Against Cylinder
//! Intersect segment S(t)=sa+t(dir), 0<=t<=1 against cylinder specified by p, q and r.
//! @param sa The initial point.
//! @param dir Magnitude and direction for sa.
//! @param p Center point of side 1 cylinder.
//! @param q Center point of side 2 cylinder.
//! @param r Radius of cylinder.
//! @param[out] t Proporition along line segment.
//! @return CylinderIsectTypes
CylinderIsectTypes IntersectSegmentCylinder(
const Vector3& sa, const Vector3& dir, const Vector3& p, const Vector3& q, const float r, float& t);

//! Capsule ray intersect types.
enum CapsuleIsectTypes
{
    ISECT_RAY_CAPSULE_SA_INSIDE = -1, //!< The ray starts inside the cylinder
    ISECT_RAY_CAPSULE_NONE, //!< No intersection
    ISECT_RAY_CAPSULE_PQ, //!< Along the PQ segment
    ISECT_RAY_CAPSULE_P_SIDE, //!< On the P side
    ISECT_RAY_CAPSULE_Q_SIDE, //!< On the Q side
};

//! This is a quick implementation of segment capsule based on segment cylinder \ref IntersectSegmentCylinder
//! segment sphere intersection. We can optimize it a lot once we fix the ray
//! cylinder intersection.
//! @param sa The beginning of the line segment.
//! @param dir The direction and length of the segment.
//! @param p Center point of side 1 capsule.
//! @param q Center point of side 1 capsule.
//! @param r The radius of the capsule.
//! @param[out] t Proporition along line segment.
//! @return CapsuleIsectTypes
CapsuleIsectTypes IntersectSegmentCapsule(
const Vector3& sa, const Vector3& dir, const Vector3& p, const Vector3& q, const float r, float& t);

//! Intersect segment S(t)=A+t(B-A), 0<=t<=1 against convex polyhedron specified
//! by the n halfspaces defined by the planes p[]. On exit tfirst and tlast
//! define the intersection, if any.
//! @param sa The beggining of the line segment.
//! @param dir The direction and length of the segment.
//! @param p Planes that compose a convex ponvex polyhedron.
//! @param numPlanes number of planes.
//! @param[out] tfirst Proportion along the line segment where the line enters.
//! @param[out] tlast Proportion along the line segment where the line exits.
//! @param[out] iFirstPlane The plane where the line enters.
//! @param[out] iLastPlane The plane where the line exits.
//! @return True if intersects else false.
bool IntersectSegmentPolyhedron(
const Vector3& sa,
const Vector3& dir,
const Plane p[],
int numPlanes,
float& tfirst,
float& tlast,
int& iFirstPlane,
int& iLastPlane);

//! Calculate the line segment closestPointSegment1<->closestPointSegment2 that is the shortest route between
//! two segments segment1Start<->segment1End and segment2Start<->segment2End. Also calculate the values of segment1Proportion and
//! segment2Proportion where closestPointSegment1 = segment1Start + (segment1Proportion * (segment1End - segment1Start))
//! closestPointSegment2 = segment2Start + (segment2Proportion * (segment2End - segment2Start))
//! If segments are parallel returns a solution.
//! @param segment1Start Start of segment 1.
//! @param segment1End End of segment 1.
//! @param segment2Start Start of segment 2.
//! @param segment2End End of segment 2.
//! @param[out] segment1Proportion The proporition along segment 1 [0..1]
//! @param[out] segment2Proportion The proporition along segment 2 [0..1]
//! @param[out] closestPointSegment1 Closest point on segment 1.
//! @param[out] closestPointSegment2 Closest point on segment 2.
//! @param epsilon The minimum square distance where a line segment can be treated as a single point.
void ClosestSegmentSegment(
const Vector3& segment1Start,
const Vector3& segment1End,
const Vector3& segment2Start,
const Vector3& segment2End,
float& segment1Proportion,
float& segment2Proportion,
Vector3& closestPointSegment1,
Vector3& closestPointSegment2,
float epsilon = 1e-4f);

//! Calculate the line segment closestPointSegment1<->closestPointSegment2 that is the shortest route between
//! two segments segment1Start<->segment1End and segment2Start<->segment2End.
//! If segments are parallel returns a solution.
//! @param segment1Start Start of segment 1.
//! @param segment1End End of segment 1.
//! @param segment2Start Start of segment 2.
//! @param segment2End End of segment 2.
//! @param[out] closestPointSegment1 Closest point on segment 1.
//! @param[out] closestPointSegment2 Closest point on segment 2.
//! @param epsilon The minimum square distance where a line segment can be treated as a single point.
void ClosestSegmentSegment(
const Vector3& segment1Start,
const Vector3& segment1End,
const Vector3& segment2Start,
const Vector3& segment2End,
Vector3& closestPointSegment1,
Vector3& closestPointSegment2,
float epsilon = 1e-4f);

//! Calculate the point (closestPointOnSegment) that is the closest point on
//! segment segmentStart/segmentEnd to point. Also calculate the value of proportion where
//! closestPointOnSegment = segmentStart + (proportion * (segmentEnd - segmentStart))
//! @param point The point to test
//! @param segmentStart The start of the segment
//! @param segmentEnd The end of the segment
//! @param[out] proportion The proportion of the segment  L(t) = (end - start) * t
//! @param[out] closestPointOnSegment The point along the line segment
void ClosestPointSegment(
const Vector3& point,
const Vector3& segmentStart,
const Vector3& segmentEnd,
float& proportion,
Vector3& closestPointOnSegment);

//! Calculate the distance squared from the provided point to the closest point on
//! segment segmentStart/segmentEnd.
//! @param point The point to test
//! @param segmentStart The start of the segment
//! @param segmentEnd The end of the segment
//! @return the distance squared from the point to the segment.
float PointSegmentDistanceSq(const Vector3& point, const Vector3& segmentStart, const Vector3& segmentEnd);
}

