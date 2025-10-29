use crate::modules::{token::Token, value::Value};
use std::fmt;

pub trait LoxError: fmt::Display {
    fn get_value(&self) -> Value;
    fn error_type(&self) -> String;
}


#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing.")
    }
}

impl LoxError for ParseError {
    fn get_value(&self) -> Value {
        Value::Nil
    }
    fn error_type(&self) -> String {
        "ParseError".into()
    }
}


#[derive(Debug, Clone)]
pub struct RuntimeError {
    token: Option<Token>,
    message: String
}

impl RuntimeError {
    pub fn new(token: Option<Token>, message: &str) -> Self {
        Self { token: token, message: message.into() }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token {
            Some(tok) => write!(f, "{}\n[line {}]", self.message, tok.line),
            None => write!(f, "{}", self.message)
        }
    }
}

impl LoxError for RuntimeError {
    fn get_value(&self) -> Value {
        Value::Nil
    }
    fn error_type(&self) -> String {
        "RuntimeError".into()
    }
}


pub struct PrintError {}

impl LoxError for PrintError {
    fn get_value(&self) -> Value {
        Value::Nil
    }
    fn error_type(&self) -> String {
        "PrintError".into()
    }
}

impl PrintError {
    pub fn new() -> Self { Self {} }
}

impl fmt::Display for PrintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error printing.")
    }
}

pub struct ReturnError {
    value: Value,
}

impl fmt::Display for ReturnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl LoxError for ReturnError {
    fn get_value(&self) -> Value {
        self.value.clone()
    }
    fn error_type(&self) -> String {
        "ReturnError".into()
    }
}

impl ReturnError {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
