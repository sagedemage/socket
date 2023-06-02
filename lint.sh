#!/bin/sh
# lint the project using clippy
cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used  -W clippy::missing_docs_in_private_items -D warnings