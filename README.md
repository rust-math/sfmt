sfmt
=====

[![Crate](https://img.shields.io/crates/v/sfmt.svg)](https://crates.io/crates/sfmt)
[![docs.rs](https://docs.rs/sfmt/badge.svg)](https://docs.rs/sfmt)
[![DOI](https://zenodo.org/badge/118722822.svg)](https://zenodo.org/badge/latestdoi/118722822)

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
