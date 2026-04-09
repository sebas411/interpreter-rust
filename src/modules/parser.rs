use crate::modules::{errors::ParseError, expressions::Expr, statements::Stmt, token::Token, value::Value};
use super::super::error_token;

type Result<T> = std::result::Result<T, ParseError>;

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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == "SEMICOLON" {return}

            match self.peek().token_type.as_str() {
                "CLASS" | "FUN" | "VAR" | "FOR" | "IF" |
                "WHILE" | "PRINT" | "RETURN" => return,
                _ => ()
            }

            self.advance();
        }
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

    // Expression generators

    fn expression(&mut self) -> Result<Box<Expr>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>> {
        let expr = self.logic_or()?;
        if self.match_type(vec!["EQUAL"]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable { name, distance: _ } = expr.as_ref() {
                return Ok(Box::new(Expr::Assign { name: name.clone(), value, distance: None }))
            } else if let Expr::Get { object, name } = expr.as_ref() {
                return Ok(Box::new(Expr::Set { object: object.clone(), name: name.clone(), value }))
            }
            self.error(equals, "Invalid assignment target.");
        }
        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Box<Expr>> {
        let expr = self.logic_and()?;
        if self.match_type(vec!["OR"]) {
            let or_token = self.previous();
            let left = expr;
            let right = self.logic_or()?;
            return Ok(Box::new(Expr::Logical { left: left, operator: or_token, right: right }))
        }
        Ok(expr)
    }

    fn logic_and(&mut self) -> Result<Box<Expr>> {
        let expr = self.equality()?;
        if self.match_type(vec!["AND"]) {
            let and_token = self.previous();
            let left = expr;
            let right = self.logic_and()?;
            return Ok(Box::new(Expr::Logical { left, operator: and_token, right }))
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>> {
        let mut current_expr = self.comparison()?;
        while self.match_type(vec!["BANG_EQUAL", "EQUAL_EQUAL"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.comparison()?;
            current_expr = Box::new(Expr::Binary { left: first, operator , right: second });
        }
        return Ok(current_expr);
    }

    fn comparison(&mut self) -> Result<Box<Expr>> {
        let mut current_expr = self.term()?;
        while self.match_type(vec!["GREATER", "GREATER_EQUAL", "LESS", "LESS_EQUAL"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.term()?;
            current_expr = Box::new(Expr::Binary { left: first, operator , right: second });
        }
        return Ok(current_expr);
    }

    fn term(&mut self) -> Result<Box<Expr>> {
        let mut current_expr = self.factor()?;
        while self.match_type(vec!["PLUS", "MINUS"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.factor()?;
            current_expr = Box::new(Expr::Binary { left: first, operator , right: second });
        }
        return Ok(current_expr);
    }

    fn factor(&mut self) -> Result<Box<Expr>> {
        let mut current_expr = self.unary()?;
        while self.match_type(vec!["STAR", "SLASH"]) {
            let first = current_expr;
            let operator = self.previous();
            let second = self.unary()?;
            current_expr = Box::new(Expr::Binary { left: first, operator , right: second });
        }
        return Ok(current_expr);
    }

    fn unary(&mut self) -> Result<Box<Expr>> {
        if self.match_type(vec!["BANG", "MINUS"]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary { operator, right: right }))
        }
        self.call()
    }

    fn call(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.primary()?;

        loop {
            if self.match_type(vec!["LEFT_PAREN"]) {
                expr = self.finish_call(expr)?;
            } else if self.match_type(vec!["DOT"]) {
                let name = self.consume("IDENTIFIER", "Expect property name after '.'.")?;
                expr = Box::new(Expr::Get { object: expr, name });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> Result<Box<Expr>> {
        let mut arguments = vec![];
        if !self.check("RIGHT_PAREN") {
            arguments.push(self.expression()?);
            while self.match_type(vec!["COMMA"]) {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
            }
        }
        let paren = self.consume("RIGHT_PAREN", "Expect ')' after arguments.")?;
        Ok(Box::new(Expr::Call { callee: callee, paren, arguments }))
    }

    fn primary(&mut self) -> Result<Box<Expr>> {
        if self.match_type(vec!["FALSE"]) {
            return Ok(Box::new(Expr::Literal(Value::Bool(false))));
        }
        if self.match_type(vec!["TRUE"]) {
            return Ok(Box::new(Expr::Literal(Value::Bool(true))));
        }
        if self.match_type(vec!["NIL"]) {
            return Ok(Box::new(Expr::Literal(Value::Nil)));
        }
        if self.match_type(vec!["NUMBER"]) {
            return Ok(Box::new(Expr::Literal(Value::from_number_and_literal(self.previous().literal.parse().unwrap(), &self.previous().literal))))
        }
        if self.match_type(vec!["STRING"]) {
            return Ok(Box::new(Expr::Literal(Value::Str(self.previous().literal))))
        }
        if self.match_type(vec!["IDENTIFIER"]) {
            return Ok(Box::new(Expr::Variable { name: self.previous(), distance: None }))
        }
        if self.match_type(vec!["LEFT_PAREN"]) {
            let expr = self.expression()?;
            self.consume("RIGHT_PAREN", "Expect ')' after expression.")?;
            return Ok(Box::new(Expr::Grouping { expression: expr }))
        }
        Err(self.error(self.peek(), "Expect expression."))
    }

    // End expression generators

    // Statement generators

    fn declaration(&mut self) -> Result<Stmt> {
        if self.match_type(vec!["VAR"]) {
            return self.var_declaration()
        }
        if self.match_type(vec!["CLASS"]) {
            return self.class_declaration()
        }
        if self.match_type(vec!["FUN"]) {
            return self.function("function")
        }
        self.statement()
    }

    fn class_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume("IDENTIFIER", "Expect class name.")?;
        self.consume("LEFT_BRACE", "Expect '{' before class body.")?;

        let mut methods = vec![];
        while !self.check("RIGHT_BRACE") && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume("RIGHT_BRACE", "Expect '}' after class body.")?;
        Ok(Stmt::Class { name, methods })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt> {
        let name = self.consume("IDENTIFIER", &format!("Expect {} name.", kind))?;
        self.consume("LEFT_PAREN", &format!("Expect '(' after {} name.", kind))?;
        let mut parameters = vec![];
        if !self.check("RIGHT_PAREN") {
            parameters.push(self.consume("IDENTIFIER", "Expect parameter name.")?);
            while self.match_type(vec!["COMMA"]) {
                if parameters.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 parameters.");
                }
                parameters.push(self.consume("IDENTIFIER", "Expect parameter name.")?);
            }
        }
        self.consume("RIGHT_PAREN", "Expect ')' after parameters")?;
        self.consume("LEFT_BRACE", &format!("Expect '{{' before {} body.", kind))?;
        let body = self.block()?;
        Ok(Stmt::Function { name, params: parameters, body })
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume("IDENTIFIER", "Expect variable name.")?;

        let mut initializer = None;
        if self.match_type(vec!["EQUAL"]) {
            initializer = Some(self.expression()?);
        }

        self.consume("SEMICOLON", "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name: name, initializer: initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_type(vec!["PRINT"]) {
            return self.print_statement()
        }
        if self.match_type(vec!["LEFT_BRACE"]) {
            return Ok(Stmt::Block { statements: self.block()? })
        }
        if self.match_type(vec!["IF"]) {
            return self.if_statement()
        }
        if self.match_type(vec!["RETURN"]) {
            return self.return_statement()
        }
        if self.match_type(vec!["WHILE"]) {
            return self.while_statement()
        }
        if self.match_type(vec!["FOR"]) {
            return self.for_statement()
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let keyword = self.previous();
        let mut value = None;
        if !self.check("SEMICOLON") {
            value = Some(self.expression()?);
        }
        self.consume("SEMICOLON", "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, value })
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume("LEFT_PAREN", "Expected '(' after 'for'.")?;

        let initializer;
        if self.match_type(vec!["VAR"]) {
            initializer = Some(self.var_declaration()?);
        } else if self.match_type(vec!["SEMICOLON"]) {
            initializer = None;
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let condition;
        if self.check("SEMICOLON") {
            condition = None;
        } else {
            condition = Some(self.expression()?);
        }
        self.consume("SEMICOLON", "Expected ';' after loop condition.")?;

        let increment;
        if self.check("RIGHT_PAREN") {
            increment = None;
        } else {
            increment = Some(self.expression()?);
        }
        self.consume("RIGHT_PAREN", "Expected ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            let increment_statement = Stmt::Expression(increment);
            body = Stmt::Block { statements: vec![body, increment_statement ] };
        }

        let mut while_condition = Box::new(Expr::Literal(Value::Bool(true)));
        if let Some(condition) = condition {
            while_condition = condition;
        }
        body = Stmt::While { condition: while_condition, body: Box::new(body) };

        if let Some(initializer) = initializer {
            body = Stmt::Block { statements: vec![initializer, body] };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume("LEFT_PAREN", "Expected '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume("RIGHT_PAREN", "Expected ')' after condition.")?;
        let statement = self.statement()?;
        Ok(Stmt::While { condition, body: Box::new(statement) })
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume("LEFT_PAREN", "Expected '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume("RIGHT_PAREN", "Expected ')' after if condition.")?;
        let then_statement = self.statement()?;
        let mut else_statement = None;
        if self.match_type(vec!["ELSE"]) {
            else_statement = Some(Box::new(self.statement()?))
        }

        Ok(Stmt::If { condition, then_branch: Box::new(then_statement), else_branch: else_statement })
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.check("RIGHT_BRACE") && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume("RIGHT_BRACE", "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume("SEMICOLON", "Expect ';' after value.")?;
        Ok(Stmt::Expression(value))
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume("SEMICOLON", "Expect ';' after expression.")?;
        Ok(Stmt::Print(expr))
    }

    // End statement generators

    fn consume(&mut self, token_type: &str, message: &str) -> Result<Token> {
        if self.check(token_type) {return Ok(self.advance())};
        Err(self.error(self.peek(), message))
    }

    fn error(&self, token: Token, message: &str) -> ParseError {
        error_token(token, message);
        ParseError
    }

    pub fn parse_expr(&mut self) -> Option<Box<Expr>> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            let res = self.declaration();
            if res.is_ok() {
                statements.push(res.unwrap());
            } else {
                self.synchronize();
                return None
            }
        }
        Some(statements)
    }

}
