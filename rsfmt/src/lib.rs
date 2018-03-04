extern crate rand;
extern crate stdsimd;

mod sfmt;

use rand::Rng;
use stdsimd::simd::*;

#[derive(Clone)]
pub struct SFMT {
    /// the 128-bit internal state array
    pub state: [i32x4; sfmt::SFMT_N],
    /// index counter to the 32-bit internal state array
    pub idx: usize,
}

impl SFMT {
    pub fn new(seed: u32) -> Self {
        let mut sfmt = SFMT {
            state: [i32x4::new(0, 0, 0, 0); sfmt::SFMT_N],
            idx: 0,
        };
        sfmt::sfmt_init_gen_rand(&mut sfmt, seed);
        sfmt
    }

    fn pop(&mut self) -> u32 {
        let val = self.state[self.idx / 4].extract((self.idx % 4) as u32) as u32;
        self.idx += 1;
        val
    }

    fn gen_all(&mut self) {
        sfmt::sfmt_gen_rand_all(self);
        self.idx = 0;
    }
}

impl Rng for SFMT {
    fn next_u32(&mut self) -> u32 {
        if self.idx >= sfmt::SFMT_N32 {
            self.gen_all();
        }
        self.pop()
    }
}
