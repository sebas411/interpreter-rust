use std::fmt;
use crate::modules::visitor::Visitor;

#[derive(Clone, Debug)]
pub enum Value {
    Number(String),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s)    => write!(f, "{}", s),
            Value::Bool(b)   => write!(f, "{}", b),
            Value::Nil       => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Unary {
        operator: String,
        expression: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal(Value)
}

impl Expr {
    pub fn accept(&self, visitor: &dyn Visitor) -> String {
        match self {
            Expr::Unary {operator: _, expression: _} => {
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