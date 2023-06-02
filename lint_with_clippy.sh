#!/bin/sh
# lint the project using clippy
cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::missing_docs_in_private_items -D warnings