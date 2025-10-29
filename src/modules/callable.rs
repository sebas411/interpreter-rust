use std::fmt::Debug;

use crate::modules::{environment::Environment, errors::LoxError, interpreter::Interpreter, statements::Stmt, token::Token, value::Value};
use chrono::prelude::Utc;

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

pub trait LoxCallable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value>;
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct LoxFunction {
    body: Vec<Stmt>,
    params: Vec<Token>,
    name: Token,
}

impl LoxFunction {
    pub fn new(declaration: &Stmt) -> Self {
        if let Stmt::Function { name, params, body } = declaration.clone() {
            return Self { body, params, name }
        }
        Self { body: vec![], params: vec![], name: Token { token_type: "IDENTIFIER".into(), lexeme: "".into(), literal: "".into(), line: 0 } }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
        let mut environment = Environment::new_with_enclosing(&interpreter.get_global());
        for i in 0..self.params.len() {
            environment.define(&self.params[i].lexeme, &arguments[i]);
        }
        interpreter.execute_block(self.body.clone(), environment)?;
        Ok(Value::Nil)
    }
    fn to_string(&self) -> String {
        format!("<fn {}>", self.name.lexeme)
    }
}

#[derive(Debug)]
pub struct ClockFunction {}

impl LoxCallable for ClockFunction {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Result<Value> {
        Ok(Value::from_number(Utc::now().timestamp() as f64))
    }
    fn to_string(&self) -> String {
        "<native fn>".into()
    }
}

impl ClockFunction {
    pub fn new() -> Self {
        Self {  }
    }
}
