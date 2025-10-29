use crate::modules::{callable::{ClockFunction, LoxCallable}, errors::{LoxError, RuntimeError}, token::Token, value::Value};
use std::{collections::HashMap, rc::Rc};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    functions: HashMap<String, Rc<dyn LoxCallable>>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut functions_map: HashMap<String, Rc<dyn LoxCallable>> = HashMap::new();
        functions_map.insert("clock".into(), Rc::new(ClockFunction::new()));
        let mut values_map: HashMap<String, Value> = HashMap::new();
        values_map.insert("clock".into(), Value::FunctionName("clock".into()));
        Self { values: values_map, functions: functions_map, enclosing: None }
    }

    pub fn new_with_enclosing(env: &Environment) -> Self {
        Self { values: HashMap::new(), functions: HashMap::new(), enclosing: Some(Box::new(env.clone())) }
    }

    pub fn define(&mut self, name: &str, value: &Value) {
        self.values.insert(name.into(), value.clone());
    }

    pub fn define_function(&mut self, name: &str, value: Rc<dyn LoxCallable>) {
        self.functions.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(())
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value)
        }

        Err(Box::new(RuntimeError::new(Some(name.clone()), &format!("Undefined variable '{}'.", name.lexeme))))
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone())
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name)
        }
        let err = RuntimeError::new(Some(name.clone()), &format!("Undefined variable '{}'.", name.lexeme));
        Err(Box::new(err))
    }

    pub fn get_function(&self, name: &str) -> Result<Rc<dyn LoxCallable>> {
        if self.functions.contains_key(name) {
            return Ok(self.functions.get(name).unwrap().clone())
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get_function(name)
        }
        let err = RuntimeError::new(None, &format!("Undefined variable '{}'.", name));
        Err(Box::new(err))
    }

    pub fn get_parent(&self) -> Environment {
        if let Some(env) = &self.enclosing {
            return *env.clone();
        }
        return Environment::new();
    }
}
