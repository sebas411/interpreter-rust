use std::{fmt::Debug, rc::Rc, sync::RwLock};

use crate::modules::{environment::Environment, errors::LoxError, interpreter::Interpreter, statements::Stmt, token::Token, value::Value};
use chrono::prelude::Utc;

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

pub trait LoxCallable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value>;
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    body: Vec<Stmt>,
    params: Vec<Token>,
    name: Token,
    closure: Rc<RwLock<Environment>>,
}

impl LoxFunction {
    pub fn new(declaration: &Stmt, closure: Rc<RwLock<Environment>>) -> Self {
        if let Stmt::Function { name, params, body } = declaration.clone() {
            return Self { body, params, name, closure: closure }
        }
        Self { body: vec![], params: vec![], name: Token { token_type: "IDENTIFIER".into(), lexeme: "".into(), literal: "".into(), line: 0 }, closure: Rc::new(RwLock::new(Environment::new())) }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
        //set environment for function
        let environment = Rc::new(RwLock::new(Environment::new_with_enclosing(self.closure.clone())));

        // add arguments
        for i in 0..self.params.len() {
            environment.write().unwrap().define(&self.params[i].lexeme, &arguments[i]);
        }
        // run function
        let result = interpreter.execute_block(self.body.clone(), environment);


        match result {
            Err(e) => {
                if e.error_type() == "ReturnError" {
                    return Ok(e.get_value())
                } else {
                    return Err(e)
                }
            },
            Ok(()) => (),
        }
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
