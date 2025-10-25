use crate::modules::{token::Token, value::Value, visitor::Visitor};

#[derive(Clone, Debug)]
pub enum Expr {
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal(Value)
}

impl Expr {
    pub fn accept(&self, visitor: &dyn Visitor) -> Value {
        match self {
            Expr::Unary {operator: _, right: _} => {
                visitor.visit_unary(self)
            },
            Expr::Binary { left: _, operator: _, right: _ } => {
                visitor.visit_binary(self)
            },
            Expr::Literal(_) => {
                visitor.visit_literal(self)
            },
            Expr::Grouping { expression: _ } => {
                visitor.visit_grouping(self)
            }
        }
    }
}