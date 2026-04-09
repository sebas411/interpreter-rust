use std::{collections::HashMap, rc::Rc, sync::RwLock};
use crate::{modules::{callable::{LoxCallable, LoxFunction}, class::LoxClass, environment::Environment, errors::{LoxError, ReturnError, RuntimeError}, expressions::Expr, statements::Stmt, token::Token, value::Value, visitor::{ExprVisitor, StmtVisitor}}, runtime_error};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

pub struct Interpreter {
    environment: Rc<RwLock<Environment>>,
    globals: Rc<RwLock<Environment>>,
}

impl ExprVisitor for Interpreter {
    fn visit_binary(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Binary { left, operator, right } = expr {
            let left = self.evaluate(left)?;
            let right = self.evaluate(right)?;

            match operator.token_type.as_str() {
                "BANG_EQUAL" => return Ok(Value::Bool(!self.is_equal(left, right))),
                "EQUAL_EQUAL" => return Ok(Value::Bool(self.is_equal(left, right))),
                "PLUS" => {
                    if !(left.is_number() && right.is_number() || left.is_string() && right.is_string()) {
                        return Err(Box::new(RuntimeError::new(Some(operator.clone()), "Operands must be two numbers or two strings.")))
                    }
                }
                _ => (),
            }

            if let (Value::Num(n_left), Value::Num(n_right)) = (&left, &right) {
                match operator.token_type.as_str() {
                    "PLUS" => return Ok(Value::Num(n_left + n_right)),
                    "MINUS" => return Ok(Value::Num(n_left - n_right)),
                    "STAR" => return Ok(Value::Num(n_left * n_right)),
                    "SLASH" => return Ok(Value::Num(n_left / n_right)),
                    "GREATER" => return Ok(Value::Bool(n_left > n_right)),
                    "GREATER_EQUAL" => return Ok(Value::Bool(n_left >= n_right)),
                    "LESS" => return Ok(Value::Bool(n_left < n_right)),
                    "LESS_EQUAL" => return Ok(Value::Bool(n_left <= n_right)),
                    _ => ()
                }
            } else if let (Value::Str(s_left), Value::Str(s_right)) = (&left, &right) {
                if operator.token_type == "PLUS" {
                    return Ok(Value::Str(format!("{}{}", s_left, s_right)));
                }
            }
            return Err(Box::new(RuntimeError::new(Some(operator.clone()), "Operands must be numbers.")))
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error")))
    }
    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Grouping { expression } = expr {
            return Ok(self.evaluate(expression)?)
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error")))
    }
    fn visit_literal(&self, expr: &Expr) -> Result<Value> {
        if let Expr::Literal(value) = expr {
            return Ok(value.clone())
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error")))
    }
    fn visit_unary(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Unary { operator, right } = expr {
            let right = self.evaluate(&right)?;
            match operator.token_type.as_str() {
                "MINUS" => {
                    if let Value::Num(n) = right {
                        return Ok(Value::Num(-n))
                    } else {
                        let my_error = RuntimeError::new(Some(operator.clone()), "Operand must be a number.");
                        return Err(Box::new(my_error))
                    }
                },
                "BANG" => return Ok(Value::Bool(!self.is_truthy(&right))),
                _ => ()
            }
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error")))
    }
    fn visit_variable(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Variable {name, distance} = expr {
            return self.lookup_variable(name, *distance);
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error")))
    }
    fn visit_assign(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Assign { name, value, distance } = expr {
            let value = self.evaluate(value)?;

            match distance {
                Some(distance) => {
                    self.environment.write().unwrap().assign_at(*distance, name, &value)?
                }
                None => {
                    self.globals.write().unwrap().assign(name, &value)?;
                }
            }
            return Ok(value)
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error")))
    }
    fn visit_logical(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Logical { left, operator, right } = expr {
            let left = self.evaluate(left)?;
            if operator.token_type == "OR" && self.is_truthy(&left) || operator.token_type == "AND" && !self.is_truthy(&left) {
                return Ok(left)
            } else {
                let right = self.evaluate(right)?;
                return Ok(right)
            }
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error")))
    }
    fn visit_call(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Call { callee, paren, arguments } = expr {
            let callee = self.evaluate(callee)?;

            let arguments = arguments.iter().map(|argument| self.evaluate(argument)).collect::<Result<Vec<Value>>>()?;

            let function = self.get_callable(&callee, paren)?;
            if arguments.len() != function.arity() {
                let err = RuntimeError::new(Some(paren.clone()), &format!("Expected {} arguments but got {}.", function.arity(), arguments.len()));
                return Err(Box::new(err))
            }
            return function.call(self, arguments);
        }
        Err(Box::new(RuntimeError::new(None, "Can only call functions and classes.")))
    }
    fn visit_get(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Get { object, name } = expr {
            let object = self.evaluate(object)?;
            if let Value::Instance(instance) = object {
                return instance.read().unwrap().get(name);
            }
            return Err(Box::new(RuntimeError::new(Some(name.clone()), "Only instances have properties.")))
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error.")))
    }
    fn visit_set(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Set { object, name, value } = expr {
            let object = self.evaluate(object)?;

            if let Value::Instance(instance) = object {
                let value = self.evaluate(value)?;
                instance.write().unwrap().set(&name, value.clone());
                return Ok(value)
            } else {
                return Err(Box::new(RuntimeError::new(Some(name.clone()), "Only instances have fields.")))
            }
        }
        Err(Box::new(RuntimeError::new(None, "Unknown runtime error.")))
    }
}

impl StmtVisitor for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Expression(expr) = stmt {
            self.evaluate(expr)?;
        }
        Ok(())
    }
    fn visit_print(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Print(expr) = stmt {
            let val = self.evaluate(expr)?;
            if let Value::Num(n) = val {
                println!("{}", n.value());
            } else {
                println!("{}", val);
            }
        }
        Ok(())
    }
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Var { name, initializer } = stmt {
            let mut value = Value::Nil;
            if let Some(init) = initializer {
                value = self.evaluate(init)?;
            }
            self.environment.write().unwrap().define(&name.lexeme, &value);
        }
        Ok(())
    }
    fn visit_block(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Block { statements } = stmt {
            self.execute_block(statements.to_vec(), Rc::new(RwLock::new(Environment::new_with_enclosing(self.environment.clone()))))?;
        }
        Ok(())
    }
    fn visit_if_statement(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::If { condition, then_branch, else_branch } = stmt {
            let conditional_value = self.evaluate(condition)?;
            if self.is_truthy(&conditional_value) {
                self.execute(then_branch)?;
            } else if let Some(else_branch) = else_branch {
                self.execute(else_branch)?;
            }
        }
        Ok(())
    }
    fn visit_while_statement(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::While { condition, body } = stmt {
            let mut conditional_value = self.evaluate(condition)?;
            while self.is_truthy(&conditional_value) {
                self.execute(body)?;
                conditional_value = self.evaluate(condition)?;
            }
        }
        Ok(())
    }
    fn visit_function_statement(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Function { name, .. } = stmt {
            let function = LoxFunction::new(stmt, self.environment.clone());
            let function_value = Value::Function(Rc::new(function));
            self.environment.write().unwrap().define(&name.lexeme, &function_value);
        }
        Ok(())
    }
    fn visit_return_statement(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Return { keyword: _, value } = stmt {
            let mut return_value = Value::Nil;
            if let Some(value) = value {
                return_value = self.evaluate(value)?;
            }
            let return_error = ReturnError::new(return_value);
            return Err(Box::new(return_error))
        }
        Ok(())
    }
    fn visit_class(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Class { name, methods } = stmt {
            self.environment.write().unwrap().define(&name.lexeme, &Value::Nil);

            let mut klass_methods = HashMap::new();
            for method in methods {
                if let Stmt::Function { name, .. } = &method {
                    let function = LoxFunction::new(method, self.environment.clone());
                    klass_methods.insert(name.lexeme.clone(), function);
                }
            }
            let klass = LoxClass::new(&name.lexeme, klass_methods);
            self.environment.write().unwrap().assign(name, &Value::Class(klass))?;
        }
        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RwLock::new(Environment::new_globals()));
        Self { environment: globals.clone(), globals }
    }

    fn get_callable(&self, value: &Value, paren: &Token) -> Result<Rc<dyn LoxCallable>> {
        match value {
            Value::Function(function) => {
                Ok(function.clone())
            },
            Value::Class(klass) => {
                Ok(Rc::new(klass.clone()))
            },
            _ => Err(Box::new(RuntimeError::new(Some(paren.clone()), "Can only call functions and classes.")))
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            let res = self.execute(&statement);
            if let Err(e) = res {
                runtime_error(e);
                break;
            }
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }

    pub fn evaluate_expression(&mut self, expr: &Expr) {
        let val = self.evaluate(expr);
        if let Err(e) = val {
            runtime_error(e);
            return;
        }
        let val = val.unwrap_or(Value::Nil);
        if let Value::Num(n) = val {
            println!("{}", n.value());
        } else {
            println!("{}", val);
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
        expr.accept(self)
    }

    pub fn execute_block(&mut self, statements: Vec<Stmt>, environment: Rc<RwLock<Environment>>) -> Result<()> {
        let previous = self.environment.clone();

        self.environment = environment;

        for statement in statements {
            if let Err(e) = self.execute(&statement) {
                self.environment = previous;
                return Err(e)
            }
        }

        self.environment = previous;
        Ok(())
    }

    fn is_truthy(&self, object: &Value) -> bool {
        match object {
            &Value::Bool(val) => val,
            &Value::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, a: Value, b: Value) -> bool {
        a == b
    }

    fn lookup_variable(&self, name: &Token, distance: Option<usize>) -> Result<Value> {
        match distance {
            Some(distance) => {
                self.environment.read().unwrap().get_at(distance, &name)
            }
            None => {
                self.globals.read().unwrap().get(name)
            }
        }
    }

}
