use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct ValidityCriteria {
    name: String,
    criterias: Vec<Range<u32>>,
    dataset_column_index: Option<usize>,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut criterias = Vec::<ValidityCriteria>::new();
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();
    let mut valid_tickets: Vec<Vec<u32>> = Vec::new();

    for line_text in contents.lines() {
        if line_text == "" {
            break;
        }

        let mut line = line_text.split(' ').collect::<Vec<&str>>();

        let top_criteria = line.pop().unwrap();
        line.pop();
        let bot_criteria = line.pop().unwrap();

        let mut criteria = ValidityCriteria {
            name: line_text
                .split(':')
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .to_string(),
            criterias: Vec::<Range<u32>>::new(),
            dataset_column_index: None,
        };

        let top_criteria = {
            let parts = top_criteria
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            Range {
                start: parts[0],
                end: parts[1] + 1,
            }
        };
        let bot_criteria = {
            let parts = bot_criteria
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            Range {
                start: parts[0],
                end: parts[1] + 1,
            }
        };

        criteria.criterias.push(bot_criteria);
        criteria.criterias.push(top_criteria);
        criterias.push(criteria);
    }

    for (index, line) in contents.lines().enumerate() {
        if line != "your ticket:" {
            continue;
        }

        my_ticket = contents.lines().collect::<Vec<&str>>()[index + 1]
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
    }

    let mut nearby: bool = false;
    for line in contents.lines() {
        if line != "nearby tickets:" && !nearby {
            continue;
        }

        if line == "nearby tickets:" {
            nearby = true;
            continue;
        }

        nearby_tickets.push(
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut total_error_margin: u32 = 0;

    for ticket in nearby_tickets.iter() {
        let mut is_ticket_valid: bool = true;
        for value in ticket.iter() {
            let mut contains_value: bool = false;
            for criteria in criterias.iter() {
                for crit in criteria.criterias.iter() {
                    if crit.contains(value) {
                        contains_value = true;
                    }
                }
            }

            if !contains_value {
                total_error_margin += value;
                is_ticket_valid = false;
            }
        }

        if is_ticket_valid {
            valid_tickets.push(ticket.to_owned());
        }
    }

    println!(
        "Total Tickets: {}, Valid Tickets: {}",
        nearby_tickets.len(),
        valid_tickets.len()
    );
    println!("Total Error Margin: {}", total_error_margin);

    let mut criteria_suitable_column: HashMap<String, Vec<usize>> = HashMap::new();
    for c in criterias.iter() {
        criteria_suitable_column.insert(c.name.clone(), Vec::new());
    }

    // valid_tickets.push(my_ticket.clone());

    for criteria in criterias.iter_mut() {
        for column in 0..my_ticket.len() {
            let is_match = valid_tickets.iter().all(|x| {
                let x = &x[column];

                criteria.criterias[0].contains(x) || criteria.criterias[1].contains(x)
            });

            if is_match {
                criteria_suitable_column
                    .get_mut(&criteria.name)
                    .unwrap()
                    .push(column);
            }
        }
    }

    let mut utilized_columns = HashSet::<usize>::new();

    for i in 0..my_ticket.len() {
        for (name, potential_columns) in criteria_suitable_column.iter() {
            if potential_columns.len() != i + 1 {
                continue;
            }

            println!("{}", potential_columns.len());
            println!("{:?}", utilized_columns);

            let mut is_found: bool = false;

            for &p in potential_columns {
                if utilized_columns.contains(&p) {
                    continue;
                }

                let mut criteria = criterias.iter_mut().find(|x| &x.name == name).unwrap();

                utilized_columns.insert(p);
                criteria.dataset_column_index = Some(p);
                is_found = true;
                break;
            }

            if is_found {
                break;
            }
        }
    }

    // println!("{:#?}", criterias);
    // println!("{:?}", utilized_columns);

    let mut ticket_tally: u64 = 1;

    for criteria in criterias.iter() {
        if !criteria.name.starts_with("departure") {
            continue;
        }

        println!(
            "{} - {} - {}",
            criteria.name,
            my_ticket[criteria.dataset_column_index.unwrap()],
            ticket_tally
        );

        ticket_tally *= my_ticket[criteria.dataset_column_index.unwrap()] as u64;
    }

    println!("Final ticket tally: {}", ticket_tally);
}
