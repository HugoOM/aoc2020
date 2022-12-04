use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents = contents.lines().collect::<Vec<&str>>();
    let mut current_time: u128 = 0;

    let departure_timings = contents[1]
        .split(',')
        .map(|v| {
            if v == "x" {
                0
            } else {
                v.parse::<u128>().unwrap()
            }
        })
        .collect::<Vec<u128>>();

    // loop {
    //     current_time += 1;

    //     let found = departure_timings
    //         .iter()
    //         .enumerate()
    //         .all(|(index, &departure)| {
    //             if departure == 0 {
    //                 return true;
    //             };

    //             if (current_time + index as u128) % departure != 0 {
    //                 return false;
    //             };

    //             true
    //         });

    //     if found {
    //         break;
    //     }
    // }

    let mut current_solution = 0;
    let mut step_size: u64 = 1;

    // Insight here is that each previously found pattern
    // repeats itself every Least common multiple (LCM) steps
    // and LCM of primes is their product
    for (offset, &bus_id) in departure_timings.iter().enumerate() {
        if bus_id == 0 {
            continue;
        }

        for timestamp in (current_solution..u64::MAX).step_by(step_size as usize) {
            println!("Current Timestamp: {}", timestamp);
            if (timestamp + offset as u64) % bus_id as u64 == 0 {
                current_solution = timestamp;
                print!("Bus Id: {} ", bus_id);
                step_size *= bus_id as u64;
                println!("Step Size: {}", step_size);

                if step_size + current_solution > u64::MAX {
                    println!("Step Size is bigger than max");
                }

                break;
            }
        }
    }

    println!("Timestamp: {}", current_solution);
}
