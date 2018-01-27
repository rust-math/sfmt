#!/bin/bash

cargo test -v
pushd sfmt-sys
cargo test -v
popd

if [[ ${TRAVIS_RUST_VERSION} = "nightly" ]]; then
  cargo bench
fi
