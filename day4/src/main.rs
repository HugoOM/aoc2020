use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content = content.split("\n\n").collect::<Vec<&str>>();

    let required_attributes = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_passports_count = 0;

    content.iter().for_each(|v| {
        let mut missing_properties = required_attributes.len();

        let attributes = v
            .split(|c| c == ' ' || c == ':' || c == '\n')
            .collect::<Vec<&str>>();

        for (index, attribute) in attributes.iter().enumerate() {
            match *attribute {
                "byr" => {
                    let byr = attributes[index + 1].parse::<u32>().unwrap();

                    if byr >= 1920 && byr <= 2002 {
                        missing_properties -= 1;
                    }
                }
                "iyr" => {
                    let iyr = attributes[index + 1].parse::<u32>().unwrap();

                    if iyr >= 2010 && iyr <= 2020 {
                        missing_properties -= 1;
                    }
                }
                "eyr" => {
                    let eyr = attributes[index + 1].parse::<u32>().unwrap();

                    if eyr >= 2020 && eyr <= 2030 {
                        missing_properties -= 1;
                    }
                }
                "hgt" => {
                    let mut string_attrib = String::from(attributes[index + 1]);
                    if let Some(_) = string_attrib.find("in") {
                        string_attrib.truncate(string_attrib.len() - 2);
                        
                        let number_attrib = string_attrib.parse::<u32>().unwrap();
                        
                        if number_attrib >= 59 && number_attrib <= 76 {
                            missing_properties -= 1;
                        }
                    } else if let Some(_) = string_attrib.find("cm") {
                        string_attrib.truncate(string_attrib.len() - 2);

                        let number_attrib = string_attrib.parse::<u32>().unwrap();

                        if number_attrib >= 150 && number_attrib <= 193 {
                            missing_properties -= 1;
                        }
                    } else {
                        break;
                    }
                }
                "hcl" => {
                    let mut hcl = String::from(attributes[index + 1]);
                    let valid_chars = [
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
                        'f',
                    ];

                    if hcl.remove(0) != '#' {
                        break;
                    }

                    let mut is_all_hex_valid = true;

                    for c in hcl.chars() {
                        if !valid_chars.contains(&c) {
                            is_all_hex_valid = false;
                        }
                    }

                    if is_all_hex_valid {
                        missing_properties -= 1;
                    }
                }
                "ecl" => {
                    let mut ecl = attributes[index + 1];

                    let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

                    if valid_eye_colors.contains(&ecl) {
                        missing_properties -= 1;
                    }
                }
                "pid" => {
                    if let Ok(pid) = attributes[index + 1].parse::<u64>() {
                        if attributes[index + 1].chars().count() == 9 {
                            missing_properties -= 1;
                        }
                    } else {
                        break;
                    }
                }
                _ => {}
            }
        }

        if missing_properties == 0 {
            valid_passports_count += 1;
        }
    });

    println!("Valid Passports: {}", valid_passports_count);
}
