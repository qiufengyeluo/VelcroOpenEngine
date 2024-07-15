

mod vsimd;
pub mod bits;
pub mod cityhash;
pub mod sfmt;
pub mod random;
pub mod crc;


#[cfg(test)]
mod tests {
    use crate::math::crc;
    use crate::math::sfmt::Sfmt;


    #[test]
    fn it_work_crc32() {
        let crcv = crc::from_string("1121223");
        println!("crc32 from string:{}", crcv.average());
    }

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
    #[test]
    fn it_work_uuid() {
        let uid1 = UUID::create_random();
        let uidstr1 = uid1.to_string(true, true);
        println!("1 uuid random:{0}", uidstr1);
        let uid2 = UUID::create_random();
        let uidstr2 = uid2.to_string(true, true);
        println!("2 uuid random:{0}", uidstr2);
        let uid3 = uid1 + uid2;
        let uidstr3 = uid3.to_string(true, true);
        println!("3 add uuid random:{0}", uidstr3);
        let uid4 = UUID::create_string("{67452301-EFCD-5B89-98BA-DCFE10325471}");
        let uidstr4 = uid4.to_string(true, true);
        println!("4 string to uuid random:{0}", uidstr4);
    }
    */
}

