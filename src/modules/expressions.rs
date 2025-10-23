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
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Literal(Value)
}

impl Expr {
    pub fn accept(&self, visitor: &dyn Visitor) {
        match self {
            Expr::Unary {operator: _, expr: _} => {
                visitor.visit_unary(self);
            },
            Expr::Literal(_) => {
                visitor.visit_literal(self);
            }
            _ => (),
        }
    }
}