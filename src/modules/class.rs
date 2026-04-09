use core::fmt;
use std::{collections::HashMap, rc::Rc, sync::RwLock};
use crate::modules::{callable::{LoxCallable, LoxFunction}, errors::{LoxError, RuntimeError}, interpreter::Interpreter, token::Token, value::Value};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: &str, methods: HashMap<String, LoxFunction>) -> Self {
        Self { name: name.to_string(), methods }
    }
    fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }
        None
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

        if let Some(value) = self.klass.find_method(&name.lexeme) {
            return Ok(Value::Function(Rc::new(value)));
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