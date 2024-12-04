mod common { pub mod common; }
mod day4;

fn main() {
    common::common::print_solution(
        day4::solution::get_solution("./src/day4/input.txt"),
        day4::solution::get_solution("./src/day4/input_test.txt"),
        "9"
    );
}

