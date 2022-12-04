use num::bigint::BigInt;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut contents = contents
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    contents.sort();

    println!("{:?}", contents);

    contents.insert(0, 0);
    contents.push(contents.last().unwrap() + 3);

    let mut differences: HashMap<u32, u32> = HashMap::new();
    let mut diff_order: Vec<u32> = Vec::new();

    for (index, adapter) in contents.iter().enumerate() {
        if index == contents.len() - 1 {
            break;
        }

        let difference = contents[index + 1] - adapter;

        diff_order.push(difference);

        if difference != 1 && difference != 3 {
            println!("{}", difference);
        }

        let d = differences.entry(difference).or_insert(0);
        *d += 1;
    }

    println!("{:#?}", differences.values().product::<u32>());
    // println!("{:#?}", differences);
    println!(
        "{:?}",
        diff_order
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &u32)>>()
    );

    // let mut combinations_count: BigInt = BigInt::from(1);
    let mut combinations_count: u128 = 1;
    let mut covered_indexes = Vec::<usize>::new();
    for (i, d) in diff_order.iter().enumerate() {
        if covered_indexes.contains(&i) {
            continue;
        }

        covered_indexes.push(i);

        if *d == 3 {
            continue;
        }

        println!("Considering Index: {}", i);

        if diff_order[i + 1] == 1 {
            covered_indexes.push(i + 1);

            if diff_order[i + 2] == 1 {
                covered_indexes.push(i + 2);

                if diff_order[i + 3] == 1 {
                    covered_indexes.push(i + 3);
                    combinations_count *= 7;
                    println!("{}", combinations_count);
                    continue;
                }

                combinations_count *= 4;
                println!("{}", combinations_count);
                continue;
            }

            combinations_count *= 2;
        };

        println!("{}", combinations_count);
    }

    println!("Combinations Count: {}", combinations_count);
}
