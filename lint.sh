#!/bin/sh
# lint the project using clippy
cargo clippy --all-targets --all-features -- -D clippy::pedantic -D warnings