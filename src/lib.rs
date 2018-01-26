extern crate rand;
extern crate sfmt_sys as ffi;

use rand::Rng;

pub struct SFMT(ffi::SFMT);

const SFMT_MEXP: i32 = 19937;
const SFMT_N: i32 = SFMT_MEXP / 128 + 1;
const SFMT_N32: i32 = SFMT_N * 4;

impl SFMT {
    fn empty() -> Self {
        let sfmt = ffi::SFMT {
            state: [ffi::w128 { u64: [0, 0] }; 156],
            idx: 0,
        };
        SFMT(sfmt)
    }

    fn gen_all(&mut self) {
        unsafe { ffi::sfmt_gen_rand_all(&mut self.0 as *mut _) };
    }

    fn get(&self) -> u32 {
        unsafe {
            let ptr = &self.0.state[0].u[0] as *const u32;
            *ptr.offset(self.0.idx as isize)
        }
    }

    pub fn new(seed: u32) -> Self {
        let mut sfmt = Self::empty();
        unsafe { ffi::sfmt_init_gen_rand(&mut sfmt.0 as *mut _, seed) };
        sfmt
    }
}

impl Rng for SFMT {
    fn next_u32(&mut self) -> u32 {
        if self.0.idx >= SFMT_N32 {
            self.gen_all();
            self.0.idx = 0;
        }
        let u = self.get();
        self.0.idx += 1;
        u
    }
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
