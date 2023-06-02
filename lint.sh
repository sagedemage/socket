#!/bin/sh
# lint the project using clippy
cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery -A clippy::print_stdout -A clippy::print_stderr -A clippy::question_mark_used -A clippy::std_instead_of_core -A clippy::indexing_slicing -D warnings