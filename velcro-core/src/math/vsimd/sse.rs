


#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))] 
struct __m128 (f32, f32, f32, f32);

#[cfg(not(any(target_arch = "x86_64", target_arch="x86", target_arch = "arm")))]
struct __m128i (i64, i64);

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
pub type FloatType = __m128;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
pub type Int32Type = __m128i;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
pub type FloatArgType = FloatType;
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[allow(dead_code)]
pub type Int32ArgType = Int32Type;

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
const fn _mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe  fn load_aligned(ptr: *const f32) -> FloatType {
    return _mm_load_ps(ptr);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn load_aligned_i128(ptr: *const Int32Type) -> Int32Type {
    return _mm_load_si128(ptr);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn load_unaligned(ptr: *const f32) -> FloatType {
    return _mm_loadu_ps(ptr); 
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn load_unaligned_i128(ptr: *const Int32Type) -> Int32Type {
    return _mm_loadu_si128(ptr);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn store_aligned(addr: *mut f32, value: FloatType) {
    _mm_store_ps(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn store_aligned_i128(addr: *mut Int32Type, value: Int32Type) {
    _mm_store_si128(addr, value);
}


#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn store_unaligned(addr: *mut f32, value: FloatType) {
    _mm_storeu_ps(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn store_unaligned_i128(addr: *mut Int32Type, value: Int32Type) {
    _mm_storeu_si128(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn stream_aligned(addr: *mut f32, value: FloatType) {
    _mm_stream_ps(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn stream_aligned_i128(addr: *mut Int32Type, value: Int32Type) {
    _mm_stream_si128(addr, value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn convert_to_float(value: Int32Type) -> FloatType {
    return _mm_cvtepi32_ps(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn convert_to_int(value: FloatType) -> Int32Type {
    return _mm_cvttps_epi32(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn convert_to_int_nearest(value: FloatType) -> Int32Type {
    return _mm_cvtps_epi32(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn cast_to_float(value: Int32Type) -> FloatType {
    return _mm_castsi128_ps(value);
}


#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn cast_to_int(value: FloatType) -> Int32Type {
    return _mm_castps_si128(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn zero_int() -> Int32Type {
    return _mm_setzero_si128();
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn zero_float() -> FloatType {
    return cast_to_float(zero_int());
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn select_first(value: FloatType) -> f32 {
    return _mm_cvtss_f32(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn splat(value: f32) ->FloatType {
    return _mm_set1_ps(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn splat_i32(value: i32 ) -> Int32Type {
    return _mm_set1_epi32(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn splat_first(value: FloatType) -> FloatType {
    return _mm_shuffle_ps(value, value, _mm_shuffle(0, 0, 0, 0));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn splat_second(value: FloatType) -> FloatType {
    return _mm_shuffle_ps(value, value, _mm_shuffle(1, 1, 1, 1));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn splat_third(value: FloatType) -> FloatType {
    return _mm_shuffle_ps(value, value, _mm_shuffle(2, 2, 2, 2));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn splat_fourth(value: FloatType) -> FloatType {
    return _mm_shuffle_ps(value, value, _mm_shuffle(3, 3, 3, 3));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_first(a: FloatType, b: FloatType) -> FloatType {
    return _mm_blend_ps(a, b, 0b0001);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_first_f32(a: FloatType, b: f32) -> FloatType {
    return replace_first(a, splat(b));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_second(a: FloatType, b: FloatType) -> FloatType {
    return _mm_blend_ps(a, b, 0b0010);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_second_f32(a: FloatType, b: f32) -> FloatType {
    return replace_second(a, splat(b));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_third(a: FloatType, b: FloatType) -> FloatType {
    return _mm_blend_ps(a, b, 0b0100);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_third_f32(a: FloatType, b: f32) -> FloatType {
    return replace_third(a, splat(b));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_fourth(a: FloatType, b: FloatType) -> FloatType {
    return _mm_blend_ps(a, b, 0b1000);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn replace_fourth_f32(a: FloatType, b: f32) -> FloatType {
    return replace_fourth(a, splat(b));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn load_immediate(x: f32, y: f32, z: f32, w: f32) -> FloatType {
    return _mm_set_ps(w, z, y, x);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn load_immediate_i32(x: i32, y: i32, z: i32, w: i32) -> Int32Type {
    return _mm_set_epi32(w, z, y, x);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn not(value: FloatType) -> FloatType{
    let invert: Int32Type = splat_i32(u32::MAX as i32);
    return _mm_andnot_ps(value, cast_to_float(invert));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn and(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_and_ps(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn and_not(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_andnot_ps(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn or(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_or_ps(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn xor(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_xor_ps(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn not_i32(value: Int32Type) -> Int32Type {
    return cast_to_int(not(cast_to_float(value)));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn and_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_and_si128(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn and_not_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type  {
    return _mm_andnot_si128(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn or_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_or_si128(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn xor_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_xor_si128(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn floor(value: FloatType) -> FloatType {
    return _mm_floor_ps(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn ceil(value: FloatType) -> FloatType {
    return _mm_ceil_ps(value);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn round(value: FloatType) -> FloatType {
    return _mm_round_ps(value, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn truncate(value: FloatType) -> FloatType {
    return _mm_round_ps(value, _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn min(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_min_ps(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn max(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_max_ps(arg1, arg2);
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn clamp(value: FloatType, minv: FloatType, maxv: FloatType) -> FloatType {
    return max(minv, min(value, maxv));
}

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
#[inline]
#[allow(dead_code)]
pub unsafe fn min_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_min_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn max_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_max_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn clamp_i32(value: Int32Type, min: Int32Type, max: Int32Type) -> Int32Type {
    return max_i32(min, min_i32(value, max));
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_eq(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_cmpeq_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_neq(arg1: FloatType, arg2: FloatType) -> FloatType  {
    return _mm_cmpneq_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_gt(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_cmpgt_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_gt_eq(arg1: FloatType, arg2: FloatType) -> FloatType  {
    return _mm_cmpge_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_lt(arg1: FloatType, arg2: FloatType)  -> FloatType {
    return _mm_cmplt_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_lt_eq(arg1: FloatType, arg2: FloatType) -> FloatType  {
    return _mm_cmple_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_eq(arg1: FloatType, arg2: FloatType, mask: i32) -> bool
{
    let compare: Int32Type = cast_to_int(cmp_neq(arg1, arg2));
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_lt(arg1: FloatType, arg2: FloatType, mask: i32) -> bool {
    let compare: Int32Type = cast_to_int(cmp_gt_eq(arg1, arg2));
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_lt_eq(arg1: FloatType, arg2: FloatType, mask: i32) -> bool  {
    let compare: Int32Type = cast_to_int(cmp_gt(arg1, arg2));
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_gt(arg1: FloatType, arg2: FloatType, mask: i32) -> bool {
    let compare: Int32Type = cast_to_int(cmp_lt_eq(arg1, arg2));
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_gt_eq(arg1: FloatType, arg2: FloatType, mask: i32) -> bool {
    let compare: Int32Type = cast_to_int(cmp_lt(arg1, arg2));
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_eq_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_cmpeq_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_neq_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    let equal: Int32Type  = cmp_eq_i32(arg1, arg2);
    return not_i32(equal);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_gt_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_cmpgt_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_gt_eq_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    let less_than: Int32Type  = _mm_cmplt_epi32(arg1, arg2);
    return not_i32(less_than);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_lt_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_cmplt_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_lt_eq_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    let greater_than: Int32Type = cmp_gt_i32(arg1, arg2);
    return not_i32(greater_than);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_eq_i32(arg1: Int32Type, arg2: Int32Type, mask: i32) -> bool  {
    let compare: Int32Type = cmp_neq_i32(arg1, arg2);
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn select(arg1: FloatType, arg2: FloatType, mask: FloatType) -> FloatType {
    return _mm_blendv_ps(arg2, arg1, mask);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn select_i32(arg1: Int32Type, arg2: Int32Type, mask: Int32Type) -> Int32Type {
    return _mm_blendv_epi8(arg2, arg1, mask);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_lt_i32(arg1: Int32Type, arg2: Int32Type, mask: i32) -> bool  {
    let compare: Int32Type = cmp_gt_eq_i32(arg1, arg2);
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_lt_eq_i32(arg1: Int32Type, arg2: Int32Type, mask: i32) -> bool  {
    let compare:Int32Type = cmp_gt_i32(arg1, arg2);
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_gt_i32(arg1: Int32Type, arg2: Int32Type, mask: i32) -> bool {
    let compare:Int32Type = cmp_lt_eq_i32(arg1, arg2);
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn cmp_all_gt_eq_i32(arg1: Int32Type, arg2: Int32Type, mask: i32) -> bool  {
    let compare:Int32Type = cmp_lt_i32(arg1, arg2);
    return (_mm_movemask_epi8(compare) & mask) == 0;
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn add(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_add_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn sub(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_sub_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn mul(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_mul_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn madd(mul1: FloatType, mul2: FloatType, add: FloatType) -> FloatType {
    return _mm_fmadd_ps(mul1, mul2, add);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn div(arg1: FloatType, arg2: FloatType) -> FloatType {
    return _mm_div_ps(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn abs(value: FloatType) -> FloatType {
    let sign_mask: FloatType = cast_to_float(splat_i32(0x7FFFFFFF));
    return and(value, sign_mask);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn add_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_add_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn sub_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_sub_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn mul_i32(arg1: Int32Type, arg2: Int32Type) -> Int32Type {
    return _mm_mullo_epi32(arg1, arg2);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn madd_i32(mul1: Int32Type, mul2: Int32Type, add: Int32Type) -> Int32Type {
    return add_i32(mul_i32(mul1, mul2), add);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn abs_i32(value: Int32Type) -> Int32Type {
    return _mm_abs_epi32(value);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn reciprocal_estimate(value: FloatType) -> FloatType {
    return _mm_rcp_ps(value);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn reciprocal(value: FloatType) -> FloatType {
    let estimate: FloatType = reciprocal_estimate(value);
    let estimate_square: FloatType = mul(estimate, estimate);
    let estimate_double: FloatType = add(estimate, estimate);
    return sub(estimate_double, mul(value, estimate_square));
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn sqrt(value: FloatType) -> FloatType {
    return _mm_sqrt_ps(value);
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn sqrt_inv_estimate(value: FloatType) -> FloatType {
    return _mm_rsqrt_ps(value); // Faster, but roughly half the precision (12ish bits rather than 23ish)
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn sqrt_inv(value: FloatType) -> FloatType {
    let one: FloatType  = splat(1.0);
    return div(one, sqrt(value));
}

#[inline]
#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub unsafe fn mod_calculate(value: FloatType, divisor: FloatType) -> FloatType {
    return sub(value, mul(_mm_round_ps(div(value, divisor), _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC), divisor));
}
