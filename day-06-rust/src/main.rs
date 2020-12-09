use std::collections::HashSet;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-06.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-06.txt");

fn main() {
    let anyone_yes = star1(INPUT);
    println!("Sum of answers where anyone answered yes is {}", anyone_yes);

    let everyone_yes = star2(INPUT);
    println!("Sum of answers everyone answered yes is {}", everyone_yes);
}

fn star1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .flat_map(|person| person.chars())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn star2(input: &str) -> usize {
    let alphabet: HashSet<char> = HashSet::from(('a'..='z').collect());

    input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .fold(alphabet.clone(), |set, person| {
                    let choices: HashSet<char> = person.chars().collect();
                    set.intersection(&choices).copied().collect()
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        let anyone_yes = star1(INPUT);
        assert_eq!(anyone_yes, 6763);
    }

    #[test]
    fn full_star2() {
        let everyone_yes = star2(INPUT);
        assert_eq!(everyone_yes, 3512);
    }
}
