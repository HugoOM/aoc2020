use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let contents = Box::leak(Box::new(contents));

    let contents_ptr = unsafe { Box::from_raw(contents) };

    let contents = contents.split("\n\n").collect::<Vec<&str>>();

    let mut rules = HashMap::<
        usize,
        Box<
            dyn Fn(
                &'static str,
                &HashMap<usize, Box<dyn Fn(&'static str) -> nom::IResult<&str, &str>>>,
            ) -> nom::IResult<&'static str, &'static str>,
        >,
    >::new();

    for rule in contents[0].lines() {
        parsers::parse_rule(rule, &mut rules);
    }

    for rule_i in rules.keys() {
        println!("{:?}", rules.get(rule_i).unwrap()("aabbaa", &rules));
    }

    drop(contents_ptr);
}

#[allow(
    unused_imports,
    dead_code,
    unused_variables,
    unused_must_use,
    unused_mut
)]
pub(crate) mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::{is_a, tag},
        character::complete::{char, digit0, space1},
        multi::fold_many0,
        sequence::{delimited, preceded, terminated},
        IResult,
    };
    use std::collections::HashMap;

    type CClosure =
        Box<dyn Fn(&'static str, &CClosure) -> nom::IResult<&'static str, &'static str>>;

    pub fn parse_rule(
        i: &'static str,
        rules: &mut HashMap<
            usize,
            Box<
                dyn Fn(
                    &'static str,
                    &HashMap<usize, Box<dyn Fn(&'static str) -> nom::IResult<&str, &str>>>,
                ) -> nom::IResult<&'static str, &'static str>,
            >,
        >,
    ) {
        let index: IResult<&str, &str> = terminated(digit0, tag(":"))(i);
        let (i, index) = index.unwrap();
        let index = index.parse::<usize>().unwrap();

        let terms: IResult<&str, Vec<&str>> = fold_many0(
            preceded(space1, alt((tag("|"), digit0))),
            Vec::new(),
            |mut acc, x| {
                acc.push(x);
                acc
            },
        )(i);

        if terms.as_ref().unwrap().1.contains(&"\"") {
            let c = terms.as_ref().unwrap().1[0];

            rules.insert(
                index,
                Box::new(move |i: &'static str, _| delimited(tag("\""), is_a(c), tag("\""))(i)),
            );

            return;
        }

        rules.insert(
            index,
            Box::new(move |i: &'static str, rules| {
                let mut i = i.clone();
                let mut parsers_sets: Vec<Vec<usize>> = Vec::new();
                parsers_sets.push(Vec::<usize>::new());

                for &t in terms.as_ref().unwrap().1.iter() {
                    if t == "|" {
                        parsers_sets.push(Vec::<usize>::new());
                        continue;
                    }

                    parsers_sets
                        .last_mut()
                        .unwrap()
                        .push(t.parse::<usize>().unwrap());
                }

                println!("{:?}", parsers_sets);

                for parser_set in parsers_sets.iter() {
                    let mut r = i.clone();
                    for parser in parser_set.iter() {
                        let res = rules.get(parser).unwrap()(r);

                        if res.is_err() {
                            break;
                        }

                        r = res.unwrap().0;
                    }

                    if r.is_empty() {
                        return Ok(("", ""));
                    }
                }

                Err(nom::Err::Error(nom::error::make_error(
                    "Couldn't Match Sequence",
                    nom::error::ErrorKind::Tag,
                )))
            }),
        );
    }
}
