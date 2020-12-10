#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-10.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-10.txt");
#[allow(dead_code)]
const SIMPLE_SMALL_INPUT: &str = include_str!("./../../inputs/simple/day-10-small.txt");

fn main() {
    let numbers = parse_input(INPUT);

    // Star 1
    let product = star1(&numbers);
    println!("Product of counts of 1 and 3 differences is {}", product);

    // Star 2
    let combinations = star2(&numbers);
    println!("There are {} combinations of adapters", combinations);
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut input: Vec<u32> = input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    // Adapt input to add start and end
    input.sort();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);
    input
}

fn star1(numbers: &[u32]) -> usize {
    let differences: Vec<u32> = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(x, next)| next - x)
        .collect();

    let (diff1, rest): (Vec<u32>, Vec<u32>) = differences.iter().partition(|d| **d == 1);
    let (diff3, _rest): (Vec<u32>, Vec<u32>) = rest.iter().partition(|d| **d == 3);
    diff1.len() * diff3.len()
}

fn star2(numbers: &[u32]) -> usize {
    let mut comb_counter = CombinationCounter::new(numbers);
    comb_counter.ways_possible(&numbers, 0)
}

struct CombinationCounter {
    cache: Vec<Option<usize>>,
}

impl CombinationCounter {
    fn new(numbers: &[u32]) -> CombinationCounter {
        CombinationCounter {
            cache: vec![None; numbers.len()],
        }
    }

    fn ways_cache(&mut self, numbers: &[u32], full_idx: usize) -> usize {
        if let Some(x) = self.cache[full_idx] {
            return x;
        } else {
            let result = self.ways_possible(numbers, full_idx);
            self.cache[full_idx] = Some(result);
            result
        }
    }

    fn ways_possible(&mut self, numbers: &[u32], full_idx: usize) -> usize {
        let previous = numbers[0];
        let rest = &numbers[1..];
        if rest.is_empty() {
            // End reached; this is one possible combination
            return 1;
        }

        let next_count = rest.iter().take(3).filter(|x| **x <= previous + 3).count();

        if next_count == 0 {
            0 // Cannot reach the next number via this candidate
        } else {
            (0..next_count)
                .map(|idx| self.ways_cache(&rest[idx..], full_idx + 1 + idx))
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_small_star1() {
        let numbers = parse_input(SIMPLE_SMALL_INPUT);
        assert_eq!(star1(&numbers), 7 * 5);
    }

    #[test]
    fn simple_star1() {
        let numbers = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&numbers), 22 * 10);
    }

    #[test]
    fn full_star1() {
        let numbers = parse_input(INPUT);
        assert_eq!(star1(&numbers), 3034);
    }

    #[test]
    fn simple_small_star2() {
        let numbers = parse_input(SIMPLE_SMALL_INPUT);
        assert_eq!(star2(&numbers), 8);
    }

    #[test]
    fn simple_star2() {
        let numbers = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&numbers), 19208);
    }

    #[test]
    fn full_star2() {
        let numbers = parse_input(INPUT);
        assert_eq!(star2(&numbers), 259172170858496);
    }
}
