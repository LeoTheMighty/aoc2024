use crate::common::common::read;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

// xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)

    // 
    let input = read(input_path).unwrap();
    let input = input.replace("\n", "");

    let mut sum = 0;
    // Find all occurrences of mul(x,y) pattern and extract the numbers
    let mut enabled = true;
    let mut enable_indices: Vec<usize> = Vec::new();
    let mut disable_indices: Vec<usize> = Vec::new();

    let mut current_pos = 0;
    while let Some(start_idx) = input[current_pos..].find("do()") {
        let start_idx = current_pos + start_idx;
        current_pos = start_idx + 1;
        enable_indices.push(start_idx);
    }

    current_pos = 0;
    while let Some(start_idx) = input[current_pos..].find("don't()") {
        let start_idx = current_pos + start_idx;
        current_pos = start_idx + 1;
        disable_indices.push(start_idx);
    }
    println!("{:?}", enable_indices);
    println!("{:?}", disable_indices);

    current_pos = 0;
    while let Some(start_idx) = input[current_pos..].find("mul(") {
        // Is enabled is if the closest index in enable_indices is smaller than the closest index in disable_indices
        // filter out indices that are past the current position
        let find_idx = current_pos + start_idx;
        let closest_enable_idx: i32 = if let Some(idx) = enable_indices.iter().filter(|idx| idx < &&find_idx).min_by_key(|idx| find_idx - *idx) {
            *idx as i32
        } else {
            -1
        };
        let closest_disable_idx: i32 = if let Some(idx) = disable_indices.iter().filter(|idx| idx < &&find_idx).min_by_key(|idx| find_idx - *idx) {
            *idx as i32
        } else {
            -1
        };
        println!("IDX: {} ENABLE: {} DISABLE: {}", find_idx, closest_enable_idx, closest_disable_idx);
        let is_enabled = closest_disable_idx == -1 || closest_enable_idx > closest_disable_idx;
        let start_idx = current_pos + start_idx;
        current_pos = start_idx + 1;
        if let Some(end_idx) = input[start_idx..].find(')') {
            let end_idx = start_idx + end_idx;
            let content = &input[start_idx + 4..end_idx];
            println!("{}", content);
            if let Some((num1, num2)) = content.split_once(',') {
                if let (Ok(n1), Ok(n2)) = (num1.trim().parse::<i32>(), num2.trim().parse::<i32>()) {
                    println!("{} {}", n1, n2);
                    println!("{}", is_enabled);
                    if is_enabled {
                        sum += n1 * n2;
                    }
                    current_pos = end_idx + 1;
                }
            }
        } else {
            break;
        }
    }

    println!("{}", sum);
    solution = sum.to_string();

    return solution;
}
