const INPUT: &str = include_str!("./../../inputs/day-01.txt");

fn main() {
    star1(INPUT);
    star2(INPUT);
}

fn star1(input: &str) {
    let mut numbers = parse_text(input);
    numbers.sort();
    let (low, high) = find_two_numbers(&numbers, 2020).expect("No solution found!");
    let result = low * high;
    println!(
        "Star 1: Resulting numbers are {} and {}, multiplied to {}",
        low, high, result
    );
    assert_eq!(result, 988771);
}

fn star2(input: &str) {
    let mut numbers = parse_text(input);
    numbers.sort();
    let (first, second, third) = find_three_numbers(&numbers, 2020).expect("No solution found!");
    let result = first * second * third;
    println!(
        "Star 2: Resulting numbers are {}, {} and {}, multiplied to {}",
        first, second, third, result
    );
    assert_eq!(result, 171933104);
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
fn find_two_numbers(numbers: &[i32], wanted_sum: i32) -> Option<(&i32, &i32)> {
    // Only available in nightly...
    // if !numbers.is_sorted() {
    //     panic!("Numbers should be sorted before passed!");
    // }

    let mut low_iter = numbers.iter();
    let mut high_iter = numbers.iter().rev();
    let mut low = low_iter.next().expect("No numbers in input!");
    let mut high = high_iter.next().expect("No numbers in input!");

    while low <= high {
        let sum = low + high;
        if sum == wanted_sum {
            return Some((low, high));
        } else if sum < wanted_sum {
            match low_iter.next() {
                Some(x) => low = x,
                _ => return None,
            }
        } else if sum > wanted_sum {
            match high_iter.next() {
                Some(x) => high = x,
                _ => return None,
            }
        }
    }

    None
}

fn find_three_numbers(numbers: &[i32], wanted_sum: i32) -> Option<(&i32, &i32, &i32)> {
    for (idx, third) in numbers.iter().enumerate() {
        if let Some((first, second)) =
            find_two_numbers(&numbers[idx + 1..numbers.len()], wanted_sum - third)
        {
            return Some((first, second, third));
        }
    }
    None
}
