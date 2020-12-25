#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-25.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-25.txt");

fn main() {
    let input = parse_input(SIMPLE_INPUT);
    println!("The encryption key is {}", star1(input));
}

fn parse_input(input: &str) -> (u64, u64) {
    let mut lines = input.trim().lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().parse().unwrap(),
    )
}

fn star1(keys: (u64, u64)) -> u64 {
    let (card_key, door_key) = keys;
    let loops_card = find_required_loops(card_key);
    let loops_door = find_required_loops(door_key);
    let card_encryption_key = apply_loops(door_key, loops_card);
    let door_encryption_key = apply_loops(card_key, loops_door);
    assert_eq!(card_encryption_key, door_encryption_key);
    card_encryption_key
}

fn find_required_loops(key: u64) -> usize {
    let subject_number = 7;
    let mut value = 1;
    let mut loops = 0;
    while value != key {
        value = (value * subject_number) % 20201227;
        loops += 1;
    }
    loops
}

fn apply_loops(key: u64, loops: usize) -> u64 {
    let subject_number = key;
    let mut value = 1;
    for _ in 0..loops {
        value = (value * subject_number) % 20201227;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let numbers = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(numbers), 14897079);
    }

    #[test]
    fn full_star1() {
        let numbers = parse_input(INPUT);
        assert_eq!(star1(numbers), 6198540);
    }
}
