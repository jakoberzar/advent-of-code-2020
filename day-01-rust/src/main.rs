const INPUT: &str = include_str!("./../input.txt");

fn main() {
    star1(INPUT);
    star2(INPUT);
}

fn star1(input: &str) {
    let mut numbers = parse_text(input);
    numbers.sort();
    let (low, high) = find_two_numbers(&mut numbers, None).expect("No solution found!");
    println!(
        "Star 1: Resulting numbers are {} and {}, multiplied to {}",
        low,
        high,
        low * high
    );
}

fn star2(input: &str) {
    let mut numbers = parse_text(input);
    numbers.sort();
    let (first, second, third) = find_three_numbers(&mut numbers).expect("No solution found!");
    println!(
        "Star 2: Resulting numbers are {}, {} and {}, multiplied to {}",
        first,
        second,
        third,
        first * second * third
    );
}

fn parse_text(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|x| {
            x.trim()
                .parse()
                .expect(format!("Could not parse {} to i32!", x).as_str())
        })
        .collect()
}

// Assumes sorted numbers vector
// TODO: Maybe solution with receiving an iterator as a parameter would fit better?
fn find_two_numbers(numbers: &[i32], third_number: Option<i32>) -> Option<(&i32, &i32)> {
    // Only available in nightly...
    // if !numbers.is_sorted() {
    //     panic!("Numbers should be sorted before passed!");
    // }

    let mut low_iter = numbers
        .iter()
        .filter(|&x| third_number.is_none() || *x != third_number.unwrap());
    let mut high_iter = low_iter.clone().rev();
    let mut low = low_iter.next().expect("No numbers in input!");
    let mut high = high_iter.next().expect("No numbers in input!");

    while low <= high {
        let sum = low + high + third_number.unwrap_or(0);
        if sum == 2020 {
            return Some((low, high));
        } else if sum < 2020 {
            match low_iter.next() {
                Some(x) => low = x,
                _ => return None,
            }
        } else if sum > 2020 {
            match high_iter.next() {
                Some(x) => high = x,
                _ => return None,
            }
        }
    }

    None
}

fn find_three_numbers(numbers: &[i32]) -> Option<(&i32, &i32, &i32)> {
    for third in numbers.iter() {
        if let Some((first, second)) = find_two_numbers(numbers, Some(*third)) {
            return Some((first, second, third));
        }
    }
    None
}
