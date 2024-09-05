#![warn(clip::pedantic)]
#![allow(clip::many_single_char_names)]

use std::ffi::CString;

use crate::math::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Crc32{
    _value:u32,
}

impl PartialEq<Self> for Crc32 {
    fn eq(&self, other: &Self) -> bool {
        return self._value == other._value;
    }
    fn ne(&self, other: &Self) -> bool {
        return self._value != other._value;
    }
}

impl Crc32 {

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new()->Crc32{
        Crc32{
            _value:0,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_u32(value:u32)->Crc32{
        Crc32{
            _value:value,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_slice(view:String)->Crc32{
        let mut result = Crc32::new();
        if (!view.is_empty())
        {
            result.set(view.data(), view.size(), true);
        }
        result
    }

    #[inline]
    #[allow(dead_code)]
    pub unsafe fn new_ptr(data:*u8,size:u32,forceLowerCase:bool)->Crc32{

    }
    Crc32(const void* data, size_t size, bool forceLowerCase = false);
template<class ByteType, class = AZStd::enable_if_t<sizeof(ByteType) == 1>>
constexpr Crc32(const ByteType* data, size_t size, bool forceLowerCase = false);
constexpr Crc32(AZStd::span<const AZStd::byte> inputSpan);

constexpr void Add(AZStd::string_view view);
void Add(const void* data, size_t size, bool forceLowerCase = false);
template<class ByteType>
constexpr auto Add(const ByteType* data, size_t size, bool forceLowerCase = false)
-> AZStd::enable_if_t<sizeof(ByteType) == 1>;
constexpr void Add(AZStd::span<const AZStd::byte> inputSpan);

constexpr operator u32() const               { return m_value; }



constexpr bool operator!() const             { return (m_value == 0); }

static void Reflect(AZ::SerializeContext& context);

protected:
// A constant expression cannot contain a conversion from const-volatile void to any pointer to object type
// nor can it contain a reinterpret_cast, therefore overloads for const char* and uint8_t are added
void Set(const void* data, size_t size, bool forceLowerCase = false);
template<class ByteType>
constexpr auto Set(const ByteType* data, size_t size, bool forceLowerCase = false)
-> AZStd::enable_if_t<sizeof(ByteType) == 1>;
constexpr void Combine(u32 crc, size_t len);
}