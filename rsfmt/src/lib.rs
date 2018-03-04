extern crate rand;
extern crate stdsimd;

use rand::Rng;
use stdsimd::simd::*;
use stdsimd::vendor::*;

const SFMT_MEXP: usize = 19937;
const SFMT_N: usize = SFMT_MEXP / 128 + 1; // = 156
const SFMT_N32: usize = SFMT_N * 4;
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

#[derive(Clone)]
pub struct SFMT {
    /// the 128-bit internal state array
    pub state: [i32x4; SFMT_N],
    /// index counter to the 32-bit internal state array
    pub idx: usize,
}

impl SFMT {
    pub fn new(seed: u32) -> Self {
        let mut sfmt = SFMT {
            state: [i32x4::new(0, 0, 0, 0); SFMT_N],
            idx: 0,
        };
        sfmt_init_gen_rand(&mut sfmt, seed);
        sfmt
    }

    fn pop(&mut self) -> u32 {
        let val = self.state[self.idx / 4].extract((self.idx % 4) as u32) as u32;
        self.idx += 1;
        val
    }

    fn gen_all(&mut self) {
        unsafe {
            sfmt_gen_rand_all(self);
        }
        self.idx = 0;
    }
}

impl Rng for SFMT {
    fn next_u32(&mut self) -> u32 {
        if self.idx >= SFMT_N32 {
            self.gen_all();
        }
        self.pop()
    }
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

pub fn period_certification(sfmt: &mut SFMT) {
    let mut inner = 0;
    let st = &mut sfmt.state[0];
    let parity = [SFMT_PARITY1, SFMT_PARITY2, SFMT_PARITY3, SFMT_PARITY4];
    for i in 0..4 {
        inner ^= st.extract(i as u32) as u32 & parity[i];
    }
    for i in [16, 8, 4, 2, 1].iter() {
        inner ^= inner >> i;
    }
    inner &= 1;
    if inner == 1 {
        return;
    }
    for i in 0..4 {
        let mut work = 1;
        for _ in 0..32 {
            if (work & parity[i]) != 0 {
                let val = st.extract(i as u32) as u32 ^ work;
                st.replace(i as u32, val as i32);
                return;
            }
            work <<= 1;
        }
    }
}

fn iterate(pre: i32, i: i32) -> i32 {
    use std::num::Wrapping;
    let pre = Wrapping(pre);
    let i = Wrapping(i);
    (Wrapping(1812433253) * (pre ^ (pre >> 30)) + i).0
}

fn map(a: i32, idx: i32) -> (i32x4, i32) {
    let b = iterate(a, 4 * idx + 1);
    let c = iterate(b, 4 * idx + 2);
    let d = iterate(c, 4 * idx + 3);
    let a2 = iterate(c, 4 * idx + 4);
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
mod test {
    use super::*;
    use std::io::Read;

    fn read_answer() -> Result<Vec<u32>, std::io::Error> {
        let mut f = std::fs::File::open("SFMT_19937.txt")?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(buf.split(" ")
            .map(|s| s.trim().parse().expect("Failed to parse into u32"))
            .collect())
    }

    #[test]
    fn gen_u32() {
        let ans = read_answer().expect("Failed to load answers");
        let mut sfmt = SFMT::new(1234);
        for (t, val) in ans.into_iter().enumerate() {
            let r = sfmt.next_u32();
            println!("[{}] gen = {}, ans = {}", t, r, val);
            assert_eq!(r, val);
        }
    }
}
