use parse_display::{Display, FromStr};

#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-02.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-02.txt");

fn main() {
    let input = parse_input(INPUT);

    // Star 1
    let correct_count = star1(&input);
    println!("{} passwords are correct for star 1.", correct_count);

    // Star 2
    let correct_count = star2(&input);
    println!("{} passwords are correct for star 2.", correct_count);
}

fn star1(input: &[PasswordReq]) -> usize {
    input
        .iter()
        .filter(|&pass_unit| pass_unit.correct_part1())
        .count()
}

fn star2(input: &[PasswordReq]) -> usize {
    input
        .iter()
        .filter(|&pass_unit| pass_unit.correct_part2())
        .count()
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
        (self.password.chars().nth(self.num1 - 1).unwrap_or('\0') == self.letter)
            ^ (self.password.chars().nth(self.num2 - 1).unwrap_or('\0') == self.letter)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        let input = parse_input(INPUT);
        let correct_count = star1(&input);
        assert_eq!(correct_count, 569);
    }

    #[test]
    fn full_star2() {
        let input = parse_input(INPUT);
        let correct_count = star2(&input);
        assert_eq!(correct_count, 346);
    }
}
