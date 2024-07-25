#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

pub fn get_clamp<T>(value :&[T] ,min:&[T] ,max:& [T] )->T
{
    return if value < min
    {
        min
    } else if (value > max)
    {
        max
    } else {
        value
    }
}