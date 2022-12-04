use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut numbers = contents
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut current_index: usize = 25;

    let invalid_number = loop {
        let validity_range = &numbers[current_index - 25..current_index];
        let current_number = &numbers[current_index];
        let mut calculated_numbers: HashSet<u64> = HashSet::new();

        for i in validity_range {
            for j in validity_range {
                calculated_numbers.insert(i + j);
            }
        }

        if !calculated_numbers.contains(&current_number) {
            break current_number;
        }

        current_index += 1;
    };

    println!("Invalid Number: {}", invalid_number);

    current_index = 0;
    let numbers_that_sum_up_to_invalid = loop {
        let mut temp_index = current_index;
        let mut temp_total: u64 = numbers[current_index];

        while temp_total < *invalid_number {
            temp_index += 1;
            temp_total += numbers[temp_index];
        }

        if temp_total == *invalid_number {
            break &mut numbers[current_index..temp_index];
        } else {
            current_index += 1;
        }
    };

    numbers_that_sum_up_to_invalid.sort();

    println!(
        "Numbers that sum up to invalid: MIN: {} - MAX: {} :::: With key: {}",
        numbers_that_sum_up_to_invalid.first().unwrap(),
        numbers_that_sum_up_to_invalid.last().unwrap(),
        numbers_that_sum_up_to_invalid.first().unwrap()
            + numbers_that_sum_up_to_invalid.last().unwrap()
    );
}
