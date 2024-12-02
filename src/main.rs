mod common { pub mod common; }
mod day2;

fn main() {
    common::common::print_solution(
        day2::solution::get_solution("./src/day2/input.txt"),
        day2::solution::get_solution("./src/day2/input_test.txt"),
        "4"
    );
}

