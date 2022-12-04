use std::fs;

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn turn(&mut self, degrees: i32, turn_direction: char) {
        let turn_count = degrees / 90;

        for _ in 0..turn_count {
            match turn_direction {
                'R' => match self {
                    Direction::North => *self = Direction::East,
                    Direction::East => *self = Direction::South,
                    Direction::South => *self = Direction::West,
                    Direction::West => *self = Direction::North,
                },
                'L' => match self {
                    Direction::North => *self = Direction::West,
                    Direction::East => *self = Direction::North,
                    Direction::South => *self = Direction::East,
                    Direction::West => *self = Direction::South,
                },
                _ => {}
            }
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        }
    }
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
}

impl Ship {
    fn move_location(&mut self, instruction: Instruction) {
        match instruction.operation {
            'N' => self.position.y += instruction.amount,
            'S' => self.position.y -= instruction.amount,
            'E' => self.position.x += instruction.amount,
            'W' => self.position.x -= instruction.amount,
            'L' => self
                .direction
                .turn(instruction.amount, instruction.operation),
            'R' => self
                .direction
                .turn(instruction.amount, instruction.operation),
            'F' => self.move_location(Instruction {
                operation: self.direction.to_char(),
                amount: instruction.amount,
            }),
            _ => panic!("Unrecognized Instruction"),
        }
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
    };

    for instruction in instructions {
        print!(
            "Ship facing: {:?} at: {:?} - Executing Instruction: {:?}{:?} :::: ",
            ship.direction, ship.position, instruction.operation, instruction.amount
        );

        ship.move_location(instruction);

        println!("Resulting in: {:?}", ship.position);
    }

    println!("Distance: {}", ship.calculate_distance());
}
