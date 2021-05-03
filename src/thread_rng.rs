//! Thread-local RNG based on SFMT

use super::SFMT;

use rand_core::{Error, RngCore, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;

thread_local!(
    static THREAD_RNG_KEY: Rc<RefCell<SFMT<19937>>> = {
        Rc::new(RefCell::new(SFMT::from_entropy()))
    }
);

/// Thread-local RNG based on SFMT.
///
/// See the reference of the function [thread_rng](fn.thread_rng.html), which generates this struct.
#[derive(Clone)]
pub struct ThreadRng {
    rng: Rc<RefCell<SFMT<19937>>>,
}

/// Create a thread local RNG.
///
/// The seed of SFMT is generated by `rand::thread_rng()` on each thread.
///
/// ```
/// # extern crate sfmt;
/// # extern crate rand;
/// # use rand::Rng;
/// let mut rng = sfmt::thread_rng();
/// rng.gen::<u32>(); // random u32
/// ```
pub fn thread_rng() -> ThreadRng {
    ThreadRng {
        rng: THREAD_RNG_KEY.with(|t| t.clone()),
    }
}

impl RngCore for ThreadRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.borrow_mut().next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.borrow_mut().next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.borrow_mut().fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.borrow_mut().try_fill_bytes(dest)
    }
}
