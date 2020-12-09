use std::convert::TryFrom;
use std::convert::TryInto;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-08.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-08.txt");

fn main() {
    let instrs = parse_input(INPUT);

    // Star 1
    let acc = star1(&instrs);
    println!("Value of accumulator before second execution is {}", acc);

    let acc = star2(&instrs);
    println!("Value of accumulator after termination is {}", acc);
}

fn parse_input(input: &str) -> Vec<Instr> {
    input.trim().lines().map(Instr::from).collect()
}

fn star1(instrs: &Vec<Instr>) -> i32 {
    let mut instr_keeper = InstrKeeper::new(instrs);
    let mut regs = Regs::new();
    loop {
        if instr_keeper.terminated(&regs) {
            panic!("Didn't encounter an infinite loop!");
        }

        if instr_keeper.already_visited(&regs) {
            break;
        }

        regs = instr_keeper.visit(&regs).execute_on(regs);
    }

    regs.acc
}

fn star2(instrs: &Vec<Instr>) -> i32 {
    let mut instr_keeper = InstrKeeper::new(instrs);
    find_fix(&mut instr_keeper, Regs::new(), false).unwrap()
}

fn find_fix(instr_keeper: &mut InstrKeeper, regs: Regs, already_modified: bool) -> Option<i32> {
    if instr_keeper.terminated(&regs) {
        return Some(regs.acc);
    }

    if instr_keeper.already_visited(&regs) {
        return None;
    }

    let instr = instr_keeper.visit(&regs);

    // See what happens if we don't change anything
    let new_regs = instr.execute_on(regs);
    let result = find_fix(instr_keeper, new_regs, already_modified);
    if result.is_some() {
        return result;
    }

    // Try with modification
    let new_instr = instr.swap();
    if !already_modified && new_instr.is_some() {
        let new_regs = new_instr.unwrap().execute_on(regs);
        let result = find_fix(instr_keeper, new_regs, true);
        if result.is_some() {
            return result;
        }
    }

    // Reset and return
    instr_keeper.reset_visit(&regs);
    None
}

#[derive(Debug, Copy, Clone)]
struct Regs {
    acc: i32,
    pc: usize,
}

impl Regs {
    fn new() -> Regs {
        Regs { acc: 0, pc: 0 }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Instr {
    fn from(line: &str) -> Instr {
        let mut split = line.trim().split(' ');
        if split.clone().count() < 2 {
            panic!("Invalid line in input!");
        }
        let mnemonic = split.next().unwrap();
        let operand: i32 = split.next().unwrap().parse().unwrap();
        match mnemonic {
            "nop" => Instr::NOP(operand),
            "acc" => Instr::ACC(operand),
            "jmp" => Instr::JMP(operand),
            _ => panic!("Invalid instruction!"),
        }
    }

    fn swap(&self) -> Option<Instr> {
        match self {
            Instr::NOP(x) => Some(Instr::JMP(*x)),
            Instr::JMP(x) => Some(Instr::NOP(*x)),
            _ => None,
        }
    }

    fn execute_on(&self, registers: Regs) -> Regs {
        let Regs { acc, pc } = registers;

        let (new_acc, new_pc) = match self {
            Instr::NOP(_) => (acc, pc + 1),
            Instr::ACC(x) => (acc + *x, pc + 1),
            Instr::JMP(x) => (acc, (i32::try_from(pc).unwrap() + *x).try_into().unwrap()),
        };

        Regs {
            acc: new_acc,
            pc: new_pc,
        }
    }
}

struct InstrKeeper<'a> {
    instrs: &'a Vec<Instr>,
    visited: Vec<bool>,
}

impl InstrKeeper<'_> {
    fn new(instrs: &Vec<Instr>) -> InstrKeeper {
        InstrKeeper {
            instrs: instrs,
            visited: vec![false; instrs.len()],
        }
    }

    fn terminated(&self, regs: &Regs) -> bool {
        regs.pc >= self.instrs.len()
    }

    fn already_visited(&self, regs: &Regs) -> bool {
        self.visited[regs.pc]
    }

    fn visit(&mut self, regs: &Regs) -> Instr {
        self.visited[regs.pc] = true;
        self.instrs[regs.pc]
    }

    fn reset_visit(&mut self, regs: &Regs) {
        self.visited[regs.pc] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        let instrs = parse_input(INPUT);
        let acc = star1(&instrs);
        assert_eq!(acc, 1528);
    }

    #[test]
    fn full_star2() {
        let instrs = parse_input(INPUT);
        let acc = star2(&instrs);
        assert_eq!(acc, 640);
    }
}
