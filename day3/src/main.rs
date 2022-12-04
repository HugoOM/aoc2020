use std::fs;

fn main() {
    let slope_slice = fs::read_to_string("input.txt").unwrap();
    let slope_slice = slope_slice.split('\n').collect::<Vec<&str>>();

    // let mut current_position: (u32, u32) = (0, 0);
    // let mut trees_hit_count = 0;

    // while current_position.1 < slope_slice.len() as u32 {
    //     let slope_line = slope_slice[current_position.1 as usize]
    //         .chars()
    //         .collect::<Vec<char>>();

    //     if slope_line[current_position.0 as usize] == '#' {
    //         trees_hit_count += 1;
    //     }

    //     current_position.0 = (current_position.0 + 3) % slope_line.len() as u32;
    //     current_position.1 += 1;
    // }

    // println!("Trees hit: {}", trees_hit_count);

    let mut results = [0, 0, 0, 0, 0];
    let slope_width = slope_slice[0].len();

    for i in 0..=slope_slice.len() {
        let moves = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        for (m_index, m) in moves.iter().enumerate() {
            let current_position = ((m.0 * i) % slope_width, m.1 * i);

            if current_position.1 >= slope_slice.len() {
                continue;
            }

            let slope_row = slope_slice[current_position.1 as usize];

            if slope_row.chars().collect::<Vec<char>>()[current_position.0] == '#' {
                results[m_index] += 1;
            }
        }
    }

    println!("{:?}", results);

    println!(
        "Total trees hit out of all possibilities: {}",
        // results.iter().fold(1 as u64, |total, v| total * v)
        results.iter().product::<u32>()
    );
}
