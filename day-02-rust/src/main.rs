use parse_display::{Display, FromStr};

#[allow(dead_code)]
const SAMPLE_INPUT: &str = include_str!("./../sample_input.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("./../input.txt");

fn main() {
    let input = parse_input(INPUT);
    star1(&input);
    star2(&input);
}

fn star1(input: &[PasswordReq]) {
    let correct_count = input
        .iter()
        .filter(|&pass_unit| pass_unit.correct_part1())
        .count();
    println!("{} passwords are correct.", correct_count);
    assert_eq!(correct_count, 569);
}

fn star2(input: &[PasswordReq]) {
    let correct_count = input
        .iter()
        .filter(|&pass_unit| pass_unit.correct_part2())
        .count();
    println!("{} passwords are correct.", correct_count);
    assert_eq!(correct_count, 346);
}

// Password policy combined with the actual password
#[derive(Debug, Display, FromStr)]
#[display("{num1}-{num2} {letter}: {password}")]
struct PasswordReq {
    num1: usize,
    num2: usize,
    letter: char,
    password: String,
}

impl PasswordReq {
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

fn parse_input(input: &str) -> Vec<PasswordReq> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("Error while parsing line {}", line))
        })
        .collect()
}
