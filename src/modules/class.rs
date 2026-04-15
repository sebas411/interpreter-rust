use core::fmt;
use std::{collections::HashMap, rc::Rc, sync::RwLock};
use crate::modules::{callable::{LoxCallable, LoxFunction}, errors::{LoxError, RuntimeError}, interpreter::Interpreter, token::Token, value::Value};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LoxFunction>,
    superclass: Box<Option<Self>>,
}

impl LoxClass {
    pub fn new(name: &str, methods: HashMap<String, LoxFunction>, superclass: Option<Self>) -> Self {
        Self { name: name.to_string(), methods, superclass: Box::new(superclass) }
    }
    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(superclass) = self.superclass.as_ref() {
            return superclass.find_method(name);
        }

        None
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            return initializer.arity()
        }
        0
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
        let instance = Rc::new(RwLock::new(LoxInstance::new(self)));
        if let Some(initializer) = self.find_method("init") {
            initializer.bind(instance.clone()).call(interpreter, arguments)?;
        }
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

#[derive(Debug, Clone)]
pub struct LoxInstance {
    klass: LoxClass,
    fields: HashMap<String, Value>,
}

impl LoxInstance {
    fn new(klass: &LoxClass) -> Self {
        Self { klass: klass.clone(), fields: HashMap::new() }
    }
    pub fn get(&self, name: &Token, self_rwlock: Rc<RwLock<Self>>) -> Result<Value> {
        if let Some(value) = self.fields.get(&name.lexeme) {
            return Ok(value.clone())
        }

        if let Some(method) = self.klass.find_method(&name.lexeme) {
            return Ok(Value::Function(Rc::new(method.bind(self_rwlock))));
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