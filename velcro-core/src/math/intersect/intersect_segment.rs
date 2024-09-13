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
    pub unsafe fn new_2vec3(p:&Vector3,q:&Vector3)->SegmentTriangleHitTester{
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
    pub unsafe fn intersect_segment_triangle_ccw(self, a:&Vector3,b:&Vector3,c:&Vector3,normal:&Vector3,t:&f32)->bool{
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
    unsafe fn intersect<const oneSided:bool>(self,a:&Vector3,b:&Vector3,c:&Vector3,mut normal:&Vector3,mut t:&f32)->bool{

        let av = a - self._p;
        let bv = b - self._p;
        let cv = c - self._p;


        let ax = av.get_element(self._kx) - self._sx * av.get_element(self._kz);
        let ay = av.get_element(self._ky) - self._sy * av.get_element(self._kz);
        let bx = bv.get_element(self._kx) - self._sx * bv.get_element(self._kz);
        let by = bv.get_element(self._ky) - self._sy * bv.get_element(self._kz);
        let cx = cv.get_element(self._kx) - self._sx * cv.get_element(self._kz);
        let cy = cv.get_element(self._ky) - self._sy * cv.get_element(self._kz);

        let mut uv = cx * by - cy * bx;
        let mut vv = ax * cy - ay * cx;
        let mut wv = bx * ay - by * ax;


        if uv == 0.0 || vv == 0.0 || wv == 0.0
        {
            let cx_by =  cx as f64 * by as f64;
            let cy_bx =  cy as f64 * bx as f64;
            uv = (cx_by - cy_bx) as f32;
            let ax_cy = ax as f64 * cy as f64;
            let ay_cx = ay as f64 * cx as f64;
            vv = (ax_cy - ay_cx) as f32;
            let bx_ay = bx as f64 * ay as f64;
            let by_ax = by as f64 * ax as f64;
            wv = (bx_ay - by_ax) as f32;
        }

        if (uv < 0.0 || vv < 0.0 || wv < 0.0)
        {
            if oneSided
            {
                return false;
            }
            else if uv > 0.0 || vv > 0.0 || wv > 0.0
            {
                return false;
            }
        }

        let det = uv + vv + wv;

        if (det == 0.0)
        {
            return false;
        }

        let az = self._sz * av.get_element(self._kz);
        let bz = self._sz * bv.get_element(self._kz);
        let cz = self._sz * cv.get_element(self._kz);
        let tv = uv * az + vv * bz + wv * cz;

        if oneSided
        {
            if (tv < 0.0 || tv > det)
            {
                return false;
            }

            normal = (b - a).cross((c - a).borrow()).get_normalized().borrow_mut();
        }
        else
        {
            let mut det_sign =1.0;
            if !det.is_nan() {
                det_sign =  -1.0;
            }
            if ((tv * det_sign) < 0.0 || (tv * det_sign) > (det * det_sign))
            {
                return false;
            }

            normal =   ((b - a).cross((c - a).borrow()).get_normalized() * det_sign).borrow_mut();
        }

        let det_reciprocal = 1.0 / det;
        t = (tv * det_reciprocal).borrow_mut();

        return true;
    }
}