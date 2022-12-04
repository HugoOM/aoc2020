use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Bag {
    name: String,
    contents: HashMap<String, u32>,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut bags: HashMap<String, Bag> = HashMap::new();

    for bag_definition in contents.lines() {
        let bag_definition = bag_definition.split("contain").collect::<Vec<&str>>();

        let name = bag_definition[0].trim().replace("bags", "bag");
        let name = name.to_string();

        let mut bag = Bag {
            name: name.clone(),
            contents: HashMap::<String, u32>::new(),
        };

        let bag_contents_definition = bag_definition[1].split(',').collect::<Vec<&str>>();

        if bag_contents_definition[0].contains("no other bag") {
            bags.insert(name.clone(), bag);
            continue;
        }

        for contents_definition in bag_contents_definition.iter() {
            let information = contents_definition.trim().split(' ').collect::<Vec<&str>>();

            bag.contents.insert(
                format!(
                    "{} {} {}",
                    information[1],
                    information[2],
                    information[3]
                        .to_string()
                        .replace("bags", "bag")
                        .replace('.', "")
                        .as_str()
                ),
                information[0].parse::<u32>().unwrap(),
            );
        }

        bags.insert(name, bag);
    }

    let mut bags_that_can_hold_shiny_golden = 0;

    for bag in bags.values() {
        if check_contents(&bag, &bags) {
            bags_that_can_hold_shiny_golden += 1;
        }
    }

    println!(
        "Bags that can hold at least one shiny gold bag: {}",
        bags_that_can_hold_shiny_golden
    );

    println!(
        "Bags that are contained in a single shiny gold bag: {}",
        get_contents_count(bags.get(&String::from("shiny gold bag")).unwrap(), &bags)
    );
}

fn check_contents(b: &Bag, bags: &HashMap<String, Bag>) -> bool {
    if b.contents
        .keys()
        .collect::<Vec<&String>>()
        .contains(&&String::from("shiny gold bag"))
    {
        return true;
    }

    for bag_name in b.contents.keys() {
        let bag = bags.get(bag_name).unwrap();

        if check_contents(bag, bags) {
            return true;
        }
    }

    false
}

fn get_contents_count(b: &Bag, bags: &HashMap<String, Bag>) -> u32 {
    let mut count = b.contents.values().sum::<u32>();

    for (bag_name, bag_count) in b.contents.iter() {
        let bag = bags.get(bag_name).unwrap();

        count += bag_count * get_contents_count(bag, bags);
    }

    count
}
