use std::fs;

#[derive(Clone)]
struct Board {
    rows: Vec<Vec<char>>,
}

impl Board {
    fn new(board: Vec<Vec<char>>) -> Board {
        Board { rows: board }
    }

    fn print(&self) {
        for row in self.rows.iter() {
            for c in row.iter() {
                print!("{}", c);
            }
            print!("\n");
        }
        println!("\n~~~~~~~~~~~~~\n");
    }

    fn count_occupied_seats(&self) -> u32 {
        let mut occupied_seats_count = 0;
        for row in self.rows.iter() {
            for seat in row.iter() {
                if seat == &'#' {
                    occupied_seats_count += 1;
                }
            }
        }
        occupied_seats_count
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        for (row_i, row) in self.rows.iter().enumerate() {
            for (seat_i, seat) in row.iter().enumerate() {
                if seat != &other.rows[row_i][seat_i] {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let base_board = contents
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let base_board = Board::new(base_board);

    let mut boards: Vec<Board> = Vec::new();
    boards.push(base_board);

    let duplicate_board = loop {
        let processed_board = process_board(boards.iter().last().unwrap());

        if boards.contains(&processed_board) {
            break processed_board;
        }

        boards.push(processed_board);
    };

    println!(
        "Occupied seats count on duplicated board: {}",
        duplicate_board.count_occupied_seats()
    );
}

fn process_board(input: &Board) -> Board {
    let mut processed_board: Board = input.clone();

    for (row_i, row) in input.rows.iter().enumerate() {
        for (seat_i, seat) in row.iter().enumerate() {
            match seat {
                'L' => {
                    if check_surroundings(input, (seat_i, row_i)).0 == 8 {
                        processed_board.rows[row_i][seat_i] = '#';
                    }
                }
                '#' => {
                    if check_surroundings(input, (seat_i, row_i)).1 >= 5 {
                        processed_board.rows[row_i][seat_i] = 'L';
                    }
                }
                _ => continue,
            }
        }
    }

    processed_board
}

fn check_surroundings(board: &Board, seat_location: (usize, usize)) -> (u32, u32) {
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut positions_to_check: Vec<(usize, usize)> = Vec::new();

    for direction in directions {
        let mut temp_position: (usize, usize) = seat_location;
        let mut temp_direction: (i32, i32) = direction;
        let mut iterations: u32 = 1;
        let mut is_valid: bool = true;

        while board.rows[temp_position.1][temp_position.0] == '.' || iterations == 1 {
            temp_position = seat_location;
            temp_direction.0 = direction.0 * iterations as i32;
            temp_direction.1 = direction.1 * iterations as i32;

            if temp_direction.0 < 0 {
                if temp_position.0 as i32 - temp_direction.0.abs() < 0 {
                    is_valid = false;
                    break;
                }
                temp_position.0 -= temp_direction.0.abs() as usize;
            } else {
                if temp_position.0 as i32 + temp_direction.0.abs() >= board.rows[0].len() as i32 {
                    is_valid = false;
                    break;
                }
                temp_position.0 += temp_direction.0 as usize;
            }

            if temp_direction.1 < 0 {
                if temp_position.1 as i32 - temp_direction.1.abs() < 0 {
                    is_valid = false;
                    break;
                }
                temp_position.1 -= temp_direction.1.abs() as usize;
            } else {
                if temp_position.1 as i32 + temp_direction.1.abs() >= board.rows.len() as i32 {
                    is_valid = false;
                    break;
                }
                temp_position.1 += temp_direction.1 as usize;
            }

            iterations += 1;
        }

        if is_valid {
            positions_to_check.push(temp_position);
        }
    }

    let mut occupied_seats: u32 = 0;
    let mut free_seats: u32 = 0;

    free_seats += 8 - positions_to_check.len() as u32;

    for pos in positions_to_check.iter() {
        match board.rows[pos.1][pos.0] {
            '#' => occupied_seats += 1,
            'L' | '.' => free_seats += 1,
            _ => continue,
        }
    }

    (free_seats, occupied_seats)
}

/* Exercise 1
fn process_board(input: &Board) -> Board {
    let mut processed_board: Board = input.clone();

    for (row_i, row) in input.rows.iter().enumerate() {
        for (seat_i, seat) in row.iter().enumerate() {
            match seat {
                'L' => {
                    if check_surroundings(input, (seat_i, row_i)).0 == 8 {
                        processed_board.rows[row_i][seat_i] = '#';
                    }
                }
                '#' => {
                    if check_surroundings(input, (seat_i, row_i)).1 >= 4 {
                        processed_board.rows[row_i][seat_i] = 'L';
                    }
                }
                _ => continue,
            }
        }
    }

    processed_board
}

fn check_surroundings(board: &Board, seat_location: (usize, usize)) -> (u32, u32) {
    let positions_to_check: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut occupied_seats: u32 = 0;
    let mut free_seats: u32 = 0;
    let mut valid_positions_to_check: Vec<(usize, usize)> = Vec::new();

    for pos in positions_to_check.iter() {
        let mut temp_position: (usize, usize) = seat_location;
        if pos.0 < 0 {
            if temp_position.0 as i32 - pos.0.abs() < 0 {
                continue;
            }
            temp_position.0 -= pos.0.abs() as usize;
        } else {
            if temp_position.0 as i32 + pos.0.abs() >= board.rows[0].len() as i32 {
                continue;
            }
            temp_position.0 += pos.0 as usize;
        }

        if pos.1 < 0 {
            if temp_position.1 as i32 - pos.1.abs() < 0 {
                continue;
            }
            temp_position.1 -= pos.1.abs() as usize;
        } else {
            if temp_position.1 as i32 + pos.1.abs() >= board.rows.len() as i32 {
                continue;
            }
            temp_position.1 += pos.1 as usize;
        }

        valid_positions_to_check.push(temp_position);
    }

    free_seats += 8 - valid_positions_to_check.len() as u32;

    for pos in valid_positions_to_check.iter() {
        match board.rows[pos.1][pos.0] {
            '#' => occupied_seats += 1,
            'L' | '.' => free_seats += 1,
            _ => continue,
        }
    }

    (free_seats, occupied_seats)
}
*/
