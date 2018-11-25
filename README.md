sfmt
=====

[![Crate](http://meritbadge.herokuapp.com/sfmt)](https://crates.io/crates/sfmt)
[![docs.rs](https://docs.rs/sfmt/badge.svg)](https://docs.rs/sfmt)
[![Build Status](https://travis-ci.org/termoshtt/rust-sfmt.svg?branch=master)](https://travis-ci.org/termoshtt/rust-sfmt)

Rust implementation of [SIMD-oriented Fast Mersenne Twister (SFMT)] with [rand 0.5](https://docs.rs/crate/rand/0.5.0) interface using x86-SIMD in `std::arch`.
This is pure rust re-implementation, and tested on Windows/macOS/Linux.

[SIMD-oriented Fast Mersenne Twister (SFMT)]: http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/

Limitations
------------

- Supported only on x86 and x86_64 (due to original SFMT)
- Algorithms other than MT19937 are not supported (may be fixed in future release)

License
--------
MIT-License
