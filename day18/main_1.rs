use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let expressions = contents.lines().collect::<Vec<&str>>();
    let mut total = 0u64;

    for expression in expressions.iter() {
        let terms = expression
            .chars()
            .filter(|&x| x != ' ')
            .collect::<Vec<char>>();

        total += tally(&terms[..]).1;
    }

    println!("Total: {}", total);
}

fn tally(terms: &[char]) -> (usize, u64) {
    let mut total = 0;
    let mut current_operator: Option<char> = None;
    let mut skip_to = 0;

    for i in 0..terms.len() {
        if i <= skip_to && i != 0 {
            continue;
        }

        let c = terms[i];
        match c {
            '+' => current_operator = Some('+'),
            '*' => current_operator = Some('*'),
            '(' => match current_operator {
                Some('+') => {
                    let (skip, t) = tally(&terms[i + 1..]);
                    current_operator = None;
                    total += t;
                    skip_to = skip + i + 1;
                }
                Some('*') => {
                    let (skip, t) = tally(&terms[i + 1..]);
                    current_operator = None;
                    total *= t;
                    skip_to = skip + i + 1;
                }
                None => {
                    let (skip, t) = tally(&terms[i + 1..]);
                    total = t;
                    skip_to = skip + i + 1;
                }
                Some(_) => panic!(""),
            },
            ')' => return (i, total),
            _ => match current_operator {
                Some('+') => {
                    current_operator = None;
                    total += c.to_digit(10).unwrap() as u64;
                }
                Some('*') => {
                    current_operator = None;
                    total *= c.to_digit(10).unwrap() as u64;
                }
                None => total = c.to_digit(10).unwrap() as u64,
                _ => panic!(""),
            },
        }
    }

    (terms.len(), total)
}
