use crate::modules::{errors::{LoxError, RuntimeError}, token::Token, value::Value};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone())
        }
        let err = RuntimeError::new(Some(name.clone()), &format!("Undefined variable '{}'.", name.lexeme));
        Err(Box::new(err))
    }
}
