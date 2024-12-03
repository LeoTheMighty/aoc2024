mod common { pub mod common; }
mod day3;

fn main() {
    common::common::print_solution(
        day3::solution::get_solution("./src/day3/input.txt"),
        day3::solution::get_solution("./src/day3/input_test.txt"),
        "48"
    );
}

