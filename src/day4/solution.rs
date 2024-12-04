use crate::common::common::read_lines;

fn get_space(i: i32, j: i32, board: &Vec<Vec<char>>) -> char {
    if i < 0 || j < 0 || i >= board.len() as i32 || j >= board[i as usize].len() as i32 {
        return ' ';
    }

    return board[i as usize][j as usize];
}

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut board: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                board.push(line.chars().collect());
            }
        }
    }

    // iterate through i, j
    let mut xmases: i32 = 0;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let i = i as i32;
            let j = j as i32;


            let mut center = get_space(i, j, &board);
            let mut top_left = get_space(i - 1, j - 1, &board);
            let mut top_right = get_space(i - 1, j + 1, &board);
            let mut bottom_left = get_space(i + 1, j - 1, &board);
            let mut bottom_right = get_space(i + 1, j + 1, &board);
            let mut got_it = false;
            if center == 'A' {
                if ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M')) &&
                    ((bottom_left == 'M' && top_right == 'S') || (bottom_left == 'S' && top_right == 'M')) {

                    xmases += 1;

                    print!("{}", get_space(i, j, &board));

                    got_it = true;
                }
            }

            if got_it {
                print!("{}", get_space(i, j, &board));
            } else {
                // print!(".");
                print!("{}", get_space(i, j, &board));
            }

            // if get_space(i, j, &board) == 'X' {
            //     for direction in directions {
            //         if get_space(i + direction.0, j + direction.1, &board) == 'M' {
            //             if get_space(i + direction.0 * 2, j + direction.1 * 2, &board) == 'A' {
            //                 if get_space(i + direction.0 * 3, j + direction.1 * 3, &board) == 'S' {
            //                     xmases += 1;
            //                 }
            //             }
            //         }
            //     }
            // }
        }
        println!("");
    }

    solution = xmases.to_string();

    return solution;
}
