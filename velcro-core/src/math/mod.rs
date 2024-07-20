

mod vsimd;
mod constants;
pub mod sfmt;
pub mod random;
pub mod vector3;
pub mod aabb;


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

