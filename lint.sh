#!/bin/sh
# lint the project using clippy
cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used  -W clippy::missing_docs_in_private_items -W clippy::too_many_lines -W clippy::inefficient_to_string -D warnings