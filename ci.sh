#!/bin/bash
set -ex

cargo test
cd sfmt-sys && cargo test && cd -
if [[ $TRAVIS_RUST_VERSION = nightly ]]; then
  cd rsfmt
  cargo test
  cargo bench
  cd -
fi
