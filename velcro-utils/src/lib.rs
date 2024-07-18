#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]

mod bits;
pub mod sha1;
pub mod uuid;
pub mod cityhash;
pub mod crc;
pub mod base64;
pub mod hasder;

pub use uuid::UUID;
pub use cityhash::{city_hash32, city_hash64, city_hash128};
pub use hasder::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work_crc32() {
        let crcv = crc::from_string("1121223");
        println!("crc32 from string:{}", crcv.average());
    }

    #[test]
    fn it_work_city_hash32() {
        let bytes = "bors".as_bytes();
        let hash32v: u32 = cityhash::city_hash32(bytes);
        println!("city hash 32 test: {}", hash32v);
    }

    #[test]
    fn it_work_city_hash64() {
        let bytes = "bors12233fdfdfdfd".as_bytes();
        let hash64v: u64 = cityhash::city_hash64(bytes);
        println!("city hash 64 test: {}", hash64v);
    }

    #[test]
    fn it_work_uuid() {
        let uid1 = UUID::create_string("{f094d7d4-e168-4a3d-89f3-ae3b83b1f5db}");
        let uidstr1 = uid1.to_string(true, true);
        println!("1 uuid random:{0}", uidstr1);
        let uid2 = UUID::create_string("{eb582bb9-b781-4573-ae47-75ca837b8438}");
        let uidstr2 = uid2.to_string(true, true);
        println!("2 uuid random:{0}", uidstr2);
        let uid3 = uid1 + uid2;
        let uidstr3 = uid3.to_string(true, true);
        println!("3 add uuid random:{0}", uidstr3);
        let uid4 = UUID::create_string("{67452301-EFCD-5B89-98BA-DCFE10325471}");
        let uidstr4 = uid4.to_string(true, true);
        println!("4 string to uuid random:{0}", uidstr4);
    }
}
