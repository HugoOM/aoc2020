use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let numbers = contents.split('\n').collect::<Vec<&str>>();

    let mut initial_set: HashSet<i32> = HashSet::new();

    for num in numbers {
        let num = match num.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if initial_set.contains(&num) {
            panic!("Dual-entry!");
        }

        initial_set.insert(num);
    }

    // println!("{:?}", initial_set);

    // for num in initial_set.iter() {
    //     let target = 2020 - num;

    //     if initial_set.contains(&target) {
    //         panic!(
    //             "Found {} on {}, which is the number - multiplied total: {}",
    //             target,
    //             num,
    //             target * num
    //         );
    //     }
    // }

    for i in initial_set.iter() {
        for j in initial_set.iter() {
            let target = 2020 - i - j;

            if initial_set.contains(&target) {
                panic!(
                    "Found {} on {} & {}. Multiplied total: {}",
                    target,
                    i,
                    j,
                    target * i * j
                );
            }
        }
    }
}
