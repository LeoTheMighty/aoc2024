use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn just_print_solution(solution: String) {
    println!("{}", "Full Answer:".bright_green().bold());
    println!("{}", format!("{}", solution).bright_green().bold());
}

pub fn print_solution(solution: String, test_solution: String, test_answer: &str) -> bool {
    just_print_solution(solution);
    println!("{}", "\nRunning the test case...\n-----------------------\n".cyan());
    println!("{}", format!("Test Case --- Expected: {} vs Actual: {}", format!("{}", test_solution).green().bold(), format!("{}", test_answer).magenta().bold()).blue());
    if !test_solution.eq(test_answer) {
        eprintln!("{}", format!("!!!DOES NOT PASS TEST CASE!!! Expected: `{}`, Actual: `{}`", format!("{}", test_answer).green().bold(), format!("{}", test_solution).bright_magenta()).red().bold());
        return false;
    }
    println!("{}", "passes the test case :)".green().bold());
    return true;
}
