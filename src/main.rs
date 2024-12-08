mod common { pub mod common; }
mod day7;

fn main() {
    common::common::print_solution(
        day7::solution::get_solution("./src/day7/input.txt"),
        day7::solution::get_solution("./src/day7/input_test.txt"),
        "11387"
    );
}

