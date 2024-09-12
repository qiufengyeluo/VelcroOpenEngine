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
    pub unsafe fn intersect_segment_triangle_ccw(self, a:&Vector3,b:&Vector3,c:&Vector3,normal:&Vector3,t:&f32){
        return self.intersect::<{true}>(a, b, c, normal, t);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn intersect_segment_triangle(self,a:&Vector3,b:&Vector3,c:&Vector3,normal:&Vector3,t:&f32)->bool{
        return self.intersect::<{false}>(a, b, c, normal, t);
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn get_intersection_point(self, t:f32)->Vector3{
        return self._p + (self._pq * t);
    }

    #[inline]
    #[allow(dead_code)]
    unsafe fn intersect<const oneSided:bool>(self,a:&Vector3,b:&Vector3,c:&Vector3,normal:&Vector3,t:&f32)->bool{

        let A = a - self._p;
        let B = b - self._p;
        let C = c - self._p;


        let Ax = A.get_element(self._kx) - self._sx * A.get_element(self._kz);
        let Ay = A.get_element(self._ky) - self._sy * A.get_element(self._kz);
        let Bx = B.get_element(self._kx) - self._sx * B.get_element(self._kz);
        let By = B.get_element(self._ky) - self._sy * B.get_element(self._kz);
        let Cx = C.get_element(self._kx) - self._sx * C.get_element(self._kz);
        let Cy = C.get_element(self._ky) - self._sy * C.get_element(self._kz);

        let mut U = Cx * By - Cy * Bx;
        let mut V = Ax * Cy - Ay * Cx;
        let mut W = Bx * Ay - By * Ax;


        if U == 0.0 || V == 0.0 || W == 0.0
        {
            let CxBy = aznumeric_cast<double>(Cx) * aznumeric_cast<double>(By);
            let CyBx = aznumeric_cast<double>(Cy) * aznumeric_cast<double>(Bx);
            U = aznumeric_cast<float>(CxBy - CyBx);
            let AxCy = aznumeric_cast<double>(Ax) * aznumeric_cast<double>(Cy);
            let AyCx = aznumeric_cast<double>(Ay) * aznumeric_cast<double>(Cx);
            V = aznumeric_cast<float>(AxCy - AyCx);
            let BxAy = aznumeric_cast<double>(Bx) * aznumeric_cast<double>(Ay);
            let ByAx = aznumeric_cast<double>(By) * aznumeric_cast<double>(Ax);
            W = aznumeric_cast<float>(BxAy - ByAx);
        }

        if (U < 0.0 || V < 0.0 || W < 0.0)
        {
            if constexpr(oneSided)
            {
                return false;
            }
            else if U > 0.0 || V > 0.0 || W > 0.0
            {
                return false;
            }
        }

        let det = U + V + W;

        if (det == 0.0)
        {
            return false;
        }

        let Az = self._sz * A.GetElement(m_kz);
        let Bz = self._sz * B.GetElement(m_kz);
        let Cz = self._sz * C.GetElement(m_kz);
        let T = U * Az + V * Bz + W * Cz;

        // Since we're testing a segment, not a ray, T/det needs to be in [0,1] to be considered a hit, as anything outside those
        // bounds will fall beyond the endpoints of the segment.
        if constexpr(oneSided)
        {
            // For one-sided triangles, we need to have 0 <= T < = det, or else T is beyond the endpoints.
            if (T < 0.0 || T > det)
            {
                return false;
            }

            // We've determined it's a hit, so use the untransformed triangle vertices to calculate the normal of the triangle face
            // in the original coordinate space to return back from the API.
            normal = (b - a).cross((c - a)).GetNormalized();
        }
        else
        {
            // For two-sided triangles, we either need to have 0 <= T <= det if det is positive,
            // or 0 <= -T <= -det if det is negative. Otherwise, T is beyond the endpoints.
            const float detSign = signbit(det) ? -1.0 : 1.0;
            if ((T * detSign) < 0.0 || (T * detSign) > (det * detSign))
            {
                return false;
            }

            // We've determined it's a hit, so use the untransformed triangle vertices to calculate the normal of the triangle face
            // in the original coordinate space to return back from the API. For two-sided triangles, we use the sign of the determinant
            // to potentially flip the normal in the case that the back side of the triangle had the intersection.
            normal = detSign * (b - a).Cross((c - a)).GetNormalized();
        }

        // Finally, normalize T into [0, 1] space so that it represents the hit distance along the segment.
        const float detReciprocal = 1.0 / det;
        t = T * detReciprocal;

        // If the barycentric coordinates of the hit point are ever needed, they would need to be normalized into [0,1] space
        // before getting returned:
        // u = U * detReciprocal;
        // v = V * detReciprocal;
        // w = W * detReciprocal;

        return true;
    }
}