extern crate rand;
extern crate sfmt;

use rand::Rng;
use std::thread;

fn gen() -> Vec<u32> {
    let mut rng = sfmt::thread_rng();
    let mut v: Vec<u32> = Vec::new();
    for _ in 0..3 {
        v.push(rng.gen());
    }
    v
}

/// Two different thread should returns different random numbers
#[test]
#[should_panic]
fn thread_rng() {
    let th1 = thread::spawn(gen);
    let th2 = thread::spawn(gen);
    let v1 = th1.join().unwrap();
    let v2 = th2.join().unwrap();
    assert_eq!(v1, v2);
}
