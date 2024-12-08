use crate::common::common::read_lines;

fn can_be_true(number: i64, equation: Vec<i64>) -> bool {
    let num1 = equation[0];

    if equation.len() == 1 {
        return num1 == number;
    }

    let num2 = equation[1];

    let mut add_equation = equation[2..].to_vec();
    add_equation.insert(0, num1 + num2);

    if can_be_true(number, add_equation) {
        return true;
    }

    let mut mult_equation = equation[2..].to_vec();
    mult_equation.insert(0, num1 * num2);

    if can_be_true(number, mult_equation) {
        return true;
    }

    let mut concat_equation = equation[2..].to_vec();
    let concat_number = (num1.to_string() + &num2.to_string()).parse::<i64>().unwrap();
    concat_equation.insert(0, concat_number);

    if can_be_true(number, concat_equation) {
        return true;
    }

    return false;
}

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut sum = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let parts = line.split(": ").collect::<Vec<&str>>();
                let number = parts[0].parse::<i64>().unwrap();
                let equation = parts[1].split(" ").collect::<Vec<&str>>();
                let mut numbers = Vec::new();
                for part in equation {
                    if part.parse::<i64>().is_ok() {
                        numbers.push(part.parse::<i64>().unwrap());
                    }
                }
                println!("number: {}, numbers: {:?}", number, numbers);

                if can_be_true(number, numbers) {
                    println!("TRUE!!!!! number: {} can be true", number);
                    sum += number;
                } else {
                    println!("FALSE!!!!! number: {} can't be true", number);
                }
            }
        }
    }

    solution = sum.to_string();

    return solution;
}
