#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

/// `bswap32` 32位高低位交换
pub fn bswap32(x: u32) -> u32 {
    return ((x >> 24) & 0xFF) | ((x >> 8) & 0xFF00) |
        ((x << 8) & 0xFF0000) | ((x << 24) & 0xFF000000)
}

/// `bswap64` 64位高低位交换
pub fn bswap64(x: u64) -> u64 {
    return ((x >> 56) & 0xFF) | ((x >> 40) & 0xFF00) | ((x >> 24) & 0xFF0000) | ((x >> 8) & 0xFF000000) |
    ((x << 8) & 0xFF00000000) | ((x << 24) & 0xFF0000000000) | ((x << 40) & 0xFF000000000000) | ((x << 56) & 0xFF00000000000000)
}