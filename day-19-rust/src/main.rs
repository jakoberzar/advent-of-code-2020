#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-19.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-19.txt");
#[allow(dead_code)]
const SIMPLE_INPUT_STAR2: &str = include_str!("./../../inputs/simple/day-19-star2.txt");

fn main() {
    let (rules, messages) = parse_input(INPUT);

    // Star 1
    let matching = star1(&rules, &messages);
    println!("Number of messages that match star1 rules is {}", matching);

    // Star 2
    let matching = star2(&rules, &messages);
    println!("Number of messages that match star2 rules is {}", matching);
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<&str>) {
    let mut parts = input.trim().split("\n\n");

    let mut rules_indexed: Vec<(u32, Rule)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| Rule::new(line))
        .collect();
    rules_indexed.sort_by_key(|(idx, _)| *idx);

    let rules = rules_indexed.iter().map(|(_, rule)| rule.clone()).collect();
    let messages = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .collect();

    (rules, messages)
}

fn star1(rules: &Vec<Rule>, messages: &[&str]) -> usize {
    messages
        .iter()
        .filter(|message| {
            let matches = rules[0].matches(rules, **message);
            let matches = matches.is_some() && matches.unwrap() == message.len();
            matches
        })
        .count()
}

fn star2(rules: &Vec<Rule>, messages: &[&str]) -> usize {
    let generated_42 = rules[42].generate_matches(rules);
    let generated_31 = rules[31].generate_matches(rules);
    let one_rule_8_len = generated_42[0].len();
    let one_rule_11_len = generated_42[0].len() + generated_31[0].len();

    messages
        .iter()
        .filter(|message| {
            let max_times_11 = (message.len() - one_rule_8_len) / one_rule_11_len;
            for times_11 in 1..=max_times_11 {
                let times_8 = (message.len() - times_11 * one_rule_11_len) / one_rule_8_len;

                let mut first_rules = vec![42; times_8 + times_11];
                let mut second_rules = vec![31; times_11];
                first_rules.append(&mut second_rules);

                let expanded = ExpandedRule::from(first_rules);
                let matches = expanded.matches(&rules, **message);

                let correct = matches.is_some() && matches.unwrap() == message.len();
                if correct {
                    return true;
                }
            }
            return false;
        })
        .count()
}

// Another way to solve the second star, checking if chunks of string are in the generated ones
#[allow(dead_code)]
fn star2_via_chunks(rules: &Vec<Rule>, messages: &[&str]) -> usize {
    let generated_42 = rules[42].generate_matches(rules);
    let generated_31 = rules[31].generate_matches(rules);
    let one_rule_8_len = generated_42[0].len();
    let one_rule_11_len = generated_42[0].len() + generated_31[0].len();

    messages
        .iter()
        .filter(|message| {
            let max_times_11 = (message.len() - one_rule_8_len) / one_rule_11_len;
            for times_11 in 1..=max_times_11 {
                let times_8 = (message.len() - times_11 * one_rule_11_len) / one_rule_8_len;
                let mut chunks = Vec::new();
                for i in 0..(times_8 + 2 * times_11) {
                    let start = i * one_rule_8_len;
                    let end = (i + 1) * one_rule_8_len;
                    chunks.push(message[start..end].to_owned());
                }
                let correct = chunks.iter().enumerate().all(|(idx, chunk)| {
                    if idx < (times_8 + times_11) {
                        generated_42.contains(chunk)
                    } else {
                        generated_31.contains(chunk)
                    }
                });
                if correct {
                    return true;
                }
            }
            false
        })
        .count()
}

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Expanded(Box<ExpandedRule>),
    ExpandedOr(Box<ExpandedRule>, Box<ExpandedRule>),
}

impl Rule {
    fn new(line: &str) -> (u32, Rule) {
        let mut parts = line.trim().split(':');
        let idx = parts.next().unwrap().parse().unwrap();

        let rule_str = parts.next().unwrap().trim();
        let rule: Rule = if rule_str.contains('"') {
            assert!(rule_str.len() == 3);
            Rule::Char(rule_str.chars().nth(1).unwrap())
        } else if rule_str.contains('|') {
            let mut parts = rule_str.split('|');
            let first = ExpandedRule::new(parts.next().unwrap());
            let second = ExpandedRule::new(parts.next().unwrap());
            Rule::ExpandedOr(Box::new(first), Box::new(second))
        } else {
            let expanded = ExpandedRule::new(rule_str);
            Rule::Expanded(Box::new(expanded))
        };

        (idx, rule)
    }

    // Returns if matches and how many characters it consumes
    fn matches(&self, rules: &Vec<Rule>, input: &str) -> Option<usize> {
        match self {
            Rule::Char(c) => {
                if input.len() == 0 {
                    return None;
                }

                if input.chars().next().unwrap() == *c {
                    Some(1)
                } else {
                    None
                }
            }
            Rule::Expanded(expanded) => expanded.matches(rules, input),
            Rule::ExpandedOr(first, second) => first
                .matches(rules, input)
                .or_else(|| second.matches(rules, input)),
        }
    }

    fn generate_matches(&self, rules: &Vec<Rule>) -> Vec<String> {
        match self {
            Rule::Char(c) => vec![String::from(*c)],
            Rule::Expanded(expanded) => expanded.generate_matches(rules),
            Rule::ExpandedOr(first, second) => {
                let mut first_generated = first.generate_matches(rules);
                let mut second_generated = second.generate_matches(rules);
                first_generated.append(&mut second_generated);
                first_generated
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ExpandedRule {
    subrules: Vec<u32>,
}

impl ExpandedRule {
    fn new(rule_str: &str) -> ExpandedRule {
        let subrules = rule_str
            .trim()
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();
        ExpandedRule { subrules }
    }

    fn from(subrules: Vec<u32>) -> ExpandedRule {
        ExpandedRule { subrules }
    }

    fn matches(&self, rules: &Vec<Rule>, input: &str) -> Option<usize> {
        let mut consumed = 0;
        for subrule_idx in &self.subrules {
            let rule = &rules[*subrule_idx as usize];
            match rule.matches(rules, &input[consumed..]) {
                Some(x) => {
                    consumed += x;
                }
                None => return None,
            }
        }
        Some(consumed)
    }

    fn generate_matches(&self, rules: &Vec<Rule>) -> Vec<String> {
        let mut strings = vec!["".to_owned()];
        for subrule_idx in &self.subrules {
            let rule = &rules[*subrule_idx as usize];
            let generated = rule.generate_matches(rules);
            let mut new_strings = Vec::new();
            for gen in &generated {
                for current in &strings {
                    new_strings.push(format!("{}{}", current, gen));
                }
            }
            strings = new_strings;
        }
        strings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let (rules, messages) = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&rules, &messages), 2);
    }

    #[test]
    fn full_star1() {
        let (rules, messages) = parse_input(INPUT);
        assert_eq!(star1(&rules, &messages), 285);
    }

    #[test]
    fn simple_star2() {
        let (rules, messages) = parse_input(SIMPLE_INPUT_STAR2);
        assert_eq!(star2(&rules, &messages), 12);
    }

    #[test]
    fn full_star2() {
        let (rules, messages) = parse_input(INPUT);
        assert_eq!(star2(&rules, &messages), 412);
    }
}
