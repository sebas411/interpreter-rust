use std::fmt::Debug;

use crate::modules::{errors::LoxError, interpreter::Interpreter, statements::Stmt, value::Value};
use chrono::prelude::Utc;

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

pub trait LoxCallable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value>;
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct UserFunction {
    arity: usize,
    body: Option<Stmt>,
}

impl UserFunction {
    pub fn new(arity: usize) -> Self {
        Self { arity, body: None }
    }
}

impl LoxCallable for UserFunction {
    fn arity(&self) -> usize {
        self.arity
    }
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value> {
        Ok(Value::Nil)
    }
    fn to_string(&self) -> String {
        "<Custom fn>".into()
    }
}

#[derive(Debug)]
pub struct ClockFunction {}

impl LoxCallable for ClockFunction {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &Interpreter, _arguments: Vec<Value>) -> Result<Value> {
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
