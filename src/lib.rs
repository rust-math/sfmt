//!  Rust implementation of [SIMD-oriented Fast Mersenne Twister (SFMT)] using [stable SIMD]
//!
//! [SIMD-oriented Fast Mersenne Twister (SFMT)]: http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/
//! [stable SIMD]: https://github.com/rust-lang/rfcs/blob/master/text/2325-stable-simd.md
//!
//! ```
//! use rand::{Rng, FromEntropy};
//! let mut rng = sfmt::SFMT::from_entropy();
//! let r = rng.gen::<u32>();
//! println!("random u32 number = {}", r);
//! ```

mod sfmt;
mod thread_rng;

use rand_core::{impls, Error, RngCore, SeedableRng};

pub use crate::thread_rng::{thread_rng, ThreadRng};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// State of SFMT
///
/// This struct implements random number generation through `rand::Rng`.
#[derive(Clone)]
pub struct SFMT {
    /// the 128-bit internal state array
    state: [__m128i; sfmt::SFMT_N],
    /// index counter to the 32-bit internal state array
    idx: usize,
}

fn extract(vals: __m128i, imm: usize) -> u32 {
    unsafe {
        match imm {
            0 => _mm_extract_epi32(vals, 0) as u32,
            1 => _mm_extract_epi32(vals, 1) as u32,
            2 => _mm_extract_epi32(vals, 2) as u32,
            3 => _mm_extract_epi32(vals, 3) as u32,
            _ => unreachable!(),
        }
    }
}

fn insert(vals: &mut __m128i, val: i32, imm: usize) {
    let updated = unsafe {
        match imm {
            0 => _mm_insert_epi32(*vals, val, 0),
            1 => _mm_insert_epi32(*vals, val, 1),
            2 => _mm_insert_epi32(*vals, val, 2),
            3 => _mm_insert_epi32(*vals, val, 3),
            _ => unreachable!(),
        }
    };
    ::std::mem::replace(vals, updated);
}

impl SFMT {
    fn pop32(&mut self) -> u32 {
        let val = extract(self.state[self.idx / 4], self.idx % 4);
        self.idx += 1;
        val
    }

    fn pop64(&mut self) -> u64 {
        let p = self.state.as_ptr() as *const u32;
        let val = unsafe {
            let p = p.offset(self.idx as isize);
            *(p as *const u64) // reinterpret cast [u32; 2] -> u64
        };
        self.idx += 2;
        val
    }

    fn gen_all(&mut self) {
        sfmt::sfmt_gen_rand_all(self);
        self.idx = 0;
    }
}

impl SeedableRng for SFMT {
    type Seed = [u8; 4];

    fn from_seed(seed: [u8; 4]) -> Self {
        let mut sfmt = SFMT {
            state: unsafe { [_mm_setzero_si128(); sfmt::SFMT_N] },
            idx: 0,
        };
        let seed = unsafe { *(seed.as_ptr() as *const u32) };
        sfmt::sfmt_init_gen_rand(&mut sfmt, seed);
        sfmt
    }
}

impl RngCore for SFMT {
    fn next_u32(&mut self) -> u32 {
        if self.idx >= sfmt::SFMT_N32 {
            self.gen_all();
        }
        self.pop32()
    }

    fn next_u64(&mut self) -> u64 {
        if self.idx >= sfmt::SFMT_N32 - 1 {
            // drop last u32 if idx == N32-1
            self.gen_all();
        }
        self.pop64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{FromEntropy, Rng};

    #[test]
    fn random() {
        let mut rng = SFMT::from_entropy();
        for _ in 0..sfmt::SFMT_N * 20 {
            // Generate many random numbers to test the overwrap
            let r = rng.gen::<u64>();
            if r % 2 == 0 {
                let _r = rng.gen::<u32>();
            } // shift SFMT.idx randomly
        }
    }
}
