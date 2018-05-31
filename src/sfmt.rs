//! Rust re-implementation of SFMT

use std::simd::{i32x4, u32x4};

use super::SFMT;

const SFMT_MEXP: usize = 19937;
pub const SFMT_N: usize = SFMT_MEXP / 128 + 1; // = 156
pub const SFMT_N32: usize = SFMT_N * 4;

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
const SFMT_PARITY1: u32 = 0x00000001;
const SFMT_PARITY2: u32 = 0x00000000;
const SFMT_PARITY3: u32 = 0x00000000;
const SFMT_PARITY4: u32 = 0x13c9e684;

fn mm_recursion(a: i32x4, b: i32x4, c: i32x4, d: i32x4) -> i32x4 {
    unsafe {
        use std::arch::x86_64::*;
        use std::mem::transmute;
        let a = transmute::<i32x4, __m128i>(a);
        let b = transmute::<i32x4, __m128i>(b);
        let c = transmute::<i32x4, __m128i>(c);
        let d = transmute::<i32x4, __m128i>(d);
        let mask = transmute::<u32x4, __m128i>(SFMT_MASK);
        let y = _mm_srli_epi32(b, SFMT_SR1);
        let z = _mm_srli_si128(c, SFMT_SR2);
        let v = _mm_slli_epi32(d, SFMT_SL1);
        let z = _mm_xor_si128(z, a);
        let z = _mm_xor_si128(z, v);
        let x = _mm_slli_si128(a, SFMT_SL2);
        let y = _mm_and_si128(y, mask);
        let z = _mm_xor_si128(z, x);
        transmute(_mm_xor_si128(z, y))
    }
}

pub fn sfmt_gen_rand_all(sfmt: &mut SFMT) {
    let st = &mut sfmt.state;
    let mut r1 = st[SFMT_N - 2];
    let mut r2 = st[SFMT_N - 1];
    for i in 0..(SFMT_N - SFMT_POS1) {
        st[i] = mm_recursion(st[i], st[i + SFMT_POS1], r1, r2);
        r1 = r2;
        r2 = st[i];
    }
    for i in (SFMT_N - SFMT_POS1)..SFMT_N {
        st[i] = mm_recursion(st[i], st[i + SFMT_POS1 - SFMT_N], r1, r2);
        r1 = r2;
        r2 = st[i];
    }
}

pub fn period_certification(sfmt: &mut SFMT) {
    let mut inner = 0_u32;
    let st = &mut sfmt.state[0];
    let parity = [SFMT_PARITY1, SFMT_PARITY2, SFMT_PARITY3, SFMT_PARITY4];
    for i in 0..4 {
        inner ^= st.extract(i) as u32 & parity[i];
    }
    for i in [16, 8, 4, 2, 1].iter() {
        inner ^= inner >> i;
    }
    inner &= 1;
    if inner == 1 {
        return;
    }
    for i in 0..4 {
        let mut work = 1_u32;
        for _ in 0..32 {
            if (work & parity[i]) != 0 {
                let val = st.extract(i) as u32 ^ work;
                let val = st.replace(i, val as i32);
                ::std::mem::replace(st, val);
                return;
            }
            work = work << 1;
        }
    }
}

fn iterate(pre: i32, i: i32) -> i32 {
    use std::num::Wrapping;
    let pre = Wrapping(pre as u32);
    let i = Wrapping(i as u32);
    (Wrapping(1812433253) * (pre ^ (pre >> 30)) + i).0 as i32
}

fn map(a: i32, idx: i32) -> (i32x4, i32) {
    let b = iterate(a, 4 * idx + 1);
    let c = iterate(b, 4 * idx + 2);
    let d = iterate(c, 4 * idx + 3);
    let a2 = iterate(d, 4 * idx + 4);
    (i32x4::new(a, b, c, d), a2)
}

pub fn sfmt_init_gen_rand(sfmt: &mut SFMT, seed: u32) {
    let mut pre = seed as i32;
    for (idx, v) in sfmt.state.iter_mut().enumerate() {
        let (v_, pre_) = map(pre, idx as i32);
        *v = v_;
        pre = pre_;
    }
    sfmt.idx = SFMT_N32;
    period_certification(sfmt);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::{fs, io, io::BufRead};

    fn read_answer(filename: &str) -> Result<Vec<i32x4>, io::Error> {
        let f = io::BufReader::new(fs::File::open(filename)?);
        Ok(f.lines()
            .map(|line| {
                let vals: Vec<_> = line.unwrap()
                    .split(" ")
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                i32x4::new(vals[0], vals[1], vals[2], vals[3])
            })
            .collect())
    }

    #[test]
    fn test_init() {
        let seed: u32 = 1234;
        let seed = unsafe { *(&seed as *const u32 as *const [u8; 4]) };
        let sfmt = SFMT::from_seed(seed);
        let ans = read_answer("check/init1234.txt").unwrap();
        for (v, a) in sfmt.state.iter().zip(ans.iter()) {
            println!("v = {:?}", v);
            println!("a = {:?}", a);
            assert_eq!(v, a);
        }
    }

    #[test]
    fn test_mm_recursion() {
        let a = i32x4::new(1, 2, 3, 4);
        let b = i32x4::new(431, 232, 83, 14);
        let c = i32x4::new(213, 22, 93, 234);
        let d = i32x4::new(112, 882, 23, 124);
        let z = mm_recursion(a.into(), a, a.into(), a);
        let zc = i32x4::new(33816833, 50856450, 67896067, 1049604); // calculated by C code
        assert_eq!(z, zc);
        let z = mm_recursion(a.into(), b, c.into(), d);
        let zc = i32x4::new(398459137, 1355284994, -363068669, 32506884); // calculated by C code
        assert_eq!(z, zc);
    }
}
