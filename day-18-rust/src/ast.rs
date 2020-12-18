#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add,
    Multiply,
}

pub enum Expr {
    Binary(Box<BinaryExpr>),
    Digit(u32),
}

impl Expr {
    pub fn evaluate(&self) -> u64 {
        match self {
            Expr::Binary(binary) => binary.evaluate(),
            Expr::Digit(x) => *x as u64,
        }
    }
}

pub struct BinaryExpr {
    pub left: Expr,
    pub op: Op,
    pub right: Expr,
}

impl BinaryExpr {
    pub fn new(left: Expr, op: Op, right: Expr) -> BinaryExpr {
        BinaryExpr { left, op, right }
    }

    fn evaluate(&self) -> u64 {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        match self.op {
            Op::Add => left + right,
            Op::Multiply => left * right,
        }
    }
}
