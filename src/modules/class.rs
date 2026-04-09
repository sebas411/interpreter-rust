use core::fmt;
use std::{collections::HashMap, rc::Rc, sync::RwLock};
use crate::modules::{callable::LoxCallable, errors::{LoxError, RuntimeError}, interpreter::Interpreter, token::Token, value::Value};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Debug, Clone, Hash)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> Result<Value> {
        let instance = LoxInstance::new(self);
        Ok(Value::Instance(Rc::new(RwLock::new(instance))))
    }
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    klass: LoxClass,
    fields: HashMap<String, Value>,
}

impl LoxInstance {
    fn new(klass: &LoxClass) -> Self {
        Self { klass: klass.clone(), fields: HashMap::new() }
    }
    pub fn get(&self, name: &Token) -> Result<Value> {
        if let Some(value) = self.fields.get(&name.lexeme) {
            return Ok(value.clone())
        }
        Err(Box::new(RuntimeError::new(Some(name.clone()), &format!{"Undefined property '{}'.", name.lexeme})))
    }
    pub fn set(&mut self, name: &Token, value: Value) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.klass.name)
    }
}