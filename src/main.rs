mod common { pub mod common; }
mod day5;

fn main() {
    common::common::print_solution(
        day5::solution::get_solution("./src/day5/input.txt"),
        day5::solution::get_solution("./src/day5/input_test.txt"),
        "123"
    );
}

