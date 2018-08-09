extern crate rand;
extern crate sfmt;

use rand::{RngCore, SeedableRng};
use sfmt::*;
use std::io::Read;

fn read_answer() -> Result<Vec<u32>, std::io::Error> {
    let mut f = std::fs::File::open("check/SFMT_19937.txt")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf
        .split(" ")
        .map(|s| s.trim().parse().expect("Failed to parse into u32"))
        .collect())
}

#[test]
fn gen_u32() {
    let ans = read_answer().expect("Failed to load answers");
    let seed: u32 = 1234;
    let seed = unsafe { *(&seed as *const u32 as *const [u8; 4]) };
    let mut sfmt = SFMT::from_seed(seed); // 1234 = 0x4D2
    for (t, val) in ans.into_iter().enumerate() {
        let r = sfmt.next_u32();
        println!("[{}] gen = {}, ans = {}", t, r, val);
        assert_eq!(r, val);
    }
}
