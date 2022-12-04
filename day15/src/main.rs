use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let base_numbers: Vec<usize> = contents
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut numbers_last_called: HashMap<usize, usize> = HashMap::new();
    let mut called_stack: Vec<usize> = Vec::with_capacity(300000);

    for (index, &number) in base_numbers.iter().enumerate() {
        called_stack.push(number);

        if index == base_numbers.len() - 1 {
            continue;
        }

        numbers_last_called.insert(number, index);
    }

    for i in base_numbers.len()..30000000 {
        let &process_number = called_stack.last().unwrap();

        if numbers_last_called.contains_key(&process_number) {
            let last_called = numbers_last_called.get_mut(&process_number).unwrap();
            called_stack.push((i - 1) - *last_called);
            *last_called = i - 1;
        } else {
            called_stack.push(0);
            numbers_last_called.insert(process_number, i - 1);
        }
    }

    println!("Last number called: {}", called_stack.last().unwrap());
}
