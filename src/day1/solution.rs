use crate::common::common::read_lines;

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    // Initialize the lists outside the loop
    let mut first_word_list: Vec<i32> = Vec::new();
    let mut second_word_list: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                println!("{:?}", line);
                let mut line_split = line.split(' ').filter(|s| !s.is_empty());

                let first_word_int = match line_split.next().and_then(|s| s.parse::<i32>().ok()) {
                    Some(num) => num,
                    None => continue,
                };
                println!("{:?}", first_word_int);
                let second_word_int = match line_split.next().and_then(|s| s.parse::<i32>().ok()) {
                    Some(num) => num,
                    None => continue,
                };
                println!("{:?}", second_word_int);

                // Collect the integers into the lists
                first_word_list.push(first_word_int);
                second_word_list.push(second_word_int);

                // Bubble sort first_word_list
                for i in 0..first_word_list.len() {
                    for j in 0..(first_word_list.len() - i - 1) {
                        if first_word_list[j] > first_word_list[j + 1] {
                            first_word_list.swap(j, j + 1);
                        }
                    }
                }

                for i in 0..second_word_list.len() {
                    for j in 0..(second_word_list.len() - i - 1) {
                        if second_word_list[j] > second_word_list[j + 1] {
                            second_word_list.swap(j, j + 1);
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", first_word_list);
    println!("{:?}", second_word_list);
    let mut sum = 0;
    for i in 0..first_word_list.len() {
        // count how many times the first word is in the second word list
        let mut count = 0;
        for j in 0..second_word_list.len() {
            if first_word_list[i] == second_word_list[j] {
                count += 1;
            }
        }

        sum += (first_word_list[i] * count);
    }

    solution = sum.to_string();

    return solution;
}
