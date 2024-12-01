mod common { pub mod common; }
mod day1;

fn main() {
    common::common::print_solution(
        day1::solution::get_solution("./src/day1/input.txt"),
        day1::solution::get_solution("./src/day1/input_test.txt"),
        "31"
    );
}

