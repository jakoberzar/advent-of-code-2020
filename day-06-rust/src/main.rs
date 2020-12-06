use std::collections::HashSet;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-06.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-06.txt");

fn main() {
    star1(INPUT);
    star2(INPUT);
}

fn star1(input: &str) {
    let sum: usize = input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .flat_map(|person| person.chars())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();

    println!("Sum of answers where anyone answered yes is {}", sum);
}

fn star2(input: &str) {
    let alphabet: HashSet<char> = HashSet::from(('a'..='z').collect());
    let sum: usize = input
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
        .sum();

    println!("Sum of answers everyone answered yes is {}", sum);
}
