use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read<P>(filename: P) -> io::Result<String>
where P: AsRef<Path> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    std::io::Read::read_to_string(&mut file, &mut contents)?;
    Ok(contents)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn just_print_solution(solution: String) {
    println!("{}", "Full Answer:".bright_green().bold());
    println!("{}", solution.bright_green().bold());
}

pub fn print_solution(solution: String, test_solution: String, test_answer: &str) -> bool {
    just_print_solution(solution);
    println!("{}", "\nRunning the test case...\n-----------------------\n".cyan());
    println!("{}", format!("Test Case --- Expected: {} vs Actual: {}", test_solution.green().bold(), test_answer.magenta().bold()).blue());
    if !test_solution.eq(test_answer) {
        eprintln!("{}", format!("!!!DOES NOT PASS TEST CASE!!! Expected: `{}`, Actual: `{}`", test_answer.green().bold(), test_solution.bright_magenta()).red().bold());
        return false;
    }
    println!("{}", "passes the test case :)".green().bold());
    true
}
