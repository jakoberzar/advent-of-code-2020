use std::collections::HashMap;
use std::convert::TryInto;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-14.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-14.txt");
#[allow(dead_code)]
const SIMPLE_STAR2_INPUT: &str = include_str!("./../../inputs/simple/day-14-star2.txt");

const MEMORY_BITS: usize = 36;
const MEMORY_MASK: u64 = (1 << MEMORY_BITS) - 1;

fn main() {
    let instructions = parse_input(INPUT);

    // Star 1
    let sum = star1(&instructions);
    println!("Sum of memory values is {}", sum);

    // Star 2
    let sum = star2(&instructions);
    println!("Sum of memory values with version 2 is {}", sum);
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(Instruction::new).collect()
}

fn star1(instructions: &[Instruction]) -> u64 {
    let mut machine = Machine::new();
    for instr in instructions.iter() {
        machine.execute(instr);
    }
    machine.memory_sum()
}

fn star2(instructions: &[Instruction]) -> u64 {
    let mut machine = Machine::new();
    for instr in instructions.iter() {
        machine.execute_version2(instr);
    }
    machine.memory_sum()
}

#[derive(Clone, Debug, Default)]
struct Mask {
    ones: u64,
    zeros: u64,
    floating: Vec<u8>,
}

impl Mask {
    fn new(str_mask: &str) -> Mask {
        let chars_with_idx = Mask::get_chars_with_bit_idx(str_mask);
        let ones = Mask::binary_mask(&chars_with_idx, '1');
        let zeros = Mask::binary_mask(&chars_with_idx, '0');
        let floating = Mask::get_floating(&chars_with_idx);
        Mask {
            ones,
            zeros,
            floating,
        }
    }

    fn get_chars_with_bit_idx(input: &str) -> Vec<(u8, char)> {
        let max_idx = input.len() - 1;
        input
            .chars()
            .enumerate()
            .map(|(idx, letter)| ((max_idx - idx).try_into().unwrap(), letter))
            .collect()
    }

    fn binary_mask(chars_with_idx: &[(u8, char)], letter: char) -> u64 {
        chars_with_idx
            .iter()
            .filter(|(_, ch)| *ch == letter)
            .map(|(idx, _)| 1 << idx)
            .sum()
    }

    fn get_floating(chars_with_idx: &[(u8, char)]) -> Vec<u8> {
        chars_with_idx
            .iter()
            .filter(|&(_, ch)| *ch == 'X')
            .map(|(idx, _)| *idx)
            .collect()
    }

    fn transform_value(&self, value: u64) -> u64 {
        let with_ones = value | self.ones;

        // 0 x 0 -> 0
        // 1 x 0 -> 1
        // 0 x 1 -> 0
        // 1 x 1 -> 0
        with_ones & (MEMORY_MASK ^ self.zeros)
    }

    fn obtain_addresses(&self, address: usize) -> impl Iterator<Item = usize> + '_ {
        let address = self.ones as usize | address;

        let floating_combinations = 1 << self.floating.len();
        (0..floating_combinations).map(move |floating_i| {
            // This closure maps the number (for example, numbers from 0 to 8),
            // to the addresses where floating bits are modified to represent the given numbers
            let mut address = address;
            let mut modifier: usize = floating_i;
            for idx in self.floating.iter().rev() {
                let bit = modifier & 0x1;
                let bit_mask: usize = MEMORY_MASK as usize ^ (1 << idx);
                address = (address & bit_mask) | (bit << idx);
                modifier = modifier >> 1;
            }
            address
        })
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Mask(Mask),
    Write(usize, u64),
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let input = input.trim();
        if input.starts_with("mask = ") {
            Instruction::Mask(Mask::new(input.strip_prefix("mask = ").unwrap()))
        } else if input.starts_with("mem[") {
            let mut parts = input.split(" = ");
            let address: usize = parts
                .next()
                .unwrap() // Got first part, mem[xxx]
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap() // Got number
                .parse()
                .unwrap();
            let value: u64 = parts.next().unwrap().parse().unwrap();
            Instruction::Write(address, value)
        } else {
            panic!("Invalid instruction string!");
        }
    }
}

#[derive(Clone, Debug)]
struct Machine {
    mask: Mask,
    memory: HashMap<usize, u64>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            mask: Mask::default(),
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.mask = mask.clone(),
            Instruction::Write(address, value) => {
                self.memory
                    .insert(*address, self.mask.transform_value(*value));
            }
        }
    }

    fn execute_version2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Mask(mask) => self.mask = mask.clone(),
            Instruction::Write(address, value) => {
                for new_address in self.mask.obtain_addresses(*address) {
                    self.memory.insert(new_address, *value);
                }
            }
        }
    }

    fn memory_sum(&self) -> u64 {
        self.memory.iter().map(|(_, v)| *v).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let instructions = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&instructions), 165);
    }

    #[test]
    fn full_star1() {
        let instructions = parse_input(INPUT);
        assert_eq!(star1(&instructions), 6559449933360);
    }

    #[test]
    fn simple_star2() {
        let instructions = parse_input(SIMPLE_STAR2_INPUT);
        assert_eq!(star2(&instructions), 208);
    }

    #[test]
    fn full_star2() {
        let instructions = parse_input(INPUT);
        assert_eq!(star2(&instructions), 3369767240513);
    }
}
