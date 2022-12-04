use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let contents = contents.lines().collect::<Vec<&str>>();

    // println!("{:#?}", contents);

    let mut ids: Vec<u32> = Vec::new();

    for seat in contents {
        // let mut row: Vec<char> = Vec::new();

        // let row: &mut [u8] = &mut [0b0; 8];

        // row.push('0');
        // let mut col: Vec<u8> = Vec::new();

        // for (i, c) in seat.chars().enumerate() {
        //     match c {
        //         // 'F' => row &= 0xF0,
        //         'F' => row[i + 1] = 0b0,
        //         'B' => row[i + 1] = 0b1,
        //         'R' => {}
        //         'L' => {}
        //         _ => break,
        //     }
        // }

        // let row: u32 = seat.chars().fold(0, |acc, b| {
        //     let bit = if b == 'F' {
        //         return acc;
        //     } else if b == 'B' {
        //         1
        //     } else {
        //         return acc;
        //     };
        //     acc * 2 + bit as u32
        // });

        let mut row: u32 = 0b0;
        let mut invalid_count = 0;

        for (i, c) in seat.chars().rev().enumerate() {
            if c != 'B' {
                if c != 'F' {
                    invalid_count += 1;
                }
                continue;
            }

            row |= 0b1 << (i - invalid_count);
        }

        let seat: u32 = seat.chars().fold(0, |acc, b| {
            let bit = if b == 'R' {
                1
            } else if b == 'L' {
                0
            } else {
                return acc;
            };
            acc * 2 + bit as u32
        });
        // println!("Row: {} - Seat: {}", row, seat);

        ids.push(row * 8 + seat);
    }

    println!("Max: {}", ids.iter().max().unwrap());

    ids.sort();

    println!("{:#?}", ids);

    let mut my_seat_id: u32 = 0;

    for (index, id) in ids.iter().enumerate() {
        if ids[index + 1] == id + 2 {
            my_seat_id = id + 1;
            break;
        }
    }

    println!("My seat id: {}", my_seat_id);
}
