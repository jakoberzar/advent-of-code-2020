#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-18.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-18.txt");

mod ast;
mod lexer;
mod parser;
mod parser_star2;

fn main() {
    let tokens = parse_input(SIMPLE_INPUT);

    // Star 1
    println!("Sum of expressions is {}", star1(&tokens));

    // Star 2
    println!(
        "Sum of expressions by having addition higher precedence is {}",
        star2(&tokens)
    );
}

fn parse_input(input: &str) -> Vec<Vec<lexer::Token>> {
    input
        .trim()
        .lines()
        .map(|line| lexer::lex_line(line))
        .collect()
}

fn star1(tokens: &[Vec<lexer::Token>]) -> u64 {
    tokens
        .iter()
        .map(|tokens| {
            let mut parser = parser::Parser::new(&tokens);
            let expr = parser.parse();
            expr.evaluate()
        })
        .sum()
}

fn star2(tokens: &[Vec<lexer::Token>]) -> u64 {
    tokens
        .iter()
        .map(|tokens| {
            let mut parser = parser_star2::Parser::new(&tokens);
            let expr = parser.parse();
            expr.evaluate()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let tokens = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&tokens), 26335);
    }

    #[test]
    fn full_star1() {
        let tokens = parse_input(INPUT);
        assert_eq!(star1(&tokens), 75592527415659);
    }

    #[test]
    fn simple_star2() {
        let tokens = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&tokens), 46 + 1445 + 669060 + 23340);
    }

    #[test]
    fn full_star2() {
        let tokens = parse_input(INPUT);
        assert_eq!(star2(&tokens), 360029542265462);
    }
}
