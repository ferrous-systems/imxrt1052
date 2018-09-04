#!/usr/bin/env bash

cargo install --force --version 0.13.1 svd2rust &&
cargo install --force --version 0.99.4 rustfmt-nightly &&
cargo install --force --version 0.3.0 form &&

rm -rf src && mkdir src &&
svd2rust -i MIMXRT1052.xml &&
form -i lib.rs -o src && rm lib.rs &&
cargo fmt &&
rustfmt build.rs
