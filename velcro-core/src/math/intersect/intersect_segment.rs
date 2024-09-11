#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]


pub struct SegmentTriangleHitTester{

}
impl SegmentTriangleHitTester{
    class SegmentTriangleHitTester
    {
    public:
    //! Construct a hit tester for segment pq that can be used to compare against multiple triangles.
    //! @param p Segment start point.
    //! @param q Segment end point.
    SegmentTriangleHitTester(const AZ::Vector3& p, const AZ::Vector3& q);

    //! For the given segment pq and a triangle abc (CCW), returns whether the segment intersects the
    //! triangle and if so, also returns the time of intersection along the segment and the triangle normal.
    //! @param a Triangle vertex 1.
    //! @param b Triangle vertex 2.
    //! @param c Triangle vertex 3.
    //! @param normal Triangle normal at the intersection point if segment intersects triangle.
    //! @param t Time of intersection along the segment [0.0 (p), 1.0 (q)] if segment intersects triangle.
    //! @return true if the segment intersects the triangle otherwise false.
    bool IntersectSegmentTriangleCCW(
    const AZ::Vector3& a, const AZ::Vector3& b, const AZ::Vector3& c, AZ::Vector3& normal, float& t) const
    {
    constexpr bool oneSided = true;
    return Intersect<oneSided>(a, b, c, normal, t);
}

//! Same as \ref IntersectSegmentTriangleCCW without respecting the triangle (a,b,c) vertex order (i.e. double sided).
//! @param a Triangle vertex 1.
//! @param b Triangle vertex 2.
//! @param c Triangle vertex 3.
//! @param normal Triangle normal at the intersection point if segment intersects triangle.
//! @param t Time of intersection along the segment [0.0 (p), 1.0 (q)] if segment intersects triangle.
//! @return true if the segment intersects the triangle otherwise false.
bool IntersectSegmentTriangle(
const AZ::Vector3& a, const AZ::Vector3& b, const AZ::Vector3& c, AZ::Vector3& normal, float& t) const
{
constexpr bool oneSided = false;
return Intersect<oneSided>(a, b, c, normal, t);
}


//! Gets the segment intersection point for a given Time of intersection 't'.
//! @param t Time of intersection along the segment [0.0 (p), 1.0 (q)].
//! @return The point on the segment where the intersection occurred.
AZ::Vector3 GetIntersectionPoint(float t) const
{
return m_p + (m_pq * t);
}

private:
template<bool oneSided>
bool Intersect(const AZ::Vector3& a, const AZ::Vector3& b, const AZ::Vector3& c, AZ::Vector3& normal, float& t) const;

AZ::Vector3 m_p;    //! Segment start point
AZ::Vector3 m_pq;   //! Segment direction and length

// This algorithm "rotates" the segment and triangle into a coordinate space where the largest component of the segment
// is on the Z axis, while preserving a counter-clockwise winding. The following values are used to remap the XYZ dimensions
// for this rotation.
int m_kx; //! Which XYZ axis is "renamed" to the X dimension for the algorithm.
int m_ky; //! Which XYZ axis is "renamed" to the Y dimension for the algorithm.
int m_kz; //! Which XYZ axis is "renamed" to the Z dimension for the algorithm.

// The segment and triangle are converted to a coordinate space where the segment is a unit ray of (0, 0, 1).
// The following values are used to transform the triangles through translation, shear, and scale.
float m_sx; //! Shear constant to use with the renamed X dimension
float m_sy; //! Shear constant to use with the renamed Y dimension
float m_sz; //! Shear constant to use with the renamed Z dimension
};
}