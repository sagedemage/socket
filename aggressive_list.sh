#!/bin/sh
# aggresive lint
#cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::restriction -A clippy::print_stdout -A clippy::print_stderr -A clippy::question_mark_used -A clippy::indexing_slicing -A clippy::std_instead_of_core -D warnings

cargo clippy  --all  -- -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery -D warnings