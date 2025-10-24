use crate::modules::{expressions::{Expr, Value}, token::Token};
use super::super::error_token;
use std::fmt;

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing.")
    }
}

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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut current_expr = self.comparison();
        while self.match_type(vec!["BANG_EQUAL", "EQUAL_EQUAL"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.comparison();
            current_expr = Expr::Binary { left: Box::new(first), operator , right: Box::new(second) };
        }
        return current_expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut current_expr = self.term();
        while self.match_type(vec!["GREATER", "GREATER_EQUAL", "LESS", "LESS_EQUAL"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.term();
            current_expr = Expr::Binary { left: Box::new(first), operator , right: Box::new(second) };
        }
        return current_expr;
    }

    fn term(&mut self) -> Expr {
        let mut current_expr = self.factor();
        while self.match_type(vec!["PLUS", "MINUS"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.factor();
            current_expr = Expr::Binary { left: Box::new(first), operator , right: Box::new(second) };
        }
        return current_expr;
    }

    fn factor(&mut self) -> Expr {
        let mut current_expr = self.unary();
        while self.match_type(vec!["STAR", "SLASH"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.unary();
            current_expr = Expr::Binary { left: Box::new(first), operator , right: Box::new(second) };
        }
        return current_expr;
    }

    fn unary(&mut self) -> Expr {
        if self.match_type(vec!["BANG", "MINUS"]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary { operator, right: Box::new(right) }
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_type(vec!["FALSE"]) {
            return Expr::Literal(Value::Bool(false));
        }
        if self.match_type(vec!["TRUE"]) {
            return Expr::Literal(Value::Bool(true));
        }
        if self.match_type(vec!["NIL"]) {
            return Expr::Literal(Value::Nil);
        }
        if self.match_type(vec!["NUMBER"]) {
            return Expr::Literal(Value::Number(self.previous().literal))
        }
        if self.match_type(vec!["STRING"]) {
            return Expr::Literal(Value::Str(self.previous().literal))
        }
        if self.match_type(vec!["LEFT_PAREN"]) {
            let expr = self.expression();
            // TODO: add syntax error if it doesnt end with paren
            self.consume("RIGHT_PAREN", "Expect ')' after expression.");
            return Expr::Grouping { expression: Box::new(expr) }
        }
        panic!("{}", self.error(self.peek(), "Expect expression."));
    }

    fn consume(&mut self, token_type: &str, message: &str) -> Token {
        if self.check(token_type) {return self.advance()};
        let error = self.error(self.peek(), message);
        panic!("{}", error);
    }

    fn error(&self, token: Token, message: &str) -> ParseError {
        error_token(token, message);
        ParseError
    }

    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

}
