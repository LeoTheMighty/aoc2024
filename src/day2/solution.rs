use crate::common::common::read_lines;

fn is_safe(vec: Vec<&str>) -> bool {
    let mut prev = -1;
    let mut is_increasing = false;
    let mut is_safe = true;
    let first: i32 = vec[0].parse().unwrap();
    let second: i32 = vec[1].parse().unwrap();
    if first < second {
        is_increasing = true;
    } else {
        is_increasing = false;
    }
    for part in vec {
        // read int
        let num: i32 = part.parse().unwrap();

        // safety is the whole parts list is either increasing or decreasing
        if prev != -1 {
            // Any two adjacent levels differ by at least one and at most three.
            println!("increasing: {:?}, num: {:?}, prev: {:?}", is_increasing, num, prev);

            if is_increasing && num < prev {
                is_safe = false;
                break;
            } else if !is_increasing && num > prev {
                is_safe = false;
                break;
            }

            let diff = (num - prev).abs();
            println!("diff: {:?}", diff);
            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
        }

        prev = num;
    }

    return is_safe;
}

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut count = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                // split line by spaces
                let parts: Vec<&str> = line.split(" ").collect();

                // iterate
                let parts_copy = parts.clone();
                let mut safe = is_safe(parts_copy);
                if !safe {
                    // iterate by index
                    for i in 0..parts.len() {
                        let mut parts_copy = parts.clone();
                        // remove part
                        parts_copy.remove(i);
                        // check if safe
                        if is_safe(parts_copy) {
                            safe = true;
                            break;
                        }
                    }
                }

                println!("is_safe: {:?}", safe);

                if safe {
                    count += 1;
                }
            }
        }
    }

    solution = count.to_string();

    return solution;
}
