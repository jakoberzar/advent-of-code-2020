use crate::ast;
use crate::lexer;

use lexer::Token;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ast::Expr {
        let parser_tree = self.parse_binary_expr();
        let ast_tree = Parser::transform_binary(&parser_tree);
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

    fn parse_binary_expr(&mut self) -> BinaryExpr {
        let left = self.parse_primary_expr();
        let token = self.peek();
        if let Some(Token::Operator(op)) = token {
            let op = *op;
            self.current += 1; // consume it now
            let right = self.parse_binary_expr();
            BinaryExpr::Multiple(Box::new((left, op, right)))
        } else {
            BinaryExpr::Single(Box::new(left))
        }
    }

    fn parse_primary_expr(&mut self) -> PrimaryExpr {
        let token = self.consume();
        match token {
            Token::Digit(x) => PrimaryExpr::Digit(*x),
            Token::LeftPar => {
                let binary = self.parse_binary_expr();
                let _right_par = self.consume();
                PrimaryExpr::Expr(binary)
            }
            _ => panic!("Invalid token!"),
        }
    }

    fn transform_binary(binary: &BinaryExpr) -> ast::Expr {
        match binary {
            BinaryExpr::Single(primary) => Parser::transform_primary(primary),
            BinaryExpr::Multiple(tree) => {
                let left_ast = Parser::transform_primary(&tree.0);
                let op_ast = Parser::transform_operator(tree.1);
                Parser::make_tree_left(left_ast, op_ast, &tree.2)
            }
        }
    }

    fn make_tree_left(left_ast: ast::Expr, op_ast: ast::Op, right_tree: &BinaryExpr) -> ast::Expr {
        match &right_tree {
            BinaryExpr::Single(primary) => {
                let right_ast = Parser::transform_primary(primary);
                let binary_ast = ast::BinaryExpr::new(left_ast, op_ast, right_ast);
                ast::Expr::Binary(Box::new(binary_ast))
            }
            BinaryExpr::Multiple(higher) => {
                let right_ast = Parser::transform_primary(&higher.0);
                let lower_ast = ast::BinaryExpr::new(left_ast, op_ast, right_ast);

                let outer_left = ast::Expr::Binary(Box::new(lower_ast));
                let outer_op = Parser::transform_operator(higher.1);
                Parser::make_tree_left(outer_left, outer_op, &higher.2)
            }
        }
    }

    fn transform_primary(primary: &PrimaryExpr) -> ast::Expr {
        match primary {
            PrimaryExpr::Digit(x) => ast::Expr::Digit(*x),
            PrimaryExpr::Expr(expr) => Parser::transform_binary(expr),
        }
    }

    fn transform_operator(op: lexer::Op) -> ast::Op {
        match op {
            lexer::Op::Plus => ast::Op::Add,
            lexer::Op::Star => ast::Op::Multiply,
        }
    }
}
enum BinaryExpr {
    Single(Box<PrimaryExpr>),
    Multiple(Box<(PrimaryExpr, lexer::Op, BinaryExpr)>),
}

enum PrimaryExpr {
    Digit(u32),
    Expr(BinaryExpr),
}
