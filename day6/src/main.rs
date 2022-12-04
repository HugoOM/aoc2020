use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents = contents.split("\n\n").collect::<Vec<&str>>();

    let mut answers_count_per_groups: Vec<u32> = Vec::new();

    // for group in contents {
    //     let answers = String::from(group).replace('\n', "");

    //     let mut h: HashSet<char> = HashSet::new();

    //     for c in answers.chars() {
    //         h.insert(c);
    //     }

    //     answers_count_per_groups.push(h.len() as u32);
    // }

    // println!(
    //     "Total answers count for all groups: {}",
    //     answers_count_per_groups.iter().sum::<u32>()
    // );

    for group_answers in contents {
        let per_individual_answers = group_answers.lines().collect::<Vec<&str>>();

        let answers_target_count = per_individual_answers.len();

        let mut all_answered_count = 0;

        let mut collected_answers: HashMap<char, u8> = HashMap::new();

        for answers in per_individual_answers {
            for answer in answers.chars() {
                let count = collected_answers.entry(answer).or_insert(0);
                *count += 1;
            }
        }

        for count in collected_answers.values() {
            if *count == answers_target_count as u8 {
                all_answered_count += 1;
            }
        }

        answers_count_per_groups.push(all_answered_count);
    }

    println!(
        "Total answers count for all groups: {}",
        answers_count_per_groups.iter().sum::<u32>()
    );
}
