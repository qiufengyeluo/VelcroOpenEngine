#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

//use std::mem;
use std::ptr::read_unaligned;

const UUID_DIGITS: [char; 22] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
    'A', 'B', 'C', 'D', 'E', 'F', 'a', 'b', 'c', 'd', 'e', 'f'];

const UUID_VALUES: [u8; 23] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 
    11, 12, 13, 14, 15, 10, 11, 12, 13, 14, 15, u8::MAX];

pub enum Variant {
    VAR_UNKNOWN         = -1,
    VAR_NCS             = 0, // 0 - -
    VAR_RFC_4122        = 2, // 1 0 -
    VAR_MICROSOFT       = 6, // 1 1 0
    VAR_RESERVED        = 7  // 1 1 1
}

pub enum Version
{
    VER_UNKNOWN         = -1,
    VER_TIME            = 1, // 0 0 0 1
    VER_DCE             = 2, // 0 0 1 0
    VER_NAME_MD5        = 3, // 0 0 1 1
    VER_RANDOM          = 4, // 0 1 0 0
    VER_NAME_SHA1       = 5, // 0 1 0 1
}

#[inline]
#[must_use]
fn get_value(c: char) -> u8 {
    let mut i = 0;
    for d in UUID_DIGITS.iter() {
        if *d == c {
            return UUID_VALUES[i];
        }
        i += 1;
    }
    return UUID_VALUES[22];
}

pub struct UUID {
    _data: [u8; 16]
}


impl UUID {
    pub fn new_null() -> Self {
        UUID {
            _data: [0; 16],
        }
    }

    pub fn new_string_kip_warnings(s :&str) -> Self {
        if s.len() == 0 {
            return UUID::new_null();
        }

        if s.len() < 32 || s.len() > 38 {
            // TODO: 打印调试警告
            return UUID::new_null();
        }

        let sary =  s.as_bytes();
       
        let mut sidx = 0;
        let mut c: char =  sary[sidx] as char;
        sidx += 1;
        //let mut has_open_brace = false;
        if c == '{' {
            c = sary[sidx] as char;
            sidx += 1;
            //has_open_brace = true;
        }
        
        let mut has_dashes = false;
        let mut id: UUID = UUID::new_null();
        let mut tidx = 0;
        while tidx < 16 {
            if tidx == 4 {
                has_dashes = c == '-';
                if has_dashes {
                    c = sary[sidx] as char;
                    sidx += 1;
                }
            }

            if has_dashes {
                if tidx == 6 || tidx == 8 || tidx == 10 {
                    if c == '-' {
                        c = sary[sidx] as char;
                        sidx += 1;
                    } else {
                        // TODO: 打印调试警告
                        return UUID::new_null();
                    }
                  
                }
            }

            id._data[tidx] = get_value(c);
            c = sary[sidx] as char;
            sidx += 1;

            id._data[tidx] <<= 4;
            id._data[tidx] |= get_value(c);

            c = sary[sidx] as char;
            sidx += 1;

            tidx += 1;
        }

        // TODO: V_Warning("Math", !has_open_brace || skipWarnings || c == '}', "Invalid UUID format %s (must be) {xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx} (or without dashes and braces)", string);
        return id;
    }


    pub fn is_null(&self) -> bool {
        let v64h: u64 = unsafe { read_unaligned(self._data.as_ptr().cast::<u64>())};
        let v64l: u64 = unsafe { read_unaligned(self._data[8..].as_ptr().cast::<u64>())};
        if v64h != 0 || v64l != 0 {
            return false;
        }
        return true;
    }

    pub fn to_string(&self, is_brackets: bool, is_dashes: bool) -> String {
        let mut result:String = String::new();
        let mut tidx = 0;
        while tidx < 16 {
            if is_dashes && (tidx == 4 || tidx == 6 || tidx == 8 || tidx == 10) {
                result.push('-');
            }

            let val: u8 = self._data[tidx];
            result.push(UUID_DIGITS[(val >> 4) as usize]);
            result.push(UUID_DIGITS[(val & 15) as usize]);

            tidx += 1;
        }

        if is_brackets {
            result.push('}');
        }

        return result;
    }
}