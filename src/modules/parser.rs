use crate::modules::{expressions::{Expr, Value}, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens, current: 0 }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == "EOF"
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn check(&self, token_type: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn match_type(&mut self, token_types: Vec<&str>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn expression(&mut self) -> Option<Expr> {
        self.factor()
    }

    fn factor(&mut self) -> Option<Expr> {
        let mut current_expr = self.unary();
        while self.match_type(vec!["STAR", "SLASH"]) {
            let first = current_expr.unwrap();
            let operator = self.previous();
            let second = self.unary().unwrap();
            current_expr = Some(Expr::Binary { left: Box::new(first), operator , right: Box::new(second) });
        }
        return current_expr;
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_type(vec!["BANG", "MINUS"]) {
            let operator = self.previous();
            let right = self.unary().unwrap();
            return Some(Expr::Unary { operator, right: Box::new(right) })
        }
        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_type(vec!["FALSE"]) {
            return Some(Expr::Literal(Value::Bool(false)));
        }
        if self.match_type(vec!["TRUE"]) {
            return Some(Expr::Literal(Value::Bool(true)));
        }
        if self.match_type(vec!["NIL"]) {
            return Some(Expr::Literal(Value::Nil));
        }
        if self.match_type(vec!["NUMBER"]) {
            return Some(Expr::Literal(Value::Number(self.previous().literal)))
        }
        if self.match_type(vec!["STRING"]) {
            return Some(Expr::Literal(Value::Str(self.previous().literal)))
        }
        if self.match_type(vec!["LEFT_PAREN"]) {
            let expr = self.expression().unwrap();
            // TODO: add syntax error if it doesnt end with paren
            return Some(Expr::Grouping { expression: Box::new(expr) })
        }
        None
    }

    pub fn parse(&mut self) -> Option<Expr> {
        return self.expression();
    }

}
