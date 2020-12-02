use std::error::Error;

#[allow(dead_code)]
const SAMPLE_INPUT: &str = include_str!("./../sample_input.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let input = parse_input(INPUT);
    star1(&input);
    star2(&input);
}

fn star1(input: &[PasswordUnit]) {
    let correct_count = input
        .iter()
        .filter(|&pass_unit| pass_unit.correct_part1())
        .count();
    println!("{} passwords are correct.", correct_count);
    assert_eq!(correct_count, 569);
}

fn star2(input: &[PasswordUnit]) {
    let correct_count = input
        .iter()
        .filter(|&pass_unit| pass_unit.correct_part2())
        .count();
    println!("{} passwords are correct.", correct_count);
    assert_eq!(correct_count, 346);
}

// Password policy combined with the actual password
#[derive(Debug)]
struct PasswordUnit<'a> {
    num1: usize,
    num2: usize,
    letter: char,
    password: &'a str,
}

impl<'a> PasswordUnit<'a> {
    fn correct_part1(&self) -> bool {
        let count = self.password.chars().filter(|x| *x == self.letter).count();

        self.num1 <= count && count <= self.num2
    }

    fn correct_part2(&self) -> bool {
        let position1 = self.num1;
        let position2 = self.num2;

        (self.password.chars().nth(position1 - 1).unwrap_or('\0') == self.letter)
            ^ (self.password.chars().nth(position2 - 1).unwrap_or('\0') == self.letter)
    }
}

fn parse_input(input: &str) -> Vec<PasswordUnit> {
    input
        .trim()
        .lines()
        .map(|line| parse_line(line).expect(&format!("Error while parsing line {}", line)))
        .collect()
}

fn parse_line(line: &str) -> Result<PasswordUnit, Box<dyn Error>> {
    let mut split_parts = line.trim().split(&['-', ' ', ':'][..]);

    if split_parts.clone().count() < 5 {
        panic!("Invalid formatting in line {}", line);
    }

    let num1: usize = split_parts.next().unwrap().parse()?;
    let num2: usize = split_parts.next().unwrap().parse()?;
    let letter = split_parts.next().unwrap().chars().next().unwrap();
    // Split one empty space between semicolon and password
    split_parts.next();
    let password = split_parts.next().unwrap();

    Ok(PasswordUnit {
        num1,
        num2,
        letter,
        password,
    })
}
