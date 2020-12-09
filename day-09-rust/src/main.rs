use std::cmp::min;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-09.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-09.txt");
const AMOUNT: usize = 25;

fn main() {
    let numbers = parse_input(INPUT);

    // Star 1
    let invalid = find_invalid_buffer(&numbers, AMOUNT);
    println!("Invalid number is {}", invalid);
    assert_eq!(556543474, *invalid);

    // Star 2
    let (idx_low, idx_high) = find_zone(&numbers, *invalid);
    let range = numbers[idx_low..=idx_high].iter();
    let (min, max) = (range.clone().min().unwrap(), range.clone().max().unwrap());
    let zone_sum = min + max;
    println!("Zone min is {}, max is {}, sum is {}", min, max, zone_sum);
    assert_eq!(76096372, zone_sum);
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn find_invalid(numbers: &[u64], amount: usize) -> &u64 {
    numbers
        .iter()
        .enumerate()
        .skip(amount)
        .find(|(idx, &x)| !contained(x, &numbers[idx - amount..*idx]))
        .expect("No invalid number found!")
        .1
}

#[allow(dead_code)]
fn contained(sum: u64, range: &[u64]) -> bool {
    for x in range {
        for y in range {
            if x + y == sum {
                return true;
            }
        }
    }
    false
}

fn find_zone(numbers: &[u64], target: u64) -> (usize, usize) {
    let mut idx_low = 0;
    let mut idx_high = 0;
    let mut sum = numbers[0];
    while sum != target {
        if sum < target {
            idx_high += 1;
            sum += numbers[idx_high];
        } else if sum > target {
            sum -= numbers[idx_low];
            idx_low += 1;
        }
    }
    (idx_low, idx_high)
}

// Smarter solution, that uses a circular buffer for calculating sum size.
// However, it turns out its not really any faster.
// Probably, because the input is too small.
fn find_invalid_buffer(numbers: &[u64], amount: usize) -> &u64 {
    let mut buffer = SumBuffer::new(numbers);
    for x in numbers[amount..numbers.len()].iter() {
        if !buffer.inside(*x) {
            return x;
        }
        buffer.compute_next_line();
    }
    panic!("No number found!");
}

const SUM_TABLE_SIZE: usize = AMOUNT * AMOUNT;
struct SumBuffer<'a> {
    origin: &'a [u64],
    sums: [u64; SUM_TABLE_SIZE],
    idx: usize,
}

impl SumBuffer<'_> {
    fn new(origin: &[u64]) -> SumBuffer {
        let mut buffer = SumBuffer {
            origin,
            sums: [0; SUM_TABLE_SIZE],
            idx: 0,
        };

        for _idx in 0..AMOUNT {
            buffer.compute_next_line();
        }

        buffer
    }

    fn inside(&self, value: u64) -> bool {
        let mut row_offset = self.idx * AMOUNT % SUM_TABLE_SIZE;
        for row in 0..AMOUNT {
            let max_col = AMOUNT - row;
            if self.sums[row_offset..row_offset + max_col].contains(&value) {
                return true;
            }
            row_offset = (row_offset + AMOUNT) % SUM_TABLE_SIZE;
        }
        false
    }

    fn compute_next_line(&mut self) {
        let offset = self.idx * AMOUNT % SUM_TABLE_SIZE;
        let left = self.origin.len() - self.idx;
        let val1 = self.origin[self.idx];
        for _col in 0..min(left, AMOUNT) {
            let sum = val1 + self.origin[self.idx + _col];
            self.sums[offset + _col] = sum;
        }
        self.idx += 1;
    }
}
