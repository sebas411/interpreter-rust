use crate::modules::expressions::Expr;

pub trait Visitor {
    fn visit_binary(&self, expr: &Expr) -> String;
    fn visit_unary(&self, expr: &Expr) -> String;
    fn visit_literal(&self, expr: &Expr) -> String;
    fn visit_grouping(&self, expr: &Expr) -> String;
}

pub struct AstPrinter;

impl Visitor for AstPrinter {
    fn visit_binary(&self, expr: &Expr) -> String {
        if let Expr::Binary { left, operator, right } = expr {
            let right = *right.clone();
            let left = *left.clone();
            return self.parenthesize(&operator.lexeme, vec![&left, &right]);
        }
        "".into()
    }
    fn visit_unary(&self, expr: &Expr) -> String {
        if let Expr::Unary { operator, right } = expr {
            let expr = *right.clone();
            return self.parenthesize(&operator.lexeme, vec![&expr])
        }
        "".into()
    }
    fn visit_literal(&self, expr: &Expr) -> String {
        if let Expr::Literal(value) = expr {
            return format!("{}", value);
        }
        "".into()
    }
    fn visit_grouping(&self, expr: &Expr) -> String {
        if let Expr::Grouping { expression } = expr {
            let expr = *expression.clone();
            return self.parenthesize("group", vec![&expr]);
        }
        "".into()
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
            parenthesized_str.push_str(&expr.accept(self));
        }
        parenthesized_str.push(')');
        parenthesized_str
    }
}