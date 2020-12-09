const INPUT: &str = include_str!("./../../inputs/day-01.txt");

fn main() {
    // Star 1
    let result = star1(INPUT);
    println!("The product of two numbers multiplied is {}", result);

    // Star 2
    let result = star2(INPUT);
    println!("The product of two numbers multiplied is {}", result);
}

fn star1(input: &str) -> i32 {
    let mut numbers = parse_text(input);
    numbers.sort();
    let (low, high) = find_two_numbers(&numbers, 2020).expect("No solution found!");
    low * high
}

fn star2(input: &str) -> i32 {
    let mut numbers = parse_text(input);
    numbers.sort();
    let (first, second, third) = find_three_numbers(&numbers, 2020).expect("No solution found!");
    first * second * third
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        let result = star1(INPUT);
        assert_eq!(result, 988771);
    }

    #[test]
    fn full_star2() {
        let result = star2(INPUT);
        assert_eq!(result, 171933104);
    }
}
