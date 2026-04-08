use core::fmt;
use crate::modules::{callable::LoxCallable, errors::LoxError, interpreter::Interpreter, value::Value};

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
        Ok(Value::Instance(instance))
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

#[derive(Debug, Clone, Hash)]
pub struct LoxInstance {
    klass: LoxClass,
}

impl LoxInstance {
    fn new(klass: &LoxClass) -> Self {
        Self { klass: klass.clone() }
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.klass.name)
    }
}