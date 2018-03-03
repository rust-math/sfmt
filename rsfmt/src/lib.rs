extern crate stdsimd;

use stdsimd::simd::*;
use stdsimd::vendor::*;

const SFMT_MEXP: usize = 19937;
const SFMT_N: usize = SFMT_MEXP / 128 + 1; // = 156
const SFMT_POS1: usize = 122;
const SFMT_SL1: i32 = 18;
const SFMT_SL2: i32 = 1;
const SFMT_SR1: i32 = 11;
const SFMT_SR2: i32 = 1;
const SFMT_MSK1: u32 = 0xdfffffef;
const SFMT_MSK2: u32 = 0xddfecb7f;
const SFMT_MSK3: u32 = 0xbffaffff;
const SFMT_MSK4: u32 = 0xbffffff6;
const SFMT_MASK: u32x4 = u32x4::new(SFMT_MSK1, SFMT_MSK2, SFMT_MSK3, SFMT_MSK4);

#[derive(Clone)]
pub struct SFMT {
    /// the 128-bit internal state array
    pub state: [i32x4; SFMT_N],
    /// index counter to the 32-bit internal state array
    pub idx: i32,
}

unsafe fn mm_recursion(a: i8x16, b: i32x4, c: i8x16, d: i32x4) -> i32x4 {
    let y = _mm_srli_epi32(b, SFMT_SR1);
    let z = _mm_srli_si128(c, SFMT_SR2);
    let v = _mm_slli_epi32(d, SFMT_SL1);
    let z = _mm_xor_si128(z, a);
    let z = _mm_xor_si128(z, v.into());
    let x = _mm_slli_si128(a, SFMT_SL2);
    let y = _mm_and_si128(y.into(), SFMT_MASK.into());
    let z = _mm_xor_si128(z, x);
    _mm_xor_si128(z, y).into()
}

pub unsafe fn sfmt_gen_rand_all(sfmt: &mut SFMT) {
    let st = &mut sfmt.state;
    let mut r1 = st[SFMT_N - 2];
    let mut r2 = st[SFMT_N - 1];
    for i in 0..(SFMT_N - SFMT_POS1) {
        st[i] = mm_recursion(st[i].into(), st[i + SFMT_POS1], r1.into(), r2);
        r1 = r2;
        r2 = st[i];
    }
    for i in (SFMT_N - SFMT_POS1)..SFMT_N {
        st[i] = mm_recursion(st[i].into(), st[i + SFMT_POS1 - SFMT_N], r1.into(), r2);
        r1 = r2;
        r2 = st[i];
    }
}
