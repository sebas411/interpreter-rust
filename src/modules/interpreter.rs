use crate::modules::{expressions::Expr, value::Value, visitor::Visitor};

pub struct Interpreter;

impl Visitor for Interpreter {
    fn visit_binary(&self, expr: &Expr) -> Value {
        if let Expr::Binary { left, operator, right } = expr {
            let left = self.evaluate(left);
            let right = self.evaluate(right);
            if let (Value::Num(n_left), Value::Num(n_right)) = (&left, &right) {
                match operator.token_type.as_str() {
                    "PLUS" => return Value::Num(n_left + n_right),
                    "MINUS" => return Value::Num(n_left - n_right),
                    "STAR" => return Value::Num(n_left * n_right),
                    "SLASH" => return Value::Num(n_left / n_right),
                    "GREATER" => return Value::Bool(n_left > n_right),
                    "GREATER_EQUAL" => return Value::Bool(n_left >= n_right),
                    "LESS" => return Value::Bool(n_left < n_right),
                    "LESS_EQUAL" => return Value::Bool(n_left <= n_right),
                    _ => ()
                }
            } else if let (Value::Str(s_left), Value::Str(s_right)) = (&left, &right) {
                if operator.token_type == "PLUS" {
                    return Value::Str(format!("{}{}", s_left, s_right));
                }
            }

            match operator.token_type.as_str() {
                "BANG_EQUAL" => return Value::Bool(!self.is_equal(left, right)),
                "EQUAL_EQUAL" => return Value::Bool(self.is_equal(left, right)),
                _ => (),
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
                    if let Value::Num(n) = right {
                        return Value::Num(-n)
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
        if let Value::Num(n) = val {
            println!("{}", n.value());
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

    fn is_equal(&self, a: Value, b: Value) -> bool {
        a == b
    }
}
