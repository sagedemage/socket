#!/bin/sh
# lint the project using clippy
cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -D warnings