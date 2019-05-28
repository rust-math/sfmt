sfmt
=====

[![Crate](http://meritbadge.herokuapp.com/sfmt)](https://crates.io/crates/sfmt)
[![docs.rs](https://docs.rs/sfmt/badge.svg)](https://docs.rs/sfmt)
[![Build Status](https://dev.azure.com/rust-math/sfmt/_apis/build/status/rust-math.sfmt?branchName=master)](https://dev.azure.com/rust-math/sfmt/_build/latest?definitionId=6&branchName=master)

Rust implementation of [SIMD-oriented Fast Mersenne Twister (SFMT)] interface using x86-SIMD in `std::arch`.
This is pure rust re-implementation, and tested on Windows/macOS/Linux.

[SIMD-oriented Fast Mersenne Twister (SFMT)]: http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/

Limitations
------------

- Supported only on x86 and x86_64 (due to original SFMT)
- Algorithms other than MT19937 are not supported (may be fixed in future release)

License
--------
MIT-License
