use crate::modules::{callable::ClockFunction, errors::{LoxError, RuntimeError}, token::Token, value::Value};
use std::{collections::HashMap, rc::Rc, sync::RwLock};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RwLock<Environment>>>,
}

impl Environment {
    pub fn new_globals() -> Self {
        let mut values_map: HashMap<String, Value> = HashMap::new();
        let function_value = Value::Function(Rc::new(ClockFunction::new()));
        values_map.insert("clock".into(), function_value);
        Self { values: values_map, enclosing: None }
    }

    pub fn new() -> Self {
        Self { values: HashMap::new(), enclosing: None }
    }

    pub fn new_with_enclosing(env: Rc<RwLock<Environment>>) -> Self {
        Self { values: HashMap::new(), enclosing: Some(env) }
    }

    pub fn define(&mut self, name: &str, value: &Value) {
        self.values.insert(name.into(), value.clone());
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(())
        }
        Err(Box::new(RuntimeError::new(Some(name.clone()), &format!("Undefined variable '{}'. Line {}", name.lexeme, name.line))))
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone())
        }
        Err(Box::new(RuntimeError::new(Some(name.clone()), &format!("Undefined variable '{}'. Line {}", name.lexeme, name.line))))
    }

    pub fn get_enclosing(&self) -> Option<Rc<RwLock<Self>>> {
        self.enclosing.clone()
    }

    pub fn contains(&self, key: &str) -> bool {
        if self.values.contains_key(key) {
            true
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.read().unwrap().contains(key)
        } else {
            false
        }
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Value> {
        if distance == 0 {
            self.get(name)
        } else {
            self.ancestor(distance).unwrap().read().unwrap().get(name)
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: &Value) -> Result<()> {
        match distance {
            0 => self.assign(name, value),
            _ => self.ancestor(distance).unwrap().write().unwrap().assign(name, value),
        }
    }

    fn ancestor(&self, distance: usize) -> Option<Rc<RwLock<Environment>>> {
        if distance == 0 {
            return None
        }
        let mut environment = self.enclosing.clone()?;
        for _ in 1..distance {
            let temp = environment.read().unwrap().enclosing.clone()?;
            environment = temp;
        }
        Some(environment)
    }
}
