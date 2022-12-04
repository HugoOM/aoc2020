use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut current_mask = Vec::<char>::new();
    let mut memory: Vec<u64> = vec![0u64; 100000];

    for line in contents.lines() {
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

        let mut value_as_bits = format!("{:0>36b}", value);

        for (i, &c) in current_mask.iter().enumerate() {
            if c == 'X' {
                continue;
            }

            let mut char_vec = value_as_bits.chars().collect::<Vec<char>>();
            char_vec[i] = c;
            value_as_bits = char_vec.iter().collect::<String>();
        }

        memory[mem_addr] = u64::from_str_radix(&value_as_bits, 2).unwrap();
    }

    println!("Total tally: {}", memory.iter().sum::<u64>());
}
