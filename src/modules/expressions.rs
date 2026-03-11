use std::fmt;

use crate::modules::{errors::LoxError, token::Token, value::Value, visitor::ExprVisitor};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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
    Assign {
        name: Token,
        value: Box<Expr>,
        distance: Option<usize>,
    },
    Literal(Value),
    Variable {
        name: Token,
        distance: Option<usize>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    }
}

impl Expr {
    pub fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Value> {
        match self {
            Expr::Unary { .. } => {
                visitor.visit_unary(self)
            },
            Expr::Binary { .. } => {
                visitor.visit_binary(self)
            },
            Expr::Literal(_) => {
                visitor.visit_literal(self)
            },
            Expr::Grouping { .. } => {
                visitor.visit_grouping(self)
            },
            Expr::Variable { .. } => {
                visitor.visit_variable(self)
            },
            Expr::Assign { .. } => {
                visitor.visit_assign(self)
            },
            Expr::Logical { .. } => {
                visitor.visit_logical(self)
            },
            Expr::Call { .. } => {
                visitor.visit_call(self)
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign { name, value, distance: _ } => {
                f.write_fmt(format_args!("{} = {}", name.lexeme, value))
            }
            Self::Binary { left, operator, right } => {
                f.write_fmt(format_args!("{} {} {}", left, operator.lexeme, right))
            }
            Self::Literal(lit) => {
                f.write_fmt(format_args!("{}", lit))
            }
            Self::Variable { name, distance: _ } => {
                f.write_fmt(format_args!("{}", name.lexeme))
            }
            _ => f.write_str("[some expression]")
        }
    }
}