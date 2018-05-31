//!  Rust implementation of [SIMD-oriented Fast Mersenne Twister (SFMT)] using [stable SIMD]
//!
//! [SIMD-oriented Fast Mersenne Twister (SFMT)]: http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/
//! [stable SIMD]: https://github.com/rust-lang/rfcs/blob/master/text/2325-stable-simd.md
//!
//! ```
//! # extern crate rand;
//! # extern crate sfmt;
//! use rand::Rng;
//! let mut rng = sfmt::SFMT::new(1234);  // seed
//! let r = rng.gen::<u32>();
//! println!("random u32 number = {}", r);
//! ```

#![feature(stdsimd)]

extern crate rand;
extern crate rand_core;

mod sfmt;
mod thread_rng;

use rand_core::{impls, Error, RngCore};
use std::simd::*;

pub use thread_rng::{thread_rng, ThreadRng};

/// State of SFMT
///
/// This struct implements random number generation through `rand::Rng`.
#[derive(Clone)]
pub struct SFMT {
    /// the 128-bit internal state array
    state: [i32x4; sfmt::SFMT_N],
    /// index counter to the 32-bit internal state array
    idx: usize,
}

impl SFMT {
    /// Create a new state from a seed.
    pub fn new(seed: u32) -> Self {
        let mut sfmt = SFMT {
            state: [i32x4::new(0, 0, 0, 0); sfmt::SFMT_N],
            idx: 0,
        };
        sfmt::sfmt_init_gen_rand(&mut sfmt, seed);
        sfmt
    }

    fn pop32(&mut self) -> u32 {
        let val = self.state[self.idx / 4].extract(self.idx % 4) as u32;
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
    use rand::Rng;

    #[test]
    fn random() {
        let mut rng = SFMT::new(1234);
        for _ in 0..sfmt::SFMT_N * 20 {
            // Generate many random numbers to test the overwrap
            let r = rng.gen::<u64>();
            if r % 2 == 0 {
                let _r = rng.gen::<u32>();
            } // shift SFMT.idx randomly
        }
    }
}
