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

    fn primary(&self) -> Option<Expr> {
        if self.check("FALSE") {
            return Some(Expr::Literal(Value::Bool(false)));
        }
        if self.check("TRUE") {
            return Some(Expr::Literal(Value::Bool(true)));
        }
        if self.check("NIL") {
            return Some(Expr::Literal(Value::Nil));
        }
        None
    }

    pub fn parse(&self) -> Option<Expr> {
        return self.primary();
    }

}
