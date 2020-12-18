use crate::ast;
use crate::lexer;

use lexer::Token;

use parse_tree::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ast::Expr {
        let parse_tree = self.parse_mul_binary_expr();
        let ast_tree = transformer::make_ast(&parse_tree);
        ast_tree
    }

    fn peek(&self) -> Option<&Token> {
        if self.current == self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.current])
    }

    fn consume(&mut self) -> &Token {
        assert!(self.current < self.tokens.len());
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn parse_mul_binary_expr(&mut self) -> LowPrecedenceBinary {
        let left = self.parse_add_binary_expr();
        let token = self.peek();
        if let Some(Token::Operator(lexer::Op::Star)) = token {
            self.current += 1; // consume it now
            let right = self.parse_mul_binary_expr();
            LowPrecedenceBinary::Multiple(Box::new((left, lexer::Op::Star, right)))
        } else {
            LowPrecedenceBinary::Single(Box::new(left))
        }
    }

    fn parse_add_binary_expr(&mut self) -> HighPrecedenceBinary {
        let left = self.parse_primary_expr();
        let token = self.peek();
        if let Some(Token::Operator(lexer::Op::Plus)) = token {
            self.current += 1; // consume it now
            let right = self.parse_add_binary_expr();
            HighPrecedenceBinary::Multiple(Box::new((left, lexer::Op::Plus, right)))
        } else {
            HighPrecedenceBinary::Single(Box::new(left))
        }
    }

    fn parse_primary_expr(&mut self) -> Primary {
        let token = self.consume();
        match token {
            Token::Digit(x) => Primary::Digit(*x),
            Token::LeftPar => {
                let binary = self.parse_mul_binary_expr();
                let _right_par = self.consume();
                Primary::Expr(binary)
            }
            _ => panic!("Invalid token!"),
        }
    }
}

mod transformer {
    use crate::ast;
    use crate::lexer;

    use super::parse_tree::*;

    pub fn make_ast(parse_tree: &LowPrecedenceBinary) -> ast::Expr {
        transform_low_precedence_binary(parse_tree)
    }

    fn transform_low_precedence_binary(binary: &LowPrecedenceBinary) -> ast::Expr {
        match binary {
            LowPrecedenceBinary::Single(binary) => transform_high_prec_binary(binary),
            LowPrecedenceBinary::Multiple(tree) => {
                let left_ast = transform_high_prec_binary(&tree.0);
                let op_ast = transform_operator(tree.1);
                make_low_prec_tree_left(left_ast, op_ast, &tree.2)
            }
        }
    }

    fn make_low_prec_tree_left(
        left_ast: ast::Expr,
        op_ast: ast::Op,
        right_tree: &LowPrecedenceBinary,
    ) -> ast::Expr {
        match &right_tree {
            LowPrecedenceBinary::Single(binary) => {
                let right_ast = transform_high_prec_binary(binary);
                let binary_ast = ast::BinaryExpr::new(left_ast, op_ast, right_ast);
                ast::Expr::Binary(Box::new(binary_ast))
            }
            LowPrecedenceBinary::Multiple(higher) => {
                let right_ast = transform_high_prec_binary(&higher.0);
                let lower_ast = ast::BinaryExpr::new(left_ast, op_ast, right_ast);

                let outer_left = ast::Expr::Binary(Box::new(lower_ast));
                let outer_op = transform_operator(higher.1);
                make_low_prec_tree_left(outer_left, outer_op, &higher.2)
            }
        }
    }

    fn transform_high_prec_binary(binary: &HighPrecedenceBinary) -> ast::Expr {
        match binary {
            HighPrecedenceBinary::Single(primary) => transform_primary(primary),
            HighPrecedenceBinary::Multiple(tree) => {
                let left_ast = transform_primary(&tree.0);
                let op_ast = transform_operator(tree.1);
                make_high_prec_tree_left(left_ast, op_ast, &tree.2)
            }
        }
    }

    fn make_high_prec_tree_left(
        left_ast: ast::Expr,
        op_ast: ast::Op,
        right_tree: &HighPrecedenceBinary,
    ) -> ast::Expr {
        match &right_tree {
            HighPrecedenceBinary::Single(primary) => {
                let right_ast = transform_primary(primary);
                let binary_ast = ast::BinaryExpr::new(left_ast, op_ast, right_ast);
                ast::Expr::Binary(Box::new(binary_ast))
            }
            HighPrecedenceBinary::Multiple(higher) => {
                let right_ast = transform_primary(&higher.0);
                let lower_ast = ast::BinaryExpr::new(left_ast, op_ast, right_ast);

                let outer_left = ast::Expr::Binary(Box::new(lower_ast));
                let outer_op = transform_operator(higher.1);
                make_high_prec_tree_left(outer_left, outer_op, &higher.2)
            }
        }
    }

