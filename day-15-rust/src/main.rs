use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryInto,
};

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-15.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-15.txt");

fn main() {
    let starting_numbers = parse_input(INPUT);

    // Star 1
    println!("Number on turn 2020 is {}", star1(&starting_numbers));

    // Star 2
    println!("Number on turn 30000000 is {}", star2(&starting_numbers));
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn star1(starting_numbers: &[usize]) -> u32 {
    play_game(starting_numbers, 2020)
}

fn star2(starting_numbers: &[usize]) -> u32 {
    play_game(starting_numbers, 30000000)
}

// This solution uses a vector to store the numbers.
// The vector gets pretty big, but is faster than a hashmap.
fn play_game(starting_numbers: &[usize], turns: u32) -> u32 {
    let mut number_turns: Vec<Option<u32>> = Vec::new();

    // Number said will never be larger than the amount of turns
    number_turns.resize(turns as usize, None);

    let mut turn = 1;
    for number in starting_numbers.iter() {
        insert_and_get(&mut number_turns, *number, turn);
        turn += 1;
    }

    let mut last_number = starting_numbers.last().unwrap().to_owned();
    while turn <= turns {
        let existing = insert_and_get(&mut number_turns, last_number, turn - 1);
        last_number = match existing {
            Some(x) => ((turn - 1) - x) as usize,
            None => 0,
        };
        turn += 1;
    }

    last_number.try_into().unwrap()
}

fn insert_and_get(number_turns: &mut Vec<Option<u32>>, number: usize, turn: u32) -> Option<u32> {
    let previous = number_turns[number];
    number_turns[number] = Some(turn);
    previous
}

// This solution uses a hashmap.
// It takes 2.7s instead of 0.7s on release, but consumes 70 MB instead of 235 MB.
#[allow(dead_code)]
fn play_game_hash(starting_numbers: &[usize], turns: u32) -> u32 {
    let mut number_turns: HashMap<usize, u32> = HashMap::new();

    let mut turn = 1;
    for number in starting_numbers.iter() {
        number_turns.insert(*number, turn);
        turn += 1;
    }

    let mut last_number = starting_numbers.last().unwrap().to_owned();
    while turn <= turns {
        let entry = number_turns.entry(last_number);
        last_number = match &entry {
            Entry::Occupied(x) => ((turn - 1) - x.get()) as usize,
            Entry::Vacant(_) => 0,
        };
        entry.and_modify(|e| *e = turn - 1).or_insert(turn - 1);
        turn += 1;
    }

    last_number.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let starting_numbers = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&starting_numbers), 436);
    }

    #[test]
    fn full_star1() {
        let starting_numbers = parse_input(INPUT);
        assert_eq!(star1(&starting_numbers), 273);
    }

    #[test]
    fn simple_star2() {
        let starting_numbers = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&starting_numbers), 175594);
    }

    #[test]
    fn full_star2() {
        let starting_numbers = parse_input(INPUT);
        assert_eq!(star2(&starting_numbers), 47205);
    }
}
