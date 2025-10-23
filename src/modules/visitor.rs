use crate::modules::expressions::Expr;

pub trait Visitor {
    fn visit_unary(&self, expr: &Expr);
    fn visit_literal(&self, expr: &Expr);
}

pub struct AstPrinter;

impl Visitor for AstPrinter {
    fn visit_unary(&self, expr: &Expr) {
        if let Expr::Unary { operator, expr } = expr {
            print!("{} ", operator);
            let expr = *expr.clone();
            expr.accept(self);
        }
    }
    fn visit_literal(&self, expr: &Expr) {
        if let Expr::Literal(value) = expr {
            print!("{}", value);
        }
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print_tree(&self, expr: &Expr) {
        expr.accept(self);
        println!();
    }
}