    fn transform_primary(primary: &Primary) -> ast::Expr {
        match primary {
            Primary::Digit(x) => ast::Expr::Digit(*x),
            Primary::Expr(expr) => transform_low_precedence_binary(expr),
        }
    }

    fn transform_operator(op: lexer::Op) -> ast::Op {
        match op {
            lexer::Op::Plus => ast::Op::Add,
            lexer::Op::Star => ast::Op::Multiply,
        }
    }
}

// TODO: Transform higher precedence to normal precedence.
/*
mod precedence_equalizer {
    use super::parse_tree::*;

    fn equalize_binary(binary: LowPrecedenceBinary) -> LowPrecedenceBinary {
        equalize_low_prec_binary(binary)
    }

    fn equalize_low_prec_binary(binary: LowPrecedenceBinary) -> LowPrecedenceBinary {
        match binary {
            LowPrecedenceBinary::Single(primary) => downgrade_high_prec_binary(*primary),
            LowPrecedenceBinary::Multiple(binary) => {
                let right = equalize_low_prec_binary(binary.2);

                if let HighPrecedenceBinary::Multiple(inner_binary) = binary.0 {
                    // 1 + 2 * 3
                    // inner_binary * right
                    // 1 + (2 * 3)

                    let wrapped = LowPrecedenceBinary::Single(Box::new(inner_binary.2));
                    let right = equalize_low_prec_binary(wrapped);


                    let primary = equalize_primary(inner_binary.0);
                    let left = HighPrecedenceBinary::Single(Box::new(primary));


                    LowPrecedenceBinary::Multiple(Box::new(left, inner_binary.1, right))
                } else {
                    let left = equalize_high_prec_binary(*binary.0);
                    LowPrecedenceBinary::Multiple(Box::new(left, binary.1, right))
                }

            }
        }
    }

    fn equalize_high_prec_binary(binary: HighPrecedenceBinary) -> HighPrecedenceBinary {
        match binary {
            HighPrecedenceBinary::Single(primary) => {
                HighPrecedenceBinary::Single(Box::new(equalize_primary(*primary)))
            }
            HighPrecedenceBinary::Multiple(binary) => {
                panic!("Invalid case!");
                // let left = equalize_primary(binary.0);
                // let right = equalize_high_prec_binary(binary.2);
                // HighPrecedenceBinary::Multiple(Box::new(left, binary.1, right))
            }
        }
    }

    fn downgrade_high_prec_binary(binary: HighPrecedenceBinary) -> LowPrecedenceBinary {
        if let HighPrecedenceBinary::Multiple(inner_binary) = *binary {
            let primary = equalize_primary(inner_binary.0);
            let left = HighPrecedenceBinary::Single(Box::new(primary));

            let wrapped = LowPrecedenceBinary::Single(Box::new(inner_binary.2));
            let right = equalize_low_prec_binary(wrapped);

            LowPrecedenceBinary::Multiple(Box::new(left, inner_binary.1, right))
        } else {
            LowPrecedenceBinary::Single(Box::new(equalize_high_prec_binary(*binary)))
        }
    }

    fn equalize_primary(primary: Primary) -> Primary {
        match primary {
            Primary::Digit(x) => Primary::Digit(x),
            Primary::Expr(expr) => Primary::Expr(equalize_low_prec_binary(expr)),
        }
    }
}
*/

mod parse_tree {
    use crate::lexer;

    // Binary expression with lower precedence level
    pub enum LowPrecedenceBinary {
        Single(Box<HighPrecedenceBinary>),
        Multiple(Box<(HighPrecedenceBinary, lexer::Op, LowPrecedenceBinary)>),
    }

    // Binary expression with higher precedence level
    pub enum HighPrecedenceBinary {
        Single(Box<Primary>),
        Multiple(Box<(Primary, lexer::Op, HighPrecedenceBinary)>),
    }

    // Primary expression; digit or (expr)
    pub enum Primary {
        Digit(u32),
        Expr(LowPrecedenceBinary),
    }
}
