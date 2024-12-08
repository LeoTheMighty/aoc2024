mod common { pub mod common; }
mod day8;

fn main() {
    common::common::print_solution(
        day8::solution::get_solution("./src/day8/input.txt"),
        day8::solution::get_solution("./src/day8/input_test.txt"),
        "34"
    );
}

