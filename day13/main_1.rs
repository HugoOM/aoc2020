use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents = contents.lines().collect::<Vec<&str>>();

    let departure_time = contents[0].parse::<u32>().unwrap();
    let bus_schedules = contents[1]
        .split(',')
        .filter(|&v| v != "x")
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut current_time = departure_time;

    let bus_id = loop {
        if let Some(bus_id) = bus_schedules.iter().find(|&&s| current_time % s == 0) {
            break bus_id;
        } else {
            current_time += 1;
        }
    };

    println!("{}", bus_id * (current_time - departure_time));
}
