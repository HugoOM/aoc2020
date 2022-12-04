use std::fs;
use std::ops::Range;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents: Vec<&str> = contents.split('\n').collect();

    let (valid_criteria_1_count, valid_criteria_2_count): (u32, u32) =
        contents.iter().fold((0, 0), |mut current_counts, value| {
            let raw_split_values = value.split(' ').collect::<Vec<&str>>();

            // Upper bound is exclusive
            let bounds = raw_split_values[0].split('-').collect::<Vec<&str>>();
            let bounds = Range {
                start: bounds[0].parse::<usize>().unwrap(),
                end: bounds[1].parse::<usize>().unwrap() + 1,
            };

            let c: char = String::from(raw_split_values[1]).remove(0);

            let pw = raw_split_values[2];

            let target_char_count = pw.chars().fold(0, |pw_char_count, pw_char| {
                if pw_char == c {
                    pw_char_count + 1
                } else {
                    pw_char_count
                }
            });

            let mut is_criteria_2_valid: bool = false;

            for (i, pw_char) in pw.chars().enumerate() {
                let i = i + 1;
                if i != bounds.start && i != bounds.end - 1 {
                    continue;
                }

                if pw_char == c {
                    is_criteria_2_valid = !is_criteria_2_valid;
                }
            }

            if bounds.contains(&target_char_count) {
                current_counts.0 += 1
            }

            if is_criteria_2_valid {
                current_counts.1 += 1
            }

            current_counts
        });

    println!(
        "Valid passwords count - criteria 1: {}",
        valid_criteria_1_count
    );
    println!(
        "Valid passwords count - criteria 2: {}",
        valid_criteria_2_count
    );
}
