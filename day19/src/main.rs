use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub struct Rule {
    rulesets: Vec<Vec<usize>>,
    regex: Regex,
    value: Option<char>,
}
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents = contents.split("\n\n").collect::<Vec<&str>>();
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    for r in contents[0].lines() {
        parsers::parse_rule(r, &mut rules);
    }

    let rule8_regex = Regex::new(&format!(
        "^{}+$",
        &recurse(rules.get(&42usize).unwrap(), &rules)
    ))
    .unwrap();

    let rule31_regex = Regex::new(&format!(
        "{}",
        &recurse(rules.get(&31usize).unwrap(), &rules)
    ))
    .unwrap();

    let rule42_regex = Regex::new(&format!(
        "{}",
        &recurse(rules.get(&42usize).unwrap(), &rules)
    ))
    .unwrap();

    let rule11_regex = Regex::new(&format!(
        "^(?P<r42>{}+)(?P<r31>{}+)$",
        &recurse(rules.get(&42usize).unwrap(), &rules),
        &recurse(rules.get(&31usize).unwrap(), &rules),
    ))
    .unwrap();

    println!(
        "Valid Count: {}",
        contents[1]
            .lines()
            .filter(|&line| {
                // ! Rule 0 is BOTH 8 and 11...
                // if rule8_regex.is_match(line) {
                //     println!("Line {} matched rule 8!", line);
                //     return true;
                // }

                if !rule11_regex.is_match(line) {
                    return false;
                }

                let rule11_capture = rule11_regex
                    .captures_iter(line)
                    .collect::<Vec<regex::Captures>>();

                let rule11_capture = rule11_capture.first().unwrap();

                // println!(
                //     "For line: {} - Rule 42 Match: {:?}",
                //     line,
                //     rule11_capture.name("r42").unwrap().as_str()
                // );
                // println!(
                //     "For line: {} - Rule 31 Match: {:?}",
                //     line,
                //     rule11_capture.name("r31").unwrap().as_str()
                // );

                let r42_count = rule42_regex
                    .find_iter(rule11_capture.name("r42").unwrap().as_str())
                    .count();

                let r31_count = rule31_regex
                    .find_iter(rule11_capture.name("r31").unwrap().as_str())
                    .count();

                // println!("Matches 42: {} - 31: {}", r42_count, r31_count);

                if r42_count >= r31_count + 1 {
                    return true;
                }

                false
            })
            .count()
    );
}

fn recurse(rule: &Rule, rules: &HashMap<usize, Rule>) -> String {
    if let Some(c) = rule.value {
        return c.to_string();
    }

    let rulesets = rule
        .rulesets
        .iter()
        .map(|set| {
            set.iter()
                .map(|rule_index| recurse(rules.get(rule_index).unwrap(), rules))
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    format!("(?:{})", rulesets.join("|"))
}

#[allow(
    unused_imports,
    dead_code,
    unused_variables,
    unused_must_use,
    unused_mut
)]
pub(crate) mod parsers {
    use crate::Rule;
    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag},
        character::complete::{char, digit0, space1},
        multi::fold_many0,
        sequence::{delimited, preceded, terminated},
        IResult,
    };
    use regex::Regex;
    use std::collections::HashMap;

    pub fn parse_rule(i: &str, rules: &mut HashMap<usize, Rule>) {
        let index: IResult<&str, &str> = terminated(digit0, tag(":"))(i);
        let (i, index) = index.unwrap();
        let index = index.parse::<usize>().unwrap();

        rules.insert(
            index,
            Rule {
                value: None,
                rulesets: Vec::new(),
                regex: Regex::new("").unwrap(),
            },
        );
        let v = rules.get_mut(&index).unwrap();

        if i.contains("\"") {
            let c: IResult<&str, &str> = delimited(tag(" \""), is_a("ab"), tag("\""))(i);
            let c = c.unwrap().1.chars().collect::<Vec<char>>();

            v.value = Some(c[0]);

            return;
        }

        v.rulesets.push(Vec::new());

        let terms: IResult<&str, Vec<&str>> = fold_many0(
            preceded(space1, alt((tag("|"), digit0))),
            Vec::new(),
            |mut acc, x| {
                acc.push(x);
                acc
            },
        )(i);

        for &t in terms.unwrap().1.iter() {
            if t == "|" {
                v.rulesets.push(Vec::new());
                continue;
            }

            v.rulesets
                .last_mut()
                .unwrap()
                .push(t.parse::<usize>().unwrap());
        }
    }
}
