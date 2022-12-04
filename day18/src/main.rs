// https://github.com/Geal/nom/blob/master/tests/arithmetic.rs

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let expressions = contents.lines().collect::<Vec<&str>>();
    let mut total = 0i64;

    for expression in expressions.iter() {
        let expression = expression.to_string().replace(" ", "");

        total += parsers::process(&expression).unwrap().1;
    }

    println!("\nTotal: {}\n", total);
}

#[allow(dead_code)]
pub(crate) mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, digit1 as digit},
        combinator::map_res,
        multi::fold_many0,
        sequence::{delimited, preceded},
        IResult,
    };

    static mut DEPTH: u32 = 0;

    fn parens(i: &str) -> IResult<&str, i64> {
        unsafe { DEPTH += 1 };
        println!(
            "{}Calling parens on string: {}",
            get_print_depth_string(),
            i
        );

        let result = delimited(tag("("), multiply, tag(")"))(i);

        println!(
            "{}Result from parens: {:?}",
            get_print_depth_string(),
            result
        );

        unsafe { DEPTH -= 1 };
        result
    }

    fn get_number(i: &str) -> IResult<&str, i64> {
        unsafe { DEPTH += 1 };
        println!(
            "{}Calling get_number on string: {}",
            get_print_depth_string(),
            i
        );

        let result = alt((map_res(digit, std::str::FromStr::from_str), parens))(i);

        println!(
            "{}Result from get_number: {:?}",
            get_print_depth_string(),
            result
        );

        unsafe { DEPTH -= 1 };
        result
    }

    fn multiply(i: &str) -> IResult<&str, i64> {
        unsafe { DEPTH += 1 };
        println!(
            "{}Calling multiply on string: {}",
            get_print_depth_string(),
            i
        );

        println!("{}Multiply initial call: {}", get_print_depth_string(), i);
        let (i, init) = add(i)?;

        println!("{}Multiply folding call: {}", get_print_depth_string(), i);
        unsafe { DEPTH += 1 };
        let result = fold_many0(preceded(char('*'), add), init, |acc, val| {
            let new_total = acc * val;
            println!(
                "{}Rolling total from multiply: {}",
                get_print_depth_string(),
                new_total
            );
            new_total
        })(i);
        unsafe { DEPTH -= 1 };

        println!(
            "{}Result from multiply: {:?}",
            get_print_depth_string(),
            result
        );

        unsafe { DEPTH -= 1 };
        result
    }

    fn add(i: &str) -> IResult<&str, i64> {
        unsafe { DEPTH += 1 };
        println!("{}Calling add on string: {}", get_print_depth_string(), i);

        println!("{}Add initial call: {}", get_print_depth_string(), i);
        let (i, init) = get_number(i)?;

        println!("{}Add folding: {}", get_print_depth_string(), i);
        unsafe { DEPTH += 1 };
        let result = fold_many0(preceded(char('+'), get_number), init, |acc, val| {
            let new_total = acc + val;
            println!(
                "{}Rolling total from add: {}",
                get_print_depth_string(),
                new_total
            );
            new_total
        })(i);
        unsafe { DEPTH -= 1 };

        println!("{}Result from add: {:?}", get_print_depth_string(), result);

        unsafe { DEPTH -= 1 };
        result
    }

    pub fn process(i: &str) -> IResult<&str, i64> {
        println!("\nCalling process on string: {}", i);

        let result = multiply(i);

        println!("Partial Result: {:?}", result);

        result
    }

    fn get_print_depth_string() -> String {
        let mut depth_str = "".to_string();

        for _ in 0..unsafe { DEPTH } {
            depth_str += "  ";
        }

        depth_str
    }
}
