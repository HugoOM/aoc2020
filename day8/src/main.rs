use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug)]
struct State {
    index: usize,
    accumulator_value: i32,
    instruction: String,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let instructions = contents
        .lines()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    // let mut executed_instructions = HashSet::<usize>::new();
    // let mut attempted_fixes = Vec::<usize>::new();
    // let mut stack = Vec::<State>::new();
    // let mut processed_index: usize = 0;

    for (index, instruction) in instructions.iter().enumerate() {
        if instruction.contains("acc") {
            continue;
        }

        let modified_instruction = if instruction.contains("nop") {
            instruction.replace("nop", "jmp")
        } else {
            instruction.replace("jmp", "nop")
        };

        if let Ok(r) = process(modified_instruction, index, &instructions) {
            println!("{}", r);
        }
    }

    // stack.push(State {
    //     accumulator_value: 0 as i32,
    //     index: 0,
    //     instruction: instructions[0].clone(),
    // });

    // let final_state = loop {
    //     let (next_index, acc_val) = parse_and_execute_instruction(
    //         &stack.last().unwrap().instruction,
    //         stack.last().unwrap().index,
    //         stack.last().unwrap().accumulator_value,
    //     );

    //     if instructions[stack.last().unwrap().index] != stack.last().unwrap().instruction {
    //         println!(
    //             "Trying modified logic at index: {} -- From: {} to {}",
    //             stack.last().unwrap().index,
    //             instructions[stack.last().unwrap().index],
    //             stack.last().unwrap().instruction
    //         );
    //     }

    //     if next_index == instructions.len() {
    //         break stack.last().unwrap();
    //     }

    //     if executed_instructions.contains(&next_index) {
    //         let mut temp_acc: i32 = 0;

    //         while instructions[stack.last().unwrap().index].contains("acc")
    //             || attempted_fixes.contains(&stack.last().unwrap().index)
    //         {
    //             println!("Popping from stack: {:?}", stack.last().unwrap());
    //             executed_instructions.remove(&stack.last().unwrap().index);
    //             temp_acc = stack.pop().unwrap().accumulator_value;
    //         }

    //         let mut next_state = stack.pop().unwrap();

    //         attempted_fixes.push(next_state.index);
    //         executed_instructions.remove(&next_state.index);

    //         next_state.index = stack.last().unwrap().index;

    //         next_state.instruction = if next_state.instruction.contains("jmp") {
    //             next_state.instruction.replace("jmp", "nop").clone()
    //         } else {
    //             next_state.instruction.replace("nop", "jmp").clone()
    //         };

    //         // next_state.accumulator_value = temp_acc;

    //         stack.push(next_state);
    //     } else {
    //         stack.push(State {
    //             accumulator_value: acc_val,
    //             index: next_index,
    //             instruction: instructions[next_index].clone(),
    //         });

    //         executed_instructions.insert(next_index);
    //     }
    // };

    // println!(
    //     "Final Accumulator Tally with Fix: {}",
    //     final_state.accumulator_value
    // );
}

fn process(
    replacement_instruction: String,
    replacement_index: usize,
    instructions: &Vec<String>,
) -> Result<i32, String> {
    let mut accumulator: i32 = 0;
    let mut called_instructions: Vec<usize> = Vec::new();
    let mut current_index: usize = 0;

    loop {
        if current_index == instructions.len() {
            return Ok(accumulator);
        }

        let (next_index, new_total) = parse_and_execute_instruction(
            if current_index != replacement_index {
                &instructions[current_index]
            } else {
                &replacement_instruction
            },
            current_index,
            accumulator,
        );

        if called_instructions.contains(&next_index) {
            return Err("Looping:!".to_string());
        }

        accumulator = new_total;

        called_instructions.push(current_index);

        current_index = next_index;
    }
}

fn parse_and_execute_instruction(instruction: &str, index: usize, acc: i32) -> (usize, i32) {
    let parsed_instruction = instruction.trim().split(' ').collect::<Vec<&str>>();

    let (op_type, amount) = (
        parsed_instruction[0],
        parsed_instruction[1].parse::<i32>().unwrap(),
    );

    match op_type {
        "nop" => (index + 1, acc),
        "acc" => (index + 1, acc + amount),
        "jmp" => {
            if amount > 0 {
                (index + amount as usize, acc)
            } else {
                (index - amount.abs() as usize, acc)
            }
        }
        _ => (index, acc),
    }
}

// executed_instructions.insert(instruction.index);

// let repeated_instruction = loop {
//     executed_instructions.insert(current_index);

//     let next_index = parse_and_execute_instruction(&instructions[current_index], current_index);

//     if executed_instructions.contains(&next_index) {
//         break next_index;
//     }

//     current_index = next_index;
// };

// println!("Repeated Instruction Index: {}", repeated_instruction);
// println!("Accumulator Value at Break: {}", unsafe { ACCUMULATOR });
