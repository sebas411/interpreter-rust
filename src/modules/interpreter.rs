use crate::modules::{expressions::{Expr, Value}, visitor::Visitor};

pub struct Interpreter;

impl Visitor for Interpreter {
    fn visit_binary(&self, _expr: &super::expressions::Expr) -> Value {
        Value::Nil
    }
    fn visit_grouping(&self, _expr: &super::expressions::Expr) -> Value {
        Value::Nil
    }
    fn visit_literal(&self, expr: &super::expressions::Expr) -> Value {
        if let Expr::Literal(value) = expr {
            return value.clone()
        }
        Value::Nil
    }
    fn visit_unary(&self, _expr: &super::expressions::Expr) -> Value {
        Value::Nil
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, expr: &Expr) {
        let val = expr.accept(self);
        println!("{}", val);
    }
}
