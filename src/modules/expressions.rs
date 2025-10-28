use crate::modules::{errors::LoxError, token::Token, value::Value, visitor::ExprVisitor};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

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
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Literal(Value),
    Variable(Token),
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
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
            Expr::Variable(_) => {
                visitor.visit_variable(self)
            },
            Expr::Assign { .. } => {
                visitor.visit_asign(self)
            },
            Expr::Logical { .. } => {
                visitor.visit_logical(self)
            },
        }
    }
}