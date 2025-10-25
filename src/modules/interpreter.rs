use crate::modules::{expressions::{Expr, Value}, visitor::Visitor};

pub struct Interpreter;

impl Visitor for Interpreter {
    fn visit_binary(&self, _expr: &Expr) -> Value {
        Value::Nil
    }
    fn visit_grouping(&self, expr: &Expr) -> Value {
        if let Expr::Grouping { expression } = expr {
            return expression.accept(self);
        }
        Value::Nil
    }
    fn visit_literal(&self, expr: &Expr) -> Value {
        if let Expr::Literal(value) = expr {
            return value.clone()
        }
        Value::Nil
    }
    fn visit_unary(&self, _expr: &Expr) -> Value {
        Value::Nil
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, expr: &Expr) {
        let val = expr.accept(self);
        if let Value::Number(_, n) = val {
            println!("{}", n);
        } else {
            println!("{}", val);
        }
    }
}
