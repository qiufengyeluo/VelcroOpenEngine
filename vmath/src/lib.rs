
pub mod bits;
pub mod cityhash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bytes = "bors".as_bytes();
        let hash32v: u32 = cityhash::city_hash32(bytes, bytes.len());
        println!("city hash 32 test: {}", hash32v);
    }
}
