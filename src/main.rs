mod common { pub mod common; }
mod day6;

fn main() {
    common::common::print_solution(
        day6::solution::get_solution("./src/day6/input.txt"),
        day6::solution::get_solution("./src/day6/input_test.txt"),
        "6"
    );
}

