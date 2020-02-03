#!/bin/bash

set -xeuo pipefail

rustup default stable
rm -rf src/
svd2rust -i ATSAMD11D14AS.svd
form -i lib.rs -o src/
rm lib.rs
cargo fmt
rustup default nightly
git status
