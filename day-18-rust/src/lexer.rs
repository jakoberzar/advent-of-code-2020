#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Star,
}
#[derive(Debug, Clone, Copy)]
pub enum Token {
    Digit(u32),
    Operator(Op),
    LeftPar,
    RightPar,
}

pub fn lex_line(line: &str) -> Vec<Token> {
    line.trim()
        .chars()
        .filter(|c| *c != ' ')
        .map(|c| match c {
            '+' => Token::Operator(Op::Plus),
            '*' => Token::Operator(Op::Star),
            '(' => Token::LeftPar,
            ')' => Token::RightPar,
            d if d.is_digit(10) => Token::Digit(d.to_digit(10).unwrap()),
            _ => panic!("Unknown character encountered!"),
        })
        .collect()
}

/*
pub struct Lexer {
    input: &str,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        Lexer { input, position: 0 }
    }

    fn lex(&self) -> Option<Token> {
        if self.position == self.input.len() {
            return None;
        }

        let char = self.input.chars()
    }
}
*/
