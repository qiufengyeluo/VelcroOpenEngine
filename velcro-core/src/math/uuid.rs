#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

//use crate::math::sfmt::*;
use crate::math::sha1::*;
use std::ops;
use std::cmp::Ordering;
use std::ptr;
use std::ptr::read_unaligned;

const UUID_DIGITS: [char; 22] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
    'A', 'B', 'C', 'D', 'E', 'F', 'a', 'b', 'c', 'd', 'e', 'f'];

const UUID_VALUES: [u8; 23] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 
    11, 12, 13, 14, 15, 10, 11, 12, 13, 14, 15, u8::MAX];

pub enum Variant {
    VarUnknown         = -1,
    VarNcs             = 0, // 0 - -
    VarRfc4122         = 2, // 1 0 -
    VarMicrosoft       = 6, // 1 1 0
    VarReserved        = 7  // 1 1 1
}

pub enum Version
{
    VerUnknown         = -1,
    VerTime            = 1, // 0 0 0 1
    VerDce             = 2, // 0 0 1 0
    VerNameMd5         = 3, // 0 0 1 1
    VerRandom          = 4, // 0 1 0 0
    VerNameSha1        = 5, // 0 1 0 1
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

// PartialEq 是否相等
#[derive(Eq, Copy, Clone)]
pub struct UUID {
    _data: [u8; 16]
}


impl UUID {
    pub fn new() -> Self {
        UUID {
            _data: [0; 16],
        }
    }

    pub fn new_null() -> Self {
        UUID {
            _data: [0; 16],
        }
    }

    pub fn new_string(s :&str) -> UUID {
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

    //=========================================================================
    // new_name 通过一个名字字符串创建 UUID
    //=========================================================================
    pub fn new_name(name: &str) -> UUID {
        return Self::new_data(name.as_bytes(), name.as_bytes().len());
    }

    //=========================================================================
    // new_data 通过一个二进制数据创建 UUID
    //=========================================================================
    pub fn new_data(data: &[u8], size: usize) -> UUID {
        if size > 0 {
            let mut sa: Sha1 = Sha1::new();
            sa.process_bytes(data, size);
            
            let digest = sa.get_digest();
            
            let mut id: UUID = UUID::new_null();
            for i in 0..4 {
                id._data[i * 4]     = (digest.get(i) >> 24 & 0xFF) as u8;
                id._data[i * 4 + 1] = (digest.get(i) >> 16 & 0xFF) as u8;
                id._data[i * 4 + 2] = (digest.get(i) >> 8 & 0xFF) as u8;
                id._data[i * 4 + 3] = (digest.get(i) >> 0 & 0xFF) as u8;
            }

             // variant VAR_RFC_4122
             id._data[8] &= 0xBF;
             id._data[8] |= 0x80;

             // version VER_NAME_SHA1
             id._data[6] &= 0x5F;
             id._data[6] |= 0x50;

             return id;
        }

        return Self::new_null();
    }

    /*pub fn new_random() -> Self {

    }*/

    /// is_null UUID 是否是个为创建的对象
    /// @return true.空对象 false.不是空对象
    pub fn is_null(&self) -> bool {
        let v64h: u64 = unsafe { read_unaligned(self._data.as_ptr().cast::<u64>())};
        let v64l: u64 = unsafe { read_unaligned(self._data[8..].as_ptr().cast::<u64>())};
        if v64h != 0 || v64l != 0 {
            return false;
        }
        return true;
    }

    /// get_variant 获取 UUID 对象格式标准
    pub fn get_variant(&self) -> Variant {
        let val = self._data[8];
        if (val & 0x80) == 0x00
        {
            return Variant::VarNcs;
        }
        else if (val & 0xC0) == 0x80
        {
            return Variant::VarRfc4122;
        }
        else if (val & 0xE0) == 0xC0
        {
            return Variant::VarMicrosoft;
        }
        else if (val & 0xE0) == 0xE0
        {
            return Variant::VarReserved;
        }
     
        return Variant::VarUnknown;
    }

    /// get_version 返回 UUID 版本格式信息
    pub fn get_version(&self) -> Version {
        let val = self._data[6];
        if (val & 0xF0) == 0x10
        {
            return Version::VerTime;
        }
        else if (val & 0xF0) == 0x20
        {
            return Version::VerDce;
        }
        else if (val & 0xF0) == 0x30
        {
            return Version::VerNameMd5;
        }
        else if (val & 0xF0) == 0x40
        {
            return Version::VerRandom;
        }
        else if (val & 0xF0) == 0x50
        {
            return Version::VerNameSha1;
        }
   
        return Version::VerUnknown;
    }

    /// to_string 将 UUID 输出为一个字符串
    /// @param bool is_brackets 是否有大括号
    /// @param bool is_deshes   是否有分割符
    /// @return string 转换后的字符串
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

impl ops::Add for UUID {
    type Output = UUID;

    fn add(self, other: UUID) -> UUID {
        let merged_data_len = self._data.len().wrapping_mul(2);
        let mut merged_data: Vec<u8> = Vec::<u8>::with_capacity(merged_data_len);

        unsafe { ptr::copy(self._data.as_ptr(), merged_data.as_mut_ptr(), self._data.len()) };
        unsafe { ptr::copy(other._data.as_ptr(), merged_data.as_mut_ptr().wrapping_add(4), other._data.len()) };

        return Self::new_data(merged_data.as_mut(), merged_data_len);
    }
}


impl PartialEq<Self> for UUID {
    fn eq(&self, rhs: &Self) -> bool { 
        let lv64h: u64 = unsafe { read_unaligned(self._data.as_ptr().cast::<u64>())};
        let lv64l: u64 = unsafe { read_unaligned(self._data[8..].as_ptr().cast::<u64>())};

        let rv64h: u64 = unsafe { read_unaligned(rhs._data.as_ptr().cast::<u64>())};
        let rv64l: u64 = unsafe { read_unaligned(rhs._data[8..].as_ptr().cast::<u64>())};

         if lv64h != rv64h || lv64l != rv64l {
            return false;
         }
         return true;
    }
}

impl PartialOrd for UUID {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        return Some(self._data.cmp(&rhs._data));
    }
}

impl Ord for UUID {
    fn cmp(&self, rhs: &Self) -> Ordering {
        return self._data.cmp(&rhs._data);
    }
}