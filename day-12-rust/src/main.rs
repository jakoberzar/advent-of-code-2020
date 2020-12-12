use std::convert::TryInto;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-12.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-12.txt");

fn main() {
    let input = parse_input(INPUT);

    // Star 1
    let distance = star1(&input);
    println!("The distance between start and end is {}", distance);

    // Star 2
    let distance = star2(&input);
    println!(
        "The distance between start and end using waypoint is {}",
        distance
    );
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| Instruction::new(&line.trim()))
        .collect()
}

fn star1(instructions: &[Instruction]) -> u32 {
    let mut position = Position::new();
    for instr in instructions {
        position.apply(instr);
    }
    position.manhattan_distance_from_start()
}

fn star2(instructions: &[Instruction]) -> u32 {
    let mut position = Position::new();
    for instr in instructions {
        position.apply_waypoint(instr);
    }
    position.manhattan_distance_from_start()
}

#[derive(Copy, Clone, Debug)]
enum SkyDirection {
    North,
    East,
    South,
    West,
}

impl SkyDirection {
    fn rotated_right(&self) -> SkyDirection {
        match self {
            SkyDirection::North => SkyDirection::East,
            SkyDirection::East => SkyDirection::South,
            SkyDirection::South => SkyDirection::West,
            SkyDirection::West => SkyDirection::North,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum MoveDirection {
    Forward,
    Left,
    Right,
}

#[derive(Clone, Debug)]
enum InstructionDirection {
    Sky(SkyDirection),
    Move(MoveDirection),
}

impl InstructionDirection {
    fn new(letter: char) -> InstructionDirection {
        use InstructionDirection::{Move, Sky};
        match letter {
            'N' => Sky(SkyDirection::North),
            'E' => Sky(SkyDirection::East),
            'S' => Sky(SkyDirection::South),
            'W' => Sky(SkyDirection::West),
            'F' => Move(MoveDirection::Forward),
            'L' => Move(MoveDirection::Left),
            'R' => Move(MoveDirection::Right),
            _ => panic!("Invalid letter!"),
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    direction: InstructionDirection,
    amount: u32,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let amount: u32 = line[1..].parse().unwrap();
        let letter = line.chars().next().unwrap();

        Instruction {
            direction: InstructionDirection::new(letter),
            amount,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
    direction: SkyDirection,
    waypoint: Waypoint,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            direction: SkyDirection::East,
            waypoint: Waypoint { x: 10, y: 1 },
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        let amount: u32 = instruction.amount;
        let amount_signed: i32 = amount.try_into().unwrap();

        use InstructionDirection::{Move, Sky};
        match instruction.direction {
            Sky(SkyDirection::North) => self.y += amount_signed,
            Sky(SkyDirection::South) => self.y -= amount_signed,
            Sky(SkyDirection::East) => self.x += amount_signed,
            Sky(SkyDirection::West) => self.x -= amount_signed,
            Move(MoveDirection::Forward) => self.forward(amount),
            Move(MoveDirection::Right) => self.rotate_right(amount),
            Move(MoveDirection::Left) => self.rotate_right(360 - amount),
        }
    }

    fn apply_waypoint(&mut self, instruction: &Instruction) {
        let amount: u32 = instruction.amount;
        let amount_signed: i32 = amount.try_into().unwrap();

        use InstructionDirection::{Move, Sky};
        match instruction.direction {
            Sky(SkyDirection::North) => self.waypoint.y += amount_signed,
            Sky(SkyDirection::South) => self.waypoint.y -= amount_signed,
            Sky(SkyDirection::East) => self.waypoint.x += amount_signed,
            Sky(SkyDirection::West) => self.waypoint.x -= amount_signed,
            Move(MoveDirection::Forward) => self.forward_waypoint(amount),
            Move(MoveDirection::Right) => self.rotate_right(amount),
            Move(MoveDirection::Left) => self.rotate_right(360 - amount),
        }
    }

    fn manhattan_distance_from_start(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }

    fn forward(&mut self, amount: u32) {
        let amount: i32 = amount.try_into().unwrap();
        match self.direction {
            SkyDirection::East => self.x += amount,
            SkyDirection::West => self.x -= amount,
            SkyDirection::North => self.y += amount,
            SkyDirection::South => self.y -= amount,
        }
    }

    fn forward_waypoint(&mut self, amount: u32) {
        let amount: i32 = amount.try_into().unwrap();
        self.x += self.waypoint.x * amount;
        self.y += self.waypoint.y * amount;
    }

    fn rotate_right(&mut self, degrees: u32) {
        for _ in 0..(degrees / 90) {
            self.direction = self.direction.rotated_right();
            // Waypoint is always in the quadrant that is left of the direction axis
            // It always rotates the same way: (10, 4) -> (4, -10)
            self.waypoint = Waypoint {
                x: self.waypoint.y,
                y: -self.waypoint.x,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let input = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&input), 25);
    }

    #[test]
    fn full_star1() {
        let input = parse_input(INPUT);
        assert_eq!(star1(&input), 1007);
    }

    #[test]
    fn simple_star2() {
        let input = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&input), 286);
    }

    #[test]
    fn full_star2() {
        let input = parse_input(INPUT);
        assert_eq!(star2(&input), 41212);
    }
}
