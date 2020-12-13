use std::convert::TryFrom;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-13.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-13.txt");

fn main() {
    let (timestamp, buses) = parse_input(INPUT);

    // Star 1
    let product = star1(timestamp, &buses);
    println!(
        "Product of first bus with its time difference is {}",
        product
    );

    // Star 2
    let timestamp = star2(&buses);
    println!("Found timestamp that matches constraints: {}", timestamp);
}

fn parse_input(input: &str) -> (u32, Vec<Bus>) {
    let mut lines = input.trim().lines();
    let depart = lines.next().unwrap().parse().unwrap();
    let buses: Vec<Bus> = lines.next().unwrap().split(',').map(Bus::new).collect();
    (depart, buses)
}

fn star1(timestamp: u32, buses: &[Bus]) -> u32 {
    let (id, diff): (&u32, u32) = buses
        .iter()
        .filter(|bus| match bus {
            Bus::Unknown => false,
            _ => true,
        })
        .map(|bus| match bus {
            Bus::ID(id) => (id, id - (timestamp % id)),
            Bus::Unknown => panic!("This variant should have been already filtered out!"),
        })
        .min_by_key(|(_id, diff)| *diff)
        .unwrap();
    id * diff
}

fn star2(buses: &[Bus]) -> u64 {
    let mut requirements: Vec<(usize, u32)> = buses
        .iter()
        .enumerate()
        .filter(|(_idx, bus)| match bus {
            Bus::ID(_) => true,
            Bus::Unknown => false,
        })
        .map(|(idx, bus)| match bus {
            Bus::ID(id) => (idx, *id),
            Bus::Unknown => panic!("This variant should have been already filtered out!"),
        })
        .collect();

    chinese_remainders_sieving(&mut requirements)
}

fn chinese_remainders_sieving(requirements: &mut [(usize, u32)]) -> u64 {
    requirements.sort_by_key(|(_, id)| -(i32::try_from(*id).unwrap()));

    let input: Vec<(u64, u64)> = requirements
        .iter()
        .map(|&(delay, id)| {
            let delay = delay as u64;
            let id = id as u64;
            ((id - (delay % id)) % id, id)
        })
        .collect();

    let mut solution = input.iter().next().unwrap().0;
    let mut add = input.iter().next().unwrap().1;
    for &(delay_next, id_next) in input.iter().skip(1) {
        while solution % id_next != delay_next {
            solution += add;
        }
        add *= id_next;
    }

    solution
}

// Original solution, since I didn't Google anything and I could just let it
// run while eating lunch and doing other things, so it finished as I had finished :)
// It (only) took 3 hours.
#[allow(dead_code)]
fn brute_force_star2(requirements: &mut [(usize, u32)]) -> u64 {
    requirements.sort_by_key(|(_, id)| -(i32::try_from(*id).unwrap()));

    println!("Requirements: {:?}", requirements);

    let max: u64 = requirements.iter().map(|&(_, id)| id as u64).product(); // added afterwards

    // Searching for number
    let mut k = 1u64;
    let mut tested = 1000u64;
    loop {
        let mut constraints = requirements.iter();
        let &(first_delay, first_id) = constraints.next().unwrap();

        let timestamp = k * first_id as u64 - first_delay as u64;

        let holds = constraints.all(|&(delay, id)| (timestamp + delay as u64) % id as u64 == 0);

        if holds {
            return timestamp;
        }

        while timestamp > tested {
            println!(
                "Tried first {} timestamps, {}% of range",
                tested,
                (timestamp as f64) / (max as f64) * 100.0
            );
            tested *= 10;
        }

        k += 1;
    }
}

#[derive(Debug, Copy, Clone)]
enum Bus {
    ID(u32),
    Unknown,
}

impl Bus {
    fn new(id: &str) -> Bus {
        if id == "x" {
            Bus::Unknown
        } else {
            Bus::ID(id.parse().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let (timestamp, buses) = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(timestamp, &buses), 59 * 5);
    }

    #[test]
    fn full_star1() {
        let (timestamp, buses) = parse_input(INPUT);
        assert_eq!(star1(timestamp, &buses), 2935);
    }

    #[test]
    fn simpler_star2() {
        let (_, buses) = parse_input("1\n17,x,13,19");
        assert_eq!(star2(&buses), 3417);
    }

    #[test]
    fn simple_star2() {
        let (_, buses) = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&buses), 1068781);
    }

    #[test]
    fn bigger_star2() {
        let (_, buses) = parse_input("1\n1789,37,47,1889");
        assert_eq!(star2(&buses), 1202161486);
    }

    #[test]
    fn full_star2() {
        let (_, buses) = parse_input(INPUT);
        assert_eq!(star2(&buses), 836024966345345);
    }
}
