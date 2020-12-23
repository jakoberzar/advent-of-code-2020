#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-23.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-23.txt");

mod vector_linked_list;
use vector_linked_list::VectorLinkedList;

fn main() {
    let input = parse_input(INPUT);

    let result1 = star1(&input);
    println!("Result of first star is {}", result1);

    let result2 = star2(&input);
    println!("Result of second star is {}", result2);
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect()
}

fn star1(numbers: &[u32]) -> u64 {
    let mut crab_game = CrabGameFaster::new(numbers);
    for _ in 0..100 {
        crab_game.execute_move();
    }
    crab_game.calculate_order()
}

fn star2(numbers: &[u32]) -> u64 {
    let mut crab_game = CrabGameFaster::new_with_million(numbers);
    for _ in 0..10_000_000 {
        crab_game.execute_move();
    }
    crab_game.calculate_next_two()
}

#[derive(Debug, Clone)]
struct CrabGameFaster {
    numbers: VectorLinkedList,
    current_cup_val: u32,
    max: u32,
}

impl CrabGameFaster {
    fn new(numbers: &[u32]) -> Self {
        CrabGameFaster {
            max: *numbers.iter().max().unwrap(),
            numbers: VectorLinkedList::new(&numbers),
            current_cup_val: numbers[0],
        }
    }

    fn new_with_million(numbers: &[u32]) -> Self {
        let mut numbers = Vec::from(numbers);
        let max = numbers.iter().max().unwrap();
        for i in max + 1..=1_000_000 {
            numbers.push(i);
        }
        CrabGameFaster {
            numbers: VectorLinkedList::new(&numbers),
            current_cup_val: numbers[0],
            max: 1_000_000,
        }
    }

    fn execute_move(&mut self) {
        // Find the three cups
        let taken_out = self.get_three_followers();

        // Select the destination cup
        let destination_cup_val = self.find_destination_cup_val(&taken_out);

        // Move the cups
        self.move_three_followers(destination_cup_val, &taken_out);

        // Determine new current cup position
        self.current_cup_val = self.numbers.follows(self.current_cup_val);
    }

    fn get_three_followers(&self) -> [u32; 3] {
        let mut taken_out = [0; 3];
        taken_out[0] = self.numbers.follows(self.current_cup_val);
        taken_out[1] = self.numbers.follows(taken_out[0]);
        taken_out[2] = self.numbers.follows(taken_out[1]);
        taken_out
    }

    fn find_destination_cup_val(&self, taken_out: &[u32; 3]) -> u32 {
        let mut cup_value = self.current_cup_val - 1;
        while cup_value == 0 || taken_out.contains(&cup_value) {
            if cup_value == 0 {
                cup_value = self.max;
            } else {
                cup_value -= 1;
            }
        }
        cup_value
    }

    fn move_three_followers(&mut self, destination_cup_val: u32, taken_out: &[u32; 3]) {
        // Rewire the current cup
        let new_current_cup_follower = self.numbers.follows(taken_out[2]);
        self.numbers
            .rewire(self.current_cup_val, new_current_cup_follower);
        // Rewire the destination cup and taken out elements
        let destination_cup_follower = self.numbers.follows(destination_cup_val);
        self.numbers.rewire(destination_cup_val, taken_out[0]);
        self.numbers.rewire(taken_out[2], destination_cup_follower);
    }

    fn calculate_order(&self) -> u64 {
        let mut acc = 0;
        let mut current = self.numbers.follows(1);
        for _ in 0..self.numbers.len() - 1 {
            acc = acc * 10 + (current as u64);
            current = self.numbers.follows(current);
        }
        acc
    }

    fn calculate_next_two(&self) -> u64 {
        let first_follower = self.numbers.follows(1);
        let second_follower = self.numbers.follows(first_follower);
        first_follower as u64 * second_follower as u64
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct CrabGame {
    numbers: Vec<u32>,
    current_cup: usize,
    max: u32,
}

#[allow(dead_code)]
impl CrabGame {
    fn new(numbers: &[u32]) -> Self {
        CrabGame {
            max: *numbers.iter().max().unwrap(),
            numbers: Vec::from(numbers),
            current_cup: 0,
        }
    }

    fn new_with_million(numbers: &[u32]) -> Self {
        let mut numbers = Vec::from(numbers);
        let max = numbers.iter().max().unwrap();
        for i in max + 1..=1_000_000 {
            numbers.push(i);
        }
        CrabGame {
            numbers,
            current_cup: 0,
            max: 1_000_000,
        }
    }

    fn execute_move(&mut self) {
        let current_cup_val = self.numbers[self.current_cup];

        // Take out three cups
        let taken_out = self.take_three_out();

        // Select the destination cup
        let destination_cup = self.find_destination_cup(current_cup_val, &taken_out);

        // Insert the cups back
        self.put_three_back(destination_cup, &taken_out);

        // Determine new current cup position
        self.update_current_cup(current_cup_val);
    }

    fn take_three_out(&mut self) -> [u32; 3] {
        let mut taken_out = [0; 3];
        let removing_idx = (self.current_cup + 1) % self.numbers.len();
        for idx in 0..3 {
            if removing_idx < self.numbers.len() {
                taken_out[idx] = self.numbers.remove(removing_idx);
            } else {
                taken_out[idx] = self.numbers.remove(0);
            }
        }
        taken_out
    }

    fn find_destination_cup(&self, current_cup_val: u32, taken_out: &[u32; 3]) -> usize {
        let destination_cup_val = {
            let mut val = current_cup_val - 1;
            while val == 0 || taken_out.contains(&val) {
                if val == 0 {
                    val = self.max;
                } else {
                    val -= 1;
                }
            }
            val
        };

        let destination_cup = self
            .numbers
            .iter()
            .position(|x| *x == destination_cup_val)
            .unwrap();

        destination_cup
    }

    fn put_three_back(&mut self, destination_cup: usize, taken_out: &[u32; 3]) {
        let insertion_point = (destination_cup + 1) % self.numbers.len();
        for x in taken_out.iter().rev() {
            self.numbers.insert(insertion_point, *x);
        }
    }

    fn update_current_cup(&mut self, current_cup_val: u32) {
        let new_current_cup_pos = self
            .numbers
            .iter()
            .position(|x| *x == current_cup_val)
            .unwrap();
        self.current_cup = (new_current_cup_pos + 1) % self.numbers.len();
    }

    fn calculate_order(&self) -> u64 {
        let idx1 = self.numbers.iter().position(|x| *x == 1).unwrap();
        self.numbers
            .iter()
            .skip(idx1 + 1)
            .chain(self.numbers.iter().take(idx1))
            .fold(0, |acc, x| acc * 10 + (*x as u64))
    }

    fn calculate_next_two(&self) -> u64 {
        let idx1 = self.numbers.iter().position(|x| *x == 1).unwrap();
        return self.numbers[idx1 + 1] as u64 * self.numbers[idx1 + 2] as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let numbers = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&numbers), 67384529);
    }

    #[test]
    fn full_star1() {
        let numbers = parse_input(INPUT);
        assert_eq!(star1(&numbers), 68245739);
    }

    #[test]
    fn simple_star2() {
        let numbers = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&numbers), 149245887792);
    }

    #[test]
    fn full_star2() {
        let numbers = parse_input(INPUT);
        assert_eq!(star2(&numbers), 219634632000);
    }
}
