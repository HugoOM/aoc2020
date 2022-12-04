use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut current_mask = Vec::<char>::new();
    // let mut memory: Vec<u64> = vec![0u64; u64::MAX as usize];
    let mut memory: HashMap<usize, u64> = HashMap::new();

    for line in contents.lines() {
        let mut x_positions: Vec<usize> = Vec::new();

        if line.starts_with("mask") {
            current_mask = line
                .split(' ')
                .last()
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
            continue;
        }

        let mem_addr = line.split('[').collect::<Vec<&str>>()[1]
            .split(']')
            .collect::<Vec<&str>>()[0]
            .parse::<usize>()
            .unwrap();

        let value = line
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let mut addr_as_bits = format!("{:0>36b}", mem_addr);

        for (i, &c) in current_mask.iter().enumerate() {
            if c == 'X' {
                x_positions.push(i);
                continue;
            }

            if c == '0' {
                continue;
            }

            let mut char_vec = addr_as_bits.chars().collect::<Vec<char>>();
            char_vec[i] = c;
            addr_as_bits = char_vec.iter().collect::<String>();
        }

        let mut possibilities: HashSet<usize> = HashSet::new();
        for (i, _) in x_positions.iter().enumerate() {
            toggle_x_mask_values(
                x_positions.clone(),
                &mut possibilities,
                addr_as_bits.chars().collect::<Vec<char>>(),
            );
        }

        for mem_addr in possibilities {
            memory.insert(mem_addr, value);
        }
    }

    println!("Total tally: {}", memory.values().sum::<u64>());
}

fn toggle_x_mask_values<'a>(
    x_positions: Vec<usize>,
    possibilities: &'a mut HashSet<usize>,
    address: Vec<char>,
) -> &'a HashSet<usize> {
    let current_pos = *x_positions.first().unwrap();
    let mut address = address.clone();

    if x_positions.len() == 1 {
        address[current_pos] = '0';
        possibilities
            .insert(usize::from_str_radix(address.iter().collect::<String>().as_str(), 2).unwrap());
        address[current_pos] = '1';
        possibilities
            .insert(usize::from_str_radix(address.iter().collect::<String>().as_str(), 2).unwrap());
    } else {
        let mut r_positions = x_positions.clone();
        r_positions.remove(0);
        address[current_pos] = '0';
        toggle_x_mask_values(r_positions.clone(), possibilities, address.clone());
        address[current_pos] = '1';
        toggle_x_mask_values(r_positions.clone(), possibilities, address.clone());
    }

    possibilities
}
