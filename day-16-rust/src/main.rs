use parse_display::{Display, FromStr};

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-16.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-16.txt");

fn main() {
    let (rules, mine, nearby) = parse_input(INPUT);

    // Star 1
    let invalid_value_sum = star1(&rules, &nearby);
    println!("Sum of invalid values is {}", invalid_value_sum);

    // Star 2
    let invalid_value_sum = star2(&rules, &mine, &nearby);
    println!("Sum of invalid values is {}", invalid_value_sum);
}

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut parts = input.trim().split("\n\n");
    let rules: Vec<Rule> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let my_numbers = parts.next().unwrap().lines().skip(1).next().unwrap();
    let mine = Ticket::new(my_numbers);

    let nearby = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| Ticket::new(line))
        .collect();

    (rules, mine, nearby)
}

fn star1(rules: &[Rule], nearby: &[Ticket]) -> u64 {
    let mut invalid_sum = 0;
    for ticket in nearby {
        invalid_sum += ticket.find_invalid_value(rules).map_or(0, |val| *val) as u64;
    }
    invalid_sum
}

fn star2(rules: &[Rule], mine: &Ticket, nearby: &[Ticket]) -> usize {
    let mut valid_tickets: Vec<Ticket> = get_valid_tickets(rules, nearby);
    valid_tickets.push(mine.clone());

    let mut rule_rows: Vec<usize> = Vec::new(); // mapping of row -> rule
    let mut rule_checker = RuleChecker::new(rules, &valid_tickets);
    find_order(&mut rule_rows, &mut rule_checker);

    println!("Found order: {:?}", rule_rows);

    rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.field.starts_with("departure"))
        .map(|(idx, _)| {
            let rule_row = rule_rows.iter().position(|val| *val == idx).unwrap();
            mine.values[rule_row] as usize
        })
        .product()
}

fn get_valid_tickets(rules: &[Rule], tickets: &[Ticket]) -> Vec<Ticket> {
    tickets
        .iter()
        .filter(|ticket| ticket.is_valid(rules))
        .map(|ticket| ticket.clone())
        .collect()
}

fn find_order(rule_rows: &mut Vec<usize>, rule_checker: &mut RuleChecker) -> bool {
    let row = rule_rows.len();
    let rule_count = rule_checker.rules.len();
    if row >= rule_count {
        return true;
    }

    for rule_idx in 0..rule_count {
        // Check if already contained
        if rule_rows.contains(&rule_idx) {
            continue;
        }

        // Check if rule valid
        let valid = rule_checker.check_rule_row(rule_idx, row);
        if !valid {
            continue;
        }

        // Check if other rules can be assigned somewhere
        rule_rows.push(rule_idx);
        let valid = find_order(rule_rows, rule_checker);
        if valid {
            return true;
        }
        rule_rows.pop();
    }

    false
}

#[derive(Clone, Debug, Display, FromStr)]
#[display("{field}: {min1}-{max1} or {min2}-{max2}")]
struct Rule {
    field: String,
    min1: u32,
    max1: u32,
    min2: u32,
    max2: u32,
}

impl Rule {
    fn value_valid(&self, value: u32) -> bool {
        (self.min1 <= value && value <= self.max1) || (self.min2 <= value && value <= self.max2)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ticket {
    values: Vec<u32>,
}

impl Ticket {
    fn new(line: &str) -> Ticket {
        let values = line
            .trim()
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();
        Ticket { values }
    }

    fn find_invalid_value(&self, rules: &[Rule]) -> Option<&u32> {
        self.values
            .iter()
            .find(|value| !rules.iter().any(|rule| rule.value_valid(**value)))
    }

    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.find_invalid_value(rules).is_none()
    }
}

#[derive(Debug)]
struct RuleChecker<'a, 'b> {
    rules: &'a [Rule],
    tickets: &'b [Ticket],
    row_size: usize,
    rule_row_cache: Vec<Option<bool>>,
}

impl<'a, 'b> RuleChecker<'a, 'b> {
    fn new(rules: &'a [Rule], tickets: &'b [Ticket]) -> RuleChecker<'a, 'b> {
        let row_size = tickets[0].values.len();
        RuleChecker {
            rules,
            tickets,
            row_size,
            rule_row_cache: vec![None; rules.len() * row_size],
        }
    }

    fn check_rule_row(&mut self, rule_idx: usize, row: usize) -> bool {
        assert!(rule_idx < self.rules.len());
        assert!(row < self.row_size);

        // Check cache
        let cache_idx = rule_idx * self.row_size + row;
        let cached = self.rule_row_cache[cache_idx];
        if cached.is_some() {
            return cached.unwrap();
        }

        // Calculate value
        let rule = &self.rules[rule_idx];
        let valid = self
            .tickets
            .iter()
            .all(|ticket| rule.value_valid(ticket.values[row]));

        // Store result into cache
        self.rule_row_cache[cache_idx] = Some(valid);

        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let (rules, _, nearby) = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&rules, &nearby), 4 + 55 + 12);
    }

    #[test]
    fn full_star1() {
        let (rules, _, nearby) = parse_input(INPUT);
        assert_eq!(star1(&rules, &nearby), 19093);
    }

    #[test]
    fn full_star2() {
        let (rules, mine, nearby) = parse_input(INPUT);
        assert_eq!(star2(&rules, &mine, &nearby), 5311123569883);
    }

    #[test]
    fn get_valid_tickets_works() {
        let (rules, _, nearby) = parse_input(SIMPLE_INPUT);
        let valid = vec![Ticket {
            values: vec![7, 3, 47],
        }];
        assert_eq!(get_valid_tickets(&rules, &nearby), valid);
    }

    #[test]
    fn rule_checker_works() {
        let (rules, mine, nearby) = parse_input(SIMPLE_INPUT);
        let tickets = vec![mine, nearby[0].clone()];
        let mut rule_checker = RuleChecker::new(&rules, &tickets);

        assert_eq!(rule_checker.check_rule_row(0, 0), true);
        assert_eq!(rule_checker.check_rule_row(0, 1), true);
        assert_eq!(rule_checker.check_rule_row(0, 2), false);
        assert_eq!(rule_checker.check_rule_row(1, 0), true);
        assert_eq!(rule_checker.check_rule_row(1, 1), false);
        assert_eq!(rule_checker.check_rule_row(1, 2), false);
        assert_eq!(rule_checker.check_rule_row(2, 0), false);
        assert_eq!(rule_checker.check_rule_row(2, 1), false);
        assert_eq!(rule_checker.check_rule_row(2, 2), true);
        assert_eq!(rule_checker.check_rule_row(0, 0), true);
    }
}
