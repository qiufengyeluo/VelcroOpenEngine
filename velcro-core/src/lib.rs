#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]

pub mod parallel;
pub mod math;
pub mod interface;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work_city_hash32() {
        let bytes = "bors".as_bytes();
        let hash32v: u32 = math::cityhash::city_hash32(bytes, bytes.len());
        println!("city hash 32 test: {}", hash32v);
    }

    #[test]
    fn it_work_city_hash64() {
        let bytes = "bors12233fdfdfdfd".as_bytes();
        let hash64v: u64 = math::cityhash::city_hash64(bytes, bytes.len());
        println!("city hash 64 test: {}", hash64v);
    }

    #[test]
    fn it_work_random() {
        println!("random: {}", math::random::get_random::<i32>().unwrap());
        let mut sft = math::sfmt::Sfmt::new();
        println!("sfmt: {}", sft.rand_r32());
    }
}