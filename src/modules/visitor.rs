use crate::modules::{expressions::Expr, value::Value};

pub trait Visitor {
    fn visit_binary(&self, expr: &Expr) -> Value;
    fn visit_unary(&self, expr: &Expr) -> Value;
    fn visit_literal(&self, expr: &Expr) -> Value;
    fn visit_grouping(&self, expr: &Expr) -> Value;
}

pub struct AstPrinter;

impl Visitor for AstPrinter {
    fn visit_binary(&self, expr: &Expr) -> Value {
        if let Expr::Binary { left, operator, right } = expr {
            let right = *right.clone();
            let left = *left.clone();
            return Value::Str(self.parenthesize(&operator.lexeme, vec![&left, &right]));
        }
        Value::Str("".into())
    }
    fn visit_unary(&self, expr: &Expr) -> Value {
        if let Expr::Unary { operator, right } = expr {
            let expr = *right.clone();
            return Value::Str(self.parenthesize(&operator.lexeme, vec![&expr]))
        }
        Value::Str("".into())
    }
    fn visit_literal(&self, expr: &Expr) -> Value {
        if let Expr::Literal(value) = expr {
            return Value::Str(format!("{}", value));
        }
        Value::Str("".into())
    }
    fn visit_grouping(&self, expr: &Expr) -> Value {
        if let Expr::Grouping { expression } = expr {
            let expr = *expression.clone();
            return Value::Str(self.parenthesize("group", vec![&expr]));
        }
        Value::Str("".into())
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print_tree(&self, expr: &Expr) {
        let printable = expr.accept(self);
        println!("{}", printable);
    }
    fn parenthesize(&self, name: &str, expressions: Vec<&Expr>) -> String {
        let mut parenthesized_str = String::new();
        parenthesized_str.push_str(&format!("({}", name));
        for expr in expressions {
            parenthesized_str.push(' ');
            if let Value::Str(my_val) = expr.accept(self) {
                parenthesized_str.push_str(&my_val);
            } else {
                panic!()
            }
        }
        parenthesized_str.push(')');
        parenthesized_str
    }
}