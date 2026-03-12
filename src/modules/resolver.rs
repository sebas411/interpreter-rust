use std::collections::HashMap;
use crate::{error, modules::{errors::{LoxError, ResolverError}, expressions::Expr, statements::Stmt, token::Token}};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Debug, Clone)]
enum FunctionType {
    NONE,
    FUNCTION,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl Resolver {
    pub fn new() -> Self {
        Self { scopes: Vec::new(), current_function: FunctionType::NONE }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn resolve_multi(&mut self, statements: &mut Vec<Stmt>) -> Result<()> {
        for statement in statements {
            self.resolve(statement)?;
        }
        Ok(())
    }

    fn resolve(&mut self, stmt: &mut Stmt) -> Result<()> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_multi(statements)?;
                self.end_scope();
            }
            Stmt::Expression(expr) => {
                self.resolve_expr(expr)?;
            }
            Stmt::Function { name, ..} => {
                self.declare(name);
                self.define(name);
    
                self.resolve_function(stmt, FunctionType::FUNCTION)?;
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.resolve_expr(condition)?;
                self.resolve(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve(else_branch)?;
                }
            }
            Stmt::Print(expr) => {
                self.resolve_expr(expr)?;
            }
            Stmt::Return { keyword, value } => {
                if let FunctionType::NONE = self.current_function {
                    error(keyword.line, "Can't return from top-level code.");
                }

                if let Some(value) = value {
                    self.resolve_expr(value)?;
                }
            }
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(initializer) = initializer {
                    self.resolve_expr(initializer)?;
                }
                self.define(name);
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve(body)?;
            }
        }
        Ok(())
    }

    fn resolve_function(&mut self, stmt: &mut Stmt, function_type: FunctionType) -> Result<()> {
        if let Stmt::Function { name: _, params, body } = stmt {
            let enclosing_function = self.current_function.clone();
            self.current_function = function_type;
            self.begin_scope();
            for param in params {
                self.declare(param);
                self.define(param);
            }
            self.resolve_multi(body)?;
            self.end_scope();
            self.current_function = enclosing_function;
            Ok(())
        } else {
            Err(Box::new(ResolverError::new()))
        }
    }

    fn resolve_expr(&mut self, expr: &mut Expr) -> Result<()> {
        match expr {
            Expr::Assign { name, value, distance } => {
                self.resolve_expr(value)?;
                self.resolve_local(distance, name);
            }
            Expr::Binary { left, operator: _, right } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Call { callee, paren: _, arguments } => {
                self.resolve_expr(callee)?;
                for argument in arguments {
                    self.resolve_expr(argument)?;
                }
            }
            Expr::Grouping { expression } => {
                self.resolve_expr(expression)?;
            }
            Expr::Literal(_) => (),
            Expr::Logical { left, operator: _, right } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(right)?;
            }
            Expr::Variable { name, distance } => {
                if let Some(scope) = self.scopes.last() &&
                let Some(val) = scope.get(&name.lexeme) && !val {
                        error(name.line, "Can't read local variable in its own initializer.");
                }
                self.resolve_local(distance, name);
            }
        }
        Ok(())
    }

    fn resolve_local(&mut self, distance: &mut Option<usize>, name: &Token) {
        let mut depth = None;
        for i in (0..self.scopes.len()).rev() {
            if let Some(scope) = self.scopes.get(i) && scope.contains_key(&name.lexeme) {
                depth = Some(self.scopes.len() - i - 1);
                break;
            }
        }
        *distance = depth;
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                error(name.line, "Already a variable with this name in this scope.");
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }
}
