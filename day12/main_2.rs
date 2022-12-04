use std::fs;

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Ship {
    position: Position,
    direction: Direction,
    waypoint: Waypoint,
}

impl Ship {
    fn move_to_waypoint(&mut self, distance: i32) {
        self.position = Position {
            x: self.position.x + self.waypoint.position.x * distance,
            y: self.position.y + self.waypoint.position.y * distance,
        };
    }

    fn calculate_distance(&self) -> u32 {
        self.position.x.abs() as u32 + self.position.y.abs() as u32
    }
}

#[derive(Debug)]
struct Instruction {
    operation: char,
    amount: i32,
}

#[derive(Debug)]
struct Waypoint {
    position: Position,
}

impl Waypoint {
    fn move_waypoint(&mut self, instruction: Instruction) {
        match instruction.operation {
            'N' => self.position.y += instruction.amount,
            'S' => self.position.y -= instruction.amount,
            'E' => self.position.x += instruction.amount,
            'W' => self.position.x -= instruction.amount,
            _ => panic!("Unrecognized Waypoint move operator"),
        }
    }

    fn turn(&mut self, instruction: Instruction) {
        let turn_count = instruction.amount / 90;

        for _ in 0..turn_count {
            match instruction.operation {
                'R' => {
                    self.position = Position {
                        x: self.position.y,
                        y: -self.position.x,
                    }
                }
                'L' => {
                    self.position = Position {
                        x: -self.position.y,
                        y: self.position.x,
                    }
                }
                _ => panic!("Unrecognized Waypoint turn operator"),
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let instructions = contents
        .lines()
        .map(|raw_instruction| {
            let parts = raw_instruction.split_at(1);
            Instruction {
                operation: parts.0.chars().collect::<Vec<char>>()[0],
                amount: parts.1.parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Instruction>>();

    let mut ship = Ship {
        position: Position { x: 0, y: 0 },
        direction: Direction::East,
        waypoint: Waypoint {
            position: Position { x: 10, y: 1 },
        },
    };

    for instruction in instructions {
        match instruction.operation {
            'R' | 'L' => ship.waypoint.turn(instruction),
            'N' | 'W' | 'S' | 'E' => ship.waypoint.move_waypoint(instruction),
            'F' => ship.move_to_waypoint(instruction.amount),
            _ => panic!("Unrecognized operation"),
        }
    }

    println!("Distance: {}", ship.calculate_distance());
}
