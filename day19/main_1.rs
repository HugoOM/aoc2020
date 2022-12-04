use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
pub struct Rule {
    rulesets: Vec<Vec<usize>>,
    value: Option<char>,
}
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents = contents.split("\n\n").collect::<Vec<&str>>();
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut valid: HashSet<String> = HashSet::new();

    for r in contents[0].lines() {
        parsers::parse_rule(r, &mut rules)
    }

    recurse(
        rules.get(&0).unwrap().rulesets[0].clone(),
        &mut String::from(""),
        &rules,
        &mut valid,
    );

    println!(
        "Total: {}",
        contents[1].lines().filter(|&x| valid.contains(x)).count()
    );
}

fn recurse(
    mut steps: Vec<usize>,
    s: &mut String,
    rules: &HashMap<usize, Rule>,
    valid: &mut HashSet<String>,
) {
    if steps.is_empty() {
        valid.insert(s.clone());
        return;
    }

    let current_rule = rules.get(steps.first().unwrap()).unwrap();

    steps.remove(0);

    if let Some(v) = current_rule.value {
        s.push(v);
        recurse(steps, s, rules, valid);
    } else {
        for ruleset in current_rule.rulesets.iter() {
            let mut s = s.clone();
            let mut steps = steps.clone();

            for &rule in ruleset.iter().rev() {
                steps.insert(0, rule);
            }

            recurse(steps, &mut s, rules, valid);
        }
    }
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
