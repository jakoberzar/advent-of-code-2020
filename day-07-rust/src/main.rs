use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-07.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-07.txt");

// The solution currently uses a hashmap for the rules, and/or a simple vector iteration.
// However, using graphs would probably be much more elegant.
// They are however difficult to use in Rust due to the lifetime constraints.
// TODO: Read https://github.com/nrc/r4cppp/blob/master/graphs/README.md and try a graph implementation.
// Benchmarking and comparing the solutions might also be cool :)

fn main() {
    let rules = parse_input(INPUT);

    // Star 1
    let bag_count = star1(&rules);
    println!(
        "Found {} types of bags that can contain the gold bag.",
        bag_count
    );

    let in_gold_bag = star2(&rules);
    println!("There can be {} bags in the gold bag.", in_gold_bag);
}

fn star1(rules: &Vec<Rule>) -> usize {
    let mut found: HashSet<&String> = HashSet::new();
    let mut searching: VecDeque<&String> = VecDeque::new();

    // Insert gold rule to 'searching'
    searching.push_back(&find_gold_bag(&rules).bag);

    // Found bags that can contain it
    while let Some(searched) = searching.pop_front() {
        rules
            .iter()
            .filter(|rule| rule.contents.iter().any(|(_, name)| name == searched))
            .for_each(|Rule { bag, .. }| {
                if !found.contains(bag) {
                    found.insert(bag);
                    searching.push_back(bag);
                }
            });
    }

    found.len()
}

fn star2(rules: &Vec<Rule>) -> u32 {
    let rule_map: HashMap<&String, &Rule> =
        HashMap::from_iter(rules.iter().map(|rule| &rule.bag).zip(rules.iter()));

    let gold_bag = find_gold_bag(rules);
    amount_containing_bags(&rule_map, gold_bag)
}

struct Rule {
    bag: String,
    contents: Vec<(u32, String)>,
}

fn parse_input(input: &str) -> Vec<Rule> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut words = line.trim().split(' ');
            if words.clone().count() < (3 + 1 + 3) {
                panic!(format!("Rule {} is invalid!", line));
            }
            let container = format!("{} {}", words.next().unwrap(), words.next().unwrap());
            let mut contained: Vec<(u32, String)> = Vec::new();
            words.next(); // bags
            words.next(); // contain
            while let Some(s) = words.next() {
                if s == "no" {
                    break;
                }
                // There are containments left
                let amount: u32 = s.parse().unwrap();
                let bag = format!("{} {}", words.next().unwrap(), words.next().unwrap());
                contained.push((amount, bag));
                words.next(); // bags, or bags.
            }

            Rule {
                bag: container,
                contents: contained,
            }
        })
        .collect()
}

fn find_gold_bag(rules: &Vec<Rule>) -> &Rule {
    rules
        .iter()
        .find(|rule| rule.bag == "shiny gold")
        .expect("No shiny gold bag found!")
}

fn amount_containing_bags(rule_map: &HashMap<&String, &Rule>, container: &Rule) -> u32 {
    container
        .contents
        .iter()
        .map(|(amount, name)| {
            amount + amount * amount_containing_bags(rule_map, rule_map.get(name).unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_star1() {
        let rules = parse_input(INPUT);
        let bag_count = star1(&rules);
        assert_eq!(bag_count, 208);
    }

    #[test]
    fn full_star2() {
        let rules = parse_input(INPUT);
        let in_gold_bag = star2(&rules);
        assert_eq!(in_gold_bag, 1664);
    }
}
