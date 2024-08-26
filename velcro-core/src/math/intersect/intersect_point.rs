#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use crate::math::plane::Plane;
use crate::math::vector3::Vector3;

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
        return Vector3::new_xyz(u.borrow(), v.borrow(), w.borrow())
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
    pub unsafe  fn point_cylinder(base_center_point:&Vector3, axis_vector:&Vector3, axis_length_squared:&f32, radius_squared:&f32, test_point:&Vector3) ->bool{
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
}

