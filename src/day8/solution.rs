use crate::common::common::read_lines;

const EMPTY: char = ' ';

fn get_object(board: &[Vec<char>], pos: (i32, i32)) -> char {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= board.len() as i32 || pos.1 >= board[pos.0 as usize].len() as i32 {
        return EMPTY;
    }
    board[pos.0 as usize][pos.1 as usize]
}

fn set_object(board: &mut [Vec<char>], pos: (i32, i32), object: char) {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= board.len() as i32 || pos.1 >= board[pos.0 as usize].len() as i32 {
        return;
    }

    board[pos.0 as usize][pos.1 as usize] = object;
}

fn print_board(board: &[Vec<char>]) {
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}

fn find_cells_around(board: &[Vec<char>], pos: (i32, i32), object: char) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();

    for (row_index, row) in board.iter().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            if *cell == object {
                if pos.0 == row_index as i32 && pos.1 == cell_index as i32 {
                    // don't get ourselves
                    continue;
                }

                cells.push((row_index as i32, cell_index as i32));
            }
        }
    }

    cells
}

pub fn get_solution(input_path: &str) -> String {
    // Create a board
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut antenna_board = Vec::new();
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.flatten() {
                board.push(line.chars().collect());
                // push just dots, always be of len 100
                antenna_board.push(vec!['.'; line.len()]);
        }
    }

    // print_board(&board);

    // iterate through the board
    for row_index in 0..board.len() as i32 {
        for cell_index in 0..board[row_index as usize].len() as i32 {
            let cell = get_object(&board, (row_index, cell_index));
            if cell != '.' {
                // check if the cell is a number
                let cell_locations = find_cells_around(&board, (row_index, cell_index), cell);

                for location in cell_locations {
                    // prepare a vector for the direction of the antinode
                    let direction = (location.0 - row_index, location.1 - cell_index);

                    let mut antinode_location = (location.0 + direction.0, location.1 + direction.1);

                    while get_object(&antenna_board, antinode_location) != EMPTY {
                            set_object(&mut antenna_board, antinode_location, '#');


                        antinode_location.0 += direction.0;
                        antinode_location.1 += direction.1;
                    }
                }
            }
        }
    }

    // change all non-"." characters to '#'
    for row_index in 0..antenna_board.len() {
        for cell_index in 0..antenna_board[row_index].len() {
            let cell = get_object(&board, (row_index as i32, cell_index as i32));
            if cell != '.' {
                set_object(&mut antenna_board, (row_index as i32, cell_index as i32), '#');
            }
        }
    }
    print_board(&board);

    print_board(&antenna_board);

    antenna_board.iter()
                 .flat_map(|row| row.iter().filter(|cell| **cell == '#'))
                 .count().to_string()

}
