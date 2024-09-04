pub mod vsimd;
pub mod intersect;
pub mod sfmt;
pub mod random;
pub mod vector3;
pub mod aabb;
pub mod plane;
mod simd_math;
mod math_utils;
mod vector2;
mod vector4;
mod simd_math_vec1_sse;
mod simd_math_vec1_neon;
mod simd_math_vec2_sse;
mod simd_math_vec2_neon;
mod simd_math_vec3_sse;
mod simd_math_vec3_neon;
mod simd_math_vec4_neon;
mod simd_math_vec4_sse;
mod common_sse;
mod vectorn;
mod matrix3x3;
mod transform;
mod quaternion;
mod capsule;
mod line_segment;
mod ray;
mod color;
mod obb;
mod shape_intersection;
mod sphere;
mod frustum;
mod matrix3x4;
mod matrix4x4;

#[cfg(test)]
mod tests {
    use crate::math::sfmt::Sfmt;

    #[test]
    fn it_work_sfmt_random() {
        let mut srandom =  Sfmt::new();

        let v = srandom.rand32();
        println!("sfmt random u32:{}", v);

        let vf = srandom.rand_r32();
        println!("sfmt random f64:{}", vf);

        let vf1 = srandom.rand_r32_1();
        println!("sfmt random f64_1:{}", vf1);

        let vf2 = srandom.rand_r32_2();
        println!("sfmt random f64_2:{}", vf2);
    }

    /* 

    */
}

