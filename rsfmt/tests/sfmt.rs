extern crate rand;
extern crate rsfmt;

use rsfmt::*;
use rand::Rng;
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
