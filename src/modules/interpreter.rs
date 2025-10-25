use crate::modules::{expressions::{Expr, Value}, visitor::Visitor};

pub struct Interpreter;

impl Visitor for Interpreter {
    fn visit_binary(&self, expr: &Expr) -> Value {
        if let Expr::Binary { left, operator, right } = expr {
            let left = self.evaluate(left);
            let right = self.evaluate(right);
            if let (Value::Number(_, n_left), Value::Number(_, n_right)) = (left, right) {
                match operator.token_type.as_str() {
                    "PLUS" => return Value::from_number(n_left + n_right),
                    "MINUS" => return Value::from_number(n_left - n_right),
                    "STAR" => return Value::from_number(n_left * n_right),
                    "SLASH" => return Value::from_number(n_left / n_right),
                    _ => ()
                }
            }
        }
        Value::Nil
    }
    fn visit_grouping(&self, expr: &Expr) -> Value {
        if let Expr::Grouping { expression } = expr {
            return self.evaluate(expression);
        }
        Value::Nil
    }
    fn visit_literal(&self, expr: &Expr) -> Value {
        if let Expr::Literal(value) = expr {
            return value.clone()
        }
        Value::Nil
    }
    fn visit_unary(&self, expr: &Expr) -> Value {
        if let Expr::Unary { operator, right } = expr {
            let right = self.evaluate(&right);
            match operator.token_type.as_str() {
                "MINUS" => {
                    if let Value::Number(_, n) = right {
                        return Value::from_number(-n)
                    } else {
                        return Value::Nil
                    }
                },
                "BANG" => return Value::Bool(!self.is_truthy(right)),
                _ => ()
            }
        }
        Value::Nil
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, expr: &Expr) {
        let val = self.evaluate(expr);
        if let Value::Number(_, n) = val {
            println!("{}", n);
        } else {
            println!("{}", val);
        }
    }

    fn evaluate(&self, expr: &Expr) -> Value {
        expr.accept(self)
    }

    fn is_truthy(&self, object: Value) -> bool {
        match object {
            Value::Bool(val) => val,
            Value::Nil => false,
            _ => true,
        }
    }
}
