use crate::common::common::read_lines;

fn sort_by_rules(numbers: &mut [i32], rule_map: &std::collections::HashMap<i32, Vec<i32>>) -> bool {
    let n = numbers.len();
    
    // Bubble sort with rule checking
    let mut swapped = false;
    for i in 0..n {
        for j in 0..(n - i - 1) {
            let current = numbers[j];
            let next = numbers[j + 1];
            
            // Check if next number has rules
            if let Some(must_come_before) = rule_map.get(&next) {
                // If next number must come before current number, swap
                if must_come_before.contains(&current) {
                    numbers.swap(j, j + 1);
                    swapped = true;
                }
            }
        }
    }

    swapped
}

pub fn get_solution(input_path: &str) -> String {
    let mut is_first_section = true;
    let mut rule_map: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    let mut valid_sum = 0;

    if let Ok(lines) = read_lines(input_path) {
        for line in lines.map_while(Result::ok) {
                if is_first_section {
                    // if line is only whitespace, go to next section
                    if line.trim().is_empty() {
                        is_first_section = false;
                        continue;
                    }

                    let numbers: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();

                    let before = numbers[0];
                    let after = numbers[1];

                    rule_map.entry(before).or_default().push(after);
                } else {
                    // The list of page numbers 
                    let mut numbers: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();

                    println!("SECTION: {:?}", numbers);

                    let swapped = sort_by_rules(&mut numbers, &rule_map);

                    if swapped {
                        valid_sum += numbers[numbers.len() / 2];
                    }


                    // Now we need to update the list to be correct via the rules.
                    // iterate from the back of the list



                    /*
                    // We need to filter out the invalid page ordering
                    // The rules dictate the which pages need to go before the other one
                    let mut valid = true;
                    // For each page, check all of the numbers before and see if it breaks any rules
                    let mut before_pages = vec![];
                    for page in numbers {
                        // Check if the page breaks any rules
                        let temp_vec = vec![];
                        let page_rules = rule_map.get(&page).unwrap_or(&temp_vec);

                        println!("rules: {:?}", page_rules);

                        for before_page in before_pages.iter() {
                            if page_rules.contains(&before_page) {
                                valid = false;
                                break;
                            }
                        }

                        if !valid {
                            break;
                        }

                        // Insert at the beginning of the vector
                        before_pages.insert(0, page);
                    }

                    if valid {
                        valid_sum += middle_number;
                    }
                    */
                }
        }
    }

    valid_sum.to_string()
}
