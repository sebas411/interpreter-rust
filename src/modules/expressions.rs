use std::fmt;
use crate::modules::{token::Token, visitor::Visitor};

#[derive(Clone, Debug)]
pub enum Value {
    Number(String, f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn from_number(num: f64) -> Self {
        let add_decimal;
        if num == (num as i64) as f64 {
            add_decimal = ".0";
        } else {
            add_decimal = "";
        }
        let num_str_rep = format!("{}{}", num, add_decimal);
        Self::Number(num_str_rep, num)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(s, _) => write!(f, "{}", s),
            Value::Str(s)    => write!(f, "{}", s),
            Value::Bool(b)   => write!(f, "{}", b),
            Value::Nil       => write!(f, "nil"),
        }
    }
}

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