use super::SFMT;

use std::rc::Rc;
use std::cell::RefCell;
use rand::{self, Rng};

thread_local!(
    static THREAD_RNG_KEY: Rc<RefCell<SFMT>> = {
        let mut rng = rand::thread_rng();
        Rc::new(RefCell::new(SFMT::new(rng.gen())))
    }
);

#[derive(Clone)]
pub struct ThreadRng {
    rng: Rc<RefCell<SFMT>>,
}

pub fn thread_rng() -> ThreadRng {
    ThreadRng {
        rng: THREAD_RNG_KEY.with(|t| t.clone()),
    }
}
