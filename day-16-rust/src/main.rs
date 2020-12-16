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
    let departure_product = star2(&rules, &mine, &nearby);
    println!("Product of departure columns is {}", departure_product);
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

    let mut order_finder = RuleOrderFinder::new(rules, &valid_tickets);
    order_finder.optimize();

    if !order_finder.order_established() {
        panic!("Multiple variants possible!");
    }

    order_finder
        .established
        .iter()
        .enumerate()
        .filter(|(_, rule)| rule.unwrap().field.starts_with("departure"))
        .map(|(idx, _)| mine.values[idx] as usize)
        .product()
}

fn get_valid_tickets(rules: &[Rule], tickets: &[Ticket]) -> Vec<Ticket> {
    tickets
        .iter()
        .filter(|ticket| ticket.is_valid(rules))
        .map(|ticket| ticket.clone())
        .collect()
}

#[derive(Clone, Debug, Display, FromStr, Eq, PartialEq)]
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

#[derive(Debug, Clone)]
struct RuleOrderFinder<'a> {
    order: Vec<Vec<&'a Rule>>,          // map of col -> possible rules
    established: Vec<Option<&'a Rule>>, // map of col -> established rule
}

impl<'a, 'b> RuleOrderFinder<'a> {
    fn new(rules: &'a [Rule], tickets: &[Ticket]) -> RuleOrderFinder<'a> {
        let order = RuleOrderFinder::find_initial(rules, tickets);
        RuleOrderFinder {
            order,
            established: vec![None; rules.len()],
        }
    }

    fn find_initial<'x>(rules: &'x [Rule], tickets: &[Ticket]) -> Vec<Vec<&'x Rule>> {
        let rule_len = rules.len();
        let mut valid_rules: Vec<Vec<&Rule>> = Vec::new();
        valid_rules.reserve(rule_len);

        for col_idx in 0..rule_len {
            let valid_rules_col = rules
                .iter()
                .filter(|rule| {
                    tickets
                        .iter()
                        .all(|ticket| rule.value_valid(ticket.values[col_idx]))
                })
                .collect();
            valid_rules.push(valid_rules_col);
        }

        valid_rules
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Current rules:");
        for (idx, valid_rule) in self.established.iter().enumerate() {
            if valid_rule.is_some() {
                println!("{} has established rule {:?}", idx, valid_rule.unwrap());
            }
        }
        for (idx, valid_rule) in self.order.iter().enumerate() {
            println!("{} has {} valid rules", idx, valid_rule.len());
        }
    }

    fn optimize(&mut self) {
        let mut indicies: Vec<usize> = self
            .established
            .iter_mut()
            .enumerate()
            .filter(|(_, value)| value.is_none())
            .map(|(idx, _)| idx)
            .collect();

        let mut changed = 1;
        while changed > 0 {
            let mut remove: Vec<usize> = Vec::new();
            for &idx in &indicies {
                if self.order[idx].len() == 1 {
                    let rule = self.order[idx][0];
                    self.established[idx] = Some(rule);

                    for &order_idx in &indicies {
                        let col_order = &mut self.order[order_idx];
                        let rule_idx = col_order.iter().position(|col_rule| *col_rule == rule);
                        if let Some(x) = rule_idx {
                            col_order.remove(x);
                        }
                    }

                    remove.push(idx);
                }
            }
            indicies.retain(|x| !remove.contains(&x));
            changed = remove.len();
        }
    }

    fn order_established(&self) -> bool {
        self.established.iter().all(|rule| rule.is_some())
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
}
