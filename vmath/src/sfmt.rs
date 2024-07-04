#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

const MEXP: i32 = 19937;
const N: i32    = MEXP / 128 + 1;

//union W128_T {
//    si: i128
//}