use crate::common::common::read_lines;

const EMPTY: char = ' ';

fn get_object(board: &Vec<Vec<char>>, pos: (i32, i32)) -> char {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= board.len() as i32 || pos.1 >= board[pos.0 as usize].len() as i32 {
        return EMPTY;
    }
    return board[pos.0 as usize][pos.1 as usize];
}

fn set_object(board: &mut Vec<Vec<char>>, pos: (i32, i32), object: char) {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= board.len() as i32 || pos.1 >= board[pos.0 as usize].len() as i32 {
        return;
    }

    board[pos.0 as usize][pos.1 as usize] = object;
}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    // (-1, 0) -> (0, 1)
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let index = directions.iter().position(|&d| d == direction).unwrap();
    let new_index = (index + 1) % directions.len();
    return directions[new_index];

    // use sin and cos, subtract 90 degrees
    // Calculate angle in radians from direction vector
    // println!("direction: {:?}", direction);
    // let angle = (direction.0 as f64).atan2(direction.1 as f64);
    // let new_angle = angle - std::f64::consts::PI / 2.0;

    // let new_x = (direction.0 as f64 * new_angle.cos() - direction.1 as f64 * new_angle.sin()) as i32;
    // let new_y = (direction.0 as f64 * new_angle.sin() + direction.1 as f64 * new_angle.cos()) as i32;

    // println!("new direction: {:?}", (new_x, new_y));
    // return (new_x, new_y);
}

fn clone_board(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    return board.iter().map(|row| row.clone()).collect();
}

fn will_loop(board: &Vec<Vec<char>>, pos: (i32, i32), direction: (i32, i32)) -> bool {
    let mut current_pos = pos;
    let mut current_direction = direction;
    // initialize hash set with pos and direction
    let mut visited = std::collections::HashSet::new();

    loop {
        if visited.contains(&(current_pos, current_direction)) {
            return true;
        }

        let next_pos = (current_pos.0 + current_direction.0, current_pos.1 + current_direction.1);
        let next_object = get_object(&board, next_pos);

        if next_object == EMPTY {
            // They are done 
            return false;
        } else if next_object == '#' {
            // turn right
            visited.insert((current_pos, current_direction));
            current_direction = turn_right(current_direction);
        } else {
            // move forward
            visited.insert((current_pos, current_direction));
            current_pos = next_pos;
        }
    }
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        println!("{:?}", row);
    }
}

pub fn get_solution(input_path: &str) -> String {
    let mut solution: String = "".to_owned();

    let mut board: Vec<Vec<char>> = vec![];
    let mut start_pos: (i32, i32) = (0, 0);
    if let Ok(lines) = read_lines(input_path) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let mut row: Vec<char> = vec![];
                for (i, mut c) in line.chars().enumerate() {
                    if c == '^' {
                        c = '.';
                        start_pos = (board.len() as i32, i as i32);
                    }
                    row.push(c);
                }
                board.push(row);
            }
        }
    }

    // walk the man through the board
    let mut pos = start_pos;
    let mut direction: (i32, i32) = (-1, 0);
    let mut obstruction_set = std::collections::HashSet::new();
    let mut visited = std::collections::HashSet::new();
    visited.insert(pos);
    loop {
        // check object in front of guard
        let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
        let next_object = get_object(&board, next_pos);
        // println!("next_object: {:?}", next_object);

        if next_object == EMPTY {
            // They are done 
            // println!("done!");
            break;
        } else if next_object == '#' {
            // turn left
            // println!("turn right");
            direction = turn_right(direction);
        } else {
            let mut cloned_board = clone_board(&board);
            // Add a potential barrier and see if it loops
            set_object(&mut cloned_board, next_pos, '#');
            if will_loop(&cloned_board, pos, direction) {
                if !visited.contains(&next_pos) {
                    obstruction_set.insert(next_pos);
                }
            }
            // move forward
            // println!("move forward");
            // set_object(&mut board, next_pos, 'X');
            pos = next_pos;
            visited.insert(pos);
        }
        // if board.len() <= 10 {
        //     print_board(&board);
        // }
    }

    // count all 'X's
    // let mut count = 0;
    // for row in board {
    //     for c in row {
    //         if c == 'X' {
    //             count += 1;
    //         }
    //     }
    // }

    solution = obstruction_set.len().to_string();

    return solution;
}
