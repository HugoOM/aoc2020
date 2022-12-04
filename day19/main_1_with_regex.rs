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

    let rule0_regex = Regex::new(&format!(
        "^{}$",
        &recurse(rules.get(&0usize).unwrap(), &rules)
    ))
    .unwrap();

    println!("Rule 0 Regex: {:#?}", rule0_regex);

    println!(
        "Valid Count: {}",
        contents[1]
            .lines()
            .filter(|&line| rule0_regex.is_match(line))
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
