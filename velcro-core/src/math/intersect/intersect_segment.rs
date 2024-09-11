#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ptr;

use crate::math::vector3::Vector3;

pub struct SegmentTriangleHitTester{
    _p:Vector3,
    _pq:Vector3,

    _kx:i32,
    _ky:i32,
    _kz:i32,

    _sx:f32,
    _sy:f32,
    _sz:f32
}
impl SegmentTriangleHitTester{

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new()->SegmentTriangleHitTester {
        SegmentTriangleHitTester {
            _p:Vector3::new(),
            _pq:Vector3::new(),
            _kx:0,
            _ky:0,
            _kz:0,
            _sx:0.0,
            _sy:0.0,
            _sz:0.0

        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn segment_triangle_hit_tester(p:&Vector3,q:&Vector3)->SegmentTriangleHitTester{
       let mut result = SegmentTriangleHitTester::new();
        result._p = p.to_owned();
        result._pq = q-p;
        let x_magnitude = result._pq.get_x().abs();
        let y_magnitude = result._pq.get_y().abs();
        let z_magnitude = result._pq.get_z().abs();


        result._kz = 0;
        result._kx = 1;
        result._ky = 2;

        if (z_magnitude >= y_magnitude)
        {
            if (z_magnitude >= x_magnitude)
            {
                result._kz = 2;
                result._kx = 0;
                result._ky = 1;
            }
        }
        else
        {
            if (y_magnitude >= x_magnitude)
            {
                result._kz = 1;
                result._kx = 2;
                result._ky = 0;
            }
        }

        if result._pq.get_element(result._kz) < 0.0
        {
            ptr::swap(result._kx.borrow_mut(),result._ky.borrow_mut());
        }

        result._sz = 1.0 / result._pq.get_element(result._kz);
        result._sx = result._pq.get_element(result._kx) * result._sz;
        result._sy = result._pq.get_element(result._ky) * result._sz;
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn IntersectSegmentTriangleCCW(a:){

    }
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


}