use std::convert::TryFrom;
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
enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Forward(u32),
    Right(u32),
    Left(u32),
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let amount: u32 = line[1..].parse().unwrap();
        let letter = line.chars().next().unwrap();
        match letter {
            'N' => Instruction::North(amount),
            'S' => Instruction::South(amount),
            'E' => Instruction::East(amount),
            'W' => Instruction::West(amount),
            'F' => Instruction::Forward(amount),
            'R' => Instruction::Right(amount),
            'L' => Instruction::Left(amount),
            _ => panic!("Invalid letter!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotated_by(&self, degrees: u32) -> Direction {
        let mut direction = *self;
        for _ in 0..(degrees / 90) {
            direction = direction.rotated_right();
        }
        direction
    }

    fn rotated_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
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
    direction: Direction,
    waypoint: Waypoint,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            direction: Direction::East,
            waypoint: Waypoint { x: 10, y: 1 },
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(amount) => self.y += i32::try_from(*amount).unwrap(),
            Instruction::South(amount) => self.y -= i32::try_from(*amount).unwrap(),
            Instruction::East(amount) => self.x += i32::try_from(*amount).unwrap(),
            Instruction::West(amount) => self.x -= i32::try_from(*amount).unwrap(),
            Instruction::Forward(amount) => self.forward(*amount),
            Instruction::Right(amount) => self.rotate_right(*amount),
            Instruction::Left(amount) => self.rotate_right(360 - *amount),
        }
    }

    fn apply_waypoint(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(amount) => self.waypoint.y += i32::try_from(*amount).unwrap(),
            Instruction::South(amount) => self.waypoint.y -= i32::try_from(*amount).unwrap(),
            Instruction::East(amount) => self.waypoint.x += i32::try_from(*amount).unwrap(),
            Instruction::West(amount) => self.waypoint.x -= i32::try_from(*amount).unwrap(),
            Instruction::Forward(amount) => self.forward_waypoint(*amount),
            Instruction::Right(amount) => self.rotate_right(*amount),
            Instruction::Left(amount) => self.rotate_right(360 - *amount),
        }
    }

    fn manhattan_distance_from_start(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }

    fn forward(&mut self, amount: u32) {
        let amount: i32 = amount.try_into().unwrap();
        match self.direction {
            Direction::East => self.x += amount,
            Direction::West => self.x -= amount,
            Direction::North => self.y += amount,
            Direction::South => self.y -= amount,
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
