#!/bin/bash

# Create a module (arg 1: module name, arg 2: test input solution)
# "NAME SPECIFIED" - (make sure we don't overwrite one accidentally)
#   - mod.rs (just contains "pub mod solution")
#   - solution.rs (contains "pub fn print_solution() {}")
#   - input_test.txt (contains the test input data)

set -o noclobber

echo "Creating solution directory $1"
if [ ! -d "src/$1" ]; then
  # Control will enter here if $DIRECTORY doesn't exist.
  mkdir "src/$1"
  touch "src/$1/mod.rs"
  echo "pub mod solution;" >> "src/$1/mod.rs"
  cp "src/common/solution.rs" "src/$1/solution.rs"

  touch "src/$1/input_test.txt"
else
  echo "Directory already exists, aborting..."
fi

echo "mod common { pub mod common; }
mod $1;

fn main() {
    common::common::print_solution(
        $1::solution::get_solution(\"./src/$1/input.txt\"),
        $1::solution::get_solution(\"./src/$1/input_test.txt\"),
        \"$2\"
    );
}
" >| "./src/main.rs"
