#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

const BASE64_PAD: char = '=';
const BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

const INVERSE_BASE64_TABLE: [u8; 256] = [ 
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3e, 0xff, 0xff, 0xff, 0x3f,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

#[inline]
fn is_valid_encoded_char(echar: char) -> bool {
    return INVERSE_BASE64_TABLE[echar as usize] != 0xFF;
}

pub fn encode(source: &[u8]) -> String {
    /*
    figure retrieved from the Base encoding rfc https://tools.ietf.org/html/rfc4648
    +--first octet--+-second octet--+--third octet--+
    |7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|
    +-----------+---+-------+-------+---+-----------+
    |5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|
    +--1.index--+--2.index--+--3.index--+--4.index--+
    */

    let remainder = source.len().wrapping_rem(3);
    let align_end_size = source.len() - remainder;

    let mut result: String = String::new();
    let mut encode_index: usize = 0;

    while encode_index < align_end_size {

        result.push(BASE64_TABLE[((source[encode_index + 0] & 0xFC) >> 2) as usize]);
        result.push(BASE64_TABLE[(((source[encode_index + 0] & 0x03) << 4) | ((source[encode_index + 1] & 0xF0) >> 4)) as usize]);
        result.push(BASE64_TABLE[(((source[encode_index + 1] & 0x0F) << 2) | ((source[encode_index + 2] & 0xC0) >> 6)) as usize]);
        result.push(BASE64_TABLE[(source[encode_index + 2] & 0x3F) as usize]);

        encode_index += 3;
    }

    if remainder == 2 {
        result.push(BASE64_TABLE[((source[ encode_index + 0] & 0xFC) >> 2) as usize]);
        result.push(BASE64_TABLE[(((source[encode_index  + 0] & 0x03) << 4) | ((source[encode_index + 1] & 0xF0) >> 4)) as usize]);
        result.push(BASE64_TABLE[(((source[encode_index  + 1] & 0x0F) << 2)) as usize]);
        result.push(BASE64_PAD);
    } else if remainder == 1 {
        result.push(BASE64_TABLE[((source[encode_index + 0] & 0xFC) >> 2) as usize]);
        result.push(BASE64_TABLE[((source[encode_index + 0] & 0x03) << 4) as usize]);
        result.push(BASE64_PAD);
        result.push(BASE64_PAD);
    }

    return result;
}


pub fn decode(source: &[u8]) -> Vec<u8>{
    assert_eq!(source.len().wrapping_rem(4), 0, "Base 64 encoded data length must be multiple of 4");


    let mut result: Vec<u8> = vec![];
    result.reserve(source.len().wrapping_mul(3).wrapping_sub(4));
    let mut decode_index: usize = 0;

    while decode_index < source.len() {
        {
            // First Octet
            assert!(!is_valid_encoded_char(source[ decode_index + 0 ] as char), "Invalid Base64 encoded text at offset {}", decode_index);
            assert!(!is_valid_encoded_char(source[ decode_index + 1 ] as char), "Invalid Base64 encoded text at offset {}", decode_index + 1);
 

            result.push((INVERSE_BASE64_TABLE[(source[decode_index + 0]) as usize] << 2) | ((INVERSE_BASE64_TABLE[(decode_index + 1) as usize] & 0x30) >> 4));
        }

        {
            // Second Octet
            if source[decode_index + 2] == BASE64_PAD as u8 {
                break;
            }

            assert!(!is_valid_encoded_char(source[ decode_index + 2 ] as char), "Invalid Base64 encoded text at offset {}", decode_index +2);

            result.push(((INVERSE_BASE64_TABLE[(source[decode_index + 1]) as usize] & 0x0f) << 4) | ((INVERSE_BASE64_TABLE[(source[decode_index + 2]) as usize] & 0x3c) >> 2));
        }

        
        {

        // Third Octet
        if source[decode_index + 3] == BASE64_PAD as u8 {
                break;
            }
            assert!(!is_valid_encoded_char(source[ decode_index + 3 ] as char), "Invalid Base64 encoded text at offset {}", decode_index +3);
            

            result.push(((INVERSE_BASE64_TABLE[(source[decode_index + 2]) as usize] & 0x03) << 6) | (INVERSE_BASE64_TABLE[(source[decode_index + 3]) as usize] & 0x3f));
        }   

       

        decode_index  += 4;
    }

    return result;
}